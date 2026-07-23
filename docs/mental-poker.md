# Mental poker (SRA)

SRA-style mental poker here means **commutative layered encryption** on the **secp256k1** curve so multiple parties can shuffle / deal without a trusted dealer.

## Math (sketch)

- Map a payload id to a curve point via try-and-increment hash-to-curve (SHA-256 of `id ‖ counter`).
- Encrypt: `Enc_sk(P) = sk · P` (scalar multiply).
- Peel: multiply by `sk⁻¹ mod n`.

Because multiplication is commutative, parties can encrypt and peel in different orders and still recover the payload when every layer is removed.

## GDScript surface

```gdscript
var api = ClassDB.instantiate("FairPlayApi")
var kp = api.generate_keypair()
# Keep kp.private_key local only

var card = api.encrypt_payload("card:AS", kp["private_key"])
# card = { ciphertext, layers }

var next = api.encrypt_layer(card["ciphertext"], card["layers"], other_private_key)
var peeled = api.peel_layer(next["ciphertext"], next["layers"], other_private_key)
```

## Integrity extras (Rust core)

The pure-Rust core includes:

- **Keychain** admission policies (validate public keys, reject duplicates)
- **Shuffle commit–reveal** — commit to a permutation, apply to encrypted deck, later open and verify
- **Multi-seat simulator** used in unit tests for multi-party scenarios

These are covered by `cargo test -p manamesh_fairplay_core`. Not all are exposed on `FairPlayApi` yet.

## Design rules

1. Private keys never leave the local process (no wire, no shared board state).
2. Peers exchange public keys, layered ciphertexts (points), peel results, and intentional openings only.
3. Wrong peels / wrong keys fail payload recovery — authenticated transport is still your responsibility ([netcode](netcode.md)).

## Limits

- Not a substitute for TLS / WebRTC authenticity.
- Commit–reveal shuffle integrity is **not** a zero-knowledge shuffle proof.
- No external audit; treat v0.1 as a toolkit under development.
