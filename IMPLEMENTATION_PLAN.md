# Implementation & Testing Plan — ManaMesh FairPlay for Godot

| Field | Value |
|-------|--------|
| **Product** | **ManaMesh FairPlay for Godot** |
| **PRD** | [PRD.md](./PRD.md) |
| **Status** | **Ready for Phase 0** — do not start large core coding until §12 decision log is filled |
| **Repo** | [cyotee/fairplay-crypto-godot](https://github.com/cyotee/fairplay-crypto-godot) |
| **Monorepo path** | `packages/fairplay-crypto-godot` in manamesh-games |
| **License** | MIT (entire product) |
| **Architecture** | C# crypto core + GDScript facade; dual GDScript/C# APIs |
| **Last updated** | 2026-07-20 |

This plan implements the PRD in sequenced work packages with an explicit **testing strategy** at each phase. It does **not** authorize full implementation until Phase 0 spike decisions are recorded in §12.

**Sibling reference (inspiration only):** [fairplay-crypto](https://github.com/cyotee/fairplay-crypto) C# modules and tests — reimplement/adapt; no runtime dependency.

---

## 1. Goals and done bars

### 1.1 Milestone A — First engineering ship

| Deliverable | Done when |
|-------------|-----------|
| C# core P0/P1 | SRA, keychain, secp256k1 validation, commitments, **commit–reveal shuffle proofs**, Merkle, hashing, wire DTOs (no secrets) |
| Core tests | Extensive unit + multi-party scenario tests on plain .NET CI (`dotnet test`) |
| Godot addon | Installable `addons/manamesh_fairplay/` with C# glue + **GDScript facade** (no second crypto impl) |
| Dual-language samples | Mental-poker loop, poker-shaped, Merkle battleship — **GDScript + C#** entry points |
| Simulator | In-process multi-seat simulator for offline teaching and automated scenario tests |
| MultiplayerAPI sample | At least one official sample using Godot high-level multiplayer (host/client, RPCs); private keys never on wire |
| Platforms | Export **smoke** for Editor, desktop, **mobile**, and **HTML5** (or documented stakeholder-approved blocker if infeasible after spike) |
| Docs | Threat model, BYO-netcode guide, dual-language quickstart, Godot C# install prerequisites |

### 1.2 Milestone B — Public publish (lean)

| Deliverable | Done when |
|-------------|-----------|
| Feature set | Same as A (P0/P1 only) — **no** Shamir / threshold / Paillier / ZK gate |
| Packaging | Asset Library–ready package under **ManaMesh FairPlay for Godot** |
| Polish | Third-Party Notices, README, limitations section, listing copy (non-gambling, non-cryptocurrency) |
| Platforms | A smoke matrix green (or explicit supported-platform list if a target was re-scoped) |

### 1.3 Explicitly out of critical path

- Full production multiplayer game / matchmaking / WebRTC join codes  
- Advanced crypto modules (Shamir, threshold, Paillier, true ZK shuffle)  
- External security audit  
- Wire compatibility with TS or sibling C# Unity package  
- Godot 3 support  

---

## 2. Architecture (implementation shape)

### 2.1 Layers

```
┌──────────────────────────────────────────────────────────────┐
│  L2 Samples                                                  │
│  mental_poker_loop | poker_shaped | merkle_battleship        │
│  multiplayer_api (MultiplayerAPI)                            │
│  Dual entry: *.gd + *.cs                                     │
└────────────────────────────▲─────────────────────────────────┘
                             │ uses
┌────────────────────────────┴─────────────────────────────────┐
│  L1 Godot addon — addons/manamesh_fairplay/                  │
│  C# Godot glue (GodotObject / Node helpers as needed)        │
│  GDScript facade (marshals only — no crypto math)            │
│  Wire serializers · optional signals · plugin.cfg            │
└────────────────────────────▲─────────────────────────────────┘
                             │ references
┌────────────────────────────┴─────────────────────────────────┐
│  L0 C# core — csharp/ManaMesh.FairPlay.Core (name TBD)       │
│  Pure managed C# — NO GodotEngine types in L0                │
│  NO MultiplayerAPI — NO game rules                           │
│  SRA · Keychain · Shuffle commit/reveal · Merkle · Wire DTOs │
│  MultiSeatSimulator (in-process transport for tests/samples) │
└──────────────────────────────────────────────────────────────┘
```

### 2.2 Netcode philosophy (locked)

| What we ship | What we do not ship |
|--------------|---------------------|
| Stable **protocol DTOs** (public keys, ciphertexts, peels, commitments, shuffle commit/open) | Required multiplayer stack |
| Docs: serialize → send on your RPC → validate on receipt | Matchmaking, relay, lobby, WebRTC |
| Optional message sink/source interfaces | Hard dependency on a third-party netcode addon |
| **MultiSeatSimulator** for offline samples + tests | Production lobby product |
| **One MultiplayerAPI sample** teaching host/client flow | “Official” only-netcode path for all samples |

**Rules:**

1. L0 never imports `Godot` or MultiplayerAPI.  
2. L1 may define Godot-facing serializers and thin nodes; it must not reimplement SRA/Merkle math.  
3. Samples may use MultiplayerAPI; integrators may ignore it and use their own transport.  
4. Private keys stay local on each peer — only public material and protocol messages are sent.

### 2.3 Dual-language surface

| Surface | Audience | Implementation |
|---------|----------|----------------|
| C# API | Godot C# projects | Call L0 (and thin L1 helpers) directly |
| GDScript facade | GDScript projects | Thin scripts/classes that call into C# core via Godot C# interop |

**Prerequisite for all consumers:** Godot build with **.NET / C# support**, even if application scripts are GDScript-only. Document in README and Asset Library listing.

### 2.4 Core design principles

1. Idiomatic C# in L0; idiomatic GDScript in the facade.  
2. Private keys never appear in shared state or envelope payloads.  
3. Game-agnostic names in core (`ParticipantId`, `LayeredCiphertext`, `MerkleProof`).  
4. Commit–reveal shuffle proofs required for post-commit order-swap anti-cheat.  
5. Drift from TS and sibling Unity C# APIs is allowed and expected.  
6. Prefer maintained EC libraries over hand-rolled secp256k1.

### 2.5 Marketplace positioning (locked)

**What we are:** P2P / multiplayer cryptography for securing secrets and verifying fairness — any genre.

**What we are not:** gambling product; cryptocurrency / wallet product; boardgame.io port; real-money tooling.

Listing language: *fair play*, *secret state*, *P2P integrity*, *commitments*, *verifiable shuffle (commit–reveal)*, *multiplayer cryptography*. Avoid casino/betting/token/NFT framing.

---

## 3. Repository layout (target)

```
fairplay-crypto-godot/
├── PRD.md
├── IMPLEMENTATION_PLAN.md          # this file
├── README.md
├── LICENSE
├── THIRD_PARTY_NOTICES.md          # when deps land
├── docs/
│   ├── threat-model.md
│   ├── algorithms/
│   │   ├── sra-mental-poker.md
│   │   ├── keychain.md
│   │   ├── shuffle-commit-reveal.md
│   │   └── merkle.md
│   ├── integration-byo-netcode.md
│   ├── godot-integration.md        # dual API + facade
│   ├── platform-matrix.md          # export smoke results
│   ├── asset-library-listing-draft.md
│   └── research/
│       ├── crypto-stack.md         # Phase 0
│       ├── godot-csharp-gdscript.md
│       └── export-matrix.md
├── csharp/
│   ├── ManaMesh.FairPlay.sln
│   ├── ManaMesh.FairPlay.Core/
│   │   └── ManaMesh.FairPlay.Core.csproj
│   └── ManaMesh.FairPlay.Core.Tests/
├── addons/
│   └── manamesh_fairplay/
│       ├── plugin.cfg
│       ├── FairPlayPlugin.cs       # optional editor plugin
│       ├── runtime/                # C# Godot glue
│       └── facade/                 # GDScript facade scripts
├── samples/
│   ├── mental_poker_loop/
│   ├── poker_shaped/
│   ├── merkle_battleship/
│   └── multiplayer_api/
├── project.godot                   # sample host project (C# enabled)
├── tests/                          # Godot-side / GUT or headless if used
├── vectors/                        # optional JSON fixtures only
└── .github/workflows/
    └── ci.yml                      # dotnet test (+ optional Godot headless)
```

Exact assembly names and addon slug finalized in Phase 0 (§12).

---

## 4. Module breakdown (Milestone A core)

| Module | Namespace (tentative) | Responsibilities | Priority |
|--------|----------------------|------------------|----------|
| Primitives | `ManaMesh.FairPlay.Secp256k1` | Points, scalars, validation via chosen lib | P0 |
| Hashing | `ManaMesh.FairPlay.Hashing` | SHA-256, commitment digests | P0 |
| Keychain | `ManaMesh.FairPlay.Keychain` | Admit public keys, policies, duplicate rejection | P0 |
| SRA / mental poker | `ManaMesh.FairPlay.MentalPoker` | Keygen, encrypt layer, peel, deck helpers, card-id↔point mapping | P0 |
| Commitments | `ManaMesh.FairPlay.Commitment` | Bind arbitrary bytes / deck digests | P1 |
| Shuffle proofs | `ManaMesh.FairPlay.Shuffle` | Commit permutation, apply, open, verify (anti-cheat) | P1 |
| Merkle | `ManaMesh.FairPlay.Merkle` | Tree, proof generate/verify | P1 |
| Wire DTOs | `ManaMesh.FairPlay.Wire` | Serializable envelopes (**no secrets**) | P1 |
| Transport abstractions | `ManaMesh.FairPlay.Transport` | Sink/source interfaces + `MultiSeatSimulator` | P1 |
| GDScript facade | `addons/.../facade/` | Marshal dictionaries/PackedByteArray ↔ core | A |
| Godot Multiplayer sample | `samples/multiplayer_api/` | Host/client fair-play message flow | A |

### Post-publish / experimental (not B gate)

| Module | Notes |
|--------|--------|
| Shamir SSS | Optional later |
| Threshold suite | Feldman DKG, EC ElGamal-style, DLEQ, ECDSA |
| Paillier | Productized or documented limits |
| ZK shuffle | Only if marketing claims need it |

---

## 5. Phased work packages

### Phase 0 — Research spikes & bootstrap (blocking)

**Duration guide:** ~1–2 weeks focused spikes.

| ID | Task | Output | Tests / verification |
|----|------|--------|----------------------|
| 0.1 | Crypto stack spike: managed secp256k1 candidates (e.g. NBitcoin.Secp256k1), RNG, SHA-256 under **Godot C#** | `docs/research/crypto-stack.md` + decision | Minimal encrypt/hash console or Godot script green |
| 0.2 | **C# core ↔ GDScript facade** interop spike (call encrypt/peel from `.gd`) | `docs/research/godot-csharp-gdscript.md` | GDScript unit call asserts round-trip |
| 0.3 | **Export matrix spike:** desktop, Android and/or iOS, HTML5 with C# | `docs/research/export-matrix.md` pass/fail | Smoke export runs or recorded blockers |
| 0.4 | Godot version + .NET TFM floor (latest stable now; oldest practical 4.x) | Record in research + README draft | Document in §12 |
| 0.5 | Scaffold repo: solution, empty core + tests, addon skeleton, `project.godot` | Compiling empty core | `dotnet test` empty green |
| 0.6 | CI skeleton (GitHub Actions: `dotnet test`) | `.github/workflows/ci.yml` | PR template green |
| 0.7 | Naming decision: assembly, namespaces, addon folder, Asset Library title | §12 decision log | — |
| 0.8 | HTML5/mobile go/no-go stakeholder checkpoint if spike fails | Re-scope note or proceed | Sign-off if re-scope |

**Exit criteria:** Crypto stack chosen; GDScript→C# path proven; export matrix recorded (all A targets green **or** approved re-scope); decision log complete; empty solution + CI green.

**Kill / escalate criteria:** If HTML5 Godot C# cannot run the chosen crypto path, **stop and re-scope A platforms with stakeholders** before Phase 1 bulk coding.

---

### Phase 1 — C# core P0 + tests

| ID | Task | Output | Tests (required) |
|----|------|--------|------------------|
| 1.1 | Project structure, nullable, analyzers, CI hard gate | Green CI | `dotnet test` on PR |
| 1.2 | Secp256k1 facade + validation helpers | API | Valid/invalid points; identity; serialization round-trip |
| 1.3 | Hashing + secure random wrappers | API | Deterministic hash vectors; RNG non-zero length |
| 1.4 | Keychain + strict policy (mental-poker default) | API | Admit OK; reject invalid curve point; reject duplicates; policy variants |
| 1.5 | SRA: keygen, encrypt, peel, multi-layer, deck helpers | API | 2–4 party encrypt→peel to plaintext; wrong-key peel fails |
| 1.6 | Card/payload mapping strategy (C#-idiomatic) | `docs/algorithms/sra-mental-poker.md` | Mapping injectivity tests for deck domain |
| 1.7 | Client-side sk↔pk binding helper | API | Mismatch rejection before encrypt |
| 1.8 | Algorithm notes draft | `docs/algorithms/*` P0 set | Doc review checklist |

**Exit criteria:** Multi-party encrypt → peel works; invalid keys/peels rejected; CI green; no `Godot` references in L0.

---

### Phase 2 — C# core P1: commitments, shuffle anti-cheat, Merkle, wire

| ID | Task | Output | Tests (required) |
|----|------|--------|------------------|
| 2.1 | Commitment create/verify | API | Happy path + wrong opening fails |
| 2.2 | Shuffle: commit permutation, apply to layered deck, open, verify | API | **Anti-cheat suite** (§6.2) |
| 2.3 | Threat model for commit–reveal shuffle | `docs/threat-model.md` | Claims match implemented API |
| 2.4 | Merkle tree + proofs | API | Include/exclude proofs; tampered leaf fails |
| 2.5 | Wire DTO catalog (no secrets) + transport interfaces | API + doc draft | Serialization round-trip; **secret-field lint tests** |
| 2.6 | `MultiSeatSimulator` | In-process multi-party bus | N-seat scenario tests without Godot |

**Exit criteria:** Shuffle tamper tests green; Merkle green; simulator drives a full mental-poker scenario in pure .NET tests; threat model merged.

---

### Phase 3 — Godot addon + dual samples + MultiplayerAPI + platforms (Milestone A)

| ID | Task | Output | Tests / verification |
|----|------|--------|----------------------|
| 3.1 | Addon layout per Phase 0 (`plugin.cfg`, runtime, facade) | Importable addon | Clean project install steps documented |
| 3.2 | GDScript facade covering all A public ops | `facade/*.gd` | GDScript call coverage vs C# API matrix (§6.3) |
| 3.3 | C# Godot glue (if needed beyond pure L0) | Thin only | No crypto math duplication |
| 3.4 | Sample: **mental_poker_loop** (GDScript + C#) | Scenes/scripts | Simulator-driven offline run; automated scenario where possible |
| 3.5 | Sample: **poker_shaped** (GDScript + C#) | Scenes/scripts | Same |
| 3.6 | Sample: **merkle_battleship** (GDScript + C#) | Scenes/scripts | Commit/challenge/open demo |
| 3.7 | Sample: **multiplayer_api** | Host/client scene | RPC path uses wire DTOs only; manual + scripted checklist |
| 3.8 | Integration guide (BYO netcode + MultiplayerAPI notes) | `docs/integration-byo-netcode.md`, `docs/godot-integration.md` | Review |
| 3.9 | Platform smoke matrix | `docs/platform-matrix.md` | Editor + desktop + mobile + HTML5 per §6.5 |
| 3.10 | Asset Library listing draft | `docs/asset-library-listing-draft.md` | Positioning language check |
| 3.11 | README quickstart (C# + GDScript) | README | 15-minute install dry-run |

**Exit criteria:** Milestone A complete — dual-language samples run; MultiplayerAPI sample runs; L0 tests green; platform matrix filled; no private keys on sample wire paths.

---

### Phase 4 — Milestone B: lean public publish

| ID | Task | Output | Verification |
|----|------|--------|--------------|
| 4.1 | Docs polish, limitations, security honesty | Final docs | Threat model + no ZK overclaims |
| 4.2 | `THIRD_PARTY_NOTICES.md` + license audit | Notices | All deps compatible with MIT distribution |
| 4.3 | Package layout for Asset Library | Submission tree | Install from packaged zip on clean machine |
| 4.4 | Listing copy final | Listing draft | Non-gambling / non-cryptocurrency wording |
| 4.5 | Tag release (e.g. `v0.1.0`) | GitHub release | CI green on tag |
| 4.6 | Submit / prepare Asset Library entry | **ManaMesh FairPlay for Godot** | Submission checklist |

**Exit criteria:** Lean B complete — A feature set only; public packaging ready; platform support documented.

---

### Phase 5 — Post-publish (optional)

| ID | Task |
|----|------|
| 5.1 | Advanced modules (Shamir, threshold, Paillier, ZK) as experimental packages |
| 5.2 | Full multiplayer demo game |
| 5.3 | WebRTC / join-code demos |
| 5.4 | Optional official adapters for popular Godot net patterns |
| 5.5 | Redot compatibility pass |

---

## 6. Testing strategy (assurance without audit)

### 6.1 Test layers

| Layer | Tooling | When | Focus |
|-------|---------|------|--------|
| **L0 unit** | xUnit (or NUnit) + `dotnet test` | Phase 1–2, every PR | Math, policies, edge cases |
| **L0 scenario** | Same | Phase 1–2 | N-party mental poker, shuffle tamper, Merkle cheats, full simulator loop |
| **Property / fuzz** | Where useful | Phase 2+ | Permutations, layer counts, parsing, Merkle depth |
| **Negative** | Required | All phases | Bad points, duplicate keys, wrong peel, secret-in-payload |
| **Facade parity** | Godot headless and/or thin C# harness calling facade | Phase 3 | Every public GDScript op matches C# core behavior |
| **Sample smoke** | Editor manual + scripted checklists | Phase 3 | Dual-language samples |
| **MultiplayerAPI** | 2-instance Editor/export or headless multiplayer | Phase 3 | Host/client message flow; no sk on wire |
| **Export smoke** | Godot export templates | Phase 0 spike + Phase 3 | Desktop, mobile, HTML5 |
| **CI** | GitHub Actions | Phase 0+ | `dotnet test` required; Godot headless optional once stable |

### 6.2 Shuffle anti-cheat tests (mandatory)

1. Commit permutation P, shuffle with P, open P → verify **OK**  
2. Commit P, shuffle with P′ ≠ P, open P → verify **fail**  
3. Commit P, open wrong nonce/perm → **fail**  
4. (Optional property) random N, random P — open always verifies iff permutation matches  

### 6.3 Dual-language API coverage matrix (Milestone A)

Maintain a table in `docs/godot-integration.md` (filled in Phase 3):

| Capability | C# API test | GDScript facade test | Sample (CS) | Sample (GD) |
|------------|-------------|----------------------|-------------|-------------|
| Keygen / keychain admit | ✓ | ✓ | mental_poker | mental_poker |
| Layer encrypt / peel | ✓ | ✓ | mental_poker | mental_poker |
| Shuffle commit/open | ✓ | ✓ | mental_poker / poker | same |
| Merkle commit/proof | ✓ | ✓ | battleship | battleship |
| Wire encode/decode | ✓ | ✓ | multiplayer_api | multiplayer_api |

**Rule:** No public A capability ships C#-only without a GDScript facade entry (and vice versa for documented facade methods).

### 6.4 Security-oriented tests

| Test class | Examples |
|------------|----------|
| Key safety | Serializing envelopes never includes private key fields |
| Binding | Encrypt rejected if local sk does not match published pk |
| Keychain | Reject non-curve points, infinity (if policy), duplicates |
| Peel integrity | Wrong participant order / wrong layer fails closed |
| Commitment | Wrong opening fails; truncated proofs fail |
| Sample audit | Grep/static check samples for accidental sk logging or RPC of sk |

### 6.5 Platform smoke checklist (Milestone A)

For each target: **Editor**, **desktop** (at least one of macOS/Windows/Linux), **mobile** (Android and/or iOS), **HTML5**:

| Check | Pass criteria |
|-------|----------------|
| Build | Export succeeds with documented Godot/.NET versions |
| Run sample | At least mental-poker simulator sample runs (or reduced smoke script) |
| Crypto path | One encrypt→peel or hash+keychain op succeeds at runtime |
| Notes | Record OS, Godot version, export options, known limitations |

Results live in `docs/platform-matrix.md`.

### 6.6 CI policy

| Check | Required from |
|-------|----------------|
| `dotnet test` (L0) | End of Phase 0 scaffold |
| No Godot types in L0 project | Phase 1 (analyzer or path convention) |
| Godot headless sample/facade tests | Best-effort from Phase 3; not a Phase 1 gate |
| Export matrix | Manual/semi-automated Phase 3; artifacts optional |

### 6.7 Inspiration from sibling suite (optional)

When useful during port learning, mirror *kinds* of tests from `packages/fairplay-crypto/csharp/ManaMesh.Crypto.Tests` (keychain, SRA, shuffle anti-cheat, Merkle, simulator scenarios). Do **not** require identical vectors or shared binaries.

---

## 7. Documentation deliverables

| Doc | Phase |
|-----|--------|
| Research: crypto stack, GDScript interop, export matrix | 0 |
| Algorithm notes (SRA, keychain, shuffle, Merkle) | 1–2 |
| Threat model (commit–reveal honesty) | 2 |
| BYO netcode + MultiplayerAPI sample notes | 3 |
| Godot dual-language integration guide | 3 |
| Platform matrix | 0 + 3 |
| Asset Library listing draft | 3–4 |
| README quickstart | 3 |
| THIRD_PARTY_NOTICES | 4 |

---

## 8. Risks and mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| **HTML5 + Godot C# + crypto lib fails** | Blocks A as written | Phase 0 kill criteria; re-scope platforms with stakeholders before Phase 1 bulk work |
| Mobile export / AOT issues | Blocks A mobile | Spike early; prefer pure managed libs; document IL trimming exclusions |
| GDScript facade becomes a second crypto impl | Security / drift bugs | Facade-only marshal rule; parity tests; code review checklist |
| Scope creep into netcode product | Delays A | MultiplayerAPI is **one sample**; core stays BYO |
| Dual-language sample maintenance cost | Slow Phase 3 | Shared scenario logic in L0 tests; samples stay thin UIs |
| Overclaiming shuffle ZK | Misleading users | Docs + listing: commit–reveal only until post-publish ZK |
| Crypto NuGet incompatible with Godot | Blocks core | Phase 0 candidate bake-off under real Godot C# |
| .NET requirement surprises GDScript users | Adoption friction | README + listing call out C#-enabled Godot requirement upfront |
| Drift from sibling ManaMesh.Crypto confuses maintainers | Extra support load | Explicit “sibling, not shared runtime” in README |

---

## 9. Dependency policy

| Allowed in L0 core | Forbidden in L0 |
|--------------------|-----------------|
| Chosen managed crypto/hash libraries | `GodotSharp` / `Godot.*` |
| BCL | MultiplayerAPI, ENet, WebRTC addons |
| | boardgame.io, JS interop, UnityEngine |
| | `fairplay-crypto` Unity package as package reference |

| Allowed in L1 addon | Forbidden as hard deps |
|---------------------|------------------------|
| L0 reference | Third-party netcode frameworks as required deps |
| Godot engine assemblies | Reimplemented SRA/Merkle in GDScript |
| GDScript facade scripts | |

| Allowed in samples | Notes |
|--------------------|--------|
| MultiplayerAPI | Official sample only |
| Simulator | Default offline path |

---

## 10. Team workflow

1. Work in `packages/fairplay-crypto-godot` (this submodule).  
2. Small commits / PRs tagged by task ID (`0.3`, `1.5`, `2.2`, …).  
3. Bump submodule pointer in `manamesh-games` when convenient.  
4. **Phase 0 decision log (§12) must be filled before large core coding.**  
5. Do not expand into advanced modules until lean B is released (unless explicitly re-prioritized).  
6. Any A platform failure → stakeholder checkpoint, not silent scope cut.

---

## 11. Acceptance checklists

### Phase 0 exit

- [ ] Crypto stack decided and documented  
- [ ] GDScript → C# core call proven  
- [ ] Export matrix documented for desktop, mobile, HTML5 (pass or approved re-scope)  
- [ ] Godot/.NET version floor recorded  
- [ ] Scaffold + CI `dotnet test` green  
- [ ] §12 decision log complete  

### Milestone A exit

- [ ] Phases 0–3 complete  
- [ ] `dotnet test` green for L0  
- [ ] Shuffle tamper tests green  
- [ ] Keychain / SRA / Merkle scenario tests green  
- [ ] `MultiSeatSimulator` full mental-poker scenario in pure .NET  
- [ ] GDScript facade covers A public API matrix  
- [ ] Three headline samples run offline (GDScript + C#)  
- [ ] MultiplayerAPI sample runs host/client without private keys on wire  
- [ ] L0 has zero Godot references  
- [ ] Platform matrix: Editor + desktop + mobile + HTML5 (or approved re-scope)  
- [ ] Threat model + integration + dual-language docs published  
- [ ] README quickstart dry-run ≤ 15 minutes  

### Milestone B exit (lean public publish)

- [ ] Milestone A exit still green  
- [ ] THIRD_PARTY_NOTICES complete  
- [ ] Asset Library package installs cleanly  
- [ ] Listing copy: ManaMesh FairPlay for Godot; no gambling/cryptocurrency framing  
- [ ] No advanced-module or ZK overclaims  
- [ ] GitHub release tagged  

---

## 12. Phase 0 decision log (fill before Phase 1)

| Decision | Options considered | Choice | Date | Notes |
|----------|-------------------|--------|------|-------|
| Secp256k1 library | NBitcoin.Secp256k1; others | _TBD_ | | Must work under Godot C# exports |
| L0 TFM | netstandard2.1; net8.0; Godot-recommended | _TBD_ | | Align with Godot .NET version |
| Godot min version | Latest stable; oldest practical 4.x | _TBD_ | | After packaging spike |
| Assembly / NuGet-style name | `ManaMesh.FairPlay.Core` vs alternate | _TBD_ | | |
| Addon folder slug | `manamesh_fairplay` vs alternate | _TBD_ | | Asset Library constraints |
| GDScript facade style | Autoload singleton; static helpers; RefCounted API | _TBD_ | | From interop spike |
| HTML5 status | Supported / deferred with approval | _TBD_ | | Kill criteria if unsupported |
| Mobile status | Android only / iOS only / both | _TBD_ | | |
| MultiplayerAPI sample scope | 4th sample vs networked mental_poker | _TBD_ | | Prefer thin 4th sample if clearer |
| Test framework | xUnit vs NUnit | _TBD_ | | Prefer xUnit unless Godot template forces otherwise |
| Optional golden vectors | Yes/no for early port | _TBD_ | | Default: C#-owned tests sufficient |

---

## 13. Suggested execution order (summary)

```
Phase 0 spikes + scaffold + CI
    → Phase 1 P0 crypto + tests
        → Phase 2 P1 shuffle/Merkle/wire/simulator + tests
            → Phase 3 addon + facade + dual samples + MultiplayerAPI + export smoke
                → Phase 4 lean public publish
                    → Phase 5 optional advanced / demos
```

**Parallelism tips:**

- Docs (algorithm notes, threat model drafts) can track Phase 1–2.  
- Sample UI polish can lag core scenario correctness.  
- Export automation can trail first manual smoke, but A does not close without matrix.

---

## 14. Document history

| Date | Change |
|------|--------|
| 2026-07-20 | Initial implementation & testing plan aligned to locked PRD (C# core + GDScript facade, lean B, dual APIs, MultiplayerAPI sample, all-platform A smoke) |
