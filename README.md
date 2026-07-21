# ManaMesh FairPlay for Godot

**MIT** fair-play cryptography for **Godot 4**: commit–reveal (dice / hidden values), public-key keychain, SRA mental poker, shuffle integrity (commit–reveal), Merkle proofs.

**Architecture:** Rust crypto core + **GDExtension** · **GDScript-first** · no .NET required.

**Not** a gambling or cryptocurrency product.

## Status

Milestone A core + desktop GDExtension are implemented and tested. HTML5 requires Emscripten (see docs).

| Doc | Purpose |
|-----|---------|
| [PRD.md](./PRD.md) | Product requirements |
| [IMPLEMENTATION_PLAN.md](./IMPLEMENTATION_PLAN.md) | Phased plan |
| [docs/godot-integration.md](./docs/godot-integration.md) | Install + GDScript usage |
| [docs/threat-model.md](./docs/threat-model.md) | Commit–reveal honesty |
| [docs/integration-byo-netcode.md](./docs/integration-byo-netcode.md) | Wire rules |

## Quick start (core tests)

```bash
cd rust
cargo test -p manamesh_fairplay_core
```

## Quick start (Godot)

```bash
cd rust && cargo build -p manamesh_fairplay_godot --release
# copy dylib into addons/manamesh_fairplay/bin/ (see godot-integration.md)
cd ..
godot --path . --import   # once
godot --headless --path . --script res://scripts/smoke_desktop.gd
```

### GDScript commit–reveal (Liar’s Dice–shaped)

```gdscript
var api = ClassDB.instantiate("FairPlayApi")
var faces := PackedByteArray([1, 3, 3, 5, 6])
var c = api.commit_dice_hand(faces)
# send only c.commitment_hex to peers
assert(api.verify_commitment(c.commitment_hex, faces, c.nonce_hex))
```

## Layout

- `rust/manamesh_fairplay_core` — pure crypto (`cargo test`)
- `rust/manamesh_fairplay_godot` — GDExtension
- `addons/manamesh_fairplay` — Godot addon
- `samples/dice_commit_reveal` — offline sample
- `samples/multiplayer_dice` — networked message pattern (no sk on wire)

## Downstream game

A separate **Liar’s Dice** game repo can depend on this addon for multiplayer fairness (Steam/Itch/GOG/mobile/web). This library does not implement that game.

## License

MIT — see [LICENSE](./LICENSE).
