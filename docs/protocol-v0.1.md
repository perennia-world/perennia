# OpenWorld Protocol v0.1

## Goal
Prove deterministic world-state transitions before adding P2P networking or consensus.

## Invariants
- Same state + same valid action = same next state.
- Gold cannot be created or destroyed by TRANSFER_GOLD.
- A nonce can be consumed only once.
- Every state-changing action must have a valid Ed25519 signature.
- Player identity is derived from the SHA-256 hash of the Ed25519 public key.

## Current components
- world-protocol: action/event contracts.
- world-crypto: keys, signatures, player IDs.
- world-core: deterministic state transition engine.
- world-simulator: 10,000-action consistency simulation across three state instances.

## Explicit non-goals
No networking, blockchain, BFT consensus, tokens, houses, territories, combat, AI, or mobile UI yet.
