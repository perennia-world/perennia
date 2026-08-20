//! Generates normative test vectors for docs/spec/test-vectors/ (spec conformance).
use world_crypto::PlayerKeypair;
use world_protocol::{
    asset_transfer_payload, hex, ConsensusLevel, OwnedObject, SignedAction, UnsignedAction,
    ACTION_ASSET_TRANSFER, ACTION_VERSION,
};

fn main() {
    let alice = PlayerKeypair::from_seed([1; 32]);
    let bob = PlayerKeypair::from_seed([2; 32]);
    let obj = OwnedObject {
        object_id: [0xAA; 32],
        owner: alice.public_key_bytes(),
        version: 0,
        data_hash: [0xBB; 32],
    };
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
    let signature: [u8; 64] = alice.sign(&unsigned.signing_message()).try_into().unwrap();
    let signed = SignedAction {
        unsigned: unsigned.clone(),
        signature,
    };

    println!("{{");
    println!("  \"description\": \"OWP v0.1 asset.transfer conformance vector (deterministic seed keys)\",");
    println!("  \"alice_seed\": \"{}\",", hex(&[1u8; 32]));
    println!(
        "  \"alice_pubkey\": \"{}\",",
        hex(&alice.public_key_bytes())
    );
    println!("  \"bob_pubkey\": \"{}\",", hex(&bob.public_key_bytes()));
    println!("  \"object\": {{");
    println!("    \"object_id\": \"{}\",", hex(&obj.object_id));
    println!("    \"version\": {},", obj.version);
    println!("    \"data_hash\": \"{}\",", hex(&obj.data_hash));
    println!(
        "    \"canonical_cbor\": \"{}\",",
        hex(&obj.canonical_bytes())
    );
    println!("    \"state_ref\": \"{}\"", hex(&obj.state_ref()));
    println!("  }},");
    println!(
        "  \"unsigned_action_cbor\": \"{}\",",
        hex(&unsigned.canonical_bytes())
    );
    println!("  \"action_id\": \"{}\",", hex(&unsigned.action_id()));
    println!("  \"signature\": \"{}\",", hex(&signature));
    println!(
        "  \"signed_action_cbor\": \"{}\"",
        hex(&signed.canonical_bytes())
    );
    println!("}}");
}
