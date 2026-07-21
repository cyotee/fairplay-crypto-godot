//! secp256k1 helpers via the bitcoin-core `secp256k1` Rust crate.

use num_bigint::BigUint;
use num_traits::Zero;
use rand::RngCore;
use secp256k1::{PublicKey, Scalar, Secp256k1, SecretKey};

use crate::error::{Error, Result};
use crate::hashing::sha256;
use crate::hexutil::{from_hex, from_hex_32, to_hex};

/// secp256k1 curve order n
const ORDER_N_HEX: &str = "fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141";

#[derive(Clone, Debug)]
pub struct KeyPair {
    pub public_key_hex: String,
    pub private_key_hex: String,
}

pub fn generate_keypair() -> KeyPair {
    let secp = Secp256k1::new();
    loop {
        let mut sk_bytes = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut sk_bytes);
        if let Ok(sk) = SecretKey::from_slice(&sk_bytes) {
            let pk = PublicKey::from_secret_key(&secp, &sk);
            return KeyPair {
                public_key_hex: to_hex(pk.serialize()),
                private_key_hex: to_hex(sk.secret_bytes()),
            };
        }
    }
}

pub fn secret_key_from_hex(hex: &str) -> Result<SecretKey> {
    let bytes = from_hex_32(hex)?;
    SecretKey::from_slice(&bytes).map_err(|_| Error::InvalidPrivateKey)
}

pub fn public_key_from_hex(hex: &str) -> Result<PublicKey> {
    let bytes = from_hex(hex)?;
    PublicKey::from_slice(&bytes).map_err(|_| Error::InvalidPublicKey)
}

pub fn validate_public_key(hex: &str) -> bool {
    public_key_from_hex(hex).is_ok()
}

pub fn public_key_to_hex(pk: &PublicKey) -> String {
    to_hex(pk.serialize())
}

pub fn private_matches_public(private_key_hex: &str, public_key_hex: &str) -> bool {
    let secp = Secp256k1::new();
    let Ok(sk) = secret_key_from_hex(private_key_hex) else {
        return false;
    };
    let Ok(expected) = public_key_from_hex(public_key_hex) else {
        return false;
    };
    let derived = PublicKey::from_secret_key(&secp, &sk);
    derived.serialize() == expected.serialize()
}

/// Require local sk matches published pk before encrypt (client-side bind).
pub fn require_sk_matches_pk(private_key_hex: &str, public_key_hex: &str) -> Result<()> {
    if private_matches_public(private_key_hex, public_key_hex) {
        Ok(())
    } else {
        Err(Error::KeyMismatch)
    }
}

pub fn multiply_point_by_secret(point: &PublicKey, sk: &SecretKey) -> Result<PublicKey> {
    let secp = Secp256k1::new();
    let scalar = Scalar::from_be_bytes(sk.secret_bytes()).map_err(|_| Error::InvalidPrivateKey)?;
    point
        .mul_tweak(&secp, &scalar)
        .map_err(|e| Error::Crypto(e.to_string()))
}

pub fn multiply_point_by_scalar_bytes(point: &PublicKey, scalar32: &[u8; 32]) -> Result<PublicKey> {
    let secp = Secp256k1::new();
    let scalar = Scalar::from_be_bytes(*scalar32).map_err(|_| Error::InvalidPrivateKey)?;
    point
        .mul_tweak(&secp, &scalar)
        .map_err(|e| Error::Crypto(e.to_string()))
}

/// Modular inverse of a 32-byte scalar mod n (secp256k1 order).
pub fn inverse_mod_n(scalar32: &[u8; 32]) -> Result<[u8; 32]> {
    let n = BigUint::parse_bytes(ORDER_N_HEX.as_bytes(), 16).unwrap();
    let x = BigUint::from_bytes_be(scalar32);
    if x.is_zero() || x >= n {
        return Err(Error::InvalidPrivateKey);
    }
    let inv = mod_inverse(&x, &n).ok_or(Error::InvalidPrivateKey)?;
    let bytes = inv.to_bytes_be();
    let mut out = [0u8; 32];
    if bytes.len() > 32 {
        return Err(Error::InvalidPrivateKey);
    }
    out[32 - bytes.len()..].copy_from_slice(&bytes);
    Ok(out)
}

fn mod_inverse(a: &BigUint, m: &BigUint) -> Option<BigUint> {
    // n is prime ⇒ a^(n-2) ≡ a^{-1} (mod n)
    if a.is_zero() {
        return None;
    }
    Some(a.modpow(&(m - BigUint::from(2u32)), m))
}

/// Deterministic hash-to-curve (try-and-increment on x-coordinate).
pub fn hash_to_point(payload_id: &str) -> Result<PublicKey> {
    let data = payload_id.as_bytes();
    for counter in 0u8..=255 {
        let mut input = data.to_vec();
        input.push(counter);
        let hash = sha256(&input);
        // Try even/odd compressed prefixes 0x02 / 0x03
        for prefix in [0x02u8, 0x03u8] {
            let mut candidate = [0u8; 33];
            candidate[0] = prefix;
            candidate[1..].copy_from_slice(&hash);
            if let Ok(pk) = PublicKey::from_slice(&candidate) {
                return Ok(pk);
            }
        }
    }
    Err(Error::HashToPoint(payload_id.to_string()))
}

pub fn hash_to_point_hex(payload_id: &str) -> Result<String> {
    Ok(public_key_to_hex(&hash_to_point(payload_id)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keypair_roundtrip_match() {
        let kp = generate_keypair();
        assert!(validate_public_key(&kp.public_key_hex));
        assert!(private_matches_public(
            &kp.private_key_hex,
            &kp.public_key_hex
        ));
        assert!(!private_matches_public(
            &kp.private_key_hex,
            &generate_keypair().public_key_hex
        ));
    }

    #[test]
    fn hash_to_point_stable() {
        let a = hash_to_point_hex("die:6").unwrap();
        let b = hash_to_point_hex("die:6").unwrap();
        assert_eq!(a, b);
        assert_ne!(a, hash_to_point_hex("die:1").unwrap());
    }

    #[test]
    fn inverse_roundtrip_scalar() {
        let kp = generate_keypair();
        let sk = from_hex_32(&kp.private_key_hex).unwrap();
        let inv = inverse_mod_n(&sk).unwrap();
        // sk * inv ≡ 1 mod n  — check via point: G*sk * inv = G
        let secp = Secp256k1::new();
        let sk_key = SecretKey::from_slice(&sk).unwrap();
        let g_sk = PublicKey::from_secret_key(&secp, &sk_key);
        let back = multiply_point_by_scalar_bytes(&g_sk, &inv).unwrap();
        // Should equal generator * 1 = G
        let one = {
            let mut o = [0u8; 32];
            o[31] = 1;
            o
        };
        let g = PublicKey::from_secret_key(&secp, &SecretKey::from_slice(&one).unwrap());
        assert_eq!(back.serialize(), g.serialize());
    }
}
