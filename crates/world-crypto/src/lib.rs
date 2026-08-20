//! Ed25519 (RFC 8032) + SHA-256 primitives for Perennia / OWP v0.1.

use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand_core::OsRng;
use sha2::{Digest, Sha256};

pub struct PlayerKeypair {
    signing: SigningKey,
}

impl PlayerKeypair {
    pub fn generate() -> Self {
        Self { signing: SigningKey::generate(&mut OsRng) }
    }

    /// Deterministic keypair for tests, simulators, and test vectors.
    pub fn from_seed(seed: [u8; 32]) -> Self {
        Self { signing: SigningKey::from_bytes(&seed) }
    }

    pub fn public_key_bytes(&self) -> [u8; 32] {
        self.signing.verifying_key().to_bytes()
    }

    pub fn actor_id_hex(&self) -> String {
        actor_id_hex(&self.public_key_bytes())
    }

    pub fn sign(&self, message: &[u8]) -> Vec<u8> {
        self.signing.sign(message).to_bytes().to_vec()
    }
}

/// Human-readable actor id: lowercase hex SHA-256 of the public key.
pub fn actor_id_hex(public_key: &[u8; 32]) -> String {
    let digest = Sha256::digest(public_key);
    digest.iter().map(|b| format!("{b:02x}")).collect()
}

pub fn verify(public_key: &[u8; 32], message: &[u8], signature: &[u8]) -> bool {
    let Ok(verifying_key) = VerifyingKey::from_bytes(public_key) else { return false; };
    let Ok(sig_bytes): Result<[u8; 64], _> = signature.try_into() else { return false; };
    let signature = Signature::from_bytes(&sig_bytes);
    verifying_key.verify(message, &signature).is_ok()
}
