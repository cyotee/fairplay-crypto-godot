# Toolchain notes (Phase 0)

| Tool | Version used |
|------|----------------|
| Godot | 4.7.1.stable.official |
| Rustc / Cargo | 1.97 (Homebrew) |
| godot-rust `godot` crate | 0.3.5 (`api-4-3` feature; runtime 4.7 OK) |
| secp256k1 | 0.29 (bitcoin-core bindings) |
| Host | macOS x86_64 |

## Build core tests

```bash
cd rust && cargo test -p manamesh_fairplay_core
```

## Build GDExtension

```bash
cd rust && cargo build -p manamesh_fairplay_godot --release
```

Copy dylib into `addons/manamesh_fairplay/bin/` per `.gdextension` library keys.

## Cross-compiling all platforms locally

The addon ships one binary per platform (see `.gdextension` library keys):

| Platform | File | How |
|----------|------|-----|
| macOS universal | `lib…godot.macos.universal.dylib` | native clang, `lipo` arm64 + x86_64 |
| Linux x86_64 | `lib…godot.linux.x86_64.so` | `cargo zigbuild` (glibc 2.17 pin) |
| Windows x86_64 | `…godot.windows.x86_64.dll` | `cargo zigbuild` (mingw via zig) |
| Web (wasm) | `…godot.wasm` (+ `.threads.wasm`) | `scripts/build_web.sh` (emsdk + nightly) |

Tooling: `rustup target add aarch64-apple-darwin x86_64-unknown-linux-gnu x86_64-pc-windows-gnu`,
plus `zig` and `cargo install cargo-zigbuild` for the Linux/Windows C cross-compile of `secp256k1-sys`.

**Gotcha (macOS):** if Homebrew rust is on `PATH` before `~/.cargo/bin`, `cargo`/`rustc`
resolve to Homebrew's toolchain, which has **only** the host std — cross-target builds then fail
with `can't find crate for core … target may not be installed`. `rustup run stable` does **not**
fix this when the profile forces `/usr/local/bin` first. Pin the toolchain by absolute path instead:

```bash
TC="$HOME/.rustup/toolchains/stable-x86_64-apple-darwin/bin"
export RUSTC="$TC/rustc" CARGO="$TC/cargo"
cd rust
"$TC/cargo" build   -p manamesh_fairplay_godot --release --target aarch64-apple-darwin
"$TC/cargo" build   -p manamesh_fairplay_godot --release --target x86_64-apple-darwin
"$TC/cargo" zigbuild -p manamesh_fairplay_godot --release --target x86_64-unknown-linux-gnu.2.17
"$TC/cargo" zigbuild -p manamesh_fairplay_godot --release --target x86_64-pc-windows-gnu
# macOS universal:
lipo -create target/{aarch64,x86_64}-apple-darwin/release/libmanamesh_fairplay_godot.dylib \
  -output ../addons/manamesh_fairplay/bin/libmanamesh_fairplay_godot.macos.universal.dylib
codesign --force -s - ../addons/manamesh_fairplay/bin/libmanamesh_fairplay_godot.macos.universal.dylib
```

CI (`.github/workflows/release.yml`) builds each platform on a **native** runner instead, which
sidesteps all of the above and is the source of truth for released binaries.

## First open

Godot must scan GDExtensions once (`--import` or editor open) so `FairPlayApi` registers.
