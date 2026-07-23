# Bring-your-own netcode

The library does **not** depend on MultiplayerAPI, ENet, WebRTC, or any lobby.
Samples may *illustrate* MultiplayerAPI-shaped dictionaries; you plug in your transport.

## Safe on the wire

| Material | Example |
|----------|---------|
| Public keys | After your own admission / handshake |
| Commitment digests | `commitment_hex` |
| Commitment openings | message + nonce when the protocol chooses to reveal |
| Layered ciphertexts | Point hex + layer count |
| Peel results | Updated ciphertext / layers |
| Shuffle commitments / opens | Public integrity material |
| Merkle roots / proofs | Public |

## Never on the wire

| Material | Why |
|----------|-----|
| Private / secret keys | Breaks the security model entirely |
| Uncommitted dice faces before open | Defeats commit–reveal (unless you intentionally skip fairness) |
| Long-term secrets in shared game state | Same as private keys |

## Sample pattern

`samples/multiplayer_dice/` builds dictionaries like:

```gdscript
var wire_commit := {
    "type": "commitment",
    "label": "dice_hand",
    "commitment_hex": ca["commitment_hex"],
}
# NO private_key, NO nonce until open phase
```

Replace the local `public_message` signal with your `rpc` / custom transport.

## Checklist for integrators

1. Serialize **only** public fields into RPCs / packets.  
2. Keep nonces and private keys in local dictionaries keyed by seat / peer.  
3. On open, re-verify every commitment with `verify_commitment` / `verify_die_face`.  
4. Reject peers that send fields named like `private_key` / `sk` (sample asserts this).  
5. Use authenticated channels if cheating by identity swap matters for your game.
