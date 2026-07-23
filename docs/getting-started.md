# Getting started

## Requirements

- **Godot 4.4+** (developed and CI-smoked on **4.7.x**)
- **No .NET** for GDScript consumers
- A prebuilt GDExtension binary for your platform (or build from source)

## Install from a release (recommended)

1. Download the latest [GitHub Release](https://github.com/cyotee/fairplay-crypto-godot/releases) assets, or clone this repository.
2. Copy `addons/manamesh_fairplay/` into your Godot project (keep the `bin/` folder with platform binaries).
3. Open the project once so Godot registers GDExtensions (`Project → Reload Current Project`, or run with `--import`).
4. Confirm the class exists:

```gdscript
var api = ClassDB.instantiate("FairPlayApi")
print(api.version())  # e.g. "0.1.0"
```

Or use the thin facade:

```gdscript
var fair := FairPlay.new()  # res://addons/manamesh_fairplay/facade/fair_play.gd
print(fair.version())
```

## Commit–reveal in one minute

```gdscript
var api = ClassDB.instantiate("FairPlayApi")
var faces := PackedByteArray([1, 3, 3, 5, 6])
var c: Dictionary = api.commit_dice_hand(faces)

# PUBLIC: broadcast only the commitment
var commitment_hex: String = c["commitment_hex"]

# PRIVATE until reveal: faces + nonce
var nonce_hex: String = c["nonce_hex"]

# Later, open and let peers verify
var ok: bool = api.verify_commitment(commitment_hex, faces, nonce_hex)
assert(ok)
```

See [Commit–reveal](commit-reveal.md) and the offline sample `samples/dice_commit_reveal/`.

## Run this repo’s smoke scripts

With Godot on your `PATH` and a matching binary under `addons/manamesh_fairplay/bin/`:

```bash
godot --path . --import   # once, generates .godot/
godot --headless --path . --script res://scripts/smoke_desktop.gd
# expect: SMOKE_DESKTOP_OK
```

Sample scenes:

| Path | Purpose |
|------|---------|
| `samples/dice_commit_reveal/` | Offline commit → open → verify |
| `samples/multiplayer_dice/` | RPC-shaped public messages (no private keys on wire) |

## Core tests (no Godot)

```bash
cd rust
cargo test -p manamesh_fairplay_core
```

## Next steps

- Wire multiplayer: [Bring-your-own netcode](netcode.md)
- Full method list: [GDScript API](api.md)
- Platform matrix: [Platforms](platforms.md)
- From-source / web builds: [Building from source](building.md)
