# PRD — ManaMesh FairPlay for Godot

| Field | Value |
|-------|--------|
| **Product** | **ManaMesh FairPlay for Godot** |
| **Repo** | [cyotee/fairplay-crypto-godot](https://github.com/cyotee/fairplay-crypto-godot) |
| **Status** | Planning — Phase 0 decisions locked; ready for remaining bootstrap spikes |
| **License** | **MIT** (entire library, samples, and docs — no commercial split) |
| **Primary engine** | **Godot 4.4+** (develop on latest stable 4.7.x; Redot best-effort) |
| **Distribution** | Open source: GitHub, Godot Asset Library (when ready), git submodule / clone |
| **Architecture** | **Rust crypto core + GDExtension** (primary); GDScript first-class; C# second-class via GDExtension |
| **API consumers** | **GDScript first-class**; C# optional/secondary (call into GDExtension) |
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
| 15 | **Export targets (Milestone A)** | **Editor + desktop + HTML5** required. **Mobile post-publish.** |
| 16 | **Primary technical backend** | **Rust crypto core + Godot 4 GDExtension** (full pivot from C# core). |
| 17 | **Why not C# primary** | Official Godot 4 docs: **C# projects cannot export to web**. HTML5 is required for A; GDExtension WASM side modules can. See [docs/research/platform-constraints.md](./docs/research/platform-constraints.md). |
| 18 | **Milestone B bar** | **Lean B:** P0/P1 only + docs + Asset Library packaging. No advanced-module gate. |
| 19 | **Algorithm parity** | Behavioral sibling; optional golden vectors only. |
| 20 | **Netcode in A** | Simulator + **networked mental_poker** MultiplayerAPI sample (not a separate fourth demo game). |
| 21 | **Godot version** | Develop on **latest stable 4.7.x**; **minimum Godot 4.4+**. |
| 22 | **Secp256k1** | **Rust** `libsecp256k1` / `rustcrypto` / maintained Rust secp256k1 crate (Phase 0 picks exact crate). |
| 23 | **Addon install** | Standard Godot addon + prebuilt/native GDExtension binaries for supported platforms; **no .NET requirement** for GDScript users. |
| 24 | **C# consumers** | Documented second-class path: call extension from C# when needed; **no dual crypto implementation**. |
| 25 | **Mobile** | **Out of Milestone A**; post-publish (Android/iOS GDExtension build matrix later). |
| 26 | **HTML5** | **In Milestone A** via GDExtension web (Emscripten side module). Phase 0 must prove web load path. |
| 27 | **Tests** | **Rust `cargo test`** for L0; optional golden vectors; Godot sample smoke. (No xUnit C# core.) |
| 28 | **Naming** | Crate/package working name under `manamesh_fairplay` / `manamesh-fairplay`; listing **ManaMesh FairPlay for Godot**. |
| 29 | **Facade shape** | GDExtension classes (Rust) + **thin GDScript wrappers** for ergonomics; no crypto math in GDScript. |
| 30 | **Superseded** | Prior “C# core + GDScript facade”, “dual API day one”, “all platforms including mobile for A”, and “NBitcoin/Secp256k1.Net” decisions are **void**. |

### Two different “done” bars

| Bar | Meaning |
|-----|---------|
| **Milestone A** | Rust L0 (P0/P1) + `cargo test` + GDExtension addon + GDScript samples + simulator + networked mental-poker MultiplayerAPI sample + **export smoke: Editor, desktop, HTML5**. Mobile not required. |
| **Milestone B (lean)** | A polished for Asset Library: docs, threat model, notices, listing. No advanced crypto modules required. |

---

## 1. Problem

Competitive multiplayer games need **provably fair** dealing, hidden placement, and cooperative reveal without a trusted dealer.

ManaMesh has TS (`boardgameio-crypto`) and C#/Unity (`fairplay-crypto`) stacks. **Godot** needs a first-class MIT package. Research showed Godot 4 **C# cannot target HTML5**, so a C#-primary core would permanently exclude web — unacceptable given the product’s web demo / itch-style distribution goals. **Rust GDExtension** is the path that keeps GDScript ergonomics **and** web export.

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
| Mobile export gate for A | Deferred post-publish |
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
| mental_poker_loop | Offline simulator; **also** MultiplayerAPI networked variant |
| poker_shaped | Hold’em-shaped sequence, not full rules |
| merkle_battleship | Commit / challenge / open |

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

### Module map (A)

| Phase | Modules |
|-------|---------|
| P0 | Secp helpers, hash, keychain, SRA, deck helpers |
| P1 | Commitments, shuffle commit–reveal, Merkle, wire DTOs, simulator |
| A extras | GDExtension, GDScript API, samples, MultiplayerAPI mental_poker, desktop+HTML5 smoke |
| Post-B | Mobile binaries, advanced crypto, C# convenience docs |

---

## 8. Engine and platforms

| Priority | Target | A status |
|----------|--------|----------|
| 1 | Godot **4.4+** (dev on **4.7.x**) | Required |
| 2 | Editor + desktop | Required smoke |
| 3 | HTML5 + GDExtension | Required smoke |
| 4 | Mobile | Post-publish |
| 5 | Redot | Best-effort |

---

## 9. Phases (see IMPLEMENTATION_PLAN for detail)

0. Spikes: Rust toolchain, godot-rust, desktop+web GDExtension load, secp crate choice, scaffold  
1. Rust P0 + tests  
2. Rust P1 + simulator + threat model  
3. Addon + samples + MultiplayerAPI + platform smoke → **Milestone A**  
4. Lean public publish → **Milestone B**  
5. Post-publish: mobile, advanced modules, polish  

---

## 10. Success metrics

| Metric | Target |
|--------|--------|
| L0 | `cargo test` green |
| Shuffle | Anti-cheat suite green |
| Samples | Three GDScript samples + networked mental_poker |
| Platforms | Editor + desktop + HTML5 documented smoke |
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

## 14. Open items (implementation, not product direction)

1. Exact Rust secp256k1 crate (`secp256k1`, `k256`, etc.) after Phase 0 bake-off  
2. godot-rust (gdext) version pin vs Godot 4.4–4.7  
3. CI binary artifact layout for desktop + web  
4. Whether C# sample stubs ship in A or docs-only  

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
