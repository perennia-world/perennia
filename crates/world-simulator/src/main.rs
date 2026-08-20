//! Perennia world-simulator: determinism proof for the OWP v0.1 engine.
//!
//! Builds 10,000 signed `asset.transfer` actions (interleaved with double-spend
//! attempts), applies them to 3 independent replicas *from canonical wire bytes*,
//! and asserts every replica converges to the same world hash.

use world_core::{WorldError, WorldState};
use world_crypto::PlayerKeypair;
use world_protocol::{
    asset_transfer_payload, ConsensusLevel, SignedAction, UnsignedAction, ACTION_ASSET_TRANSFER,
    ACTION_VERSION,
};

const ACTORS: usize = 8;
const OBJECTS: usize = 16;
const ACTIONS: usize = 10_000;

fn main() {
    // Deterministic actors and genesis.
    let keys: Vec<PlayerKeypair> = (0..ACTORS as u8).map(|i| PlayerKeypair::from_seed([i + 1; 32])).collect();

    let build_world = || {
        let mut w = WorldState::default();
        for k in &keys {
            w.register_actor(k.public_key_bytes());
        }
        let mut objs = Vec::new();
        for i in 0..OBJECTS {
            let owner = keys[i % ACTORS].public_key_bytes();
            objs.push(w.mint_object(owner, format!("genesis-object-{i}").as_bytes()));
        }
        (w, objs)
    };

    // Reference world used to *generate* valid actions (same rules as replicas).
    let (mut gen_world, objects) = build_world();
    let mut nonces = vec![1u64; ACTORS];
    let mut owner_of: Vec<usize> = (0..OBJECTS).map(|i| i % ACTORS).collect();

    let mut wire_stream: Vec<Vec<u8>> = Vec::with_capacity(ACTIONS);
    let mut expected_rejects = 0usize;
    let mut rng: u64 = 0x5EED_5EED_5EED_5EED;
    let mut next = |m: usize| {
        // xorshift64* — deterministic, no external RNG dependency.
        rng ^= rng << 13;
        rng ^= rng >> 7;
        rng ^= rng << 17;
        (rng as usize) % m
    };

    for step in 0..ACTIONS {
        let obj_idx = next(OBJECTS);
        let from_idx = owner_of[obj_idx];
        let mut to_idx = next(ACTORS);
        if to_idx == from_idx {
            to_idx = (to_idx + 1) % ACTORS;
        }
        let current = gen_world.objects[&objects[obj_idx].object_id].clone();

        // Every 7th action is an adversarial double-spend attempt: it consumes a
        // STALE state_ref (version - 1) and must be rejected identically everywhere.
        let adversarial = step % 7 == 3 && current.version > 0;
        let input = if adversarial {
            let mut stale = current.clone();
            stale.version -= 1;
            // stale.owner may differ; keep current actor as signer to make it a pure state_ref conflict
            stale
        } else {
            current.clone()
        };

        let signer = &keys[from_idx];
        let unsigned = UnsignedAction {
            version: ACTION_VERSION,
            actor: signer.public_key_bytes(),
            action_type: ACTION_ASSET_TRANSFER.to_owned(),
            nonce: nonces[from_idx],
            created_at: 1_755_600_000_000 + step as u64,
            refs: vec![input.state_ref()],
            consensus: ConsensusLevel::None,
            scope: None,
            payload: asset_transfer_payload(&input, keys[to_idx].public_key_bytes()),
        };
        let signature: [u8; 64] = signer.sign(&unsigned.signing_message()).try_into().unwrap();
        let action = SignedAction { unsigned, signature };
        let wire = action.canonical_bytes();

        // Advance the generator world with the same action to know the expected outcome.
        match gen_world.apply_wire(&wire) {
            Ok(_) => {
                nonces[from_idx] += 1;
                owner_of[obj_idx] = to_idx;
            }
            Err(WorldError::StaleStateRef) if adversarial => expected_rejects += 1,
            Err(e) => panic!("generator produced unexpected rejection at step {step}: {e:?}"),
        }
        wire_stream.push(wire);
    }

    // Three independent replicas consume the identical wire stream.
    let mut replicas: Vec<WorldState> = (0..3).map(|_| build_world().0).collect();
    let mut rejects = [0usize; 3];
    for wire in &wire_stream {
        for (i, r) in replicas.iter_mut().enumerate() {
            if r.apply_wire(wire).is_err() {
                rejects[i] += 1;
            }
        }
    }

    let hashes: Vec<String> = replicas.iter().map(|r| r.world_hash()).collect();
    println!("Actions generated : {ACTIONS}");
    println!("Adversarial (stale state_ref) attempts: {expected_rejects}");
    println!("Replica rejects   : {rejects:?}");
    println!("Node A: {}", hashes[0]);
    println!("Node B: {}", hashes[1]);
    println!("Node C: {}", hashes[2]);

    assert_eq!(hashes[0], hashes[1]);
    assert_eq!(hashes[1], hashes[2]);
    assert!(rejects.iter().all(|r| *r == expected_rejects), "replicas disagreed on rejections");
    assert_eq!(gen_world.world_hash(), hashes[0], "generator and replicas diverged");
    println!(
        "PASS: {} actions ({} adversarial) produced identical world hash on 3 independent replicas.",
        ACTIONS, expected_rejects
    );
}
