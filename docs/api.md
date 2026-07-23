# GDScript API

Crypto math lives in Rust (`manamesh_fairplay_core`). GDScript only calls **`FairPlayApi`** (GDExtension class) or the thin **`FairPlay`** facade.

!!! tip "Private keys"
    Methods that take `private_key_hex` keep secrets **local**. Never put private keys in MultiplayerAPI RPCs, shared game state, or public wire JSON. See [netcode](netcode.md).

## Instantiation

```gdscript
# Direct GDExtension class
var api = ClassDB.instantiate("FairPlayApi")

# Optional facade (class_name FairPlay)
var fair := FairPlay.new()
```

---

## Utility

### `version() -> String`

Library version string (currently `"0.1.0"`).

### `sha256_hex(text: String) -> String`

SHA-256 of the UTF-8 bytes of `text`, returned as lowercase hex.

---

## Keypairs

### `generate_keypair() -> Dictionary`

| Key | Type | Notes |
|-----|------|--------|
| `public_key` | `String` | Compressed public key hex — safe to share after admission |
| `private_key` | `String` | **Local only** |

### `validate_public_key(public_key_hex: String) -> bool`

Returns `true` if the hex decodes to a valid compressed secp256k1 public key.

---

## Commit–reveal

Commitment scheme: `SHA-256(message ‖ nonce)` with a random 32-byte nonce.

### `commit_bytes(message: PackedByteArray) -> Dictionary`

### `commit_die_face(face: int) -> Dictionary`

Single die face (stored as one byte). Intended for Liar’s Dice–shaped flows.

### `commit_dice_hand(faces: PackedByteArray) -> Dictionary`

Multiple faces as raw bytes (e.g. five dice).

**Return dictionary (all methods above):**

| Key | Type | On the wire? |
|-----|------|----------------|
| `commitment_hex` | `String` | **Yes** (public binding) |
| `nonce_hex` | `String` | **No** until intentional reveal |
| `message_hex` | `String` | **No** until intentional reveal |

### `verify_commitment(commitment_hex, message: PackedByteArray, nonce_hex) -> bool`

### `verify_die_face(commitment_hex, face: int, nonce_hex) -> bool`

Return `true` only if the opening matches the prior commitment.

---

## Mental poker (SRA)

Layered commutative encryption on **secp256k1**: payload → curve point, then scalar multiply per party key. Peel multiplies by the modular inverse.

### `encrypt_payload(payload_id: String, private_key_hex: String) -> Dictionary`

Maps `payload_id` to a point and encrypts under the private key.

### `encrypt_layer(ciphertext: String, layers: int, private_key_hex: String) -> Dictionary`

Adds one encryption layer.

### `peel_layer(ciphertext: String, layers: int, private_key_hex: String) -> Dictionary`

Removes one layer.

**Return dictionary:**

| Key | Type |
|-----|------|
| `ciphertext` | `String` (hex point) |
| `layers` | `int` |

Empty dictionary on error (invalid key / bad ciphertext). Prefer checking for empty results in game code.

!!! note "Core vs GDScript surface"
    The Rust core also exposes keychain admission, shuffle commit–reveal, Merkle **proofs**, and wire DTOs used in pure-Rust tests. Not every core helper is wrapped in `FairPlayApi` yet. File issues if you need a specific binding for a game.

---

## Merkle

### `merkle_root_utf8(leaves: PackedStringArray) -> String`

SHA-256 Merkle root (hex) over leaves derived from UTF-8 strings.

---

## Facade (`FairPlay`)

`addons/manamesh_fairplay/facade/fair_play.gd` forwards to `FairPlayApi` with the same method names. No crypto math in GDScript.

---

## Samples

| Sample | Demonstrates |
|--------|----------------|
| `samples/dice_commit_reveal/` | Offline commit / open / wrong-open failure |
| `samples/multiplayer_dice/` | Public message shapes without private keys |
