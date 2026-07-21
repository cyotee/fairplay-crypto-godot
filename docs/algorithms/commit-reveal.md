# Commit–reveal

`commitment = SHA256(message || nonce)`

- `commit(message)` samples a 32-byte nonce and returns hex commitment + nonce + message hex.
- `verify(commitment, message, nonce)` recomputes the digest.

Dice helpers treat faces as raw bytes (`[face]` or multi-face hand).
