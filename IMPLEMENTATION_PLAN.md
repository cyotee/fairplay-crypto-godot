# Implementation & Testing Plan — ManaMesh FairPlay for Godot

| Field | Value |
|-------|--------|
| **Product** | **ManaMesh FairPlay for Godot** |
| **PRD** | [PRD.md](./PRD.md) |
| **Status** | **Milestone A core + desktop shipped** — HTML5 blocked pending Emscripten/export templates |
| **Architecture** | **Rust L0 + GDExtension**; GDScript first-class; C# docs minimal |
| **Downstream intent** | Liar’s Dice game (separate repo): multi-store + free web MP |
| **Repo** | [cyotee/fairplay-crypto-godot](https://github.com/cyotee/fairplay-crypto-godot) |
| **License** | MIT |
| **Last updated** | 2026-07-20 |

Implements the PRD. **Do not bulk-implement crypto until Phase 0 exit** (hello GDExtension on desktop + HTML5, secp crate chosen, CI green).

**Sibling reference only:** [fairplay-crypto](https://github.com/cyotee/fairplay-crypto) algorithms/tests — reimplement in Rust; no shared runtime.

---

## 1. Goals and done bars

### 1.1 Milestone A (desktop + free web path)

| Deliverable | Done when |
|-------------|-----------|
| Rust L0 P0/P1 | Hash, **commitments / commit–reveal** (dice-friendly), keychain, secp, SRA, shuffle proofs, Merkle, wire DTOs |
| Tests | Extensive `cargo test` |
| GDExtension addon | `addons/manamesh_fairplay/` + CI prebuilt binaries (desktop + web) |
| GDScript API | Thin wrappers; no .NET |
| Samples | **dice_commit_reveal** (priority), mental_poker_loop, merkle_battleship / poker-shaped as planned |
| MultiplayerAPI | Networked sample (prefer dice commit–reveal); no sk on wire |
| Platforms | **Editor + desktop + HTML5** (**HTML5 hard gate**) |

### 1.2 Milestone A2 (game-ready mobile)

| Deliverable | Done when |
|-------------|-----------|
| Android + iOS GDExtension | CI or documented build + smoke: load extension + one crypto op |
| Notes | Required before library is ready for Play / App Store game work |

### 1.3 Milestone B (lean public library)

| Deliverable | Done when |
|-------------|-----------|
| Packaging | Asset Library–ready **ManaMesh FairPlay for Godot** |
| Polish | Docs, notices, listing; prefer A2 complete if advertising mobile |

### 1.4 Out of this library’s critical path

- **Liar’s Dice game** (separate repo after library usable)  
- Steam/Itch/GOG/store packaging for the game  
- Full production lobby / WebRTC join codes (game concern)  
- Advanced crypto modules  
- Rich C# samples  
- External audit  

---

## 2. Architecture

```
┌──────────────────────────────────────────────────────────────┐
│  L2 Samples (GDScript)                                       │
│  mental_poker_loop (+ MultiplayerAPI net)                    │
│  poker_shaped · merkle_battleship · MultiSeatSimulator UI    │
└────────────────────────────▲─────────────────────────────────┘
                             │
┌────────────────────────────┴─────────────────────────────────┐
│  L1 addons/manamesh_fairplay/                                │
│  manamesh_fairplay.gdextension · GDScript wrappers           │
│  Optional C# call notes (second-class)                       │
└────────────────────────────▲─────────────────────────────────┘
                             │
┌────────────────────────────┴─────────────────────────────────┐
│  L0 Rust (cargo workspace)                                   │
│  manamesh_fairplay_core  — pure crypto, no godot crate deps  │
│  manamesh_fairplay_godot — gdext bindings (or single crate)  │
│  secp256k1 via maintained Rust crate                         │
└──────────────────────────────────────────────────────────────┘
```

### 2.1 Why Rust (locked)

Godot 4 **cannot export C# to HTML5**. HTML5 is in Milestone A. GDExtension WASM side modules work for web; see [docs/research/platform-constraints.md](./docs/research/platform-constraints.md).

### 2.2 Netcode

| Ship | Do not ship |
|------|-------------|
| Wire DTOs (no secrets) | Required multiplayer framework |
| MultiSeatSimulator | Matchmaking / WebRTC product |
| Networked mental_poker sample | “Only MultiplayerAPI” library dependency |

### 2.3 Design principles

1. Private keys never on the wire  
2. L0 pure Rust — unit tests without Godot  
3. No crypto math in GDScript  
4. Commit–reveal shuffle anti-cheat required  
5. Drift from TS/C# siblings allowed  
6. Maintained secp implementation only  

---

## 3. Repository layout (target)

```
fairplay-crypto-godot/
├── PRD.md
├── IMPLEMENTATION_PLAN.md
├── README.md
├── LICENSE
├── docs/
│   ├── threat-model.md
│   ├── algorithms/
│   ├── integration-byo-netcode.md
│   ├── godot-integration.md
│   ├── platform-matrix.md
│   ├── asset-library-listing-draft.md
│   └── research/
│       ├── platform-constraints.md   # done
│       ├── crypto-stack.md           # Phase 0
│       ├── gdextension-web.md        # Phase 0
│       └── toolchain.md              # Phase 0
├── rust/
│   ├── Cargo.toml                    # workspace
│   ├── manamesh_fairplay_core/
│   └── manamesh_fairplay_godot/      # gdext
├── addons/manamesh_fairplay/
│   ├── manamesh_fairplay.gdextension
│   ├── plugin.cfg                    # if editor plugin needed
│   ├── bin/                          # or CI-published artifacts
│   └── facade/                       # thin .gd wrappers
├── samples/
│   ├── mental_poker_loop/            # offline + multiplayer scenes
│   ├── poker_shaped/
│   └── merkle_battleship/
├── project.godot
├── vectors/                          # optional golden fixtures
└── .github/workflows/ci.yml
```

---

## 4. Module breakdown (Milestone A)

| Module | Location | Priority |
|--------|----------|----------|
| Secp / primitives | `manamesh_fairplay_core` | P0 |
| Hashing | core | P0 |
| Keychain | core | P0 |
| SRA / mental poker | core | P0 |
| Commitments | core | P1 |
| Shuffle commit–reveal | core | P1 |
| Merkle | core | P1 |
| Wire DTOs | core | P1 |
| MultiSeatSimulator | core and/or sample helpers | P1 |
| GDExtension classes | `manamesh_fairplay_godot` | A |
| GDScript facade | `addons/.../facade` | A |

Post-publish: mobile targets, advanced crypto, richer C# samples.

---

## 5. Phased work packages

### Phase 0 — Bootstrap engineering (product decisions done)

| ID | Task | Output | Verification |
|----|------|--------|--------------|
| 0.1 | Toolchain: Rust, **gdext pin** (latest stable for Godot 4.4–4.7), Godot 4.7.x | `docs/research/toolchain.md` | Editor + build works |
| 0.2 | Integrate **`secp256k1`** Rust crate (bitcoin-core bindings) | `docs/research/crypto-stack.md` | Unit test point/scalar ops |
| 0.3 | **Hello GDExtension** desktop | Load + one GDScript call | Pass |
| 0.4 | **Hello GDExtension HTML5** (**hard gate**) | `docs/research/gdextension-web.md` | Browser one call; COOP/COEP notes |
| 0.5 | Scaffold workspace, empty core tests, addon skeleton | Compiling | `cargo test` green |
| 0.6 | CI: `cargo test` + **release binary artifacts** (desktop + web layout) | `.github/workflows` | Artifacts attachable to releases |
| 0.7 | Naming freeze | crates + class prefix | §12 |

**Exit criteria:** Desktop + HTML5 extension load proven; secp256k1 integrated; CI green; §12 complete.

**Kill / escalate:** HTML5 failure → stakeholder meeting (free web MP depends on it).

### Phase 1 — Rust core P0 + tests (Liar’s Dice–aligned priority)

| ID | Task | Tests |
|----|------|-------|
| 1.1 | Workspace structure, clippy/fmt, CI | `cargo test` / `clippy` |
| 1.2 | Secp facade + validation (`secp256k1` crate) | Valid/invalid points; ser/de |
| 1.3 | Hashing + secure RNG wrappers | Hash vectors; RNG length |
| 1.4 | **Commitments + commit–reveal** (bytes / structured values) | Open OK / wrong opening fail — **dice-shaped scenarios** |
| 1.5 | Keychain + policies | Admit; reject bad/dupe keys |
| 1.6 | SRA multi-layer encrypt/peel + helpers | 2–4 party scenarios (general library) |
| 1.7 | sk↔pk binding helper | Mismatch rejected |
| 1.8 | Algorithm notes (commit–reveal + SRA drafts) | Doc checklist |

**Exit:** Commit–reveal dice-shaped scenarios green; SRA path green; no `godot` deps in core crate.

### Phase 2 — Rust core P1 + simulator

| ID | Task | Tests |
|----|------|-------|
| 2.1 | Commitments | Open fail cases |
| 2.2 | Shuffle commit/apply/open/verify | **§6.2 anti-cheat suite** |
| 2.3 | Threat model | Claims match API |
| 2.4 | Merkle | Tamper fails |
| 2.5 | Wire DTOs (no secrets) | Round-trip; secret-field lint |
| 2.6 | MultiSeatSimulator | Full mental-poker in pure Rust tests |

**Exit:** Simulator scenario green; threat model merged.

### Phase 3 — GDExtension + samples + platforms (Milestone A)

| ID | Task | Verification |
|----|------|--------------|
| 3.1 | gdext classes for all A ops | GDScript call matrix §6.3 |
| 3.2 | Thin GDScript facade | No math in `.gd` |
| 3.3 | Sample **dice_commit_reveal** (priority) | Offline + tests |
| 3.4 | Sample **dice_commit_reveal MultiplayerAPI** (or mental_poker net) | Host/client; no sk on wire |
| 3.5 | Sample mental_poker_loop offline | Smoke |
| 3.6 | Sample merkle_battleship / poker_shaped | Smoke |
| 3.7 | BYO netcode + integration docs | Review |
| 3.8 | Platform matrix Editor/desktop/HTML5 | `docs/platform-matrix.md` |
| 3.9 | Asset Library listing draft | Positioning language |
| 3.10 | README quickstart (GDScript, no .NET) | ≤15 min dry-run |
| 3.11 | CI release binaries for desktop + web | Downloadable artifacts |

**Exit:** Milestone A checklist §11.

### Phase 4 — Milestone A2 (mobile game-ready)

| ID | Task | Verification |
|----|------|--------------|
| 4.1 | Android GDExtension build + package | Smoke load + one crypto op |
| 4.2 | iOS GDExtension build + package (macOS host) | Smoke |
| 4.3 | CI or documented release artifacts for mobile | `platform-matrix.md` A2 section |

**Exit:** A2 checklist; library usable for Play/App Store game project.

### Phase 5 — Lean Milestone B (public library)

| ID | Task |
|----|------|
| 5.1 | Docs polish + limitations |
| 5.2 | THIRD_PARTY_NOTICES |
| 5.3 | Asset Library package + GitHub release |
| 5.4 | Listing: ManaMesh FairPlay for Godot |

### Phase 6 — After library usable

| ID | Task |
|----|------|
| 6.1 | **Separate Liar’s Dice game repo** (design + implement) |
| 6.2 | Game exports: Steam, Itch, GOG, Play, App Store, free web MP |
| 6.3 | Optional advanced crypto modules in library |

---

## 6. Testing strategy

### 6.1 Layers

| Layer | Tooling | Focus |
|-------|---------|--------|
| L0 unit/scenario | `cargo test` | Math, policies, N-party loops |
| Property/fuzz | `proptest` / similar where useful | Permutations, layers |
| Negative | Required | Bad keys, wrong peels, secret-in-payload |
| GDExtension | Godot Editor + headless if available | Facade matrix |
| MultiplayerAPI | 2-instance / multiplayer checklist | Wire hygiene |
| Export smoke | Desktop + HTML5 | §6.5 |
| CI | GitHub Actions | `cargo test` + build extension (desktop CI; web build as feasible) |

### 6.2 Shuffle anti-cheat (mandatory)

1. Commit P, shuffle P, open P → OK  
2. Commit P, shuffle P′ ≠ P, open P → fail  
3. Commit P, wrong open → fail  

### 6.3 GDScript coverage matrix (A)

Every public A capability: Rust test + GDScript callable + at least one sample path.

| Capability | cargo test | GDScript | Sample |
|------------|------------|----------|--------|
| Keychain admit | ✓ | ✓ | mental_poker |
| Encrypt / peel | ✓ | ✓ | mental_poker |
| Shuffle commit/open | ✓ | ✓ | mental_poker / poker |
| Merkle | ✓ | ✓ | battleship |
| Wire encode/decode | ✓ | ✓ | multiplayer mental_poker |

### 6.4 Security tests

- Envelopes never carry private keys  
- sk↔pk bind before encrypt  
- Keychain rejects invalid points / duplicates  
- Sample audit: no sk logging/RPC  

### 6.5 Platform smoke (A)

| Target | Pass criteria |
|--------|----------------|
| Editor | Extension loads; sample runs |
| Desktop | Export or run binary path; one crypto op |
| HTML5 | Export with Extension Support; browser one crypto op; document headers |
| Mobile | **Not required for A** |

### 6.6 Optional golden vectors

JSON fixtures for regression/learning from sibling ports — **not** wire compatibility guarantees. Prefer property/scenario tests in Rust as primary.

---

## 7. Documentation deliverables

| Doc | Phase |
|-----|--------|
| platform-constraints.md | Done |
| toolchain, crypto-stack, gdextension-web | 0 |
| algorithms/* | 1–2 |
| threat-model.md | 2 |
| integration + godot-integration | 3 |
| platform-matrix.md | 0 + 3 |
| Asset Library draft | 3–4 |
| README | 3 |
| THIRD_PARTY_NOTICES | 4 |

---

## 8. Risks and mitigations

| Risk | Mitigation |
|------|------------|
| HTML5 GDExtension/Emscripten friction | Phase 0.4 hard proof; escalate if fail |
| godot-rust version skew vs Godot 4.4–4.7 | Pin versions; CI matrix |
| Binary distribution size/complexity | CI artifacts; clear install docs |
| Scope creep to dual C# core | C# second-class only via extension |
| Overclaim ZK shuffle | Docs: commit–reveal only |
| Mobile later surprises | Explicit post-publish phase |

---

## 9. Dependency policy

| Allowed L0 | Forbidden L0 |
|------------|--------------|
| Maintained Rust crypto crates | `godot` crate in pure core (prefer split) |
| | Hand-rolled secp |
| | Game rules / MultiplayerAPI |

| Allowed L1 | Forbidden |
|------------|-----------|
| gdext / godot-rust | Second crypto implementation in GDScript |
| Prebuilt native/wasm binaries | Requiring .NET for GDScript users |

---

## 10. Workflow

1. Work in this submodule.  
2. Task IDs in commits (`0.4`, `1.5`, …).  
3. Bump manamesh-games pointer as needed.  
4. Phase 0 exit before bulk Phase 1.  
5. Do not expand advanced modules until lean B ships (unless re-prioritized).  

---

## 11. Acceptance checklists

### Phase 0 exit

- [x] Architecture + product goals locked (incl. Liar’s Dice multi-store intent)  
- [x] Secp: bitcoin-core `secp256k1` crate  
- [x] A / A2 / HTML5 hard gate locked  
- [ ] Toolchain doc + gdext pin  
- [ ] Hello GDExtension desktop  
- [ ] Hello GDExtension HTML5  
- [ ] Scaffold + CI (`cargo test` + binary artifact layout)  
- [ ] §12 remaining TBD rows filled with pins  

### Milestone A exit

- [ ] Phases 0–3 complete  
- [ ] `cargo test` green  
- [ ] Commit–reveal dice-shaped tests green  
- [ ] Shuffle anti-cheat green (P1)  
- [ ] GDScript covers A API matrix  
- [ ] dice_commit_reveal + other samples; MultiplayerAPI path  
- [ ] Editor + desktop + **HTML5** smoke documented  
- [ ] CI desktop + web binaries  
- [ ] No private keys on sample wire paths  
- [ ] Threat model + integration docs  
- [ ] README ≤15 min install (no .NET)  

### Milestone A2 exit

- [ ] Android GDExtension smoke  
- [ ] iOS GDExtension smoke  
- [ ] Artifacts or build docs for mobile  

### Milestone B exit

- [ ] A green (A2 preferred if listing claims mobile)  
- [ ] THIRD_PARTY_NOTICES  
- [ ] Asset Library package clean install  
- [ ] Listing: ManaMesh FairPlay for Godot  
- [ ] No ZK overclaims  
- [ ] Release tagged  

### Game project (out of library repo)

- [ ] Separate Liar’s Dice repo after library usable (A minimum; A2 before mobile store work)  

---

## 12. Decision log

| Decision | Choice | Date | Notes |
|----------|--------|------|-------|
| Downstream product | **Liar’s Dice** multi-store + free web MP | 2026-07-20 | Game design after library usable |
| Game repo | **Separate** from this library | 2026-07-20 | |
| Primary architecture | **Rust + GDExtension** | 2026-07-20 | C# cannot HTML5 in Godot 4 |
| API priority | **GDScript-only** for target game; C# docs minimal | 2026-07-20 | |
| Feature priority | **Commit–reveal dice / hidden values** first | 2026-07-20 | SRA still in library P1 |
| Milestone A platforms | **Editor + desktop + HTML5** | 2026-07-20 | HTML5 **hard gate** |
| Milestone A2 | **Android + iOS** right after A | 2026-07-20 | Game-ready for mobile stores |
| Secp crate | **`secp256k1` (bitcoin-core Rust bindings)** | 2026-07-20 | |
| Godot version | **4.4+ min; develop 4.7.x** | 2026-07-20 | |
| gdext pin | **Latest stable supporting 4.4–4.7** | 2026-07-20 | Exact version in Phase 0.1 doc |
| Binary distribution | **CI release artifacts** per platform | 2026-07-20 | |
| L0 tests | **Rust / cargo test** | 2026-07-20 | Optional golden vectors |
| Addon path | **`addons/manamesh_fairplay/`** | 2026-07-20 | |
| Listing name | **ManaMesh FairPlay for Godot** | 2026-07-20 | |
| MultiplayerAPI sample | Network **dice commit–reveal** (preferred) | 2026-07-20 | |
| Facade shape | GDExtension + thin GDScript wrappers | 2026-07-20 | |
| Exact gdext version | _TBD Phase 0.1_ | | Record pin |
| Crate names | Working: `manamesh_fairplay_core` / `_godot` | 2026-07-20 | Freeze 0.7 |

---

## 13. Execution order

```
Phase 0: toolchain/gdext pin → secp256k1 → hello desktop → hello HTML5 → scaffold/CI binaries
  → Phase 1 P0 (commit–reveal priority) + cargo test
    → Phase 2 P1 SRA/shuffle/Merkle/simulator
      → Phase 3 GDExtension + dice samples + MultiplayerAPI + desktop/HTML5
        → Phase 4 A2 mobile binaries
          → Phase 5 lean library publish
            → Phase 6 separate Liar’s Dice game repo
```

---

## 14. Document history

| Date | Change |
|------|--------|
| 2026-07-20 | Initial plan (C# core + GDScript facade) |
| 2026-07-20 | Rewrite: Rust GDExtension primary; HTML5 in A; mobile deferred; research-backed platform constraints; updated tests/CI/decision log |
| 2026-07-20 | Liar’s Dice multi-store goal; A2 mobile; commit–reveal dice priority; secp256k1 crate; CI binaries; separate game repo |
