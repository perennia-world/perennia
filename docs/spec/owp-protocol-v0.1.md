# OpenWorld Protocol (OWP) Protocol Specification v0.1

**Status:** Draft for implementation  
**Version:** 0.1.0  
**Target:** OWP v0.1 / prerequisite for v0.2 networking  
**Normative language:** The key words **MUST**, **MUST NOT**, **REQUIRED**, **SHOULD**, **SHOULD NOT**, and **MAY** are to be interpreted as described by RFC 2119 / RFC 8174.

---

## 1. Scope

OWP v0.1 defines the minimum protocol primitives required for independent implementations to create, sign, identify, validate, reference, propagate, and—when required—reach scoped consensus on actions.

The six primitives defined by this specification are:

1. **Identity** — who is authorized to create an action.
2. **Action** — the immutable statement proposed by an actor.
3. **Signature** — cryptographic proof that the actor authorized the action.
4. **Reference** — causal dependencies on prior accepted actions or state objects.
5. **Validation** — deterministic rules for deciding whether an action is structurally and semantically valid.
6. **Propagation** — how valid actions are advertised and retrieved by peers.

This version also defines the mandatory anti-double-transfer mechanism for actions that use **COMMUNITY** or **GLOBAL** consensus.

OWP v0.1 does **not** define economic incentives, a native token, Proof of Work, Proof of Stake, validator-election economics, smart-contract execution, privacy-preserving proofs, or final network governance.

---

## 2. Design principles

OWP implementations MUST preserve the following principles:

- **Deterministic verification:** the same valid prior state plus the same action MUST produce the same validation result.
- **Local-first operation:** an actor MAY create and sign actions while offline.
- **No central authority requirement:** protocol validity MUST NOT depend on one vendor-operated server.
- **Scoped consensus:** actions MUST NOT require global consensus unless their semantics require globally exclusive state.
- **Cryptographic identity:** protocol identity MUST NOT require a legal name, email address, telephone number, or centralized account.
- **Explicit causality:** actions that depend on prior state MUST reference that state explicitly.
- **Protocol/data separation:** OWP defines verification primitives; applications define higher-level semantics through action types and payload schemas.

---

## 3. Cryptographic profile

OWP v0.1 fixes one cryptographic profile to maximize interoperability.

### 3.1 Required algorithms

| Purpose | Algorithm |
|---|---|
| Actor signatures | Ed25519 |
| Action / object hashing | SHA-256 |
| Binary canonical encoding | Deterministic CBOR, RFC 8949 §4.2.1 |
| JSON canonical encoding | JCS, RFC 8785 |
| Text representation of binary values | lowercase hexadecimal without `0x` prefix |

Implementations MUST support Ed25519 as specified by RFC 8032.

Implementations MUST use SHA-256 for all protocol object identifiers in v0.1.

Alternative algorithms MUST NOT be used inside a v0.1 consensus domain.

### 3.2 Domain separation

All hashes and signatures MUST be domain-separated.

The following ASCII byte prefixes are normative:

```text
OWP-ACTION-v1\0
OWP-ACTION-ID-v1\0
OWP-OBJECT-ID-v1\0
OWP-VOTE-v1\0
OWP-CHECKPOINT-v1\0
```

`\0` is one zero byte (`0x00`).

---

## 4. Primitive 1 — Identity

### 4.1 Actor key

An OWP v0.1 actor is identified by an Ed25519 public key.

```text
actor_public_key := 32 raw bytes
```

The corresponding private key MUST remain under control of the actor or its authorized signing environment.

### 4.2 Actor identifier

The stable v0.1 actor identifier is:

```text
actor_id = SHA-256(
    "OWP-OBJECT-ID-v1\0" ||
    actor_public_key
)
```

`actor_id` is therefore 32 bytes.

Human-readable tooling SHOULD render it as:

```text
owp1:<64 lowercase hex chars>
```

The `owp1:` prefix is presentation-only and MUST NOT be included in signed protocol bytes.

### 4.3 Key rotation

Key rotation is not fully specified in v0.1.

Applications requiring rotation MAY define a higher-level `identity.rotate` action, but validators MUST NOT infer key replacement without an application-specific rule.

---

## 5. Primitive 2 — Action

### 5.1 Normative representation

The **normative cryptographic representation** of an OWP action is deterministic CBOR.

JSON is a secondary interoperable representation for APIs, debugging, logs, and test vectors.

**Signatures and action identifiers MUST always be calculated from deterministic CBOR, never directly from JSON bytes.**

### 5.2 UnsignedAction CBOR schema

An `UnsignedAction` MUST be encoded as a CBOR map using the following integer keys:

| CBOR key | Name | Type | Required | Meaning |
|---:|---|---|---|---|
| `0` | `version` | uint | yes | Protocol action version. MUST be `1`. |
| `1` | `actor` | bstr(32) | yes | Ed25519 public key of actor. |
| `2` | `type` | tstr | yes | Application action type. |
| `3` | `nonce` | uint64 | yes | Monotonic actor nonce within the applicable state scope. |
| `4` | `created_at` | uint64 | yes | Unix epoch milliseconds; advisory unless application rules say otherwise. |
| `5` | `refs` | array<bstr(32)> | yes | Causal action/state references. Empty array allowed. |
| `6` | `consensus` | uint | yes | `0=NONE`, `1=LOCAL`, `2=COMMUNITY`, `3=GLOBAL`. |
| `7` | `scope` | bstr(32) or null | yes | Consensus/community scope identifier; null for NONE/LOCAL. |
| `8` | `payload` | map | yes | Application-defined payload constrained by §5.4. |

Unknown top-level keys MUST cause rejection in v0.1.

### 5.3 SignedAction CBOR schema

A `SignedAction` MUST be:

```text
{
  0: UnsignedAction,
  1: signature
}
```

Where:

- key `0` contains the complete `UnsignedAction` map.
- key `1` is a 64-byte Ed25519 signature.

No additional keys are allowed in v0.1.

### 5.4 Payload restrictions

To guarantee cross-language determinism, v0.1 payload values are restricted to:

- unsigned integers;
- negative integers;
- UTF-8 text strings;
- byte strings;
- booleans;
- null;
- arrays containing allowed values;
- maps whose keys are UTF-8 text strings and whose values are allowed values.

The following MUST NOT appear in a v0.1 action payload:

- floating-point numbers;
- CBOR tags;
- `undefined`;
- indefinite-length arrays/maps/strings;
- duplicate map keys.

Money, balances, quantities, coordinates, and percentages SHOULD use application-defined fixed-point integers.

Example:

```text
12.34 coins -> 1234 atomic units
```

### 5.5 Action type syntax

`type` MUST:

- contain 1–96 ASCII characters;
- use lowercase ASCII letters, digits, hyphen, underscore, and dot only;
- start with a lowercase ASCII letter;
- be namespaced using dot notation when possible.

Examples:

```text
asset.transfer
house.join
territory.attack
credential.issue
agent.delegate
```

### 5.6 Reference ordering

`refs` MUST contain unique 32-byte identifiers.

Before encoding, `refs` MUST be sorted in ascending bytewise lexicographic order.

An implementation receiving unsorted or duplicate `refs` MUST reject the action as non-canonical.

### 5.7 Deterministic CBOR rules

OWP v0.1 MUST use the core deterministic encoding requirements of RFC 8949 §4.2.1:

- preferred/shortest integer and length encodings;
- no indefinite-length items;
- deterministic map-key ordering;
- deterministic representation of all allowed values.

Because v0.1 forbids floating-point values and CBOR tags in Actions, multiple known sources of cross-implementation ambiguity are eliminated.

---

## 6. Canonical JSON representation

### 6.1 Purpose

Canonical JSON exists for:

- HTTP APIs;
- developer tooling;
- logs;
- fixtures;
- test vectors;
- languages where inspecting CBOR is inconvenient.

Canonical JSON MUST NOT replace deterministic CBOR as the signing representation.

### 6.2 JSON schema

The equivalent JSON object is:

```json
{
  "actor": "<64-char lowercase hex Ed25519 public key>",
  "consensus": "COMMUNITY",
  "created_at": 1787130000000,
  "nonce": 7,
  "payload": {},
  "refs": [
    "<64-char lowercase hex SHA-256 id>"
  ],
  "scope": "<64-char lowercase hex scope id or null>",
  "type": "asset.transfer",
  "version": 1
}
```

The signed JSON envelope is:

```json
{
  "action": {
    "actor": "<hex>",
    "consensus": "COMMUNITY",
    "created_at": 1787130000000,
    "nonce": 7,
    "payload": {},
    "refs": [],
    "scope": "<hex>",
    "type": "asset.transfer",
    "version": 1
  },
  "signature": "<128-char lowercase hex Ed25519 signature>"
}
```

### 6.3 Canonicalization

When a canonical JSON byte representation is required, implementations MUST use RFC 8785 JSON Canonicalization Scheme (JCS).

Binary CBOR values MUST be rendered as lowercase hexadecimal strings in JSON.

JSON parsers MUST reject duplicate object member names.

Protocol values that may exceed safe IEEE-754 integer precision SHOULD be represented as decimal strings in JSON APIs. Implementations MUST convert them back to the exact CBOR integer type before cryptographic processing.

---

## 7. Primitive 3 — Signature

### 7.1 Signing bytes

Given an `UnsignedAction`:

```text
canonical_action = DeterministicCBOR(UnsignedAction)

signing_message =
    "OWP-ACTION-v1\0" ||
    canonical_action
```

The signature is:

```text
signature = Ed25519.Sign(
    actor_private_key,
    signing_message
)
```

The public key in `UnsignedAction.actor` MUST verify the signature.

### 7.2 Action identifier

The action identifier intentionally does not depend on transport metadata.

```text
action_id = SHA-256(
    "OWP-ACTION-ID-v1\0" ||
    DeterministicCBOR(UnsignedAction)
)
```

`action_id` is 32 bytes.

Because Ed25519 signatures are deterministic for the same key/message, including the signature in the identifier is unnecessary in v0.1.

### 7.3 Signature verification

An action MUST be rejected if:

- signature length is not exactly 64 bytes;
- actor public key length is not exactly 32 bytes;
- Ed25519 verification fails;
- the received CBOR is not the deterministic encoding of the decoded action.

---

## 8. Primitive 4 — Reference

OWP references establish explicit causal dependency.

### 8.1 General references

`UnsignedAction.refs` contains zero or more identifiers of actions or state objects required to validate the proposed action.

A validator MUST NOT accept an action when a REQUIRED reference is unavailable or invalid.

### 8.2 Transfer/state references

Actions that mutate exclusive state MUST include the exact current state object(s) they consume.

For example, an exclusive transferable object is modeled as:

```text
OwnedObject {
    object_id: bstr(32),
    version: uint64,
    controller: actor_id,
    data_hash: bstr(32)
}
```

Its immutable state reference is:

```text
state_ref = SHA-256(
    "OWP-OBJECT-ID-v1\0" ||
    DeterministicCBOR(OwnedObject)
)
```

A state-changing action MUST reference the `state_ref` it intends to consume.

---

## 9. Primitive 5 — Validation

Validation is performed in ordered stages.

### 9.1 Stage A — Encoding validation

A node MUST reject an action if:

- CBOR is malformed;
- encoding is not deterministic according to §5.7;
- required fields are absent;
- unknown top-level fields are present;
- field types or lengths are invalid;
- payload contains forbidden types;
- `refs` are duplicate or incorrectly ordered.

### 9.2 Stage B — Cryptographic validation

A node MUST verify:

1. actor public key format;
2. signature format;
3. Ed25519 signature;
4. locally recomputed `action_id`.

### 9.3 Stage C — Replay validation

For action types using actor nonces, the action MUST satisfy:

```text
nonce == expected_nonce(actor, scope)
```

After final acceptance:

```text
expected_nonce := expected_nonce + 1
```

A nonce alone is NOT sufficient to prevent double-transfer of independently versioned objects; §11 is normative for COMMUNITY/GLOBAL transfers.

### 9.4 Stage D — Reference validation

Every state-changing action MUST reference the exact state version it consumes.

The referenced object:

- MUST exist;
- MUST be unspent/unconsumed at the validator's finalized checkpoint;
- MUST satisfy application ownership/authorization rules.

### 9.5 Stage E — Application validation

Application rules MUST be deterministic.

Validation MUST NOT depend directly on:

- local wall-clock time;
- `Math.random()` or equivalent;
- unverified GPS;
- nondeterministic external APIs;
- an LLM response;
- mutable HTTP resources.

External information MAY be used only after being converted into a protocol-recognized, signed/verifiable input.

### 9.6 Stage F — Consensus validation

For `NONE` and `LOCAL`, protocol validity does not require a network quorum.

For `COMMUNITY` and `GLOBAL`, a state-mutating action is **proposed-valid** after Stages A–E but MUST NOT be considered finalized until it receives a valid Quorum Certificate defined in §11.

---

## 10. Consensus levels

OWP v0.1 defines four scopes.

### 10.1 NONE (`0`)

Used for statements where authorship is sufficient.

Examples:

```text
profile.publish
message.publish
offer.announce
```

No shared-state ordering is implied.

### 10.2 LOCAL (`1`)

Used when consistency is needed only between explicitly participating actors/devices.

OWP v0.1 does not define a standard quorum mechanism for LOCAL.

### 10.3 COMMUNITY (`2`)

Used when a bounded community maintains exclusive shared state.

Examples:

- ownership inside a game realm;
- community treasury;
- guild territory;
- marketplace escrow.

`scope` MUST identify the community validator domain.

### 10.4 GLOBAL (`3`)

Used for state that a deployment declares globally exclusive within the OWP network/domain.

`scope` MUST identify the global validator-set epoch/domain.

OWP v0.1 deliberately does not claim that every OWP action belongs in GLOBAL consensus.

---

## 11. Anti-double-transfer mechanism for COMMUNITY/GLOBAL

This section is normative.

A pure DAG does not, by itself, prevent two conflicting validly signed actions from attempting to consume the same state. OWP therefore combines:

1. **versioned consumable state references**;
2. **validator locking**;
3. **quorum certificates (QC)**;
4. **finalized checkpoints**.

This mechanism is required for all COMMUNITY/GLOBAL actions that mutate exclusive state.

### 11.1 Consumable input model

Every exclusively mutable object MUST have a unique versioned state reference.

A transfer-like action contains one or more consumed inputs:

```text
payload.inputs = [
    {
        "object_id": ...,
        "version": N,
        "state_ref": ...
    }
]
```

The action MAY create one or more output objects with incremented/new versions.

Two actions conflict if they consume the same `state_ref`.

### 11.2 Validator set

Each COMMUNITY/GLOBAL `scope` has a validator-set descriptor:

```text
ValidatorSet {
    scope_id
    epoch
    validators[]
    threshold
}
```

For the v0.1 reference profile:

```text
n = 3f + 1
threshold = 2f + 1
```

Examples:

```text
n = 4  -> f = 1 -> threshold = 3
n = 7  -> f = 2 -> threshold = 5
n = 10 -> f = 3 -> threshold = 7
```

Validator-set election/change is outside v0.1, but every validator set MUST have an immutable identifier and epoch.

### 11.3 Proposal validation

Before voting for an action, a validator MUST verify Stages A–E and MUST ensure every consumed `state_ref` is:

- present in its finalized state;
- controlled/authorized as required;
- not already finalized as consumed;
- not locked by that validator for another conflicting action in the same epoch/round.

### 11.4 Lock rule

For each consumable state reference, an honest validator MUST sign at most one conflicting proposal for a given consensus epoch/round.

Conceptually:

```text
lock[state_ref] = action_id
```

Once locked, a validator MUST NOT vote for another `action_id` consuming the same `state_ref` unless a protocol-defined higher-round unlock rule is satisfied.

**v0.1 simplification:** the reference implementation uses no automatic unlock inside a round. A round timeout advances to a higher round, and any unlock MUST carry evidence of the prior round state.

### 11.5 Vote

A validator vote signs:

```text
VoteBody {
    scope_id
    validator_set_epoch
    consensus_round
    action_id
}
```

Signing bytes:

```text
"OWP-VOTE-v1\0" ||
DeterministicCBOR(VoteBody)
```

### 11.6 Quorum Certificate

A Quorum Certificate contains:

```text
QuorumCertificate {
    scope_id
    validator_set_epoch
    consensus_round
    action_id
    signatures[]
}
```

A QC is valid only when:

- all signatures are from distinct validators in the applicable validator set;
- every signature is valid;
- all signatures cover the same `VoteBody`;
- signature count/weight satisfies the configured threshold.

A COMMUNITY/GLOBAL state mutation MUST NOT be finalized without a valid QC.

### 11.7 Why two conflicting transfers cannot both finalize

Assume:

```text
n = 3f + 1
QC threshold = 2f + 1
at most f validators are Byzantine
```

Any two sets of `2f + 1` validators intersect in at least `f + 1` validators.

At least one validator in that intersection must therefore be honest.

Because an honest validator MUST NOT vote for two conflicting actions consuming the same `state_ref`, two conflicting actions cannot both obtain valid QCs unless the Byzantine fault assumption is violated.

This is the v0.1 safety argument against double-transfer.

### 11.8 Finalization

When a node receives a valid QC:

1. the action becomes finalized;
2. all consumed `state_ref` values are marked consumed;
3. output state objects are created;
4. actor nonce is advanced where applicable;
5. the finalized action is inserted into the local DAG;
6. the node updates its finalized checkpoint candidate.

### 11.9 DAG role

The DAG records causality and enables parallel branches for unrelated actions.

It MUST NOT be treated as a conflict-resolution mechanism by itself.

For exclusive COMMUNITY/GLOBAL state:

```text
DAG = causality + propagation structure
QC  = conflict finality
```

Unrelated actions MAY finalize concurrently.

Conflicting actions compete for validator locks and a QC.

### 11.10 Double-transfer example

Initial state:

```text
object: GOLD-UTXO-91
amount: 100
controller: Alice
state_ref: X
```

Alice signs two actions:

```text
A: consume X -> Bob
B: consume X -> Carol
```

Both signatures may be cryptographically valid.

Validators observe the conflict.

If validators first lock on `A`, enough honest validators cannot subsequently vote for `B`.

If `A` receives a QC:

```text
X -> consumed
A -> finalized
B -> permanently invalid against that finalized state
```

Signature validity is therefore distinct from state validity.

### 11.11 Network partitions

During a partition, competing proposals MAY exist.

Nodes MUST NOT treat a proposal as finalized merely because it is locally observed.

Only a valid QC establishes v0.1 COMMUNITY/GLOBAL finality.

If neither partition can reach threshold, safety is preserved at the cost of liveness.

### 11.12 Crash recovery

Validator locks MUST be persisted before a vote is transmitted.

After restart, the validator MUST recover its locks before voting.

Failure to persist locks can enable accidental equivocation.

### 11.13 Equivocation evidence

If a validator signs conflicting votes for the same consumed input / epoch / round, both signatures constitute cryptographic equivocation evidence.

OWP v0.1 defines detection but not punishment.

Applications or later protocol versions MAY define:

- validator removal;
- reputation reduction;
- stake slashing;
- governance sanctions.

---

## 12. Finalized checkpoints

To avoid replaying an unbounded DAG, COMMUNITY/GLOBAL domains SHOULD periodically publish checkpoints.

A checkpoint contains at minimum:

```text
Checkpoint {
    scope_id
    validator_set_epoch
    sequence
    previous_checkpoint_id
    finalized_frontier[]
    state_root
}
```

`state_root` SHOULD be a deterministic authenticated state commitment (for example a Merkle root) in v0.2+.

The checkpoint is signed using the same quorum threshold as state finality.

Signing bytes:

```text
"OWP-CHECKPOINT-v1\0" ||
DeterministicCBOR(CheckpointBody)
```

v0.1 implementations MAY initially compute `state_root` as:

```text
SHA-256(DeterministicCBOR(full_finalized_state))
```

This is not scalable but is sufficient for the first demonstrator.

---

## 13. Primitive 6 — Propagation

OWP v0.1 defines protocol behavior independently of a transport.

v0.2 is expected to use libp2p.

### 13.1 Required message semantics

A transport implementation MUST support logical equivalents of:

```text
ANNOUNCE_ACTION(action_id)
GET_ACTION(action_id)
ACTION(signed_action)

ANNOUNCE_QC(action_id)
GET_QC(action_id)
QC(quorum_certificate)

GET_OBJECT(object_id_or_state_ref)
OBJECT(object)

GET_CHECKPOINT(scope_id, sequence_or_latest)
CHECKPOINT(checkpoint)
```

### 13.2 Gossip behavior

Nodes MAY gossip complete actions or identifiers.

Nodes SHOULD avoid repeatedly relaying known invalid objects.

Nodes MUST independently verify actions/QCs received from peers.

A peer MUST NEVER become trusted merely because it is a bootstrap, relay, archive, or high-uptime node.

### 13.3 Deduplication

Nodes SHOULD deduplicate actions by `action_id`.

Duplicate receipt of the same valid action MUST be idempotent.

### 13.4 Unknown dependencies

If an otherwise well-formed action references unknown objects, a node SHOULD:

1. mark it as pending;
2. retrieve missing dependencies from one or more peers;
3. resume validation after dependencies are available;
4. reject it if dependencies prove invalid.

---

## 14. State transition interface

A compliant deterministic state engine SHOULD expose the conceptual operation:

```text
apply(
    finalized_state,
    signed_action,
    optional_qc
) -> Result<new_state, validation_error>
```

For COMMUNITY/GLOBAL actions:

```text
optional_qc MUST be present and valid
```

For NONE/LOCAL actions, QC requirements depend on the applicable profile.

Implementations MUST NOT mutate finalized state before all required validation succeeds.

---

## 15. Error classes

Implementations SHOULD expose stable machine-readable errors.

Minimum v0.1 errors:

```text
OWP_ENCODING_INVALID
OWP_NON_CANONICAL
OWP_VERSION_UNSUPPORTED
OWP_FIELD_INVALID
OWP_SIGNATURE_INVALID
OWP_ACTION_ID_MISMATCH
OWP_NONCE_INVALID
OWP_REFERENCE_MISSING
OWP_REFERENCE_INVALID
OWP_STATE_ALREADY_CONSUMED
OWP_NOT_AUTHORIZED
OWP_RULE_REJECTED
OWP_SCOPE_INVALID
OWP_VALIDATOR_SET_UNKNOWN
OWP_QC_INVALID
OWP_CONSENSUS_REQUIRED
OWP_CONFLICT
```

Application-specific validation errors SHOULD use a namespace:

```text
APP_<DOMAIN>_<ERROR>
```

---

## 16. Minimal asset transfer profile

This profile exists to make v0.1 testable.

### 16.1 State object

```text
AssetOutput {
    asset_id: bstr(32)
    version: uint64
    owner: actor_id
    amount: uint64
}
```

### 16.2 Transfer action type

```text
asset.transfer
```

Payload:

```text
{
    "inputs": [
        {
            "state_ref": bstr(32)
        }
    ],
    "outputs": [
        {
            "owner": bstr(32),
            "amount": uint64
        }
    ]
}
```

### 16.3 Rules

Validators MUST verify:

```text
sum(inputs.amount) == sum(outputs.amount)
```

and:

- every input exists;
- every input is unconsumed;
- the actor controls every input, unless an application-specific authorization proof is present;
- output amounts are positive;
- integer overflow is impossible;
- every input state reference appears in `refs`;
- COMMUNITY/GLOBAL transfers possess a valid QC before finalization.

No asset can be created or destroyed through `asset.transfer`.

Mint/burn semantics require separate explicitly authorized action types.

---

## 17. Determinism test vectors

Every implementation MUST pass cross-implementation tests covering at least:

1. canonical CBOR byte equality;
2. action ID equality;
3. Ed25519 signature verification;
4. JSON-to-data-model-to-CBOR equality;
5. reference ordering;
6. malformed CBOR rejection;
7. replay nonce rejection;
8. double-transfer conflict detection;
9. QC verification;
10. deterministic final state hash.

The repository SHOULD maintain fixtures under:

```text
/docs/spec/test-vectors/
```

with:

```text
unsigned-action.json
unsigned-action.cbor.hex
signed-action.json
signed-action.cbor.hex
action-id.hex
public-key.hex
signature.hex
expected-validation.json
```

---

## 18. Security considerations

### 18.1 Sybil attacks

Ed25519 identities are cheap to create and do not provide Sybil resistance.

OWP v0.1 therefore makes no claim that “one identity equals one person” or “one key equals one validator”.

COMMUNITY/GLOBAL safety depends on the security of the configured validator-set membership mechanism.

### 18.2 Malicious clients

Clients are untrusted.

A client MAY propose any bytes; nodes MUST independently validate every action.

Local UI state, local balances, local clocks, and local game/application calculations are not authoritative.

### 18.3 Replay attacks

Actor nonce validation prevents simple replay within a scope.

Consumable state references plus QC finality prevent replay/double-use of exclusive state.

### 18.4 Validator equivocation

Votes are signed and domain-separated.

Conflicting signed votes are detectable evidence.

### 18.5 Key compromise

A compromised private key can authorize actions as that actor.

Key recovery/rotation is intentionally deferred; deployments handling meaningful value MUST define a recovery strategy before production use.

### 18.6 Denial of service

Nodes SHOULD enforce:

- maximum action size;
- maximum payload depth;
- maximum array/map sizes;
- request rate limits;
- peer scoring;
- bounded pending-dependency pools.

Exact limits are deployment profiles and not fixed by core v0.1.

---

## 19. Privacy considerations

OWP identifiers are pseudonymous, not anonymous by default.

Public keys, action timing, graph relationships, network metadata, and payload content can enable correlation.

Applications MUST NOT publish precise physical location, personal information, secrets, or private conversation data merely because the protocol permits arbitrary payloads.

Sensitive payloads SHOULD be encrypted end-to-end before publication, with only the minimum verification metadata exposed.

A future privacy profile may define encrypted envelopes and zero-knowledge proofs.

---

## 20. Versioning and compatibility

`UnsignedAction.version = 1` identifies this action schema.

A v0.1 implementation MUST reject unknown action versions rather than guessing how to interpret them.

Application action types SHOULD be independently versioned when their payload semantics change incompatibly:

```text
asset.transfer.v1
asset.transfer.v2
```

or through an application schema identifier referenced in the payload.

---

## 21. v0.1 conformance requirements

An implementation is **OWP Core v0.1 conformant** if it:

- implements the exact Action data model in §5;
- produces RFC 8949 deterministic CBOR;
- verifies Ed25519 signatures as defined in §7;
- computes action IDs exactly as defined in §7.2;
- enforces reference and nonce rules;
- rejects non-canonical or ambiguous encodings;
- produces deterministic validation results;
- implements the minimal transfer profile;
- passes the normative test vectors.

An implementation is **OWP Community/Global v0.1 conformant** only if it additionally:

- implements versioned consumable state;
- persists validator locks before voting;
- implements validator votes and QC verification;
- requires QC finality for COMMUNITY/GLOBAL mutation;
- rejects already-consumed state;
- detects conflicting validator votes;
- can recover lock state after restart.

---

## 22. Reference implementation milestones before v0.2

The v0.1 implementation should not be considered closed until it demonstrates:

```text
1. 3 independent state engines
2. 10,000 identical valid actions
3. identical final state hash
4. invalid signatures rejected
5. replay attempts rejected
6. conflicting double-transfer proposals created intentionally
7. at most one conflicting proposal obtains a QC
8. validator restart preserves locks
9. partition without quorum does not finalize conflicting state
10. JSON fixtures round-trip to identical deterministic CBOR
```

Only after these pass should OWP v0.2 add real peer-to-peer propagation.

---

## 23. Open questions intentionally deferred

The following are explicitly outside v0.1:

- validator discovery/election;
- permissionless Sybil resistance;
- validator incentives;
- weighted voting;
- slashing;
- dynamic validator-set transitions;
- BFT view-change optimization;
- privacy-preserving payloads;
- DID integration;
- Verifiable Credentials integration;
- agent delegation/capability format;
- Merkleized scalable state;
- cross-community atomic transactions;
- global namespace governance.

These MUST be treated as future protocol work, not silently invented inside v0.2 networking code.

---

## 24. Normative references

- RFC 8949 — Concise Binary Object Representation (CBOR), especially §4.2.1 deterministic encoding.
- RFC 8032 — Edwards-Curve Digital Signature Algorithm (EdDSA), Ed25519 profile.
- RFC 8785 — JSON Canonicalization Scheme (JCS).
- RFC 2119 / RFC 8174 — normative requirement keywords.

---

## 25. Summary

OWP v0.1 establishes one unambiguous verification pipeline:

```text
Identity
   ↓
UnsignedAction
   ↓
Deterministic CBOR
   ↓
Ed25519 signature
   ↓
Action ID
   ↓
Explicit references
   ↓
Deterministic validation
   ↓
[COMMUNITY/GLOBAL: validator locks + QC]
   ↓
Finalized state transition
   ↓
DAG insertion
   ↓
Propagation
```

The DAG expresses causal structure and enables concurrency.

For exclusive COMMUNITY/GLOBAL state, the DAG alone is insufficient: **versioned consumable state + validator locking + a 2f+1 quorum certificate over a 3f+1 validator set is the v0.1 double-transfer safety mechanism.**

That boundary is intentional and is a prerequisite for a meaningful OWP v0.2 network demonstrator.
