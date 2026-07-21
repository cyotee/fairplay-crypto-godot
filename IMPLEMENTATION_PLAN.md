# Implementation & Testing Plan — ManaMesh FairPlay for Godot

| Field | Value |
|-------|--------|
| **Product** | **ManaMesh FairPlay for Godot** |
| **PRD** | [PRD.md](./PRD.md) |
| **Status** | **Phase 0 decisions largely locked** — complete bootstrap spikes, then Phase 1 |
| **Architecture** | **Rust L0 + GDExtension**; GDScript first-class; C# second-class |
| **Repo** | [cyotee/fairplay-crypto-godot](https://github.com/cyotee/fairplay-crypto-godot) |
| **License** | MIT |
| **Last updated** | 2026-07-20 |

Implements the PRD. **Do not bulk-implement crypto until Phase 0 exit** (hello GDExtension on desktop + HTML5, secp crate chosen, CI green).

**Sibling reference only:** [fairplay-crypto](https://github.com/cyotee/fairplay-crypto) algorithms/tests — reimplement in Rust; no shared runtime.

---

## 1. Goals and done bars

### 1.1 Milestone A

| Deliverable | Done when |
|-------------|-----------|
| Rust L0 P0/P1 | SRA, keychain, secp validation, commitments, commit–reveal shuffle, Merkle, hashing, wire DTOs (no secrets) |
| Tests | Extensive `cargo test` unit + multi-party scenarios |
| GDExtension addon | `addons/manamesh_fairplay/` + `.gdextension` + binaries for supported A targets |
| GDScript API | Thin wrappers; all A features callable from GDScript |
| Samples | mental_poker_loop, poker_shaped, merkle_battleship (GDScript) |
| MultiplayerAPI | **Networked mental_poker** variant (host/client RPCs; no sk on wire) |
| Simulator | In-process multi-seat for offline tests/samples |
| Platforms | Smoke: **Editor + desktop + HTML5**; mobile **not** required |
| Docs | Threat model, BYO netcode, install (no .NET for GDScript path) |

### 1.2 Milestone B (lean public)

| Deliverable | Done when |
|-------------|-----------|
| Features | Same as A (P0/P1 only) |
| Packaging | Asset Library–ready **ManaMesh FairPlay for Godot** |
| Polish | THIRD_PARTY_NOTICES, listing copy, limitations |

### 1.3 Out of critical path

- Mobile export binaries  
- Full production multiplayer game / WebRTC join codes  
- Advanced crypto (Shamir, threshold, Paillier, ZK shuffle)  
- First-class dual C# crypto core  
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

### Phase 0 — Spikes & bootstrap (remaining)

| ID | Task | Output | Verification |
|----|------|--------|--------------|
| 0.1 | Toolchain: Rust, godot-rust (gdext), Godot **4.7.x** .NET **not** required for primary path | `docs/research/toolchain.md` | Editor opens sample project |
| 0.2 | Secp crate bake-off (`secp256k1`, `k256`, etc.) | `docs/research/crypto-stack.md` | Microbench + API fit for SRA |
| 0.3 | **Hello GDExtension** desktop (macOS/Linux/Windows as available) | Load extension; call one method from GDScript | Pass |
| 0.4 | **Hello GDExtension HTML5** (side module + export) | `docs/research/gdextension-web.md` | Browser runs one call; document COOP/COEP |
| 0.5 | Scaffold workspace, empty core tests, addon skeleton, CI `cargo test` | Green CI | PR green |
| 0.6 | Confirm naming: crates, addon path, class prefixes | §12 | — |
| 0.7 | Optional: C# calling GDExtension smoke (docs only if flaky) | Note in godot-integration | Best-effort |

**Exit criteria:** Desktop + HTML5 extension load proven; secp crate chosen; CI green; §12 complete.

**Kill / escalate:** If HTML5 GDExtension path fails after reasonable effort → stakeholder re-scope A (do not silently drop web).

### Phase 1 — Rust core P0 + tests

| ID | Task | Tests |
|----|------|-------|
| 1.1 | Workspace structure, clippy/fmt, CI | `cargo test` / `clippy` |
| 1.2 | Secp facade + validation | Valid/invalid points; ser/de |
| 1.3 | Hashing + secure RNG wrappers | Hash vectors; RNG length |
| 1.4 | Keychain + policies | Admit; reject bad/dupe keys |
| 1.5 | SRA multi-layer encrypt/peel + deck helpers | 2–4 party scenarios; wrong peel fails |
| 1.6 | Card/payload mapping | Injectivity for deck domain |
| 1.7 | sk↔pk binding helper | Mismatch rejected |
| 1.8 | Algorithm notes draft | Doc checklist |

**Exit:** Multi-party encrypt→peel green; no `godot` deps in core crate.

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
| 3.3 | Sample mental_poker_loop offline | Smoke |
| 3.4 | Sample mental_poker **MultiplayerAPI** | Host/client checklist; no sk on wire |
| 3.5 | Sample poker_shaped | Smoke |
| 3.6 | Sample merkle_battleship | Smoke |
| 3.7 | BYO netcode + integration docs | Review |
| 3.8 | Platform matrix Editor/desktop/HTML5 | `docs/platform-matrix.md` |
| 3.9 | Asset Library listing draft | Positioning language |
| 3.10 | README quickstart (GDScript, no .NET) | ≤15 min dry-run |

**Exit:** Milestone A checklist §11.

### Phase 4 — Lean Milestone B

| ID | Task |
|----|------|
| 4.1 | Docs polish + limitations |
| 4.2 | THIRD_PARTY_NOTICES |
| 4.3 | Asset Library package tree + install from zip |
| 4.4 | Listing final: ManaMesh FairPlay for Godot |
| 4.5 | GitHub release tag |

### Phase 5 — Post-publish

| ID | Task |
|----|------|
| 5.1 | Mobile GDExtension targets (Android/iOS) |
| 5.2 | Advanced modules (optional) |
| 5.3 | Richer C# usage samples |
| 5.4 | Full multiplayer demo / WebRTC demos |

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

- [x] Architecture locked (Rust GDExtension) — research + Q&A  
- [x] Platform A bar locked (Editor/desktop/HTML5; mobile later)  
- [ ] Toolchain doc  
- [ ] Secp crate chosen  
- [ ] Hello GDExtension desktop  
- [ ] Hello GDExtension HTML5  
- [ ] Scaffold + CI `cargo test`  
- [ ] §12 filled for remaining TBD rows  

### Milestone A exit

- [ ] Phases 0–3 complete  
- [ ] `cargo test` green  
- [ ] Shuffle anti-cheat green  
- [ ] Simulator mental-poker scenario green  
- [ ] GDScript covers A API matrix  
- [ ] Samples: three offline + MultiplayerAPI mental_poker  
- [ ] Editor + desktop + HTML5 smoke documented  
- [ ] No private keys on sample wire paths  
- [ ] Threat model + integration docs  
- [ ] README ≤15 min install (no .NET)  

### Milestone B exit

- [ ] A still green  
- [ ] THIRD_PARTY_NOTICES  
- [ ] Asset Library package clean install  
- [ ] Listing: ManaMesh FairPlay for Godot; no gambling/crypto-currency framing  
- [ ] No ZK overclaims  
- [ ] Release tagged  

---

## 12. Decision log

| Decision | Choice | Date | Notes |
|----------|--------|------|-------|
| Primary architecture | **Rust + GDExtension** | 2026-07-20 | Full pivot; C# cannot do HTML5 in Godot 4 |
| API priority | **GDScript first-class**; C# second-class via extension | 2026-07-20 | |
| Milestone A platforms | **Editor + desktop + HTML5** | 2026-07-20 | Mobile post-publish |
| Mobile for A | **Skip** | 2026-07-20 | |
| Secp (direction) | **Rust libsecp256k1 / rustcrypto family** | 2026-07-20 | Exact crate TBD Phase 0.2 |
| Exact secp crate | _TBD Phase 0.2_ | | `secp256k1` vs `k256` bake-off |
| Godot version | **4.4+ min; develop 4.7.x** | 2026-07-20 | Research: 4.7.1 latest stable |
| L0 language / tests | **Rust / cargo test** | 2026-07-20 | Supersedes xUnit/C# core |
| Addon path | **`addons/manamesh_fairplay/`** | 2026-07-20 | |
| Listing name | **ManaMesh FairPlay for Godot** | 2026-07-20 | |
| MultiplayerAPI sample | **Network mental_poker** (not separate 4th game) | 2026-07-20 | |
| Golden vectors | **Optional** | 2026-07-20 | |
| Facade shape | **GDExtension classes + thin GDScript wrappers** | 2026-07-20 | |
| C# crypto core / NBitcoin / Secp256k1.Net | **Rejected for product primary** | 2026-07-20 | Superseded by Rust pivot |
| gdext / godot-rust version | _TBD Phase 0.1_ | | Pin to Godot 4.4–4.7 support |
| Crate names | _TBD Phase 0.6_ | | Working: `manamesh_fairplay_core` / `_godot` |
| HTML5 status | **Required for A** (prove in 0.4) | 2026-07-20 | Escalate if spike fails |

---

## 13. Execution order

```
Phase 0: toolchain → secp choice → hello desktop → hello HTML5 → scaffold/CI
  → Phase 1 P0 crypto + cargo test
    → Phase 2 P1 shuffle/Merkle/wire/simulator
      → Phase 3 GDExtension + samples + MultiplayerAPI + export smoke
        → Phase 4 lean publish
          → Phase 5 mobile + optional advanced
```

---

## 14. Document history

| Date | Change |
|------|--------|
| 2026-07-20 | Initial plan (C# core + GDScript facade) |
| 2026-07-20 | Rewrite: Rust GDExtension primary; HTML5 in A; mobile deferred; research-backed platform constraints; updated tests/CI/decision log |
