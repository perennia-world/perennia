# OpenWorld Protocol (OWP)

**Signed actions that declare their own consensus scope.**

Most decentralized protocols make you choose between two extremes: either every action is just an authored statement that nobody has to agree on, or every action is pushed through one global ledger. OWP treats consensus as a *property of the action* rather than a property of the network. An action says how much agreement its own semantics require, and only actions that touch exclusive shared state pay the cost of settling it.

> **Status: experimental, v0.1 draft.** The protocol is a draft for implementation and the code is a prototype. There is no network yet. Do not use this for anything of real-world value — see [SECURITY.md](SECURITY.md).

---

## Why another protocol

This question deserves a direct answer, and the repo contains a written one: [docs/spec/owp-vs-existing-protocols.md](docs/spec/owp-vs-existing-protocols.md) is a design analysis of where OWP overlaps existing work and where it does not.

The short version — signed decentralized data and portable identity already exist, and OWP does not claim them:

- **Nostr** gives signed events, relay propagation, and pseudonymous keypair identity, with a deliberately weak global coordination model.
- **AT Protocol** gives portable DID identity, signed repositories, and content-addressed state, but does not attempt Byzantine consensus over shared exclusive state.
- **Holochain** gives agent-centric source chains and DHT validation, but explicitly avoids global consensus and does not provide strong double-spend prevention for arbitrary scarce shared state.
- **W3C DIDs / Verifiable Credentials** give interoperable identity and claims, and are intentionally not a propagation network or a replicated state machine.

What is not already covered is the combination OWP is built around:

> A general protocol for signed actions by humans, software, and AI agents, where each action explicitly declares its consensus scope, and where exclusive state can be finalized at community or global scope **without forcing all actions through a single global ledger**.

If that hypothesis is wrong, the analysis document is the place where it should be argued against.

---

## The model

**Identity is a keypair, not an account.** An actor is an Ed25519 public key. The stable identifier is `actor_id = SHA-256("OWP-OBJECT-ID-v1\0" || public_key)`, rendered for humans as `owp1:<64 hex>`. Protocol validity never requires a legal name, an email address, a phone number, or a centralized account.

**An action is an immutable signed statement.** It carries a version, the actor, an action type, a replay-protection nonce, a timestamp, explicit references to the prior state it depends on, its consensus level, an optional scope, and a typed payload. Actions are encoded as deterministic CBOR so that the same action is always the same bytes, and therefore always the same hash and the same signature input.

**Consensus is declared per action.** v0.1 defines four levels:

| Level | Value | Used when | `scope` |
|---|---|---|---|
| `NONE` | 0 | authorship alone is sufficient — profiles, messages, announcements | must be null |
| `LOCAL` | 1 | consistency is only needed between explicitly participating actors or devices | must be null |
| `COMMUNITY` | 2 | a bounded community maintains exclusive shared state — realm ownership, guild territory, marketplace escrow | required |
| `GLOBAL` | 3 | state a deployment declares globally exclusive | required |

v0.1 deliberately does not claim that every action belongs in `GLOBAL`.

**Exclusive state is consumable and versioned.** A DAG of signed actions does not by itself stop two validly signed actions from consuming the same thing. Every exclusively mutable object carries a versioned `state_ref`, and two actions conflict precisely when they consume the same `state_ref`. On top of that, `COMMUNITY` and `GLOBAL` actions use validator locking, quorum certificates, and finalized checkpoints.

**Validation is deterministic and staged.** Encoding → signature → replay → references → application rules → consensus. The same prior state plus the same action must produce the same result on every implementation, which is what makes independent verification possible at all.

---

## Quick start

```bash
cargo test --workspace
```

```bash
cargo run -p world-simulator
```

The toolchain is pinned in [rust-toolchain.toml](rust-toolchain.toml), so `rustup` will fetch the right compiler automatically.

The simulator is a determinism proof, not a demo. It builds 10,000 signed `asset.transfer` actions, deliberately interleaving adversarial double-spend attempts that reuse a stale `state_ref`, then feeds the identical wire bytes to three independent replicas that were never told what the answer should be. It asserts that all three converge on the same world hash and reject exactly the same actions:

```text
Actions generated : 10000
Adversarial (stale state_ref) attempts: 1426
Replica rejects   : [1426, 1426, 1426]
Node A: b84fe9033137886b433798c952d68bbaaf26e7c6321282fdaf537a590d85eca6
Node B: b84fe9033137886b433798c952d68bbaaf26e7c6321282fdaf537a590d85eca6
Node C: b84fe9033137886b433798c952d68bbaaf26e7c6321282fdaf537a590d85eca6
PASS: 10000 actions (1426 adversarial) produced identical world hash on 3 independent replicas.
```

It runs unoptimized on every push, so it takes several minutes — the point is the assertion, not the throughput.

---

## Repository layout

| Crate | Role |
|---|---|
| [`world-crypto`](crates/world-crypto) | Ed25519 keys, signatures, actor identifiers |
| [`world-protocol`](crates/world-protocol) | Action contracts and the deterministic CBOR codec |
| [`world-core`](crates/world-core) | Deterministic state machine — validation stages and the consumable-state model |
| [`world-simulator`](crates/world-simulator) | 10,000-action convergence proof across three replicas |

## Documentation

| Document | Contents |
|---|---|
| [Protocol specification v0.1](docs/spec/owp-protocol-v0.1.md) | Normative spec (RFC 2119). Six primitives, crypto profile, canonical encoding, validation stages, consensus levels, anti-double-transfer |
| [OWP vs existing protocols](docs/spec/owp-vs-existing-protocols.md) | Design analysis against Nostr, ATProto, Holochain, DIDs/VCs |
| [Implementation notes](docs/protocol-v0.1.md) | Goal, invariants, components, explicit non-goals |
| [Identity and anonymity](docs/identity-and-anonymity.md) | Contributor privacy model and its stated limits |
| [Public testnet plan](docs/public-testnet.md) | Phased path from CI simulation to independently operated nodes |
| [Shard Zero design](docs/spec/shard-zero-game-design-doc-v0.1.md) | The reference application driving the protocol requirements |

Cross-implementation [test vectors](docs/spec/test-vectors) are published alongside the spec. A second implementation should be able to reproduce them without reading this Rust code.

## Cryptographic profile

| Purpose | Algorithm |
|---|---|
| Actor signatures | Ed25519 (RFC 8032) |
| Hashing and object identifiers | SHA-256 |
| Binary canonical encoding | Deterministic CBOR (RFC 8949 §4.2.1) |
| JSON canonical encoding | JCS (RFC 8785) |
| Binary values as text | lowercase hex, no `0x` prefix |

All hashes and signatures are domain-separated by a normative ASCII prefix, so bytes signed for one purpose can never be replayed as another.

---

## What v0.1 is not

Stated plainly, because a protocol that hides its scope wastes everyone's time. v0.1 does **not** define economic incentives, a native token, Proof of Work, Proof of Stake, validator-election economics, smart contract execution, privacy-preserving proofs, or final network governance. Peer-to-peer networking is not implemented yet — v0.1 exists to prove deterministic world-state transitions *before* consensus and networking are built on top.

## Contributing

The project is published under a project identity rather than personal founder identities, and contributors may use pseudonyms. Before opening a pull request, read [docs/identity-and-anonymity.md](docs/identity-and-anonymity.md) — it describes the metadata hygiene expected of public commits, and it is honest that pseudonymity is not a guarantee of anonymity.

Changes to normative behavior should update the specification and the test vectors in the same change, not afterwards.

## Security

See [SECURITY.md](SECURITY.md). This is experimental software with no security guarantees. Please do not publish exploit details before a fix is available.

## License

Licensed under the [Apache License, Version 2.0](LICENSE), which grants an explicit patent license alongside the copyright permissions — relevant for a protocol intended to have independent implementations.

Unless you state otherwise, any contribution you intentionally submit for inclusion in this work is licensed under the same terms, with no additional conditions.
