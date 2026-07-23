# Changelog

## 0.1.0 — 2026-07-21

First public technical preview.

### Added

- Rust core (`manamesh_fairplay_core`): hashing, commit–reveal (dice helpers), keychain, secp256k1, SRA mental poker, shuffle commit–reveal, Merkle, wire DTOs, multi-seat simulator tests
- GDExtension (`manamesh_fairplay_godot`) + `FairPlayApi` GDScript surface
- Thin `FairPlay` GDScript facade
- Addon layout under `addons/manamesh_fairplay/` with desktop + web prebuilt binaries
- Samples: `dice_commit_reveal`, `multiplayer_dice`
- CI: `cargo test` + multi-platform release builds + Godot headless smoke
- GitHub Release `v0.1.0` assets

### Known limits

- Mobile (Android/iOS) not shipped
- HTML5 is preview / best-effort in CI
- Not every Rust core helper is exposed on `FairPlayApi`
- No external security audit
