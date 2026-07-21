# PRD — ManaMesh FairPlay for Godot

| Field | Value |
|-------|--------|
| **Product** | **ManaMesh FairPlay for Godot** |
| **Repo** | [cyotee/fairplay-crypto-godot](https://github.com/cyotee/fairplay-crypto-godot) |
| **Status** | Planning — product + spike decisions locked; remaining work is Phase 0 bootstrap engineering |
| **License** | **MIT** (entire library, samples, and docs — no commercial split) |
| **Primary engine** | **Godot 4.4+** (develop on latest stable 4.7.x; gdext pin; Redot best-effort) |
| **Distribution** | Open source: GitHub releases (CI binaries), Godot Asset Library, git submodule / clone |
| **Architecture** | **Rust crypto core + GDExtension**; GDScript first-class; C# docs minimal |
| **API consumers** | **GDScript first-class** (target game is GDScript-only) |
| **Downstream product intent** | Enable a future **Liar’s Dice** game (separate repo): single + multiplayer; Steam, Itch, GOG, Play, App Store; free **web multiplayer-only** |
| **Reference stacks** | C#: [cyotee/fairplay-crypto](https://github.com/cyotee/fairplay-crypto); TS: `@manamesh/boardgameio-crypto` |
| **Monorepo path** | `packages/fairplay-crypto-godot` submodule of [manamesh-games](https://github.com/cyotee/manamesh-games) |
| **Implementation plan** | [IMPLEMENTATION_PLAN.md](./IMPLEMENTATION_PLAN.md) |
| **Last updated** | 2026-07-20 |

---

## 0. Requirements decisions (locked)

Decisions #1–15 are product fundamentals. #16–30 supersede earlier C#-primary defaults after research + stakeholder Q&A (2026-07-20).

| # | Topic | Decision |
|---|--------|----------|
| 1 | **Primary product** | A **Godot 4** fair-play crypto library under **MIT**. |
| 2 | **Licensing** | **MIT for everything** in this repo. No commercial engine split. |
| 3 | **Relationship to fairplay-crypto** | **Behavioral sibling** — reimplement freely; no runtime dependency. |
| 4 | **Relationship to TypeScript** | Inspiration only; no wire/API contract. |
| 5 | **First milestone** | Core crypto + tests + Godot samples (simulator) + **MultiplayerAPI mental-poker path**. Full game later. |
| 6 | **API design** | **GDScript first-class** via GDExtension. **C# second-class** (call GDExtension; no separate C# crypto core). |
| 7 | **Core purity** | No lobby/matchmaking/rules engine in L0. **BYO netcode**; samples may use MultiplayerAPI. |
| 8 | **Private keys** | Never on shared multiplayer state or network payloads. |
| 9 | **Shuffle proofs** | Commit–reveal required for A/B public claims. ZK shuffle post-publish only. |
| 10 | **Security assurance** | No external audit; tests + threat-model docs + honest claims. |
| 11 | **Game domain** | Core game-agnostic; poker/battleship only in samples. |
| 12 | **Marketplace positioning** | P2P secret-security / fair-play crypto for any genre. Not gambling; not cryptocurrency. |
| 13 | **Branding** | **ManaMesh FairPlay for Godot**. Addon: `addons/manamesh_fairplay/`. Avoid “boardgame.io” in listing. |
| 14 | **Workflow** | Work in this submodule; bump pointer in manamesh-games. |
| 15 | **Export targets (Milestone A)** | **Editor + desktop + HTML5** required. **HTML5 is a hard gate** (free web multiplayer for downstream game). |
| 16 | **Primary technical backend** | **Rust crypto core + Godot 4 GDExtension** (full pivot from C# core). |
| 17 | **Why not C# primary** | Official Godot 4 docs: **C# projects cannot export to web**. See [docs/research/platform-constraints.md](./docs/research/platform-constraints.md). |
| 18 | **Milestone B bar** | **Lean B:** P0/P1 + docs + Asset Library packaging. No advanced-module gate. |
| 19 | **Algorithm parity** | Behavioral sibling; optional golden vectors only. |
| 20 | **Netcode in A** | Simulator + **networked** MultiplayerAPI sample path. |
| 21 | **Godot version** | Develop on **latest stable 4.7.x**; **minimum Godot 4.4+**; **pin gdext** to latest stable supporting that range. |
| 22 | **Secp256k1** | **bitcoin-core `secp256k1` Rust crate** (libsecp256k1 bindings). |
| 23 | **Addon install** | Addon + **CI-built prebuilt binaries** per platform in GitHub releases; source build docs optional. **No .NET** for GDScript users. |
| 24 | **C# consumers** | **Minimal docs only** — target game is GDScript-only; no dual crypto core. |
| 25 | **Mobile** | **Milestone A2** immediately after A (Android + iOS GDExtension) — required before library is “game-ready” for Play/App Store, not a soft post-publish maybe. |
| 26 | **HTML5** | **Hard gate for A** via GDExtension web side module. |
| 27 | **Tests** | **Rust `cargo test`** for L0; optional golden vectors; Godot sample smoke. |
| 28 | **Naming** | `addons/manamesh_fairplay/`; crates `manamesh_fairplay_core` / `manamesh_fairplay_godot` (working); listing **ManaMesh FairPlay for Godot**. |
| 29 | **Facade shape** | GDExtension classes + thin GDScript wrappers; no crypto math in GDScript. |
| 30 | **Downstream game** | Future **Liar’s Dice** (separate repo): single + multiplayer; Steam, Itch, GOG, Android Play, Apple App Store; free web multiplayer-only. Game design starts after library is usable. |
| 31 | **Game vs library repos** | **Library stays this repo**; game is a **separate repo** consuming the addon. |
| 32 | **Feature priority for Liar’s Dice** | Prioritize **commit–reveal for dice / hidden values + open/verify** (and supporting hash/key material). Full SRA mental-poker remains in scope as general library capability but is not the first game-driven sample focus. |
| 33 | **Superseded** | C# core primary; dual day-one C# API; NBitcoin primary; “mobile only post-publish with no A2”; pure mental-poker-first sample emphasis without commit–reveal dice priority. |

### Done bars

| Bar | Meaning |
|-----|---------|
| **Milestone A** | Rust L0 (P0/P1) + tests + GDExtension + GDScript samples (commit–reveal dice emphasis + general primitives) + simulator + MultiplayerAPI sample + **Editor + desktop + HTML5** smoke. |
| **Milestone A2 (game-ready mobile)** | Android + iOS GDExtension binaries + smoke; required before calling library ready for Play/App Store game work. |
| **Milestone B (lean public library)** | A (+ preferably A2) polished for Asset Library: docs, notices, listing. |

---

## 1. Problem

Competitive multiplayer games need **provably fair** hidden values and cooperative reveal without a trusted dealer (e.g. dice faces in **Liar’s Dice**, cards, board placement).

ManaMesh has TS and C#/Unity stacks. **Godot** needs a first-class MIT package that can ship on **desktop stores, mobile stores, and free web multiplayer**. Research showed Godot 4 **C# cannot target HTML5**, so **Rust GDExtension** is required for a single crypto stack across desktop + web (mobile via native GDExtension in A2).

---

## 2. Product vision

Ship **ManaMesh FairPlay for Godot**:

1. Multi-party fair-play primitives (SRA, keychain, Merkle, shuffle integrity)  
2. **Rust core** exposed through **GDExtension**  
3. **GDScript-first** public API; C# optional via the same extension  
4. Easy to vendor: addon + binaries, clone, or Asset Library  

**Not** boardgame.io for Godot. **Not** gambling or cryptocurrency product.

---

## 3. Goals

### 3.1 Product

- SRA mental poker loop, keychain admission, Merkle, commit–reveal shuffle  
- Private keys local by default  
- GDScript samples: mental poker, poker-shaped, Merkle battleship  
- Networked mental-poker MultiplayerAPI sample  
- MIT; Asset Library under ManaMesh FairPlay for Godot  

### 3.2 Engineering

- Pure Rust L0 testable with `cargo test` (no Godot required for math)  
- Thin GDExtension + GDScript wrappers (no second crypto impl)  
- Prebuilt binaries for Editor/desktop/HTML5 in A; mobile later  
- Honest threat model for commit–reveal  

### 3.3 Distribution

- No .NET requirement for primary (GDScript) consumers  
- Document GDExtension binary install and web export flags (extension support, COOP/COEP as needed)  

---

## 4. Non-goals

| Non-goal | Rationale |
|----------|-----------|
| C# as primary crypto core | Blocks HTML5 in Godot 4 |
| Separate dual C# crypto reimplementation | Maintenance; C# uses GDExtension |
| Mobile export gate for A | Mobile is **A2** (right after A), not part of A hard gate |
| Wire parity with TS/C# siblings | Behavioral sibling only |
| Advanced modules as B gate | Lean B |
| Godot 3 | Godot 4.4+ only |
| Gambling / cryptocurrency positioning | Policy + product |

---

## 5. Users and samples

### 5.1 Personas

| Persona | Needs |
|---------|--------|
| GDScript indie | MIT addon, samples, no C# install |
| Protocol developer | Keychain, peels, shuffle, Merkle |
| Multiplayer integrator | BYO netcode docs + MultiplayerAPI mental-poker sample |
| C# Godot user | Optional: call same GDExtension from C# |

### 5.2 Headline samples (A)

| Sample | Notes |
|--------|--------|
| **dice_commit_reveal** (priority) | Commit hidden dice / values → challenge/open/verify — Liar’s Dice–shaped, still game-agnostic types |
| mental_poker_loop | Full SRA loop offline; optional MultiplayerAPI variant |
| merkle_battleship | Commit / challenge / open board-style |
| MultiplayerAPI path | Prefer networking the **dice commit–reveal** or mental-poker sample |

### 5.3 Layering

| Layer | Role |
|-------|------|
| **L0 Rust core** | Crypto/protocol; no Godot types required for unit tests |
| **L1 GDExtension** | Registers classes for Godot; binary per platform |
| **L1b GDScript wrappers** | Ergonomic API; no math |
| **L2 Samples** | Teaching demos |

---

## 6. Architecture (locked)

```
┌─────────────────────────────────────────────┐
│  Samples (GDScript primary)                 │
│  simulator · MultiplayerAPI mental_poker    │
└───────────────────┬─────────────────────────┘
                    │
┌───────────────────▼─────────────────────────┐
│  L1 addons/manamesh_fairplay/               │
│  .gdextension + thin GDScript wrappers      │
│  (C# may call extension classes optionally) │
└───────────────────┬─────────────────────────┘
                    │
┌───────────────────▼─────────────────────────┐
│  L0 Rust crate (manamesh_fairplay / core)   │
│  SRA · keychain · shuffle · Merkle · wire   │
│  secp256k1 via maintained Rust crate        │
└─────────────────────────────────────────────┘
```

### 6.1 Platform research (summary)

| Platform | Godot 4 C# | GDExtension (Rust) |
|----------|------------|---------------------|
| Desktop | Supported | Supported |
| Android / iOS | Experimental (C#) | Possible; **out of A** for this product |
| **HTML5** | **Not supported** | Supported as WASM **side module** (godot-rust web export guide; enable Extension Support) |

Sources: Godot C# platform docs; exporting-for-web docs; godot-rust book export-web; issue #70796 (C# web). Details: [docs/research/platform-constraints.md](./docs/research/platform-constraints.md).

### 6.2 Core rules

1. No private keys on the wire  
2. No MultiplayerAPI dependency in L0  
3. No hand-rolled secp256k1  
4. Keychain admission + sk↔pk binding  
5. Honest commit–reveal messaging  
6. GDScript wrappers never reimplement crypto  

---

## 7. Sibling packages

| Concern | Decision |
|---------|----------|
| TS / C# Unity | Behavioral reference only |
| Shared runtime | None |
| Optional vectors | Allowed as test fixtures |

### Module map (A / A2)

| Phase | Modules |
|-------|---------|
| P0 | Secp helpers, hash, **commitments / commit–reveal** (dice-friendly), keychain |
| P1 | SRA, shuffle commit–reveal, Merkle, wire DTOs, simulator |
| A extras | GDExtension, GDScript API, **dice_commit_reveal** sample + others, MultiplayerAPI, desktop+HTML5 |
| A2 | Android + iOS GDExtension binaries + smoke |
| Post-B | Advanced crypto optional; separate Liar’s Dice game repo |

---

## 8. Engine and platforms

| Priority | Target | A status |
|----------|--------|----------|
| 1 | Godot **4.4+** (dev on **4.7.x**) | Required |
| 2 | Editor + desktop | Required smoke |
| 3 | HTML5 + GDExtension | **Hard gate** for A (free web MP) |
| 4 | Mobile (Android + iOS) | **Milestone A2** (game-ready for stores) |
| 5 | Redot | Best-effort |

---

## 9. Phases (see IMPLEMENTATION_PLAN for detail)

0. Bootstrap: toolchain + gdext pin, hello desktop/HTML5, secp256k1 crate, scaffold, CI binaries  
1. Rust P0 (commit–reveal foundations priority) + tests  
2. Rust P1 (SRA, shuffle, Merkle, simulator) + threat model  
3. Addon + samples + MultiplayerAPI + desktop/HTML5 → **Milestone A**  
4. Android + iOS GDExtension → **Milestone A2** (game-ready for mobile stores)  
5. Lean public library publish → **Milestone B**  
6. Separate Liar’s Dice game repo; advanced crypto optional  

---

## 10. Success metrics

| Metric | Target |
|--------|--------|
| L0 | `cargo test` green |
| Commit–reveal | Dice/hidden-value commit + open/verify green |
| Shuffle | Anti-cheat suite green (when P1 lands) |
| Samples | dice_commit_reveal + general samples; networked MultiplayerAPI path |
| Platforms A | Editor + desktop + HTML5 smoke |
| Platforms A2 | Android + iOS smoke |
| Install | Clean Godot 4.4+ project ≤ 15 minutes (no .NET) |
| B | Asset Library package; no ZK overclaims |

---

## 11. Security (shuffle honesty)

Public claims: **commit–reveal shuffle integrity**, not ZK. True ZK only if shipped later.

---

## 12. Licensing

MIT everywhere; `THIRD_PARTY_NOTICES` for Rust/secp deps and godot-rust.

---

## 13. Repo layout (planned)

```
fairplay-crypto-godot/
├── PRD.md
├── IMPLEMENTATION_PLAN.md
├── README.md
├── LICENSE
├── docs/ research/ algorithms/ threat-model.md …
├── rust/                    # L0 + GDExtension crate(s)
├── addons/manamesh_fairplay/
├── samples/
├── vectors/                 # optional
└── .github/workflows/       # cargo test + build matrix
```

---

## 14. Open items (engineering only)

1. Exact gdext crate version pin (latest stable supporting 4.4–4.7) recorded in `docs/research/toolchain.md`  
2. Hello GDExtension desktop + HTML5 proof  
3. CI workflow artifact layout for desktop + web (+ A2 mobile later)  
4. Optional golden vector set for commit–reveal dice fixtures  

---

## 15. Acceptance for leaving planning

- [x] PRD requirements locked (including architecture pivot)  
- [x] Implementation plan rewritten for Rust GDExtension  
- [x] Platform research recorded  
- [x] Phase 0 decision log filled for architecture/platform/crypto  
- [ ] Stakeholder acceptance of plan  
- [ ] Phase 0 bootstrap complete (toolchain + hello GDExtension desktop+web) before Phase 1 bulk crypto  

---

## 16. Document history

| Date | Change |
|------|--------|
| 2026-07-20 | Initial Godot MIT PRD |
| 2026-07-20 | Q&A: C# core + facade, dual API, lean B, all platforms |
| 2026-07-20 | Research + Q&A pivot: **Rust GDExtension primary**; HTML5 in A; mobile post-publish; GDScript first-class; C# second-class; Godot 4.4+; networked mental_poker sample |
| 2026-07-20 | Downstream goal: Liar’s Dice multi-store + free web MP; A2 mobile; separate game repo; commit–reveal dice priority; secp256k1 crate; CI binaries; GDScript-only game |
