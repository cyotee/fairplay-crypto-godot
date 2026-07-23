# ManaMesh FairPlay for Godot

**MIT** fair-play cryptography for **Godot 4**: commit–reveal (dice / hidden boards), public-key keychain, SRA mental poker, shuffle integrity, Merkle selective-open helpers (e.g. Battleship-style grids).

**Architecture:** Rust cryptography core + **GDExtension** · **GDScript-first** · no .NET required.

Built for **multiplayer game fairness** — hidden values, board placement, and cooperative deck deal without a trusted dealer.

| | |
|--|--|
| **Docs** | [cyotee.github.io/fairplay-crypto-godot](https://cyotee.github.io/fairplay-crypto-godot/) |
| **Releases** | [GitHub Releases](https://github.com/cyotee/fairplay-crypto-godot/releases) |
| **Status** | **v0.1 technical preview** — desktop + core tests solid; web preview; mobile not yet |
| **License** | [MIT](./LICENSE) |

## Quick start (install)

1. Copy `addons/manamesh_fairplay/` into your Godot 4.4+ project (binaries included under `bin/`).
2. Reload / import the project so the GDExtension registers.
3. Call the API:

```gdscript
var api = ClassDB.instantiate("FairPlayApi")
var faces := PackedByteArray([1, 3, 3, 5, 6])
var c = api.commit_dice_hand(faces)
# send only c.commitment_hex to peers
assert(api.verify_commitment(c.commitment_hex, faces, c.nonce_hex))
```

Full install guide: [Getting started](https://cyotee.github.io/fairplay-crypto-godot/getting-started/).

## Quick start (core tests)

```bash
cd rust
cargo test -p manamesh_fairplay_core
```

## Layout

| Path | Role |
|------|------|
| `rust/manamesh_fairplay_core` | Pure cryptography core (`cargo test`) |
| `rust/manamesh_fairplay_godot` | GDExtension bindings |
| `addons/manamesh_fairplay` | Godot addon + prebuilt binaries |
| `samples/dice_commit_reveal` | Offline commit–reveal sample |
| `samples/multiplayer_dice` | Networked message pattern (no sk on wire) |
| `docs/` | Source for the documentation site |

## Downstream games

A separate game (e.g. Liar’s Dice) can depend on this addon for multiplayer fairness. This library does not implement a full game, lobby, or store packaging.

## License

MIT — see [LICENSE](./LICENSE) and [THIRD_PARTY_NOTICES.md](./THIRD_PARTY_NOTICES.md).
