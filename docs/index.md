# ManaMesh FairPlay for Godot

**MIT** fair-play cryptography for **Godot 4**: commit–reveal (dice / hidden boards), public-key keychain, SRA mental poker, shuffle integrity, and Merkle selective-open helpers (Battleship-style grids and similar).

**Architecture:** Rust cryptography core + **GDExtension** · **GDScript-first** · no .NET required.

!!! warning "Early release (v0.1)"
    This is a **technical preview**. Desktop binaries and core tests are solid.
    HTML5 is available as a side-module build; mobile (Android/iOS) is not shipped yet.
    There is **no external security audit** — read the [threat model](threat-model.md) before relying on claims.

## What this is

| You get | You do not get |
|---------|----------------|
| Commit–reveal for hidden values (Liar’s Dice–shaped) | A full multiplayer game |
| SRA mental-poker encrypt / peel on secp256k1 | Matchmaking, lobby, or required netcode |
| Shuffle commit–reveal integrity helpers | Zero-knowledge shuffle proofs (post-preview) |
| Merkle roots / proofs for board-style commit–open (e.g. Battleship) | A full rules engine or game title |
| Prebuilt GDExtension binaries in [GitHub Releases](https://github.com/cyotee/fairplay-crypto-godot/releases) | Mobile store binaries (not shipped yet) |

## Quick taste (GDScript)

```gdscript
var api = ClassDB.instantiate("FairPlayApi")
var faces := PackedByteArray([1, 3, 3, 5, 6])
var c = api.commit_dice_hand(faces)
# Send only c.commitment_hex to peers. Keep faces + c.nonce_hex local until reveal.
assert(api.verify_commitment(c.commitment_hex, faces, c.nonce_hex))
```

## Start here

1. [Getting started](getting-started.md) — install the addon and run smoke tests  
2. [GDScript API](api.md) — `FairPlayApi` / `FairPlay` facade reference  
3. [Commit–reveal](commit-reveal.md) — primary fairness pattern for dice / hidden values  
4. [Bring-your-own netcode](netcode.md) — what may go on the wire  
5. [Threat model](threat-model.md) — honest limits of the guarantees  

## Scope

ManaMesh FairPlay is a **fairness toolkit** for multiplayer games: hidden dice, hidden board placement, cooperative deck deal, and related commit–reveal flows.

It is a library, not a full game, lobby/matchmaking stack, or boardgame.io port.
