# Godot integration — ManaMesh FairPlay

## Requirements

- Godot **4.4+** (developed on 4.7.x)
- **No .NET** required for GDScript consumers
- Prebuilt GDExtension binary for your platform under `addons/manamesh_fairplay/bin/`

## Install

1. Copy `addons/manamesh_fairplay/` into your project (or add this repo as a submodule and path-map the addon).
2. Ensure a binary exists for your platform (build with `cargo build -p manamesh_fairplay_godot --release` and copy into `bin/`).
3. Open the project once so Godot verifies GDExtensions (or run with `--import`).
4. Use either:
   - `ClassDB.instantiate("FairPlayApi")`, or
   - `FairPlay` facade at `addons/manamesh_fairplay/facade/fair_play.gd`

## Liar’s Dice–style commit–reveal

```gdscript
var api = ClassDB.instantiate("FairPlayApi")
var faces := PackedByteArray([1, 3, 3, 5, 6])
var c: Dictionary = api.commit_dice_hand(faces)
# Broadcast only c.commitment_hex to peers (public)
# Keep c.nonce_hex and faces local until reveal
var ok: bool = api.verify_commitment(c.commitment_hex, faces, c.nonce_hex)
```

## BYO netcode

Serialize public wire messages only (see core `wire` module concepts and `samples/multiplayer_dice`). Do not put `private_key` fields in RPCs. Optional MultiplayerAPI sample: `samples/multiplayer_dice/`.

## Building the extension (developers)

```bash
cd rust
cargo build -p manamesh_fairplay_godot --release
# macOS x86_64 example:
cp target/release/libmanamesh_fairplay_godot.dylib \
  ../addons/manamesh_fairplay/bin/libmanamesh_fairplay_godot.macos.x86_64.dylib
```

## HTML5

Requires Emscripten + Godot web export templates and a WASM side-module build of the extension. See `docs/research/gdextension-web.md`.
