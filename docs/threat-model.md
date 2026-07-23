# Threat model

Honest bounds for ManaMesh FairPlay **v0.1**. Claims are backed by unit tests and API design — **not** by an external audit.

## Commit–reveal honesty

Milestone focus ships **commit–reveal** integrity for:

1. **Hidden values** (e.g. dice faces in a Liar’s Dice–shaped flow)
2. **Shuffle permutations** applied to encrypted decks (Rust core)

### What commit–reveal provides

- **Binding:** after a party publishes `commitment_hex`, they cannot later open a *different* message/nonce that still verifies.
- **Detection of bait-and-switch:** wrong faces, wrong permutation, or wrong nonce fails verification.

### What it does **not** provide

- **Zero-knowledge** proofs of shuffle or dice (peers learn the value when it is opened).
- Prevention of a player choosing any valid dice values *before* committing (commitment binds the choice; it does not force fair dice rolls by itself — combine with shared RNG / mental poker if needed).
- Network-level DoS, Sybil, or transport security (BYO netcode / TLS / WebRTC).

## Private key rule

Private keys and long-term secrets **never** appear in:

- Shared multiplayer game state
- Public wire JSON / envelopes
- MultiplayerAPI RPCs in samples

Only public keys, commitments, ciphertexts, peels, and intentional *openings* (message + nonce for a prior commitment) are public materials.

## Mental poker (SRA)

Layered commutative encryption on secp256k1. Integrity of peels depends on correct key use and cooperative protocols. Wrong peels fail payload recovery. Not a substitute for authenticated channels.

## Operational assumptions

| Assumption | If violated |
|------------|-------------|
| Peers run unmodified clients | A modified client can still only open what it committed to (for commit–reveal); other cheating is game-rule dependent |
| Transport authenticates peers | Identity spoofing is out of scope for the crypto core |
| Hosts do not need to hold player private keys | Do not centralize sk material for “convenience” |

## Reporting issues

Security-sensitive bugs: open a private security advisory on the GitHub repository when available, or contact the maintainers via the repo.
