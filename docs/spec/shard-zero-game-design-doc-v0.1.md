# Shard Cero — Game Design Document v0.1

**Status:** Product/Game Design Draft  
**Scope:** First playable shard only  
**North Star:** A persistent parallel world where a player's character is their cryptographic identity and the world can outlive its creators.  
**Important:** The world's tone, fiction, visual style, names, lore, historical period, and aesthetic are intentionally **TBD**.

---

## 1. Product thesis

Shard Cero is not a technology demo. It is the smallest version of a persistent world that can become a place people choose to revisit.

The decentralization layer is infrastructure. Players should not need to understand keys, consensus, DAGs, validators, wallets, or cryptography to enjoy the game.

The first question is not:

> Can the world survive its creators?

It is:

> Is there something here I want to come back to tomorrow?

Only after that is true does persistence become emotionally valuable.

### Principles

1. **No token and no play-to-earn.**
2. **Fun first, decentralization second.**
3. **Free entry without cryptocurrency knowledge.**
4. **The player owns a persistent cryptographic character identity.**
5. **AI residents are citizens, not chat widgets.**
6. **The first economy is deliberately tiny.**
7. **Unique items are the first scarce shared state.**
8. **The protocol remains invisible during normal play.**
9. **No creator-operated server may become logically indispensable to the final architecture.**
10. **Shard Cero must be interesting with tens of humans, not require thousands.**

---

## 2. Shard Cero

Shard Cero contains:

- one city;
- dozens, not thousands, of founding human players;
- five initial AI residents;
- player workshops;
- a market;
- a small set of gatherable/common crafting inputs;
- unique crafted objects;
- player-to-player trade;
- a persistent public history of important objects.

The city is a **COMMUNITY consensus domain**.

For the first testnet, its validator set is explicit and federated. The player never needs to know this.

Shard Cero intentionally does **not** initially contain:

- cryptocurrency;
- land speculation;
- combat;
- guild wars;
- global world map;
- hundreds of crafting professions;
- procedural infinite content;
- player-created tokens;
- NFTs marketed as financial assets;
- an auction house with thousands of commodities;
- fully autonomous unrestricted AI agents.

---

# 3. Core player fantasy

The first playable fantasy is:

> **I arrived in a living place, made something that did not exist before, gave it a history, interacted with residents who remember what happened, and tomorrow the consequences will still be there.**

The valuable object is not merely an inventory entry.

A unique item accumulates provenance:

```text
created
   ↓
named
   ↓
owned
   ↓
used / discussed
   ↓
traded
   ↓
owned by someone else
   ↓
continues existing
```

This gives persistence a visible gameplay meaning.

---

# 4. Core loop

```text
Enter world
   ↓
Discover a need/opportunity
   ↓
Acquire materials
   ↓
Craft or commission an item
   ↓
Create a unique object
   ↓
Use / show / trade it
   ↓
World and residents react
   ↓
Object gains history
   ↓
Return to see what changed
```

The retention mechanism is not grinding currency.

It is the combination of:

**ownership + consequence + social recognition + changing opportunities + persistent history.**

---

# 5. First 10 minutes

The first session must prove the fantasy before explaining the system.

## Minute 0–1 — Become someone

The player launches the client and creates a character.

Behind the scenes:

```text
device
  ↓
cryptographic identity generated
  ↓
character bound to identity
```

The UI does not say:

> Generate Ed25519 keypair.

It says something equivalent to:

> Create your character.

The player chooses only minimal identity information needed for play. Recovery/backup is offered after initial engagement rather than becoming a crypto onboarding wall.

**Desired feeling:** "This person is mine."

---

## Minute 1–3 — Enter a place already in motion

The player enters the single city.

They immediately see evidence of activity:

- humans;
- resident agents;
- workshops;
- market activity;
- recent item transfers;
- requests/commissions.

No lore dump.

One resident notices the newcomer and provides a contextual introduction.

The game gives one immediate objective:

> Make your first object.

---

## Minute 3–5 — Get materials

The player receives a small starter allocation of common materials.

These are deliberately not economically valuable.

For example, abstractly:

```text
material A
material B
material C
```

Actual material names depend on the future world theme.

The player chooses between 2–3 simple recipes.

This is the first meaningful decision.

Recipes should differ in purpose rather than just stats.

Example abstract categories:

```text
utility object
social/display object
crafting tool
```

---

## Minute 5–7 — Craft something unique

The player crafts.

The resulting object receives a unique protocol identity/state reference.

Example conceptual record:

```text
Item #01F...
Creator: Player
Created: Day 1
Recipe: ...
Properties: ...
Owner: Player
```

The player can name the item.

This is the first "magic moment":

> **This exact object did not exist before I made it, and there cannot be two authoritative owners of it.**

Do not explain consensus.

Show permanence.

---

## Minute 7–9 — Someone wants it

An AI resident reacts to the new object.

Depending on what was crafted, a resident might:

- ask about it;
- offer to trade for it;
- recommend another player;
- propose a commission;
- explain that someone in the city needs that category of object.

This interaction should be partially systemic and partially generated.

The agent remembers the encounter.

---

## Minute 9–10 — First consequential choice

The player chooses:

```text
KEEP
TRADE
OFFER FOR SALE
GIFT
```

If transferred:

```text
Player A owns Item X
        ↓
signed transfer
        ↓
COMMUNITY consensus
        ↓
Player B owns Item X
```

The client presents only:

> Transfer completed.

The first ten minutes end with a visible consequence.

Example:

```text
Your first object:
[Name]

Created by you
Current owner: Resident / You
History: 1–2 events
```

And one unresolved hook for tomorrow.

---

# 6. Why the player returns on Day 2

Day 2 must not simply say:

> Craft another item.

The world must have moved without the player.

The core Day-2 promise is:

> **Things happened while you were gone.**

On return, the player receives a compact world update.

Examples:

```text
The item you sold yesterday changed hands.

A resident remembered your work and requested another object.

A player inspected one of your creations.

Demand changed for one recipe category.

A resident who owes you something is available again.

Someone has offered to trade for an object you kept.
```

Not all events happen every day.

At least one should be directly connected to the player's prior actions.

---

# 7. Day 2 loop

## Step 1 — "While you were away"

Show 1–3 meaningful changes.

Avoid generic notifications.

Good:

> The object you created yesterday now belongs to another player.

Bad:

> 17 market events occurred.

The system should prioritize **personal consequence**.

## Step 2 — Resident continuity

One AI resident recognizes the player based on yesterday.

Example behavioral structure:

```text
Agent:
met player yesterday
remembers first crafted item
knows whether player accepted/rejected offer
updates relationship state
```

The resident does not need perfect natural-language memory.

It needs **continuity**.

## Step 3 — New opportunity

The player discovers one changing economic opportunity.

Examples:

- a resident wants a specific item property;
- a recipe input is temporarily scarce;
- another player posted a commission;
- a particular craft category has excess supply;
- an unusual combination has been discovered.

## Step 4 — Decision

The player chooses how to respond:

```text
craft
trade
commission
hold
gift
negotiate
```

## Step 5 — Persistent consequence

The action changes:

- item ownership;
- item history;
- relationship memory;
- market state;
- future agent behavior.

This completes the retention loop.

---

# 8. Initial economy: craft + trade unique items

The first economy must exercise the protocol without becoming an economics simulator.

There are three fundamental resource classes.

## 8.1 Common materials

Common materials are inputs.

They can initially be issued by deterministic game systems.

They are not intended as collectibles.

Their purpose is to create crafting choices and economic friction.

Examples abstractly:

```text
Material A
Material B
Material C
Material D
```

Supply can refresh through daily activities.

We should avoid making every grain of material require BFT finality in v0.3.

Balances of fungible/common resources can use simpler account/state semantics inside the community.

---

## 8.2 Unique items

Unique items are the heart of the first economy.

Every crafted unique item has:

```text
item_id
version
creator
current_owner
recipe
properties
created_at
provenance
```

Conceptually:

```text
Item X v1
owner = Alice
      ↓
transfer
      ↓
Item X v2
owner = Bob
      ↓
transfer
      ↓
Item X v3
owner = Carol
```

The previous state is consumed.

This directly exercises OWP's anti-double-transfer mechanism.

---

# 9. Why an item becomes desirable

Scarcity alone is not enough.

Items need reasons to matter.

The initial system should combine four sources.

### Utility

An item provides some gameplay function.

### Craft signature

Crafted items contain variations influenced by:

- recipe;
- materials;
- crafter;
- controlled randomness;
- crafting conditions.

### Provenance

The game records significant history.

```text
Crafted by ...
Previously owned by ...
Used in ...
Gifted by ...
```

### Social meaning

Humans and residents can attach meaning to objects through their interactions.

A technically identical item with history can become more interesting than a new one.

---

# 10. Crafting model

Crafting must be easy to understand but produce variation.

Conceptually:

```text
Recipe
+
Materials
+
Crafter characteristics
+
Small deterministic/random seed
+
Optional technique choice
=
Unique Item
```

The server/network must agree on the resulting properties.

The client cannot declare its own rare result.

For v0.3, keep approximately:

```text
3 material families
5–8 recipes
3 meaningful properties
3 quality bands
```

No giant crafting tree.

---

# 11. Trading

Supported initial transfers:

```text
direct sale
direct swap
gift
resident purchase
resident commission
```

Do not build an auction house initially.

Human negotiation is more socially valuable at Shard Cero scale.

A trade involving a unique item becomes a consensus-requiring state transition.

Example:

```text
Alice owns X
Bob owns Y

proposal:
Alice → X
Bob   → Y

        ↓

atomic trade

        ↓

Alice owns Y
Bob owns X
```

Atomic multi-object swaps may be delayed until after single-item transfer is stable.

---

# 12. Money without cryptocurrency

Shard Cero needs a medium of exchange, but not a token.

Use an **in-world game currency**.

Properties:

- no blockchain marketing;
- no external market;
- no cash-out;
- no promise of monetary value;
- cannot be purchased/sold as an investment in the initial design;
- exists only as a gameplay accounting mechanism.

Its supply is controlled by game economic rules.

This allows us to study:

- prices;
- trade;
- scarcity;
- agent behavior;
- inflation;
- market concentration;

without turning the project into play-to-earn.

---

# 13. Economic faucets and sinks

A closed economy needs deliberate flows.

## Faucets

Currency enters through:

- resident commissions;
- city jobs;
- initial player allocation;
- limited system purchases.

## Sinks

Currency leaves through:

- crafting fees;
- material acquisition;
- repairs/services if introduced;
- resident services;
- optional customization.

The target is not a perfectly stable economy in v0.3.

The target is:

> Players regularly face interesting choices about what to make, keep, sell, and buy.

---

# 14. Market observability

Players should be able to see enough information to reason about the economy.

Initial market view:

```text
recent trades
open offers
resident requests
recently created unique items
```

Avoid:

```text
candlestick charts
financial terminology
token prices
APY
investment language
```

This is a game economy.

---

# 15. The five initial AI residents

The agents are not omniscient game masters.

Each has:

```text
identity
role
goals
private working memory
persistent structured memory
relationships
budget
allowed actions
economic constraints
```

They operate under capability limits.

Their model provider must be replaceable; the world cannot depend permanently on one AI API.

---

## Resident 1 — The Craft Mentor

**Role**

Introduces crafting and helps players understand material/recipe relationships.

**Economic role**

- issues a small number of starter commissions;
- purchases selected beginner items;
- creates early demand without buying everything.

**Memory**

Persistent structured facts:

```text
players met
first item crafted
recipes discussed
commissions offered
commissions completed
notable player preferences
relationship score/state
```

Keep conversational summaries only when useful.

**Behavior**

Does not simply answer questions.

It occasionally needs objects and generates commissions consistent with a bounded schedule.

**Inference budget**

Target:

```text
low
~1–3 meaningful model calls per active player/day
```

Most onboarding behavior should be deterministic/templates + small-model inference.

High-cost inference is reserved for memorable conversations.

---

## Resident 2 — The Merchant

**Role**

Creates market liquidity and communicates price/demand information through behavior.

**Economic role**

- buys selected item categories;
- sells common materials;
- posts requests;
- adjusts willingness-to-pay within bounded rules.

**Memory**

```text
recent market prices
inventory
trades with each player
negotiation outcomes
repeat customers
unusual objects seen
```

Market state itself should be retrieved from deterministic game data rather than memorized by the LLM.

**Behavior**

Can negotiate inside a bounded price corridor.

The model may choose dialogue/strategy.

The economic engine validates the final price.

**Inference budget**

```text
medium
~20–50 calls/day for entire Shard Cero initially
```

Batch market reasoning where possible.

Do not invoke an LLM for every price calculation.

---

## Resident 3 — The Collector

**Role**

Creates non-utilitarian demand and gives provenance emotional value.

**Economic role**

Seeks unusual items according to changing preferences.

Examples abstractly:

```text
items from new crafters
items with property X
items that changed hands multiple times
items connected to particular events
```

**Memory**

```text
collection
objects desired
objects previously rejected
creators followed
item histories
promises made
personal preferences
```

**Behavior**

The Collector can become attached to specific objects.

This creates stories such as:

> A resident has been trying to acquire your first crafted object for three days.

**Inference budget**

```text
medium-low
5–15 calls/day
```

Most candidate filtering happens deterministically.

LLM inference chooses among a small candidate set and generates interaction.

---

## Resident 4 — The Broker

**Role**

Connects supply with demand.

**Economic role**

Does not manufacture demand from nothing.

It discovers:

```text
Player A needs X
Player B can make X
Resident C owns relevant material
```

and creates opportunities.

**Memory**

```text
known player specialties
recent requests
successful introductions
failed introductions
trust/reliability observations
open commitments
```

**Behavior**

May tell a player:

> Someone in the city is looking for the kind of item you made yesterday.

This resident makes a small population feel interconnected.

**Inference budget**

```text
medium
10–30 calls/day
```

Graph/database queries identify matches first; LLM calls are used for prioritization and dialogue.

---

## Resident 5 — The Archivist

**Role**

Makes persistence visible.

This may be the most important agent for the product thesis.

**Economic role**

Minimal direct market intervention.

May occasionally commission historically interesting objects or documentation-like actions.

**Memory**

The Archivist does not rely primarily on LLM memory.

Its memory is the world's verifiable history:

```text
item provenance
creators
owners
important transfers
firsts
rare events
relationships between objects/events
```

It maintains a searchable derived index of finalized world actions.

**Behavior**

Can recognize:

```text
the first object ever crafted
the first player-to-player trade
an item with five owners
a creator's oldest surviving work
```

It turns protocol history into world culture.

**Inference budget**

```text
low-medium
5–20 calls/day
```

Retrieval is deterministic/database-based.

Inference turns retrieved facts into contextual conversation or summaries.

The LLM must never invent historical facts.

---

# 16. Agent architecture

All five residents follow the same high-level model:

```text
Persistent Agent Identity
        ↓
Role + Goals + Capabilities
        ↓
Structured Persistent Memory
        ↓
World-state retrieval
        ↓
Decision policy
        ↓
Optional LLM inference
        ↓
Proposed action
        ↓
Game-rule validation
        ↓
Signed action
        ↓
World
```

The LLM never directly mutates world state.

It can only **propose** actions.

The deterministic game engine validates them.

---

# 17. Agent memory model

Do not store entire chat histories forever as the primary memory.

Use four layers.

### Identity memory

Stable facts about the resident itself.

### Relationship memory

Per-player structured state.

```text
relationship
last_interaction
important_events
promises
preferences_learned
```

### Episodic memory

Selected summaries of notable encounters.

### World retrieval

Queryable factual state from the game.

This is authoritative for:

- ownership;
- balances;
- prices;
- provenance;
- finalized events.

Agents cannot "remember" a different owner than the world state.

---

# 18. Inference budget architecture

AI cost must scale with **interesting decisions**, not simulation ticks.

Never:

```text
every agent
× every few seconds
× LLM call
```

Instead:

```text
deterministic simulation
        ↓
interesting trigger?
        │
       yes
        ↓
retrieve relevant memory/state
        ↓
small model / policy
        ↓
need rich reasoning/dialogue?
        │
       yes
        ↓
larger model
```

Initial target for all five residents combined:

```text
normal idle world:
near-zero inference

active human interaction:
1–3 calls per meaningful encounter

background planning:
batched, scheduled, bounded
```

The testnet should record:

```text
calls / agent / day
tokens / agent / day
cost / active player / day
actions produced / inference call
player interactions / inference call
```

A resident that costs money but creates no retained behavior is a product failure.

---

# 19. What makes agents citizens rather than NPC chatbots?

An AI resident qualifies as a world citizen only if it has:

1. persistent cryptographic identity;
2. continuity across sessions;
3. persistent memory;
4. limited resources;
5. economic agency;
6. explicit capabilities;
7. ability to own items;
8. ability to sign/propose actions;
9. consequences for previous decisions;
10. no privileged bypass around world rules.

This last rule matters:

> **AI residents obey the same ownership and transfer rules as humans.**

An agent cannot duplicate an item simply because its model generated a tool call.

---

# 20. Cryptographic identity UX

The player's identity should feel like a game account, not a wallet.

First session:

```text
Create character
      ↓
key generated locally
      ↓
play
```

Later:

```text
Protect your character
      ↓
recovery/backup flow
```

The product should progressively reveal ownership concepts only when they matter.

No seed phrase should appear in the first ten minutes unless technically unavoidable.

---

# 21. What decentralization must prove

Shard Cero does not need maximal decentralization on day one.

It must establish an evolutionary path where:

```text
official nodes disappear
        ↓
community nodes remain
        ↓
world state remains verifiable
        ↓
characters/items remain valid
        ↓
new compatible clients can connect
```

The product promise is not:

> Our servers have high uptime.

It is:

> **The world's continuity is not owned by us.**

---

# 22. Roadmap

## v0.1 — Consistent engine

Goal:

Prove deterministic state and exclusive ownership.

Required:

- run all Cargo tests;
- deterministic CBOR/signatures;
- 10,000-action simulation;
- double-transfer tests;
- persisted locks;
- deterministic final state hashes.

No gameplay requirement yet.

---

## v0.2 — Three-node community consensus

Goal:

Prove Shard Cero can maintain exclusive state without one authoritative server.

Required:

- three+ independent nodes;
- static validator set;
- COMMUNITY scope;
- Tendermint-style rounds;
- PROPOSE/PREVOTE/PRECOMMIT;
- restart recovery;
- partition tests;
- double-transfer attack test;
- libp2p transport.

Success:

> Under the stated Byzantine assumptions, one unique item cannot finalize with two owners, and the network resumes progress after recoverable failures.

---

## v0.3 — Playable Shard Cero

Goal:

Prove the game loop.

Required:

- character creation;
- one city;
- crafting;
- unique items;
- direct trade;
- game currency;
- provenance;
- simple market;
- no mandatory crypto knowledge.

Critical metric:

```text
Do testers voluntarily return?
```

Infrastructure metrics are secondary.

---

## v0.4 — AI residents

Goal:

Make the city feel persistent and socially alive with a small human population.

Introduce the five residents gradually.

Measure:

- conversations initiated;
- commissions accepted;
- agent-influenced trades;
- repeat interactions;
- cost per active player;
- whether players recognize/remember residents.

---

## v0.5 — Founders

Goal:

Invite the first real community.

Target:

```text
tens of humans
not thousands
```

Founders should enter a world that already contains:

- history;
- residents;
- objects;
- economic activity.

The first cohort becomes part of permanent Shard Cero history.

---

# 23. Product success metrics

The north-star criterion is:

> **People come back without being paid.**

For the founder test, track:

```text
D1 retention
D2 return rate
D7 retention
sessions per player/week
meaningful trades/player
items created/player
items changing owner
resident interactions/player
percentage of items with >1 owner
percentage of returning players who inspect "while away"
```

Avoid optimizing early for:

```text
registered wallets
transaction count
node count
token volume
TVL
```

Those would measure the infrastructure rather than whether a world exists that people care about.

---

# 24. Primary product experiment

Shard Cero has one central hypothesis:

> **Persistent ownership + provenance + a small living economy + residents who remember you can make a tiny online world worth revisiting even before combat, progression systems, or a large population exist.**

The experiment fails if players understand that the world is decentralized but do not want to return.

It succeeds if players care about:

```text
what happened to my item?
what does that resident remember?
what is happening in the market?
what can I make today?
who owns that object now?
```

At that point, decentralization stops being a feature checklist.

It becomes the mechanism that guarantees that the world and its history can belong to its inhabitants.

---

# 25. Deliberately unresolved creative decisions

This document does **not** define:

- world name;
- visual style;
- historical period;
- fantasy/science-fiction/realist setting;
- factions;
- lore;
- city name;
- resident names;
- material names;
- recipe fiction;
- tone of dialogue;
- character art direction;
- UI art direction.

Those are founder-level creative decisions and should be decided separately before content production.

The mechanics above are designed to remain valid regardless of that choice.
