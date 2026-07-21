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

## First open

Godot must scan GDExtensions once (`--import` or editor open) so `FairPlayApi` registers.
