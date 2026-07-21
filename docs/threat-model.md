# Threat model — ManaMesh FairPlay for Godot

## Commit–reveal honesty

Milestone A ships **commit–reveal** integrity for:

1. **Hidden values** (e.g. dice faces in a Liar’s Dice–shaped flow)
2. **Shuffle permutations** applied to encrypted decks

### What commit–reveal provides

- Binding: after a party publishes `commitment_hex`, they cannot later open a *different* message/nonce that still verifies.
- Detection of bait-and-switch: wrong faces, wrong permutation, or wrong nonce fails verification.

### What it does **not** provide

- **Zero-knowledge** proofs of shuffle or dice (peers learn the value when it is opened).
- Prevention of a player choosing any valid dice values *before* committing (commitment binds the choice; it does not force fair dice rolls by itself — combine with shared RNG / mental poker if needed).
- Network-level DoS, Sybil, or transport security (BYO netcode / TLS / WebRTC).

## Private key rule

Private keys and long-term secrets **never** appear in:

- Shared multiplayer game state
- `FairplayEnvelope` / wire JSON
- MultiplayerAPI RPCs in samples

Only public keys, commitments, ciphertexts, peels, and intentional *openings* (message + nonce for a prior commitment) are public materials.

## Mental poker (SRA)

Layered commutative encryption on secp256k1. Integrity of peels depends on correct key use and cooperative protocols. Wrong peels fail payload recovery. Not a substitute for authenticated channels.
