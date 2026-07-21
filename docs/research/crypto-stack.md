# Crypto stack decision

| Choice | Decision |
|--------|----------|
| Curve | secp256k1 |
| Crate | `secp256k1` 0.29 (libsecp256k1 via rust-bitcoin bindings) |
| Hash | SHA-256 (`sha2`) |
| Commitments | SHA-256(message \|\| nonce) |
| SRA | Point × scalar layers; peel with modular inverse mod n |
| RNG | `rand` crate OS RNG |

Not used: pure-Rust `k256` as primary (secp256k1 crate preferred per plan).
