# PRD — ManaMesh FairPlay for Godot

| Field | Value |
|-------|--------|
| **Product** | **ManaMesh FairPlay for Godot** |
| **Repo** | [cyotee/fairplay-crypto-godot](https://github.com/cyotee/fairplay-crypto-godot) |
| **Status** | Planning (implementation after PRD acceptance + implementation plan) |
| **License** | **MIT** (entire library, samples, and docs — no commercial split) |
| **Primary engine** | **Godot 4** (Redot compatibility as a non-blocking goal) |
| **Distribution** | Open source: GitHub, Godot Asset Library (when ready), git submodule / clone |
| **Architecture** | **Hybrid: C# crypto core + GDScript facade** |
| **API consumers** | **GDScript and C# equally from day one** |
| **Reference stacks** | C#: [cyotee/fairplay-crypto](https://github.com/cyotee/fairplay-crypto) (`ManaMesh.Crypto`); TS: `@manamesh/boardgameio-crypto` in manamesh-games |
| **Monorepo path** | `packages/fairplay-crypto-godot` submodule of [manamesh-games](https://github.com/cyotee/manamesh-games) |
| **Last updated** | 2026-07-20 |

---

## 0. Requirements decisions (locked)

These supersede earlier defaults where they conflict. Decisions #16–23 were locked in stakeholder Q&A (2026-07-20).

| # | Topic | Decision |
|---|--------|----------|
| 1 | **Primary product** | A **Godot 4** fair-play crypto library other developers can use under **MIT**. |
| 2 | **Licensing** | **MIT for everything** published in this repo (core, Godot addon, samples, docs). No dual open-core / paid engine package in this product. |
| 3 | **Relationship to fairplay-crypto** | **Behavioral sibling**, not a thin wrapper of the Unity commercial package and **not a runtime dependency**. Reimplement freely for Godot; reuse ideas, algorithm docs, and optional golden vectors. |
| 4 | **Relationship to TypeScript** | TS package is **inspiration and behavioral reference**, not a binding API or wire contract. Drift freely for Godot idioms. |
| 5 | **First milestone** | **Core crypto + automated tests + Godot samples** including simulator **and** an official **Godot MultiplayerAPI** sample path. Full production multiplayer game is **later**. |
| 6 | **API design** | **Both GDScript and C# equally from day one** — dual public surfaces, dual samples/docs where a feature is public. Do not force TS shapes 1:1. |
| 7 | **Core purity** | Crypto core must not depend on a specific lobby, matchmaking, or game rules engine. **Bring-your-own-netcode** for library consumers; official samples may use MultiplayerAPI without making it a core dependency. |
| 8 | **Private keys** | **Never** put private keys in shared multiplayer state or network payloads. APIs and samples make the safe path the default. |
| 9 | **Shuffle proofs** | **In scope and essential** as integrity / anti-fraud signals. Ship commit–reveal for A/B public claims; document the real threat model. True ZK shuffle is **post-publish / experimental**, not a B gate. |
| 10 | **Security assurance** | **No external audit budget.** Extensive tests, threat-model docs, honest claims. |
| 11 | **Game domain** | Core stays **game-agnostic**. Poker / battleship / deck genre terms live in **samples**, not core public type names. |
| 12 | **Marketplace positioning** | Multiplayer / P2P cryptography for securing secrets and verifying fairness in **any genre**. Not gambling; not cryptocurrency. Card demos are illustrative only. |
| 13 | **Branding** | Public name: **ManaMesh FairPlay for Godot**. Repo may remain `fairplay-crypto-godot`. Avoid “boardgame.io” in public package naming. |
| 14 | **Workflow** | Day-to-day work in this repo (submodule under `manamesh-games`); bump submodule pointer in the parent monorepo when ready. |
| 15 | **Export targets (Milestone A)** | **All major Godot exports**: Editor, desktop, **mobile**, and **HTML5/web** smoke validation is part of A (not deferred). Document any platform-specific gaps honestly if a target proves infeasible after spike. |
| 16 | **Primary technical backend** | **Hybrid: C# crypto core + GDScript facade.** Math and protocol live in managed C#; GDScript calls through a thin, documented facade (not a second crypto implementation). |
| 17 | **Primary API consumers** | **Both GDScript and C# from day one** — Milestone A samples and docs cover both. |
| 18 | **Milestone B (public publish) bar** | **Lean B:** P0/P1 modules (SRA, keychain, shuffle commit–reveal, Merkle) + solid docs + packaging. Shamir / threshold / Paillier / ZK are **post-publish or experimental**, not required to list on the Asset Library. |
| 19 | **Algorithm parity** | **Behavioral sibling, reimplement freely.** No hard wire/API parity with C# `ManaMesh.Crypto` or TS. Optional golden vectors for regression only. |
| 20 | **Netcode in Milestone A** | Core remains BYO. Ship in-process multi-seat simulator **and** at least one **official Godot MultiplayerAPI sample** (high-level multiplayer nodes / RPCs). WebRTC / join-code path is **not** required for A. |
| 21 | **Godot version policy** | Develop against **latest stable Godot 4.x**; set oldest-supported 4.x after Phase 0 packaging/export spike. |
| 22 | **C# core coupling** | Godot C# core may **inspire from** sibling `ManaMesh.Crypto` source but is **owned in this repo** (reimplementation / adaptation). Do not require consumers to pull the Unity package or `fairplay-crypto` Unity tree. |
| 23 | **Addon install** | Consumers install a Godot 4 addon (C# enabled project). Document .NET / Godot C# requirements clearly for GDScript-only users who still need the C# runtime for the facade. |

### Two different “done” bars

| Bar | Meaning |
|-----|---------|
| **Milestone A — First shippable engineering bar** | C# crypto core (P0/P1) + tests + Godot addon + GDScript **and** C# samples + multi-seat simulator + **MultiplayerAPI sample** + **export smoke on Editor, desktop, mobile, and HTML5**. Not necessarily Asset Library listing. |
| **Milestone B — Public publish bar** | Milestone A polished for public use: docs, threat model, third-party notices, Asset Library packaging. **No** advanced modules (Shamir/threshold/Paillier/ZK) required. Eligible for Godot Asset Library submission. |

---

## 1. Problem

Competitive multiplayer games need **provably fair** dealing, hidden placement, and cooperative reveal when peers do not trust a central dealer or each other.

The ManaMesh ecosystem already has:

- **TypeScript** fair-play crypto for browser / boardgame.io (`boardgameio-crypto`)
- **C#** open core + Unity packaging path (`fairplay-crypto` / `ManaMesh.Crypto`)

**Godot** studios cannot cleanly consume either as a first-class Godot addon. They need the same *class of capabilities* packaged for Godot 4 under a permissive license other developers will actually adopt — with **both GDScript and C#** as first-class entry points.

---

## 2. Product vision

Ship **ManaMesh FairPlay for Godot** — an **MIT Godot 4 library** that:

1. Provides multi-party fair-play primitives (mental poker / SRA, keychain admission, Merkle, shuffle integrity)
2. Implements crypto in a **C# core** with a **GDScript facade** so both language communities are first-class
3. Stays **engine-integration-thin** in the library: crypto + protocol DTOs; samples show MultiplayerAPI without mandating it
4. Is **easy to vendor**: clone, submodule, or Asset Library install (C#-enabled Godot project)

**Framing:**  
**ManaMesh FairPlay for Godot** — cryptography for securing secrets and verifying integrity among untrusted peers in multiplayer/P2P games.  
**Not** “boardgame.io for Godot.” **Not** a gambling or cryptocurrency product.

---

## 3. Goals

### 3.1 Product goals

- Multi-party **commutative encryption (SRA)** decks: key exchange → layered encrypt → shuffle → deal → cooperative peel / reveal  
- **Public-key keychain admission** (curve checks, uniqueness policies)  
- **Private keys stay local** — safe defaults in API and samples  
- **Merkle commitments** for hidden placement / board-style games  
- **Shuffle integrity proofs** (commit–reveal) as a first-class feature with an honest threat model  
- **MIT** adoption path for indie and commercial Godot projects alike  
- **Dual language surface**: GDScript facade + C# API, both documented from day one  
- Headline samples: mental-poker loop, poker-shaped flow, Merkle battleship — each usable from GDScript and C# where practical  
- At least one sample wired through **Godot MultiplayerAPI**  

### 3.2 Engineering goals

- **C# crypto core** free of game rules and free of a hard MultiplayerAPI dependency  
- **GDScript facade** that does not reimplement crypto math  
- **Idiomatic Godot** surface (signals, Resources, documented patterns) over cloning TS APIs  
- **Extensive automated tests** as primary assurance (core tests without full editor where possible)  
- Prefer designs that do **not** require hand-rolled secp256k1  
- Document install for Godot 4.x + .NET / Godot C# support  
- **Export validation** for desktop, mobile, and HTML5 as part of Milestone A  

### 3.3 Community / distribution goals

- Publish as a library other Godot developers can use without commercial friction  
- Godot Asset Library submission at Milestone B under **ManaMesh FairPlay for Godot**  
- Clear README + threat model + algorithm notes so non-cryptographers can integrate safely  

---

## 4. Non-goals

| Non-goal | Rationale |
|----------|-----------|
| Replace `@manamesh/boardgameio-crypto` for web | TS remains browser/ManaMesh web reference |
| Replace or absorb Unity commercial packaging | Lives in `fairplay-crypto`; this repo is Godot MIT |
| Bit-identical wire format with TS or C# sibling | Drift allowed; optional golden vectors only |
| Runtime dependency on `fairplay-crypto` / Unity package | Sibling inspiration only |
| Full production multiplayer game in Milestone A | Samples + MultiplayerAPI demo only |
| Required MultiplayerAPI dependency in the crypto core | BYO netcode for library consumers |
| WebRTC / join-code matchmaking in Milestone A | Explicitly later |
| Advanced modules (Shamir, threshold, Paillier, ZK) as B gate | Lean B decision |
| Gambling / cryptocurrency product positioning | Fair-play & secret security for general games |
| External security audit before launch | No budget; testing + docs instead |
| Real-money gambling features | Legal / product risk |
| Putting poker / battleship domain types in the open core | Domain stays in samples |
| Godot 3 support | Godot 4 only unless a later decision revisits |
| Pure GDScript crypto core | Rejected; C# core + facade |

---

## 5. Users, use cases, and sample strategy

### 5.1 Personas

| Persona | Needs |
|---------|--------|
| Godot GDScript developer | MIT addon, GDScript facade + samples, clear .NET/C# project prerequisites |
| Godot C# developer | First-class C# API and samples, same crypto guarantees |
| Protocol-minded developer | Keychain, peels, shuffle integrity, Merkle — not a full game framework |
| Multiplayer implementer | Transport-agnostic DTOs; simulator + MultiplayerAPI sample as templates |
| ManaMesh contributor | Side-by-side submodule next to C# and TS references |

### 5.2 Core use cases (library)

1. Mental poker deal / reveal loop  
2. Keychain seat admission  
3. Client-side encrypt binding (local sk matches published pk)  
4. Merkle commit / open  
5. Shuffle commit → reveal / verify integrity  
6. (Post-publish / experimental) threshold, Paillier, ZK helpers  

### 5.3 Headline samples (Milestone A)

| Sample | Demonstrates |
|--------|----------------|
| **Full mental-poker loop** | Keychain admit → layered encrypt → shuffle + proof material → deal → cooperative peel / reveal |
| **Poker-shaped flow** | Same primitives in a Hold’em-*shaped* sequence (not a full rules engine) |
| **Merkle battleship** | Commit board / placement → challenge / open with Merkle proofs |
| **MultiplayerAPI path** | At least one of the above (or a thin fourth scene) wired through Godot high-level multiplayer (host/client, RPCs) without private keys on the wire |

Samples are **teaching demos**, not the core product API. A production-quality full game remains **out of Milestone A**.

**Language requirement:** For each headline sample, ship **GDScript and C#** entry points (or a single scene with dual script variants) so neither audience is second-class.

### 5.4 Library layering

| Layer | Area | Allowed | Forbidden |
|-------|------|---------|-----------|
| **L0 C# core** | Crypto / protocol primitives | key, keychain, ciphertext, layer, peel, commitment, shuffle proof, Merkle tree/proof, policy | Poker ranks, ship grids, `Godot.*` types as required deps for math |
| **L1 Godot integration** | Addon: C# glue + **GDScript facade** | Godot nodes/Resources, signals, MultiplayerAPI-friendly DTOs | Reimplementing crypto math in GDScript |
| **L2 Samples** | `samples/` | Didactic game terms, demo UI, MultiplayerAPI scenes | Leaking domain types into L0 public API |

**Principle:** L0 is testable with `dotnet test` (or equivalent) without launching the editor. L1 is what other developers install. L2 teaches usage in both languages.

---

## 6. Architecture (locked direction)

### 6.1 Chosen shape: C# core + GDScript facade

```
┌─────────────────────────────────────────────┐
│  Samples (GDScript + C#)                    │
│  simulator · MultiplayerAPI demo            │
└───────────────────┬─────────────────────────┘
                    │
┌───────────────────▼─────────────────────────┐
│  L1 Godot addon                             │
│  C# Godot glue  ·  GDScript facade scripts  │
└───────────────────┬─────────────────────────┘
                    │
┌───────────────────▼─────────────────────────┐
│  L0 ManaMesh FairPlay C# core (this repo)   │
│  SRA · keychain · shuffle · Merkle · …      │
│  No MultiplayerAPI / no game rules          │
└─────────────────────────────────────────────┘
```

### 6.2 Required outcomes

1. **GDScript-callable** facade for all Milestone A public features.  
2. **C#-callable** API for the same features without going through GDScript.  
3. **Secure, maintained** elliptic-curve / big-integer stack — no hand-rolled secp256k1.  
4. **Automated tests** for L0 in CI without the full editor; Godot headless/export smoke for L1/L2.  
5. **Export story** for desktop, mobile, and HTML5 as part of A (Godot C# / WASM constraints documented).  

### 6.3 Rules for the crypto core

1. Never put private keys in shared multiplayer state or network payloads — ciphertexts / peels / commitments only  
2. No hard dependency on MultiplayerAPI or a third-party netcode addon in L0  
3. No hand-rolled secp256k1  
4. Keychain-style admission for public keys  
5. Client-side bind: local sk matches published pk before encrypt  
6. Document integrity vs zero-knowledge honestly  
7. Prefer byte-oriented secrets; hex/base64 only at boundaries  
8. GDScript facade must not re-encode unsafe crypto; it only marshals to/from the C# core  

### 6.4 Netcode

**Library consumers: bring your own.** Ship:

- Protocol-oriented DTOs / dictionaries (documented fields)  
- Optional message sink/source interfaces or Godot signals  
- An **in-process multi-seat simulator** for tests and offline samples  
- At least one **official MultiplayerAPI sample** showing host/client message flow with fair-play payloads  

No required dependency on a particular lobby, relay, or WebRTC stack for the library itself.

### 6.5 Godot C# / GDScript project implications

- Consumers need a **Godot build with .NET / C# support** even if they only write GDScript against the facade.  
- Document this upfront in README and Asset Library listing.  
- Phase 0 spike must validate: GDScript → C# core call path, export sizes, and HTML5 C# constraints for the chosen Godot version.

---

## 7. Relationship to sibling packages

| Concern | Decision |
|---------|----------|
| `boardgameio-crypto` (TS) | Behavioral reference only |
| `fairplay-crypto` (C# / Unity) | Sibling algorithms and docs; optional vector sharing; **not** a runtime dependency |
| This repo | Godot 4 MIT product; owns its C# core copy/adaptation |
| Monorepo link | Submodule of `manamesh-games` at `packages/fairplay-crypto-godot` |
| API parity with C#/TS | **Not required** — idiomatic Godot dual API wins |
| Shared runtime code | **None required** |

### 7.1 Golden vectors

Optional **test data only** (JSON fixtures) for regression and learning ports. Not shared runtime libraries. Not a production interop guarantee.

### 7.2 Module map

| Phase | Module area | Notes |
|-------|-------------|--------|
| P0 | SRA mental poker, layered ciphertext, deck helpers | Milestone A core |
| P0 | Keychain + policies | Safety critical |
| P0 | secp256k1 via maintained library + validation | Foundation |
| P1 | Commitments + **shuffle proofs** (commit–reveal) | Essential for A; honest threat model |
| P1 | Merkle, hashing, canonical encoding | Battleship-style examples |
| A extras | Godot addon, GDScript facade, dual-language samples, simulator, MultiplayerAPI sample, **all-platform export smoke** | Milestone A |
| Post-B (optional) | Shamir, threshold suite, Paillier, ZK | Experimental / later; not B gate |

---

## 8. Engine and platform strategy

| Priority | Target | Approach |
|----------|--------|----------|
| 1 | **Godot 4** (latest stable for development) | Primary addon + samples |
| 2 | **Export matrix for A** | Editor, desktop, mobile, HTML5 smoke |
| 3 | **Redot** (Godot-compatible) | Best-effort; do not block Godot 4 releases |
| 4 | Cross-check with C#/TS siblings | Docs and optional vectors only |

**Godot version floor:** Develop on **latest stable Godot 4.x**; record oldest-supported 4.x after Phase 0 spike (C# + exports + HTML5).

**Platforms (Milestone A requirement):** Editor, standalone desktop, mobile (Android and/or iOS as practical), and HTML5. If a target is blocked by Godot C#/.NET limitations, spike must either solve it or produce a documented **blocker** for stakeholder re-scope before calling A done.

**Multiplayer:** Simulator for offline teaching; MultiplayerAPI sample for networked teaching. Production netcode remains the integrator’s job.

---

## 9. Scope phases

### Phase 0 — Spec & project bootstrap

- Repo layout: C# core, Godot addon (C# + GDScript facade), samples, docs, tests  
- Spike: Godot C# core + GDScript facade call path  
- Spike: export matrix (desktop, mobile, HTML5) with chosen Godot/.NET versions  
- Algorithm notes for P0/P1 (may adapt from `fairplay-crypto` docs)  
- CI plan (`dotnet test` for L0; Godot headless where feasible)  
- Record minimum Godot 4.x and .NET requirements in README  

### Phase 1 — C# core P0 + tests

- SRA, keychain, curve validation, deck / layer helpers  
- Extensive unit + multi-party scenario tests  

### Phase 2 — C# core P1 + shuffle proofs + Merkle

- Commitments, shuffle proof API, Merkle  
- Threat-model doc for shuffle (anti-fraud messaging)  

### Phase 3 — Milestone A: Godot package + dual samples + platforms

- Installable Godot 4 addon (`plugin.cfg`, C# assemblies, GDScript facade)  
- Samples: mental-poker loop, poker-shaped, Merkle battleship — **GDScript + C#**  
- In-process multi-seat simulator  
- Official **MultiplayerAPI** sample path  
- Export smoke: Editor + desktop + mobile + HTML5  
- Docs: install, key safety, threat model summary  

### Phase 4 — Milestone B: Public publish (lean)

- Godot Asset Library readiness under **ManaMesh FairPlay for Godot**  
- Docs polish, licensing notices, security / limitations section  
- No advanced-module gate  

### Phase 5 — Post-publish (optional)

- Advanced modules (Shamir, threshold, Paillier, ZK) as experimental packages or later releases  
- Full multiplayer demo game  
- WebRTC / join-code demos if product value appears  
- Optional cross-engine interop vectors with C#/TS  

---

## 10. Success metrics

| Metric | Target |
|--------|--------|
| Milestone A core | L0 tests green; no private keys in simulated or MultiplayerAPI sample payloads |
| Dual language | GDScript and C# samples cover the three headline flows (or explicit dual entry points) |
| MultiplayerAPI | At least one sample runs host/client (or listen server) with fair-play messages |
| Platforms | Documented smoke results for Editor, desktop, mobile, HTML5 |
| Install | Clean Godot 4 C#-enabled project can install and run a sample in under 15 minutes |
| Milestone B | Asset Library packaging complete; lean feature set only; no ZK/HE overclaims |
| License clarity | MIT on repo; third-party notices for crypto dependencies |

---

## 11. Security & shuffle-proof honesty

### 11.1 Testing as assurance

- Unit tests, multi-party scenario tests, negative tests (bad keys, bad peels, duplicate keychain entries)  
- Property / fuzz tests where useful  
- No external audit planned pre-launch  

### 11.2 Shuffle proofs (Milestone A/B public) — commit–reveal

**In scope and required** for public claims:

- Shuffler **commits** to a permutation (hash commitment) before / at shuffle  
- Deck is shuffled under encryption  
- **Later**, permutation is **revealed** and checked against the commitment  
- **Prevents:** changing deck order *after* commitment (bait-and-switch)  
- **Does not provide:** zero-knowledge that the shuffle is correct without ever revealing the permutation  

**Messaging:** “Shuffle integrity / anti-cheat (commit–reveal).”  
**True ZK shuffle** is **not** required for A or lean B; do not market it until shipped.

### 11.3 True ZK shuffle (post-publish, if ever claimed)

A ZK shuffle proof lets a player prove a valid re-encryption / permutation **without revealing the permutation**. Peers verify a proof object. Heavier engineering; optional later release only.

---

## 12. Licensing & distribution

| Artifact | Intent |
|----------|--------|
| C# crypto core | MIT |
| Godot addon / GDScript facade | MIT |
| Samples | MIT |
| Docs / algorithm notes | MIT (default with repo) |
| Third-party crypto deps | Compatible licenses only; ship `THIRD_PARTY_NOTICES` |

**No commercial Asset Store SKU in this repository.** Commercial Unity packaging remains in `fairplay-crypto` if continued separately.

**Asset Library listing name:** ManaMesh FairPlay for Godot  
**Technical addon folder (working):** `addons/manamesh_fairplay/` (finalize slug in implementation plan if needed)

---

## 13. Repository layout (planned)

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
├── csharp/                 # L0 C# core + unit tests
│   ├── ManaMesh.FairPlay.Core/   # name TBD in plan
│   └── ManaMesh.FairPlay.Core.Tests/
├── addons/
│   └── manamesh_fairplay/  # L1 Godot 4 addon (C# + GDScript facade)
├── samples/
│   ├── mental_poker_loop/  # GDScript + C# entry points
│   ├── poker_shaped/
│   ├── merkle_battleship/
│   └── multiplayer_api/    # official MultiplayerAPI path
├── tests/
└── vectors/                # optional fixtures only
```

---

## 14. Open questions (remaining)

1. **Exact C# package / assembly names** (`ManaMesh.FairPlay.Core` vs alternate)  
2. **Addon slug** finalization (`manamesh_fairplay` vs Asset Library constraints)  
3. **Oldest Godot 4.x + .NET version** after Phase 0 export spike  
4. **HTML5 / mobile feasibility** with Godot C# for this crypto stack — confirm or re-scope if blocked  
5. **Whether MultiplayerAPI sample is a fourth sample or one of the three headline flows networked**  
6. **secp256k1 / crypto NuGet choice** for Godot C# (align with or diverge from sibling NBitcoin.Secp256k1 usage)  

---

## 15. Acceptance criteria for leaving planning

Implementation may begin when:

- [x] Clarifying requirements captured in this PRD (§0), including Q&A lock (2026-07-20)  
- [ ] **Implementation plan** written (`IMPLEMENTATION_PLAN.md`) for Phases 0–3 (Milestone A) and Phase 4 (lean B)  
- [ ] Phase 0 spike plan covers C#↔GDScript facade and export matrix risks  
- [ ] Package / addon naming proposal included in the implementation plan  
- [ ] Implementation plan accepted by stakeholders  

---

## 16. Document history

| Date | Change |
|------|--------|
| 2026-07-20 | Initial PRD: Godot-first MIT library; repo `fairplay-crypto-godot`; submodule under manamesh-games |
| 2026-07-20 | Lock Q&A requirements: C# core + GDScript facade; dual GDScript/C# APIs; lean B; behavioral sibling; MultiplayerAPI sample in A; ManaMesh FairPlay branding; all-platform exports for A; latest Godot 4.x until spike |
