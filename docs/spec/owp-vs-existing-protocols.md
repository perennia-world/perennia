# OWP vs Existing Protocols

**Status:** Design analysis  
**Audience:** OWP protocol designers and implementers  
**Purpose:** Determine whether OWP introduces a sufficiently distinct primitive to justify a new protocol, and identify mechanisms that should be adopted rather than reinvented.

---

## 1. Executive summary

OWP overlaps substantially with existing decentralized protocols, but the overlap is fragmented:

- **Nostr** provides simple signed events, relay-based propagation, pseudonymous public-key identity, and a deliberately weak global coordination model.
- **AT Protocol (ATProto)** provides portable DID-based identity, signed user repositories, deterministic content-addressed state, relays/firehoses, and application-level federation, but it does not attempt Byzantine consensus over shared exclusive state.
- **Holochain** provides agent-centric source chains, DHT validation, deterministic application rules, distributed authority selection, conflict/fork detection, and optional countersigning, but explicitly avoids blockchain-style global consensus and does not provide strong double-spend prevention for arbitrary scarce shared state.
- **W3C DIDs and Verifiable Credentials (VCs)** provide interoperable identity, verification methods, claims, issuance and verification, but are intentionally not a propagation network, replicated state machine, or consensus protocol.

OWP's proposed differentiator is therefore not "signed decentralized data" or "portable identity". Those already exist.

The distinctive OWP hypothesis is:

> **A general protocol for signed actions by humans, software, and AI agents, where each action explicitly declares its consensus scope and where exclusive state can be finalized at COMMUNITY or GLOBAL scope without forcing all actions through a single global ledger.**

The strongest justification for OWP is **scoped consensus over versioned consumable state integrated directly into an action/DAG model**, plus an explicit capability/agent model later.

The weakest justification is identity, event signing, generic propagation, and causal graphs: all of those can be borrowed from or mapped onto existing systems.

The recommendation of this document is:

1. **Do not invent a new identity standard.** Keep the current raw-key identity for v0.1 test vectors, but design a future identity adapter compatible with DIDs.
2. **Do not claim the DAG itself prevents double-spend.** OWP's actual novelty must live in scoped finality and state-consumption rules.
3. **Use explicit validator membership for the first testnet.** A permissionless validator-election mechanism is not required to prove the protocol.
4. **Adopt a known round/view-change discipline for liveness rather than inventing one.** For v0.2, a minimal Tendermint-style `propose → prevote → precommit` state machine scoped per conflict domain is safer than the current one-vote lock abstraction.
5. Continue OWP only if the scoped-consensus model proves materially simpler or more composable for applications than implementing the same behavior as an application-specific layer on Holochain or another existing substrate.

---

## 2. Comparison dimensions

| Dimension | OWP v0.1 proposal | Nostr | ATProto | Holochain | W3C DIDs / VCs |
|---|---|---|---|---|---|
| Primary abstraction | Signed **Action** with explicit references and consensus scope | Signed event | Signed per-user repository records/commits | Agent source-chain actions + DHT operations | Identifier / credential / presentation |
| Identity | Ed25519 public key → `actor_id` | secp256k1 public key | DID + handle; DID document locates keys/services | Agent public key | DID methods and verification methods |
| Canonical data | Deterministic CBOR; JCS for JSON tooling | Canonical event serialization in JSON | DAG-CBOR/CAR + Merkle Search Tree | Holochain action/entry serialization and content hashes | JSON-LD / JSON and cryptographic suites |
| Causal history | Explicit `refs` DAG | Event references via tags; no required causal DAG | Per-user signed commit chain / MST root | Per-agent source chain; DHT graph | Not a history/replication protocol |
| Propagation | Transport-neutral in v0.1; libp2p planned | WebSocket relays | PDS → relay/firehose → downstream services | Peer DHT gossip and authorities | Out of scope |
| Validation | Deterministic protocol + application validation | Signature/event-shape checks; relay policy | Repository/signature/schema/service validation | Deterministic integrity-zome validation by peers | Proof/credential verification |
| Consensus | NONE / LOCAL / COMMUNITY / GLOBAL | None at protocol core | No Byzantine shared-state consensus | No global consensus; peer validation | Out of scope |
| Exclusive state | Versioned consumable state + QC finality proposed | Not safely represented as globally exclusive state by NIP-01 alone | Account repository authority is per account, not shared BFT ownership | Conflict detection; limited guarantees for scarce resources | Out of scope |
| Double-transfer | Explicit consumed `state_ref` + validator locks/QC | Application-specific | Application-specific / authoritative repository semantics | Fork/conflict detection; docs warn no perfect scarce-resource protection | Out of scope |
| Validator selection | Deferred | No validator set | No validator set for global BFT | DHT authorities selected by network topology/addressing | Out of scope |
| Offline authoring | Yes | Yes | Possible locally, but PDS is authoritative sync host | Strongly agent-local | Credentials can be held offline |
| AI/software agents | First-class design target | Keys can represent agents but no special semantics | Accounts/services can be automated | Agents are foundational, not specifically AI | Subjects/controllers can represent software but not action coordination |
| Strongest overlap with OWP | — | Signed events + relay model | Portable identity + signed data repos | Agent-centric validation and DAG-like causality | Identity and attestations |

---

## 3. Nostr

### 3.1 What Nostr already solves

Nostr's core abstraction is extremely small: a user has a keypair, creates a signed event, derives an event ID from canonical serialized event data, and sends the event to one or more relays.

NIP-01 defines an event containing:

- event ID;
- author public key;
- creation timestamp;
- event kind;
- tags;
- content;
- signature.

Relays expose subscription/query semantics and can accept, reject, store, discard, or redistribute events according to relay policies.

This gives Nostr several properties relevant to OWP:

- pseudonymous cryptographic identities;
- client-side event creation;
- independently verifiable signatures;
- content-addressed events;
- relay redundancy;
- no mandatory central server;
- loose extensibility through event kinds and NIPs.

### 3.2 What Nostr intentionally does not solve

Nostr does not attempt to make all relays agree on a single state.

A Nostr event can say:

```text
Alice transfers object X to Bob
```

and another validly signed event can say:

```text
Alice transfers object X to Carol
```

Both can be authentic statements from Alice.

Nostr core does not determine which one is the globally authoritative state transition. Replaceable/addressable event conventions can define which version a relay should retain for some kinds, but that is not Byzantine finality over a consumed scarce resource.

This is not a flaw. It is a deliberate simplicity trade-off.

### 3.3 Identity and Sybil resistance

Nostr public keys are cheap to create. NIP-01 provides identity authenticity, not proof of unique personhood or validator eligibility.

The ecosystem can build trust through:

- follow graphs;
- relay policies;
- payment/reputation systems;
- user-selected trusted service providers;
- application-specific social graphs.

This suggests a useful OWP lesson:

> **Identity creation and validator selection should remain separate problems.**

OWP should not try to make Ed25519 actor IDs inherently Sybil-resistant.

### 3.4 OWP versus Nostr

OWP should borrow Nostr's minimalism wherever possible.

OWP should only be considered meaningfully different where an action has consequences that require coordinated exclusive state.

A useful conceptual distinction is:

```text
Nostr:
"Did this key publish this event?"

OWP COMMUNITY/GLOBAL:
"Did this key publish this action,
was it valid against the referenced state,
and did the relevant consensus domain finalize it
before a conflicting state transition?"
```

If an OWP use case only needs the first question, Nostr-like semantics may be sufficient and OWP consensus should not be involved.

---

## 4. AT Protocol (ATProto)

### 4.1 What ATProto already solves

ATProto has a more structured account and data architecture.

An account has:

- a stable DID identity;
- an account hosted on a Personal Data Server (PDS);
- a signed repository;
- portable repository/state;
- service location declared through the DID document.

Repository state is represented using deterministic content-addressed structures. ATProto repositories use a Merkle Search Tree (MST), and repository updates are propagated through event streams.

PDS instances expose repository event streams. Relays aggregate upstream streams and can produce large network-wide firehoses. Repository data is self-certifying and signed.

This is extremely relevant to OWP because ATProto demonstrates that:

- identity can be portable independently of hosting;
- a user can have an authoritative signed state repository;
- propagation infrastructure can be replaceable;
- indexers/AppViews do not need to become source-of-truth authorities;
- content-addressed repository synchronization can scale without global BFT consensus.

### 4.2 Where ATProto differs fundamentally from OWP

ATProto establishes **per-account authority**, not a Byzantine replicated state machine for arbitrary state jointly controlled by many independent actors.

If Alice changes Alice's repository, Alice's repository authority signs the change.

This works extremely well for:

- posts;
- follows;
- account data;
- personal records;
- application records authored under an account.

It does not automatically solve:

```text
Alice and Bob compete to consume the same globally scarce object X.
```

That requires an application-level authority or consensus mechanism outside the normal per-account repository model.

### 4.3 Sybil resistance

ATProto largely avoids the validator-election problem because it does not use a validator set to establish one global transaction history.

Identity uniqueness is also not guaranteed merely by possessing a DID.

Moderation, indexing, reputation, and service policies occur in higher layers.

OWP lesson:

> **Avoid introducing BFT consensus for data that can have a natural single-writer authority.**

Many OWP actions may belong in a model closer to ATProto:

```text
actor-owned log
+
signed commits
+
replication
```

Only state requiring multi-party exclusivity should escalate to COMMUNITY/GLOBAL consensus.

### 4.4 OWP versus ATProto

The most important design question is whether OWP could be represented as:

```text
ATProto-like signed actor repositories
+
application-specific consensus service
```

If yes, creating an entirely new identity, repository and propagation stack would be unjustified.

OWP's case becomes stronger only if **consensus scope is part of the action model itself**, allowing the same protocol to move naturally between:

```text
NONE
LOCAL
COMMUNITY
GLOBAL
```

without changing application substrate.

---

## 5. Holochain

### 5.1 Why Holochain is the closest conceptual comparison

Holochain is explicitly agent-centric.

Each agent maintains a signed source chain containing its local history. Public actions/data are transformed into DHT operations and sent to peers responsible for validating and storing the relevant portions of the DHT.

Validation is deterministic. Application integrity rules are shared by participants, and validators independently determine whether an action follows the application's rules.

Holochain also enforces source-chain continuity and detects forks where an agent attempts to create two parallel histories.

This overlaps strongly with OWP's intended model:

```text
actor-local history
+
signed actions
+
explicit dependencies
+
deterministic validation
+
distributed storage/verification
```

### 5.2 Validation authorities

Holochain does not have a globally elected validator committee.

Instead, DHT authority for data is distributed across peers according to DHT addressing/storage responsibility. Authorities validate the data they store and can generate signed validation receipts or warrants.

This provides decentralized validation without making every peer participate in global consensus.

That is a major architectural lesson for OWP.

OWP's `NONE` and many `LOCAL` operations might eventually benefit from a Holochain-like authority model rather than explicit committees.

### 5.3 Scarce resources and double-spending

This is the point where OWP's proposed model departs most significantly.

Holochain's own documentation acknowledges that conflicts over scarce resources are difficult. Source-chain analysis and fork detection can identify conflicting use, but a malicious agent can create an alternative timeline, and the documentation does not claim perfect prevention of arbitrary double-spend.

Holochain countersigning provides coordinated atomic writes among a defined set of counterparties. It can lock the participants' source chains during a signing session and support M-of-N optional witnesses.

However, Holochain explicitly states that countersigning is **not a general double-spend prevention mechanism**; it is intended to atomically synchronize a single write across multiple source chains.

OWP's COMMUNITY/GLOBAL proposal is stronger and more expensive:

```text
consumable state_ref
+
conflict locks
+
BFT quorum
+
finality
```

That is the clearest substantive difference between OWP and Holochain.

### 5.4 Sybil resistance

Holochain avoids a permanent globally elected validator set.

DHT authorities are selected through the network's distribution model rather than through proof-of-stake or a fixed global committee.

This reduces some forms of explicit validator governance but does not magically eliminate Sybil risk; network admission and DHT integrity still matter.

For OWP, Holochain suggests a future research direction:

> Could COMMUNITY consensus validators be sampled deterministically from a larger community membership/DHT instead of being manually appointed?

That should not be attempted in v0.2.

### 5.5 OWP versus Holochain

If OWP removed COMMUNITY/GLOBAL finality, it would risk becoming a less mature reimplementation of Holochain concepts.

Therefore the protocol must be evaluated primarily on this question:

> **Does scoped BFT finality over explicit consumable state create useful applications that Holochain's eventual validation/fork-detection model cannot support cleanly?**

If not, Holochain may be the better substrate.

---

## 6. W3C DIDs and Verifiable Credentials

### 6.1 What DIDs solve

DID Core defines a generalized decentralized identifier model.

A DID document can describe:

- controllers;
- verification methods;
- public keys;
- authentication relationships;
- service endpoints.

Different DID methods define how DID documents are created, resolved, updated, and deactivated.

OWP's current:

```text
actor_public_key -> actor_id
```

is substantially narrower.

That is acceptable for a v0.1 deterministic test profile, but OWP should not position its identity layer as superior to DIDs.

### 6.2 What Verifiable Credentials solve

VCs define a model for an issuer to make tamper-evident, cryptographically verifiable claims about a subject.

The standard centers around:

```text
issuer
holder
verifier
```

and provides interoperable semantics for credential issuance and presentation.

Examples include:

- identity assertions;
- professional certifications;
- memberships;
- capabilities;
- attestations.

### 6.3 What DIDs/VCs do not solve

DIDs/VCs deliberately do not define:

- a global P2P propagation network;
- an append-only action DAG;
- shared application state;
- distributed ordering;
- exclusive object consumption;
- Byzantine consensus;
- transaction finality.

They are therefore complementary rather than directly competitive.

### 6.4 OWP integration direction

OWP should eventually allow:

```text
actor = DID-controlled verification method
```

and should allow a validation rule to reference a VC as evidence.

For example:

```text
agent.delegate
    ↓
requires VC:
"Agent X is authorized by organization Y"
```

OWP should not invent its own generalized credential standard.

---

# 7. The two unresolved protocol questions

## 7.1 Question 1 — Sybil resistance and validator-set selection

### 7.1.1 Why this is fundamental

OWP v0.1's safety statement says:

```text
n = 3f + 1
QC = 2f + 1
```

This only means something if the validator set is itself meaningful.

If one attacker can create 7 keys and declare them to be the 7 validators, the BFT math provides no protection.

Therefore:

> **Consensus safety and validator governance are separate layers, and both are necessary for a production network.**

### 7.1.2 How the compared systems address or avoid it

#### Nostr

Nostr avoids a consensus validator set entirely.

Anyone can create keys and operate relays.

Trust is chosen by users/applications through relay selection, social graphs, payment/reputation signals, and application-level policies.

**Lesson:** web-of-trust can determine whose statements/services are useful, but Nostr does not convert that trust graph into BFT finality.

#### ATProto

ATProto also avoids a shared BFT validator set.

Each account has an authoritative repository hosted by a PDS, and identity portability prevents the hosting provider from being permanently tied to identity.

Relays replicate, but they do not collectively vote on every repository mutation.

**Lesson:** prefer explicit ownership/single-writer authority where possible; consensus is unnecessary for most state.

#### Holochain

Holochain distributes validation authority through DHT responsibility rather than a permanent elected global validator committee.

Authority assignment is part of the networking/data-placement architecture.

**Lesson:** deterministic or pseudo-random selection from a broader membership pool may eventually reduce manual committee governance.

#### DIDs / VCs

They do not define consensus validator sets.

Trust in credential issuers and DID methods is contextual.

**Lesson:** trust roots can be explicit and domain-specific rather than globally universal.

### 7.1.3 Realistic OWP options

#### Option A — Explicit federation

A COMMUNITY defines exactly which validator identities participate.

Example:

```text
Community Realm A
validators:
  - V1
  - V2
  - V3
  - V4
threshold: 3
```

Membership changes require a QC from the current validator set.

**Pros**

- simple;
- auditable;
- implementable now;
- strong fit for game realms, organizations, marketplaces, consortiums;
- no token needed.

**Cons**

- permissioned;
- validator capture/governance risk;
- community bootstrap requires trust.

**Recommendation:** use this for v0.2 testnet.

#### Option B — Community Proof of Authority

Validators correspond to known pseudonymous or organizational authorities.

Examples:

```text
guild operators
community maintainers
independent node operators
partner organizations
```

Technically similar to explicit federation, but with explicit governance semantics.

**Pros:** practical and understandable.

**Cons:** not permissionless.

**Recommendation:** likely first production COMMUNITY model.

#### Option C — Web-of-trust validator selection

Validator eligibility derives from endorsements/reputation.

Conceptually:

```text
actor trust graph
      ↓
eligible validators
      ↓
committee sampling
```

**Pros**

- native to pseudonymous communities;
- avoids financial stake.

**Cons**

- Sybil resistance is difficult;
- vulnerable to trust cartels;
- algorithm becomes political;
- hard to provide clean BFT assumptions.

**Recommendation:** research only, not v0.2.

#### Option D — Deterministic committee sampling

A larger accepted membership pool exists, and each consensus scope/epoch derives a smaller committee pseudorandomly.

This is closer to mechanisms used by distributed networks and resembles the random authority concept visible in Holochain's DHT.

**Pros**

- reduces static committee capture;
- scales better than all-members consensus.

**Cons**

- still requires a Sybil-resistant membership pool;
- needs unbiased randomness;
- increases protocol complexity.

**Recommendation:** potential v1+ direction.

#### Option E — Proof of Stake

Validator weight derives from a scarce economic asset.

**Pros**

- known permissionless pattern;
- attack has economic cost.

**Cons**

- requires token/economic design;
- creates financial/regulatory complexity;
- wealth becomes governance power;
- premature for OWP.

**Recommendation:** explicitly reject for v0.x.

#### Option F — External credential-based membership

Validator eligibility requires a VC or other external attestation.

Example:

```text
Community X recognizes validator credentials
issued by organizations A, B, C.
```

**Pros**

- can bootstrap real-world consortiums;
- interoperates with W3C VC ecosystem.

**Cons**

- pushes trust to credential issuers;
- less suitable for fully permissionless communities.

**Recommendation:** good future COMMUNITY profile.

### 7.1.4 Proposed v0.2 decision

OWP v0.2 SHOULD define:

```text
StaticValidatorSet
{
    scope_id
    epoch
    validators
    threshold
}
```

Validator-set transitions SHOULD require a QC from the old set over the new descriptor.

The genesis validator set is an explicit trust root.

This does not solve permissionless Sybil resistance.

That is acceptable.

The v0.2 claim should therefore be:

> **OWP demonstrates decentralized BFT finality among an explicitly configured community validator set.**

It should NOT claim:

> permissionless global consensus.

---

## 7.2 Question 2 — Liveness and stuck locks

### 7.2.1 The current weakness

The v0.1 lock rule protects safety but leaves liveness under-specified.

Consider:

```text
V1, V2 lock Action A
V3 receives Action B
network delays messages
round expires
```

If locks cannot safely move to later proposals, an object can remain unavailable indefinitely even though no action has finalized.

Safety without liveness produces frozen state.

### 7.2.2 What Tendermint teaches us

Tendermint uses repeated rounds with:

```text
PROPOSE
   ↓
PREVOTE
   ↓
PRECOMMIT
```

Validators can lock on a value.

A validator does not simply abandon a lock after a timeout. Unlocking/change of lock requires evidence from a later round—traditionally a supermajority prevote certificate / proof-of-lock-change for another value.

Timeouts advance the protocol through rounds, and proposer rotation eventually gives a well-connected honest proposer an opportunity to move the system forward under partial synchrony.

The key lesson is:

> **A timeout advances the view; it must not erase safety evidence.**

### 7.2.3 HotStuff-style direction

HotStuff-family protocols separate the notions of:

- proposals/views;
- quorum certificates;
- locking;
- a pacemaker/view-change mechanism.

The attraction for OWP is conceptual cleanliness around QCs and view progression.

However, implementing a correct HotStuff variant is still serious consensus engineering and would significantly expand v0.2.

### 7.2.4 Could the testnet use something simpler?

Yes, but only if we clearly constrain the claim.

A testnet-only protocol could use:

```text
one consensus instance per object conflict domain
round number
deterministic proposer rotation
proposal
prevote
precommit
QC
timeouts
higher-round lock-change evidence
```

This is essentially a small Tendermint-style state machine without blocks.

We do **not** need to implement an entire blockchain.

Instead of:

```text
height = block number
```

OWP can use:

```text
height = object/version transition
```

or a scope-local sequencing slot.

### 7.2.5 Option comparison

#### Option A — Keep v0.1 single-vote locks

**Pros:** minimal code.

**Cons:** can deadlock indefinitely.

**Verdict:** insufficient for v0.2 if we want meaningful failure testing.

#### Option B — Timeout and blindly clear locks

**Pros:** trivial liveness.

**Cons:** destroys safety; conflicting actions can obtain votes across rounds.

**Verdict:** unacceptable.

#### Option C — Central testnet sequencer resolves stuck proposals

**Pros:** extremely simple.

**Cons:** weakens the exact property v0.2 is intended to demonstrate.

**Verdict:** useful only as debugging infrastructure, not protocol finality.

#### Option D — Minimal Tendermint-style rounds

Implement:

```text
ROUND r

proposer proposes A
      ↓
validators PREVOTE
      ↓
2f+1 prevotes → lock evidence
      ↓
validators PRECOMMIT
      ↓
2f+1 precommits → FINAL

timeout
      ↓
ROUND r+1

safe lock change requires
higher-round certificate
```

**Pros**

- known safety/liveness model;
- easier to reason about than an ad hoc unlock rule;
- matches our existing `3f+1 / 2f+1` assumptions;
- allows partition/recovery tests.

**Cons**

- more messages;
- requires durable round/vote state;
- requires proposer/view logic.

**Verdict:** recommended for v0.2.

#### Option E — HotStuff-inspired full QC/pacemaker design

**Pros**

- elegant QC-driven model;
- attractive future architecture;
- potential for better communication properties in mature designs.

**Cons**

- significantly more protocol work;
- easiest path is not necessarily safest when implemented from scratch;
- unnecessary to prove the first OWP hypothesis.

**Verdict:** evaluate after testnet; do not make it a v0.2 prerequisite.

### 7.2.6 Proposed v0.2 liveness decision

Replace the simplistic v0.1 COMMUNITY/GLOBAL voting description in the implementation profile with a **minimal Tendermint-inspired per-scope/per-state consensus instance**.

Required state:

```text
scope
state_slot / object_version
round
step
locked_action
locked_round
valid_action
valid_round
votes
```

Required phases:

```text
PROPOSE
PREVOTE
PRECOMMIT
FINALIZE
```

Required persistence:

- last signed vote;
- round;
- current lock;
- lock round;
- finalized QC.

Required timeout behavior:

```text
timeout != unlock

timeout => next round
```

A lock can change only when higher-round quorum evidence justifies the change.

For the testnet, proposer selection can be deterministic round-robin over the static validator set.

This is enough to test liveness under:

- offline proposer;
- delayed proposal;
- validator restart;
- network partition;
- reconnect;
- conflicting proposals.

---

# 8. What should OWP borrow instead of invent?

## 8.1 From Nostr

Borrow the philosophy:

```text
signed objects should be independently verifiable
transport nodes should not automatically become authorities
```

Do not borrow Nostr's lack of state-finality when OWP needs scarce shared resources.

## 8.2 From ATProto

Borrow:

- separation of identity from hosting;
- content-addressed state;
- signed actor-owned repositories/logs;
- replaceable synchronization infrastructure;
- indexers as derived views rather than authorities.

A future OWP actor log could look more like an ATProto repository than a raw unstructured event bag.

## 8.3 From Holochain

Borrow heavily:

- agent-centric thinking;
- deterministic validation;
- explicit dependency references;
- local-first source histories;
- validator/authority separation from application users;
- peer validation as an immune system;
- caution around scarce resources.

OWP should document explicitly why its stronger BFT finality is worth the additional cost.

## 8.4 From DIDs / VCs

Borrow or integrate:

- DID-based identity portability;
- multiple verification methods;
- key rotation/controller semantics;
- VCs for external attestations;
- credentials as evidence for authorization/validator eligibility.

Do not build OWP-specific replacements unless there is a demonstrated requirement.

---

# 9. Is OWP actually a new protocol category?

The honest answer is:

**Not yet.**

Most of the current OWP stack independently already exists elsewhere:

```text
public-key identity          → Nostr / DIDs / Holochain / ATProto
signed events/actions        → Nostr / Holochain / ATProto
content addressing           → ATProto / Holochain / many others
causal references            → Holochain / DAG systems
distributed propagation      → Nostr / ATProto / Holochain
deterministic validation     → Holochain
credentials                  → W3C VC
```

The part that remains meaningfully distinctive is the combination:

```text
ONE ACTION MODEL
      +
EXPLICIT CONSENSUS SCOPE
      +
VERSIONED CONSUMABLE STATE
      +
SCOPED BFT FINALITY
      +
LOCAL-FIRST ACTORS
      +
FUTURE CAPABILITY-BASED AI AGENTS
```

The key idea is not that OWP is "more decentralized".

It is:

> **The same signed-action protocol can represent statements that need no consensus, local agreements, community-exclusive state, and globally exclusive state without forcing every action into one universal blockchain.**

That is an interesting hypothesis.

It is not yet proof that a new protocol is necessary.

---

# 10. Could OWP be an extension of something existing?

## 10.1 OWP over Nostr

Possible architecture:

```text
OWP Action
   ↓
Nostr event
   ↓
Nostr relays

OWP QC
   ↓
special Nostr event kind
```

This could reuse an enormous amount of relay/client infrastructure.

Problems:

- Nostr's canonical event and secp256k1 signature model would wrap or replace OWP's CBOR/Ed25519 model;
- large structured consensus messages may be awkward;
- relays have no obligation to provide the dependency/storage semantics OWP requires;
- transport and protocol semantics become coupled to Nostr conventions.

**Verdict:** plausible experiment for transport, weak candidate for OWP's core state engine.

## 10.2 OWP over ATProto

Possible:

```text
actor actions stored in signed repositories
+
external scoped consensus service
```

This is architecturally credible.

However, COMMUNITY/GLOBAL state would live outside the natural per-user repository authority model.

**Verdict:** strongest alternative if OWP evolves mostly toward social/application data and less toward shared scarce state.

## 10.3 OWP as a Holochain application/framework

This is the most serious alternative.

Holochain already supplies:

- agent identities;
- source chains;
- DHT;
- peer validation;
- deterministic integrity rules;
- distributed storage;
- capabilities.

OWP could potentially be implemented as a Holochain DNA with an additional BFT committee/finality layer for scarce resources.

**Verdict:** this should be prototyped or at least spike-tested before OWP v1.0. If the extra finality layer can be cleanly added, a standalone OWP network stack may not be justified.

## 10.4 OWP + DIDs/VCs

These are complementary.

**Verdict:** integrate rather than compete.

---

# 11. Recommended architecture decision

For v0.2:

```text
IDENTITY
Ed25519 v0.1 profile
future DID adapter

ACTOR HISTORY
signed actions
explicit refs
deterministic CBOR

NO-CONSENSUS DATA
replicated/gossiped directly

COMMUNITY/GLOBAL STATE
static explicit validator set
3f+1 assumption
Tendermint-inspired rounds
prevote/precommit
QC finality

NETWORK
libp2p experiment

CREDENTIALS
defer, later integrate W3C VC

AI AGENTS
defer capability model until core finality works
```

This intentionally treats validator membership as a trust-root configuration rather than pretending Sybil resistance has been solved.

---

# 12. Decision gates before v0.2 and v1.0

## Gate A — Before v0.2

We should be able to answer:

- Can conflicting state references be detected deterministically?
- Can exactly one conflicting action finalize under `< 1/3` Byzantine validators?
- Can validators restart without forgetting locks/votes?
- Can a stalled round move forward without violating safety?
- Can an explicitly federated validator set be replaced through a signed epoch transition?

If not, networking should wait.

## Gate B — During v0.2

Build one experiment using existing infrastructure:

```text
either:
OWP actions over Nostr relays

or:
OWP scarce-state prototype in Holochain
```

The goal is not production.

The goal is to determine whether OWP requires its own networking/state substrate.

## Gate C — Before calling OWP a new protocol

OWP must demonstrate a workload where:

1. Nostr-style signed events are insufficient because state is scarce/exclusive.
2. ATProto-style single-writer repositories are insufficient because state is jointly controlled.
3. Holochain-style eventual validation/fork detection is insufficient because pre-use finality is required.
4. A global blockchain is unnecessarily expensive because only a subset of actions require consensus.
5. OWP's scoped finality handles that workload more naturally than composing existing protocols manually.

If we cannot demonstrate all five, extending an existing protocol is probably the better engineering choice.

---

# 13. Final conclusion

## What does OWP do that none of the four does directly?

None of Nostr, ATProto, Holochain, or W3C DIDs/VCs directly provides this exact abstraction:

```text
Signed Action
   │
   ├── causal dependencies
   ├── deterministic validation
   ├── actor-owned/local-first creation
   └── declared consensus requirement
          │
          ├── NONE
          ├── LOCAL
          ├── COMMUNITY
          └── GLOBAL
                    │
                    ▼
        versioned consumable state
                    │
                    ▼
            scoped BFT finality
```

Nostr gives signed statements without shared finality.

ATProto gives portable, signed, authoritative per-account repositories.

Holochain gives agent-centric distributed validation and conflict detection without general strong global/shared scarce-state finality.

DIDs/VCs give interoperable identity and verifiable claims without a state machine or propagation network.

OWP's distinctive proposal is to make **the required coordination strength an explicit property of the action/state transition itself**.

## Is that enough to justify a new protocol?

**Potentially, but not yet.**

It justifies an experiment.

It does not yet justify reimplementing an entire identity, P2P, storage, and consensus ecosystem.

The intellectually honest position is:

> OWP should continue as a protocol experiment focused narrowly on **scoped finality for agent-authored actions**.

If that primitive produces a clean implementation and enables applications that are awkward on Nostr, ATProto, or Holochain—and avoids the cost of putting everything on a global blockchain—then OWP has a credible reason to exist.

If achieving it requires rebuilding mature capabilities from those ecosystems while providing only a thin composition layer, OWP should become an extension/profile on top of an existing substrate instead of an independent network.

The next version should therefore optimize for learning, not protocol pride.

The decisive v0.2 test is not:

```text
"Can OWP nodes talk to each other?"
```

It is:

```text
"Can independent actors safely and live-ly finalize
a shared scarce state transition at COMMUNITY scope,
while unrelated actions remain consensus-free?"
```

If OWP can make that boundary simple, deterministic, interoperable, and useful, then the new protocol has a defensible technical thesis.

---

# 14. Primary references

## Nostr

- NIP-01: Basic protocol flow, events, signatures, relays  
  https://github.com/nostr-protocol/nips/blob/master/01.md

## AT Protocol

- Account model  
  https://atproto.com/specs/account
- Repository specification  
  https://atproto.com/specs/repository
- Sync / firehose specification  
  https://atproto.com/specs/sync

## Holochain

- Validation and deterministic integrity rules  
  https://developer.holochain.org/concepts/7_validation/
- DHT architecture  
  https://developer.holochain.org/concepts/4_dht/
- Countersigning  
  https://developer.holochain.org/concepts/10_countersigning/
- Agent status / scarce resource conflict discussion  
  https://developer.holochain.org/build/getting-an-agents-status/

## W3C

- Decentralized Identifiers (DID) Core v1.0  
  https://www.w3.org/TR/did-core/
- Verifiable Credentials Data Model v2.0  
  https://www.w3.org/TR/vc-data-model-2.0/

## Consensus background

- Tendermint consensus specification  
  https://docs.tendermint.com/master/spec/consensus/consensus
