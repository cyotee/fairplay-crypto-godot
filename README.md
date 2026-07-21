# ManaMesh FairPlay for Godot

**MIT-licensed** fair-play cryptography for **Godot 4**: mental poker (SRA), public-key keychain admission, Merkle commitments, shuffle integrity (commit–reveal), and multi-party protocol helpers.

**Architecture:** **Rust crypto core + GDExtension** — **GDScript-first** API. C# may call the same extension (second-class). No .NET requirement for primary consumers.

**Positioning:** cryptography for securing secrets and verifying integrity among untrusted peers — **not** a gambling product, **not** a cryptocurrency product.

## Status

**Planning / Phase 0 bootstrap.** Product decisions locked for a multi-store downstream game (Liar’s Dice intent). Remaining work: toolchain, hello GDExtension (desktop + HTML5), CI binaries.

| Doc | Purpose |
|-----|---------|
| **[PRD.md](./PRD.md)** | Product requirements |
| **[IMPLEMENTATION_PLAN.md](./IMPLEMENTATION_PLAN.md)** | Phased implementation + testing |
| **[docs/research/platform-constraints.md](./docs/research/platform-constraints.md)** | Why Rust/GDExtension (HTML5) |

## Downstream intent (not this repo)

A future **Liar’s Dice** game in a **separate repo**: single + multiplayer; Steam, Itch, GOG, Android Play, Apple App Store; free **web multiplayer-only**. Built **after** this library is usable.

## Platforms

| Milestone | Targets |
|-----------|---------|
| **A** | Editor + desktop + **HTML5** (hard gate) |
| **A2** | Android + iOS (game-ready for mobile stores) |

Godot **4.4+** (develop on latest 4.7.x). GDScript-first; no .NET required.

## License

MIT — see [LICENSE](./LICENSE).

## Monorepo note

Submodule of [manamesh-games](https://github.com/cyotee/manamesh-games) at `packages/fairplay-crypto-godot`.
