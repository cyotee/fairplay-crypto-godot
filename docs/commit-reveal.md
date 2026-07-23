# Commit–reveal

Commit–reveal is the **first-class** fairness primitive for hidden values such as dice hands.

## Scheme

```
commitment = SHA-256(message ‖ nonce)
```

- `commit(message)` samples a 32-byte nonce and returns hex commitment, nonce, and message.
- `verify(commitment, message, nonce)` recomputes the digest and compares.

Dice helpers treat faces as raw bytes:

- Single face: one byte (`[face]`)
- Hand: `PackedByteArray` of faces (e.g. five dice for Liar’s Dice–shaped play)

## Protocol shape

1. **Commit** — each player samples a nonce, commits to hidden values (`faces ‖ nonce`), and publishes only `commitment_hex`.
2. **Play** — game rules use commitments as binding claims (bids, challenges, etc.).
3. **Open** — players reveal `message` + `nonce`; peers call `verify_*` and reject bad openings.

## GDScript example

```gdscript
var api = ClassDB.instantiate("FairPlayApi")
var faces := PackedByteArray([1, 2, 3, 4, 5])
var c: Dictionary = api.commit_dice_hand(faces)

# Wire: public only
var public_msg := {
    "type": "commitment",
    "label": "dice_hand",
    "commitment_hex": c["commitment_hex"],
}

# Later open
var ok: bool = api.verify_commitment(
    c["commitment_hex"],
    faces,
    c["nonce_hex"]
)
```

Wrong faces or wrong nonce → verification fails (bait-and-switch is detectable).

## What it guarantees

| Property | Yes? |
|----------|------|
| Binding after publish | **Yes** — different open fails |
| Detect wrong open | **Yes** |
| Force fair random dice by itself | **No** — combine with shared RNG / mental poker if needed |
| Hide values forever | **No** — open reveals the message |

Full honesty bounds: [Threat model](threat-model.md).
