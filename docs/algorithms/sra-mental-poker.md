# SRA mental poker (secp256k1)

Commutative layers: `Enc_sk(P) = sk · P` on the curve. Peel multiplies by `sk⁻¹ mod n`.

Payloads map to points via try-and-increment hash-to-curve (SHA-256 of id||counter).
