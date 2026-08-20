//! Perennia world-core: deterministic state machine over OWP v0.1 actions.
//!
//! Implements validation stages from spec §9 and the consumable-state model
//! from spec §11.1 for the `asset.transfer` reference profile.

use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use world_crypto::verify;
use world_protocol::{
    hex, parse_asset_transfer, OwnedObject, ProtocolError, SignedAction, ACTION_ASSET_TRANSFER,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorldError {
    Protocol(ProtocolError),
    UnknownActor,
    InvalidSignature,
    InvalidNonce { expected: u64 },
    UnknownObject,
    NotOwner,
    StaleStateRef,
    AlreadyConsumed,
    SelfTransfer,
    UnsupportedActionType,
}

impl From<ProtocolError> for WorldError {
    fn from(e: ProtocolError) -> Self {
        WorldError::Protocol(e)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActorState {
    pub next_nonce: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct WorldState {
    /// actor pubkey -> nonce tracking
    pub actors: BTreeMap<[u8; 32], ActorState>,
    /// object_id -> current OwnedObject (latest version)
    pub objects: BTreeMap<[u8; 32], OwnedObject>,
    /// finalized-consumed state references (spec §11.1): a state_ref may be consumed at most once, ever
    pub consumed: BTreeSet<[u8; 32]>,
    /// applied action ids, in order (audit trail / DAG anchor)
    pub applied: Vec<[u8; 32]>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub action_id: [u8; 32],
    pub object_id: [u8; 32],
    pub from: [u8; 32],
    pub to: [u8; 32],
    pub new_version: u64,
    pub new_state_ref: [u8; 32],
}

impl WorldState {
    pub fn register_actor(&mut self, pubkey: [u8; 32]) {
        self.actors.entry(pubkey).or_insert(ActorState { next_nonce: 1 });
    }

    /// Genesis-mint an object owned by `owner`. object_id derives from a label so tests/sim are deterministic.
    pub fn mint_object(&mut self, owner: [u8; 32], label: &[u8]) -> OwnedObject {
        let mut h = Sha256::new();
        h.update(b"PERENNIA-GENESIS-OBJECT\0");
        h.update(label);
        let object_id: [u8; 32] = h.finalize().into();
        let data_hash: [u8; 32] = Sha256::digest(label).into();
        let obj = OwnedObject { object_id, owner, version: 0, data_hash };
        self.objects.insert(object_id, obj.clone());
        obj
    }

    /// Full validation pipeline (stateless stages A/B + stateful C/D) then state mutation.
    pub fn apply_wire(&mut self, wire: &[u8]) -> Result<Event, WorldError> {
        // Stage A/B: canonical decode + shape (spec §7.3, §5) — done inside from_wire.
        let action = SignedAction::from_wire(wire)?;
        self.apply(&action)
    }

    pub fn apply(&mut self, action: &SignedAction) -> Result<Event, WorldError> {
        let u = &action.unsigned;
        u.validate_shape()?;

        // Stage B: signature over domain-separated message (spec §7.1).
        if !verify(&u.actor, &u.signing_message(), &action.signature) {
            return Err(WorldError::InvalidSignature);
        }

        // Stage C: actor + anti-replay nonce.
        let actor = self.actors.get(&u.actor).ok_or(WorldError::UnknownActor)?;
        if u.nonce != actor.next_nonce {
            return Err(WorldError::InvalidNonce { expected: actor.next_nonce });
        }

        // Stage D: application rule.
        if u.action_type != ACTION_ASSET_TRANSFER {
            return Err(WorldError::UnsupportedActionType);
        }
        let t = parse_asset_transfer(&u.payload)?;
        if t.to == u.actor {
            return Err(WorldError::SelfTransfer);
        }
        if !self.actors.contains_key(&t.to) {
            return Err(WorldError::UnknownActor);
        }
        let obj = self.objects.get(&t.object_id).ok_or(WorldError::UnknownObject)?;
        if obj.owner != u.actor {
            return Err(WorldError::NotOwner);
        }
        // The consumed input must reference the *current* version of the object.
        let current_ref = obj.state_ref();
        if t.version != obj.version || t.state_ref != current_ref {
            return Err(WorldError::StaleStateRef);
        }
        // Anti-double-transfer at the state layer (spec §11.1): each state_ref consumed at most once.
        if self.consumed.contains(&current_ref) {
            return Err(WorldError::AlreadyConsumed);
        }

        // ---- Mutation (only after every check passed) ----
        let action_id = u.action_id();
        self.consumed.insert(current_ref);
        let new_obj = OwnedObject {
            object_id: obj.object_id,
            owner: t.to,
            version: obj.version + 1,
            data_hash: obj.data_hash,
        };
        let new_state_ref = new_obj.state_ref();
        let from = obj.owner;
        self.objects.insert(new_obj.object_id, new_obj.clone());
        self.actors.get_mut(&u.actor).unwrap().next_nonce += 1;
        self.applied.push(action_id);

        Ok(Event {
            action_id,
            object_id: new_obj.object_id,
            from,
            to: t.to,
            new_version: new_obj.version,
            new_state_ref,
        })
    }

    /// Deterministic world hash over canonical CBOR of the whole state.
    pub fn world_hash(&self) -> String {
        let mut h = Sha256::new();
        h.update(b"PERENNIA-WORLD-HASH-v1\0");
        for (pk, a) in &self.actors {
            h.update(pk);
            h.update(a.next_nonce.to_be_bytes());
        }
        for obj in self.objects.values() {
            h.update(obj.canonical_bytes());
        }
        for c in &self.consumed {
            h.update(c);
        }
        for a in &self.applied {
            h.update(a);
        }
        hex(&h.finalize())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use world_crypto::PlayerKeypair;
    use world_protocol::{asset_transfer_payload, ConsensusLevel, UnsignedAction, ACTION_VERSION};

    fn keypair(seed: u8) -> PlayerKeypair {
        PlayerKeypair::from_seed([seed; 32])
    }

    fn transfer(
        keys: &PlayerKeypair,
        obj: &OwnedObject,
        to: [u8; 32],
        nonce: u64,
    ) -> SignedAction {
        let unsigned = UnsignedAction {
            version: ACTION_VERSION,
            actor: keys.public_key_bytes(),
            action_type: ACTION_ASSET_TRANSFER.to_owned(),
            nonce,
            created_at: 1_755_600_000_000,
            refs: vec![obj.state_ref()],
            consensus: ConsensusLevel::None,
            scope: None,
            payload: asset_transfer_payload(obj, to),
        };
        let signature = keys.sign(&unsigned.signing_message());
        SignedAction { unsigned, signature: signature.try_into().unwrap() }
    }

    fn world_two_actors() -> (WorldState, PlayerKeypair, PlayerKeypair, OwnedObject) {
        let a = keypair(1);
        let b = keypair(2);
        let mut w = WorldState::default();
        w.register_actor(a.public_key_bytes());
        w.register_actor(b.public_key_bytes());
        let obj = w.mint_object(a.public_key_bytes(), b"legendary-sword");
        (w, a, b, obj)
    }

    #[test]
    fn valid_transfer_moves_ownership_and_bumps_version() {
        let (mut w, a, b, obj) = world_two_actors();
        let ev = w.apply(&transfer(&a, &obj, b.public_key_bytes(), 1)).unwrap();
        let now = &w.objects[&obj.object_id];
        assert_eq!(now.owner, b.public_key_bytes());
        assert_eq!(now.version, 1);
        assert_eq!(ev.new_version, 1);
        assert!(w.consumed.contains(&obj.state_ref()));
    }

    #[test]
    fn double_consume_of_same_state_ref_is_rejected() {
        let (mut w, a, b, obj) = world_two_actors();
        let c = keypair(3);
        w.register_actor(c.public_key_bytes());
        // Two conflicting transfers consuming the SAME state_ref (classic double-spend).
        let t1 = transfer(&a, &obj, b.public_key_bytes(), 1);
        let t2 = transfer(&a, &obj, c.public_key_bytes(), 2); // fresh nonce, stale input
        w.apply(&t1).unwrap();
        let err = w.apply(&t2).unwrap_err();
        assert!(matches!(err, WorldError::StaleStateRef | WorldError::NotOwner | WorldError::AlreadyConsumed));
        // Ownership stayed with b.
        assert_eq!(w.objects[&obj.object_id].owner, b.public_key_bytes());
    }

    #[test]
    fn forged_signature_is_rejected() {
        let (mut w, a, b, obj) = world_two_actors();
        let mallory = keypair(9);
        let mut act = transfer(&a, &obj, b.public_key_bytes(), 1);
        act.signature = mallory.sign(&act.unsigned.signing_message()).try_into().unwrap();
        assert_eq!(w.apply(&act), Err(WorldError::InvalidSignature));
    }

    #[test]
    fn replayed_action_is_rejected_by_nonce() {
        let (mut w, a, b, obj) = world_two_actors();
        let act = transfer(&a, &obj, b.public_key_bytes(), 1);
        w.apply(&act).unwrap();
        assert_eq!(w.apply(&act), Err(WorldError::InvalidNonce { expected: 2 }));
    }

    #[test]
    fn non_canonical_wire_bytes_are_rejected() {
        let (mut w, a, b, obj) = world_two_actors();
        let act = transfer(&a, &obj, b.public_key_bytes(), 1);
        let mut wire = act.canonical_bytes();
        // Re-encode the nonce (small uint) using a longer-than-necessary form: find and inflate.
        // Simplest guaranteed-noncanonical mutation: append trailing byte.
        wire.push(0x00);
        assert!(matches!(w.apply_wire(&wire), Err(WorldError::Protocol(_))));
    }

    #[test]
    fn wire_roundtrip_preserves_action_and_hash() {
        let (mut w, a, b, obj) = world_two_actors();
        let act = transfer(&a, &obj, b.public_key_bytes(), 1);
        let wire = act.canonical_bytes();
        let decoded = SignedAction::from_wire(&wire).unwrap();
        assert_eq!(decoded, act);
        assert_eq!(decoded.unsigned.action_id(), act.unsigned.action_id());
        w.apply_wire(&wire).unwrap();
    }

    #[test]
    fn identical_states_produce_identical_hashes() {
        let (mut w1, a, b, obj) = world_two_actors();
        let (mut w2, _, _, _) = world_two_actors();
        let act = transfer(&a, &obj, b.public_key_bytes(), 1);
        w1.apply(&act).unwrap();
        w2.apply_wire(&act.canonical_bytes()).unwrap();
        assert_eq!(w1.world_hash(), w2.world_hash());
    }
}

#[cfg(test)]
mod conformance {
    //! Pins the normative test vector (docs/spec/test-vectors/asset-transfer-v1.json).
    //! If canonical encoding, domain separation, or hashing ever drift, this fails.
    use world_crypto::PlayerKeypair;
    use world_protocol::{
        asset_transfer_payload, hex, ConsensusLevel, OwnedObject, UnsignedAction,
        ACTION_ASSET_TRANSFER, ACTION_VERSION,
    };

    #[test]
    fn asset_transfer_v1_vector_is_stable() {
        let alice = PlayerKeypair::from_seed([1; 32]);
        let bob = PlayerKeypair::from_seed([2; 32]);
        let obj = OwnedObject {
            object_id: [0xAA; 32],
            owner: alice.public_key_bytes(),
            version: 0,
            data_hash: [0xBB; 32],
        };
        assert_eq!(
            hex(&obj.state_ref()),
            "9d19402b793c7adaa79de4c9d309dac737b43c97ae625ed70cc1f8347a7007eb"
        );
        let unsigned = UnsignedAction {
            version: ACTION_VERSION,
            actor: alice.public_key_bytes(),
            action_type: ACTION_ASSET_TRANSFER.to_owned(),
            nonce: 1,
            created_at: 1_755_600_000_000,
            refs: vec![obj.state_ref()],
            consensus: ConsensusLevel::None,
            scope: None,
            payload: asset_transfer_payload(&obj, bob.public_key_bytes()),
        };
        assert_eq!(
            hex(&unsigned.action_id()),
            "582ed75d269dd03a7c6e21b49b8ae9e38feb68d5a4b0ab348b9b2c7287f034ab"
        );
    }
}
