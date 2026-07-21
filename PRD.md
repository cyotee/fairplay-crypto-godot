# PRD — FairPlay Crypto for Godot

| Field | Value |
|-------|--------|
| **Product** | FairPlay Crypto for Godot |
| **Repo** | [cyotee/fairplay-crypto-godot](https://github.com/cyotee/fairplay-crypto-godot) |
| **Status** | Planning (implementation after PRD acceptance + implementation plan) |
| **License** | **MIT** (entire library, samples, and docs — no commercial split) |
| **Primary engine** | **Godot 4** (Redot compatibility as a non-blocking goal) |
| **Distribution** | Open source: GitHub, Godot Asset Library (when ready), git submodule / clone |
| **Reference stacks** | C#: [cyotee/fairplay-crypto](https://github.com/cyotee/fairplay-crypto) (`ManaMesh.Crypto`); TS: `@manamesh/boardgameio-crypto` in manamesh-games |
| **Monorepo path** | `packages/fairplay-crypto-godot` submodule of [manamesh-games](https://github.com/cyotee/manamesh-games) |
| **Last updated** | 2026-07-20 |

---

## 0. Requirements decisions (locked)

These define the pivot from the Unity-oriented FairPlay line to a **Godot-first MIT library**.

| # | Topic | Decision |
|---|--------|----------|
| 1 | **Primary product** | A **Godot 4** fair-play crypto library other developers can use under **MIT**. |
| 2 | **Licensing** | **MIT for everything** published in this repo (core, Godot addon, samples, docs). No dual open-core / paid engine package in this product. |
| 3 | **Relationship to fairplay-crypto** | **Sibling product**, not a thin wrapper of the Unity commercial package. Algorithms and policies may share design intent with `ManaMesh.Crypto` and `boardgameio-crypto`; **no required binary/runtime dependency** on the Unity package. Reuse of ideas, tests, and optional golden vectors is encouraged. |
| 4 | **Relationship to TypeScript** | TS package is **inspiration and behavioral reference**, not a binding API contract. Drift freely for Godot idioms. |
| 5 | **First milestone** | **Core crypto + automated tests + Godot samples** (local / offline / simulated multiplayer). Full multiplayer demo game is **later**. |
| 6 | **API design** | Prefer **Godot-native ergonomics** (GDScript-facing API, Godot types at the boundary where helpful, clear docs). Do not force C#/TS shapes 1:1. |
| 7 | **Core purity** | Crypto core must not depend on a specific netcode stack, lobby, or game rules engine. **Bring-your-own-netcode.** |
| 8 | **Private keys** | **Never** put private keys in shared multiplayer state or network payloads. APIs and samples make the safe path the default. |
| 9 | **Shuffle proofs** | **In scope and essential** as integrity / anti-fraud signals. Ship commit–reveal first; document the real threat model. True ZK shuffle is a later bar if claimed. |
| 10 | **Security assurance** | **No external audit budget.** Extensive tests, threat-model docs, honest claims. |
| 11 | **Game domain** | Core stays **game-agnostic**. Poker / battleship / deck genre terms live in **samples**, not core public type names. |
| 12 | **Marketplace positioning** | Multiplayer / P2P cryptography for securing secrets and verifying fairness in **any genre**. Not gambling; not cryptocurrency. Card demos are illustrative only. |
| 13 | **Branding** | Product name: **FairPlay Crypto for Godot** (repo: `fairplay-crypto-godot`). Optional **ManaMesh** credit in docs as originating project; do **not** require ManaMesh branding for third-party consumers. Avoid “boardgame.io” in public package naming. |
| 14 | **Workflow** | Day-to-day work in this repo (submodule under `manamesh-games`); bump submodule pointer in the parent monorepo when ready. |
| 15 | **Export targets** | Aim for Godot 4 exports that a pure / GDExtension-friendly design can support: **desktop, mobile, and web (HTML5)** where crypto backend allows. Document any platform gaps honestly. |

### Two different “done” bars

| Bar | Meaning |
|-----|---------|
| **Milestone A — First shippable engineering bar** | Crypto core (P0/P1 fair-play modules) + tests + Godot samples (offline / simulated multi-party). Usable from a Godot project via the documented install path. Not necessarily Asset Library listing. |
| **Milestone B — Public publish bar** | Milestone A **plus** advanced modules required for public claims (Shamir / threshold suite / Paillier and/or ZK as scoped) **plus** Godot packaging polish, docs, sample scenes, and platform validation (desktop + at least a plan for mobile/web). Eligible for Godot Asset Library submission. |

---

## 1. Problem

Competitive multiplayer games need **provably fair** dealing, hidden placement, and cooperative reveal when peers do not trust a central dealer or each other.

The ManaMesh ecosystem already has:

- **TypeScript** fair-play crypto for browser / boardgame.io (`boardgameio-crypto`)
- **C#** open core + Unity packaging path (`fairplay-crypto` / `ManaMesh.Crypto`)

**Godot** studios cannot cleanly consume either as a first-class Godot addon. They need the same *class of capabilities* packaged for Godot 4 under a permissive license other developers will actually adopt.

---

## 2. Product vision

Ship an **MIT Godot 4 library** that:

1. Provides multi-party fair-play primitives (mental poker / SRA, keychain admission, Merkle, shuffle integrity, and later threshold / HE / ZK modules as published)
2. Exposes a **clear GDScript-first API** (with optional C# consumption if the architecture supports it)
3. Stays **engine-integration-thin**: crypto + protocol DTOs + samples; no mandated multiplayer framework
4. Is **easy to vendor**: clone, submodule, or Asset Library install

**Framing:**  
**FairPlay Crypto for Godot** — cryptography for securing secrets and verifying integrity among untrusted peers in multiplayer/P2P games.  
**Not** “boardgame.io for Godot.” **Not** a gambling or cryptocurrency product.

---

## 3. Goals

### 3.1 Product goals

- Multi-party **commutative encryption (SRA)** decks: key exchange → layered encrypt → shuffle → deal → cooperative peel / reveal  
- **Public-key keychain admission** (curve checks, uniqueness policies)  
- **Private keys stay local** — safe defaults in API and samples  
- **Merkle commitments** for hidden placement / board-style games  
- **Shuffle integrity proofs** as a first-class feature (honest threat model)  
- **Threshold / HE / ZK modules** before Milestone B public claims that need them  
- **MIT** adoption path for indie and commercial Godot projects alike  
- Three headline samples: mental-poker loop, poker-shaped flow, Merkle battleship  

### 3.2 Engineering goals

- **Pure crypto core** free of game rules and netcode frameworks  
- **Idiomatic Godot** surface (signals, Resources, documented autoload patterns as appropriate) over cloning C#/TS APIs  
- **Extensive automated tests** as primary assurance  
- Prefer designs that do **not** require hand-rolled secp256k1  
- Document install for Godot 4.x (minimum version chosen in implementation spike)  
- Samples run with **in-process multi-seat simulation** before real networking demos  

### 3.3 Community / distribution goals

- Publish as a library other Godot developers can use without commercial friction  
- Godot Asset Library submission at Milestone B  
- Clear README + threat model + algorithm notes so non-cryptographers can integrate safely  

---

## 4. Non-goals

| Non-goal | Rationale |
|----------|-----------|
| Replace `@manamesh/boardgameio-crypto` for web | TS remains browser/ManaMesh web reference |
| Replace or absorb Unity commercial packaging | Lives in `fairplay-crypto`; this repo is Godot MIT |
| Bit-identical wire format with TS or C# | Drift allowed; optional golden vectors for regression only |
| Full multiplayer demo game in Milestone A | Explicitly later |
| Required dependency on a specific Godot multiplayer stack | BYO netcode (`MultiplayerAPI`, custom WebRTC, etc.) |
| Gambling / cryptocurrency product positioning | Fair-play & secret security for general games |
| External security audit before launch | No budget; testing + docs instead |
| Real-money gambling features | Legal / product risk |
| Putting poker / battleship domain types in the open core | Domain stays in samples |
| Godot 3 support | Godot 4 only unless a later decision revisits |

---

## 5. Users, use cases, and sample strategy

### 5.1 Personas

| Persona | Needs |
|---------|--------|
| Godot indie / studio | MIT addon, GDScript examples, clear docs, no paid lock-in |
| Protocol-minded developer | Keychain, peels, shuffle integrity, Merkle — not a full game framework |
| Multiplayer implementer | Transport-agnostic DTOs / message shapes; local simulator for tests |
| ManaMesh contributor | Side-by-side submodule next to C# and TS references |

### 5.2 Core use cases (library)

1. Mental poker deal / reveal loop  
2. Keychain seat admission  
3. Client-side encrypt binding (local sk matches published pk)  
4. Merkle commit / open  
5. Shuffle commit → reveal / verify integrity  
6. (Publish bar) threshold, Paillier, ZK helpers as scoped  

### 5.3 Headline samples (Milestone A)

| Sample | Demonstrates |
|--------|----------------|
| **Full mental-poker loop** | Keychain admit → layered encrypt → shuffle + proof material → deal → cooperative peel / reveal |
| **Poker-shaped flow** | Same primitives in a Hold’em-*shaped* sequence (not a full rules engine) |
| **Merkle battleship** | Commit board / placement → challenge / open with Merkle proofs |

Samples are **teaching demos**, not the core product API. Full multiplayer UX (real netcode, matchmaking) remains **out of Milestone A**.

### 5.4 Library layering

| Layer | Area | Allowed | Forbidden |
|-------|------|---------|-----------|
| **L0 Core** | Crypto / protocol primitives | key, keychain, ciphertext, layer, peel, commitment, shuffle proof, Merkle tree/proof, policy | Poker ranks, ship grids, Godot scene trees as required deps for math |
| **L1 Godot integration** | Addon / plugin surface | GDScript/C# bindings, Resources, optional autoload helpers, editor convenience | Reimplementing crypto math twice |
| **L2 Samples** | `samples/` or `addons/.../examples` | Didactic game terms, demo UI | Leaking domain types into L0 public API |

**Principle:** L0 stays reusable and testable outside a running game tree where practical. L1 is the Godot-facing package other developers install.

---

## 6. Architecture direction (to finalize in implementation plan)

This PRD **does not lock a single native backend** until a short spike, but it constrains choices.

### 6.1 Required outcomes

1. **GDScript-callable** public API for Milestone A samples (primary Godot audience).  
2. **Secure, maintained** elliptic-curve / big-integer stack — no hand-rolled secp256k1.  
3. **Automated tests** runnable in CI without launching the full editor for core math (editor/headless tests for integration as needed).  
4. **Export story** documented for desktop; mobile/web gaps called out early.  

### 6.2 Candidate implementation shapes (pick in Phase 0)

| Option | Pros | Cons |
|--------|------|------|
| **A. GDExtension (Rust preferred)** | Strong crypto ecosystem; GDScript-native UX; good performance | Build matrix, export templates, learning curve |
| **B. Godot C# consuming / porting `ManaMesh.Crypto`** | Fastest algorithm parity with sibling C# repo | C# project setup; GDScript users need a bridge or dual API |
| **C. Pure GDScript + third-party crypto** | Easiest install | Likely weak/unsafe for production EC crypto; not preferred for core |

**Default recommendation for planning:** **Option A (Rust GDExtension)** for a GDScript-first MIT library, with algorithm parity inspired by C#/TS; **or Option B** if the team prioritizes fastest reuse of the existing C# core and accepts C#-centric packaging. Phase 0 spike must pick one primary path.

### 6.3 Rules for the crypto core

1. Never put private keys in shared multiplayer state or network payloads — ciphertexts / peels / commitments only  
2. No hard dependency on a specific multiplayer addon  
3. No hand-rolled secp256k1  
4. Keychain-style admission for public keys  
5. Client-side bind: local sk matches published pk before encrypt  
6. Document integrity vs zero-knowledge honestly  
7. Prefer byte-oriented secrets; hex/base64 only at boundaries  

### 6.4 Netcode

**Bring your own.** Ship:

- Protocol-oriented DTOs / dictionaries (documented fields)  
- Optional message sink/source interfaces or Godot signals  
- An **in-process multi-seat simulator** for tests and samples  

No required dependency on a particular lobby, relay, or WebRTC stack.

---

## 7. Relationship to sibling packages

| Concern | Decision |
|---------|----------|
| `boardgameio-crypto` (TS) | Behavioral reference only |
| `fairplay-crypto` (C# / Unity) | Sibling algorithms and docs; optional vector sharing; **not** a runtime dependency of this Godot package |
| This repo | Godot 4 MIT product |
| Monorepo link | Submodule of `manamesh-games` at `packages/fairplay-crypto-godot` |
| API parity with C#/TS | **Not required** — idiomatic Godot wins |
| Shared runtime code | **None required** — reimplementation or intentional shared design docs only |

### 7.1 Golden vectors

Optional **test data only** (JSON fixtures) for regression and learning ports. Not shared runtime libraries. Not a production interop guarantee unless a future PRD explicitly adds cross-engine wire compatibility.

### 7.2 Module map

| Phase | Module area | Notes |
|-------|-------------|--------|
| P0 | SRA mental poker, layered ciphertext, deck helpers | Milestone A core |
| P0 | Keychain + policies | Safety critical |
| P0 | secp256k1 via maintained library + validation | Foundation |
| P1 | Commitments + **shuffle proofs** (commit–reveal) | Essential for A; honest threat model |
| P1 | Merkle, hashing, canonical encoding | Battleship-style examples |
| A extras | Godot addon packaging + 3 samples + simulator | Milestone A |
| P2 | Shamir SSS | Before public publish if claimed |
| P2 | ECDSA, EC ElGamal-style, Feldman DKG, DLEQ | Threshold suite; before publish if claimed |
| P3 | Paillier / ZK (as designed) | Before publish claims that need them |

---

## 8. Engine and platform strategy

| Priority | Target | Approach |
|----------|--------|----------|
| 1 | **Godot 4** | Primary addon + samples |
| 2 | **Redot** (Godot-compatible) | Best-effort; do not block Godot 4 releases |
| 3 | Cross-check with C#/TS siblings | Docs and optional vectors only |

**Godot version floor:** Support a **current stable Godot 4.x**; choose the oldest 4.x still practical in Phase 0 spike (document in README).

**Platforms (goal):** Editor, desktop exports first; mobile and HTML5 as far as the chosen crypto backend allows. Document limitations (e.g. GDExtension export requirements).

**Multiplayer:** Milestone A samples use **local / simulated** multi-party flows. Real multiplayer demos later. Docs stay transport-agnostic.

---

## 9. Scope phases

### Phase 0 — Spec & project bootstrap

- Repo layout: core, Godot addon path, samples, docs, tests  
- Choose primary architecture (GDExtension vs C# vs hybrid) via spike  
- Algorithm notes for P0/P1 (may adapt from `fairplay-crypto` docs)  
- Godot version floor and export constraints  
- CI plan (core tests at minimum)  

### Phase 1 — Core P0 + tests

- SRA, keychain, curve validation, deck / layer helpers  
- Extensive unit + multi-party scenario tests  

### Phase 2 — Core P1 + shuffle proofs + Merkle

- Commitments, shuffle proof API, Merkle  
- Threat-model doc for shuffle (anti-fraud messaging)  

### Phase 3 — Milestone A: Godot package + samples

- Installable Godot 4 addon structure  
- Three samples: mental-poker loop, poker-shaped, Merkle battleship  
- Offline / simulated multi-party only  
- Platform smoke: Editor + at least one desktop export path  

### Phase 4 — Advanced modules (as required for Milestone B claims)

- Shamir, threshold suite, Paillier, ZK as scoped  
- Expand tests  

### Phase 5 — Milestone B: Public publish

- Godot Asset Library readiness  
- Docs, licensing notices, security / limitations section  
- Platform matrix validation as supported  

### Phase 6 — Post-publish

- Full multiplayer demo (optional)  
- Optional official adapters for popular Godot netcode patterns  
- Optional cross-engine interop vectors with C#/TS if product value appears  

---

## 10. Success metrics

| Metric | Target |
|--------|--------|
| Milestone A | Core tests green; three samples runnable in Godot Editor without private keys in simulated network payloads |
| Shuffle proofs | API usable from samples; docs state what is / is not detected |
| Install | Documented path works on a clean Godot 4 project in under 15 minutes |
| Milestone B | Advanced modules present as claimed; Asset Library packaging complete; no ZK/HE overclaims |
| License clarity | MIT on repo; third-party notices for crypto dependencies |

---

## 11. Security & shuffle-proof honesty

### 11.1 Testing as assurance

- Unit tests, multi-party scenario tests, negative tests (bad keys, bad peels, duplicate keychain entries)  
- Property / fuzz tests where useful  
- No external audit planned pre-launch  

### 11.2 Shuffle proofs (Milestone A) — commit–reveal

**In scope and required.** First anti-cheat layer for deck order:

- Shuffler **commits** to a permutation (hash commitment) before / at shuffle  
- Deck is shuffled under encryption  
- **Later**, permutation is **revealed** and checked against the commitment  
- **Prevents:** changing deck order *after* commitment (bait-and-switch)  
- **Does not provide:** zero-knowledge that the shuffle is correct without ever revealing the permutation  

**Messaging:** “Shuffle integrity / anti-cheat (commit–reveal)” for A.  
**True ZK shuffle** is **not** required for Milestone A; include only if Milestone B claims need it.

### 11.3 True ZK shuffle (Milestone B, if claimed)

A ZK shuffle proof lets a player prove a valid re-encryption / permutation **without revealing the permutation**. Peers verify a proof object.

| | Commit–reveal (A) | ZK shuffle (B) |
|--|-------------------|----------------|
| Order leakage | Known after reveal | Need not learn permutation to accept |
| When fraud is catchable | At reveal time | At shuffle time (invalid proof ⇒ reject) |
| Cost | Cheap | Heavier engineering |

Do not market “verifiable shuffle without revealing order” until ZK (or equivalent) ships.

---

## 12. Licensing & distribution

| Artifact | Intent |
|----------|--------|
| Crypto core | MIT |
| Godot addon / integration | MIT |
| Samples | MIT |
| Docs / algorithm notes | MIT (or CC0/MIT-equivalent project docs; default MIT with repo) |
| Third-party crypto deps | Compatible licenses only; ship `THIRD_PARTY_NOTICES` |

**No commercial Asset Store SKU in this repository.** Commercial Unity packaging remains in `fairplay-crypto` if continued separately.

---

## 13. Repository layout (planned)

Exact paths depend on Phase 0 architecture choice; conceptual layout:

```
fairplay-crypto-godot/
├── PRD.md
├── README.md
├── LICENSE                 # MIT
├── THIRD_PARTY_NOTICES.md  # when deps land
├── docs/
│   ├── threat-model.md
│   ├── algorithms/
│   └── godot-integration.md
├── core/                   # language-native crypto (e.g. Rust crate or C# lib)
├── addons/
│   └── fairplay_crypto/    # Godot 4 addon (plugin.cfg, bindings, scripts)
├── samples/
│   ├── mental_poker_loop/
│   ├── poker_shaped/
│   └── merkle_battleship/
├── tests/
└── vectors/                # optional fixtures only
```

---

## 14. Open questions

1. **Primary backend:** Rust GDExtension vs Godot C# (port/reuse `ManaMesh.Crypto`) vs hybrid — Phase 0 spike  
2. **Minimum Godot 4.x version** after spike  
3. **Public package / addon name** (`fairplay_crypto` vs alternate Asset Library slug)  
4. **Which advanced modules are mandatory** for Milestone B vs clearly “experimental”  
5. **ZK suite choice** if B claims true ZK shuffle  
6. **Whether any wire compatibility** with C#/TS is ever a product goal (default: no)  

---

## 15. Acceptance criteria for leaving planning

Implementation may begin when:

- [x] Clarifying requirements captured in this PRD (§0)  
- [ ] **Implementation plan** written (`IMPLEMENTATION_PLAN.md`) for Phases 0–3 (Milestone A), with Phases 4–5 as publish gate  
- [ ] Phase 0 architecture spike decision recorded (GDExtension vs C# vs hybrid)  
- [ ] Package / addon naming finalized  
- [ ] Implementation plan accepted by stakeholders  

---

## 16. Document history

| Date | Change |
|------|--------|
| 2026-07-20 | Initial PRD: Godot-first MIT library pivot; new repo `fairplay-crypto-godot`; submodule under manamesh-games |
