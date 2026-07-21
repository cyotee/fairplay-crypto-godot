# Bring-your-own netcode

The library does **not** depend on MultiplayerAPI, ENet, WebRTC, or any lobby.

## Public messages (safe on wire)

- Public keys (after keychain admission)
- Commitment digests
- Commitment openings (message + nonce) when the protocol chooses to reveal
- Layered ciphertexts and peel results (points only)
- Shuffle commitments and later opens
- Merkle roots / proofs

## Never on wire

- Private keys / secret keys
- Uncommitted dice faces before open (unless you intentionally skip fairness)

## Sample pattern

`samples/multiplayer_dice` shows RPC-shaped dictionaries without private key fields. Replace the local `public_message` signal with your `rpc` / custom transport.
