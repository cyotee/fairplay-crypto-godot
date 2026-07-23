# Building from source

Most game developers should use [release binaries](https://github.com/cyotee/fairplay-crypto-godot/releases). This page is for contributors and custom toolchains.

## Prerequisites

| Tool | Notes |
|------|--------|
| Rust stable | `rustup` recommended |
| Godot 4.4+ | 4.7.x for smoke parity with CI |
| (Web) Emscripten + Rust nightly + `rust-src` | See HTML5 section |
| (Cross desktop) `cargo-zigbuild` + Zig | Optional local Linux/Windows cross from macOS |

## Core tests

```bash
cd rust
cargo test -p manamesh_fairplay_core
```

## Desktop GDExtension

```bash
cd rust
cargo build -p manamesh_fairplay_godot --release
```

Copy the produced library into `addons/manamesh_fairplay/bin/` using the names expected by `manamesh_fairplay.gdextension`:

| Host build output (typical) | Addon filename |
|-----------------------------|----------------|
| `libmanamesh_fairplay_godot.dylib` (macOS) | `libmanamesh_fairplay_godot.macos.universal.dylib` (or lipo two arches) |
| `libmanamesh_fairplay_godot.so` (Linux) | `libmanamesh_fairplay_godot.linux.x86_64.so` |
| `manamesh_fairplay_godot.dll` (Windows) | `manamesh_fairplay_godot.windows.x86_64.dll` |

Then:

```bash
godot --path . --import
godot --headless --path . --script res://scripts/smoke_desktop.gd
```

## HTML5 / Web side module

```bash
# Requires emcc on PATH, rustup nightly + rust-src
./scripts/build_web.sh

# Optional: Godot web export (templates installed)
godot --headless --path . --export-release "Web" export/web/index.html
```

Serve locally:

```bash
cd export/web && python3 -m http.server 8060
# open http://127.0.0.1:8060/
```

Notes:

- Side module via `-sSIDE_MODULE=2` (see `rust/.cargo/config.toml`)
- `CFLAGS=-fPIC` for `secp256k1-sys` under Emscripten
- Nothreads build is the broader hosting default (no COOP/COEP required for threads)

## CI release builds

Push a `v*` tag (or run the **release-binaries** workflow manually) to build desktop + web artifacts and attach them to a GitHub Release. Desktop binaries are load-verified in headless Godot before packaging.

## Crypto stack

| Choice | Decision |
|--------|----------|
| Curve | secp256k1 |
| Crate | `secp256k1` (libsecp256k1 via rust-bitcoin bindings) |
| Hash | SHA-256 (`sha2`) |
| Commitments | `SHA-256(message ‖ nonce)` |
| SRA | Point × scalar layers; peel with modular inverse mod n |
| RNG | OS RNG via `rand` |
| Godot bindings | godot-rust / gdext |

Third-party licenses: [THIRD_PARTY_NOTICES.md](https://github.com/cyotee/fairplay-crypto-godot/blob/main/THIRD_PARTY_NOTICES.md) in the repository root.
