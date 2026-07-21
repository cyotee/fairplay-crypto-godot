# ManaMesh FairPlay for Godot

**MIT-licensed** fair-play cryptography for **Godot 4** multiplayer games: mental poker (SRA), public-key keychain admission, Merkle commitments, shuffle integrity, and related multi-party primitives.

**Architecture:** C# crypto core + GDScript facade — **GDScript and C# are first-class** from day one.

**Positioning:** cryptography for securing secrets and verifying integrity among untrusted peers — **not** a gambling product, **not** a cryptocurrency product.

## Status

**Planning.** Product requirements live in [PRD.md](./PRD.md). Implementation has not started.

| Doc | Purpose |
|-----|---------|
| **[PRD.md](./PRD.md)** | Product requirements (locked decisions included) |
| Sibling C# / Unity line | [cyotee/fairplay-crypto](https://github.com/cyotee/fairplay-crypto) |
| TS / browser reference | `@manamesh/boardgameio-crypto` in [manamesh-games](https://github.com/cyotee/manamesh-games) |

## Intended consumers

- Godot 4 projects using **GDScript** and/or **C#** (C#-enabled Godot build required)
- Open-source game authors who want an MIT library (Asset Library / Git submodule / clone)
- Developers who need **game-agnostic** crypto APIs with samples (mental poker, poker-shaped flow, Merkle battleship, MultiplayerAPI path)

## License

MIT — see [LICENSE](./LICENSE).

## Monorepo note

This repository is also a **git submodule** of [manamesh-games](https://github.com/cyotee/manamesh-games) at `packages/fairplay-crypto-godot` for side-by-side development with the TypeScript and C# FairPlay lines.
