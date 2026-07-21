# ManaMesh FairPlay for Godot

**MIT-licensed** fair-play cryptography for **Godot 4**: mental poker (SRA), public-key keychain admission, Merkle commitments, shuffle integrity (commit–reveal), and multi-party protocol helpers.

**Architecture:** **Rust crypto core + GDExtension** — **GDScript-first** API. C# may call the same extension (second-class). No .NET requirement for primary consumers.

**Positioning:** cryptography for securing secrets and verifying integrity among untrusted peers — **not** a gambling product, **not** a cryptocurrency product.

## Status

**Planning / Phase 0.** Product and implementation decisions are largely locked; remaining work is toolchain bootstrap and HTML5 GDExtension proof.

| Doc | Purpose |
|-----|---------|
| **[PRD.md](./PRD.md)** | Product requirements |
| **[IMPLEMENTATION_PLAN.md](./IMPLEMENTATION_PLAN.md)** | Phased implementation + testing |
| **[docs/research/platform-constraints.md](./docs/research/platform-constraints.md)** | Why not C# primary (HTML5) |

| Reference | Link |
|-----------|------|
| Sibling C# / Unity | [cyotee/fairplay-crypto](https://github.com/cyotee/fairplay-crypto) |
| TS / browser | `@manamesh/boardgameio-crypto` in [manamesh-games](https://github.com/cyotee/manamesh-games) |

## Platforms (Milestone A)

| Target | Status |
|--------|--------|
| Editor + desktop | Required |
| HTML5 (GDExtension WASM) | Required |
| Mobile | Post-publish |

Godot **4.4+** (develop on latest 4.7.x).

## License

MIT — see [LICENSE](./LICENSE).

## Monorepo note

Submodule of [manamesh-games](https://github.com/cyotee/manamesh-games) at `packages/fairplay-crypto-godot`.
