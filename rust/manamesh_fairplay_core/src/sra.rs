//! SRA commutative encryption on secp256k1 (point × scalar layers).

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::hexutil::to_hex;
use crate::secp::{
    hash_to_point, inverse_mod_n, multiply_point_by_scalar_bytes, multiply_point_by_secret,
    public_key_from_hex, public_key_to_hex, require_sk_matches_pk, secret_key_from_hex,
};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct LayeredCiphertext {
    /// Compressed point hex
    pub ciphertext: String,
    pub layers: u32,
}

/// Map payload id ↔ compressed point hex for peel recovery.
#[derive(Clone, Debug, Default)]
pub struct PayloadLookup {
    id_to_point: HashMap<String, String>,
    point_to_id: HashMap<String, String>,
}

impl PayloadLookup {
    pub fn build(payload_ids: impl IntoIterator<Item = impl AsRef<str>>) -> Result<Self> {
        let mut id_to_point = HashMap::new();
        let mut point_to_id = HashMap::new();
        for id in payload_ids {
            let id = id.as_ref().to_string();
            let pt = hash_to_point(&id)?;
            let hex = public_key_to_hex(&pt);
            point_to_id.insert(hex.to_lowercase(), id.clone());
            id_to_point.insert(id, hex);
        }
        Ok(Self {
            id_to_point,
            point_to_id,
        })
    }

    pub fn point_hex(&self, id: &str) -> Option<&str> {
        self.id_to_point.get(id).map(|s| s.as_str())
    }

    pub fn id_for_point(&self, point_hex: &str) -> Option<&str> {
        self.point_to_id
            .get(&point_hex.to_lowercase())
            .map(|s| s.as_str())
    }
}

/// Encrypt a payload id under one private key (layer 1).
pub fn encrypt_payload(payload_id: &str, private_key_hex: &str) -> Result<LayeredCiphertext> {
    let point = hash_to_point(payload_id)?;
    let sk = secret_key_from_hex(private_key_hex)?;
    let enc = multiply_point_by_secret(&point, &sk)?;
    Ok(LayeredCiphertext {
        ciphertext: public_key_to_hex(&enc),
        layers: 1,
    })
}

/// Optionally bind sk to published pk before encrypt.
pub fn encrypt_payload_bound(
    payload_id: &str,
    private_key_hex: &str,
    published_pk_hex: &str,
) -> Result<LayeredCiphertext> {
    require_sk_matches_pk(private_key_hex, published_pk_hex)?;
    encrypt_payload(payload_id, private_key_hex)
}

/// Add another encryption layer.
pub fn encrypt_layer(card: &LayeredCiphertext, private_key_hex: &str) -> Result<LayeredCiphertext> {
    let point = public_key_from_hex(&card.ciphertext)?;
    let sk = secret_key_from_hex(private_key_hex)?;
    let enc = multiply_point_by_secret(&point, &sk)?;
    Ok(LayeredCiphertext {
        ciphertext: public_key_to_hex(&enc),
        layers: card.layers + 1,
    })
}

/// Peel one layer (multiply by sk^{-1}).
pub fn peel_layer(card: &LayeredCiphertext, private_key_hex: &str) -> Result<LayeredCiphertext> {
    if card.layers == 0 {
        return Err(Error::CannotPeelPlaintext);
    }
    let point = public_key_from_hex(&card.ciphertext)?;
    let sk = secret_key_from_hex(private_key_hex)?;
    let inv = inverse_mod_n(&sk.secret_bytes())?;
    let peeled = multiply_point_by_scalar_bytes(&point, &inv)?;
    Ok(LayeredCiphertext {
        ciphertext: public_key_to_hex(&peeled),
        layers: card.layers - 1,
    })
}

/// Peel last layer and recover payload id via lookup.
pub fn peel_to_payload(
    card: &LayeredCiphertext,
    private_key_hex: &str,
    lookup: &PayloadLookup,
) -> Result<String> {
    if card.layers != 1 {
        return Err(Error::LayerMismatch {
            expected: 1,
            got: card.layers,
        });
    }
    let peeled = peel_layer(card, private_key_hex)?;
    lookup
        .id_for_point(&peeled.ciphertext)
        .map(|s| s.to_string())
        .ok_or(Error::PayloadNotFound)
}

pub fn encrypt_deck(
    payload_ids: &[impl AsRef<str>],
    private_key_hex: &str,
) -> Result<Vec<LayeredCiphertext>> {
    payload_ids
        .iter()
        .map(|id| encrypt_payload(id.as_ref(), private_key_hex))
        .collect()
}

/// Debug helper: point hex of a payload (not secret).
pub fn payload_point_hex(payload_id: &str) -> Result<String> {
    Ok(to_hex(hash_to_point(payload_id)?.serialize()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::secp::generate_keypair;

    #[test]
    fn two_party_encrypt_peel() {
        let a = generate_keypair();
        let b = generate_keypair();
        let lookup = PayloadLookup::build(["AS", "KH", "2C"]).unwrap();

        let mut card = encrypt_payload("AS", &a.private_key_hex).unwrap();
        card = encrypt_layer(&card, &b.private_key_hex).unwrap();
        assert_eq!(card.layers, 2);

        // Wrong peel order still works commutatively if both peel
        let after_b = peel_layer(&card, &b.private_key_hex).unwrap();
        let after_a = peel_layer(&after_b, &a.private_key_hex).unwrap();
        assert_eq!(after_a.layers, 0);
        assert_eq!(
            lookup.id_for_point(&after_a.ciphertext).unwrap(),
            "AS"
        );

        // Single last peel recovery after one peel
        let one = peel_layer(&card, &a.private_key_hex).unwrap();
        let id = peel_to_payload(&one, &b.private_key_hex, &lookup).unwrap();
        assert_eq!(id, "AS");
    }

    #[test]
    fn wrong_key_fails_recovery() {
        let a = generate_keypair();
        let b = generate_keypair();
        let lookup = PayloadLookup::build(["die:1", "die:2"]).unwrap();
        let card = encrypt_payload("die:1", &a.private_key_hex).unwrap();
        assert!(peel_to_payload(&card, &b.private_key_hex, &lookup).is_err());
    }

    #[test]
    fn bind_rejects_mismatch() {
        let a = generate_keypair();
        let b = generate_keypair();
        assert!(encrypt_payload_bound("x", &a.private_key_hex, &b.public_key_hex).is_err());
        encrypt_payload_bound("x", &a.private_key_hex, &a.public_key_hex).unwrap();
    }

    #[test]
    fn cannot_peel_plaintext() {
        let a = generate_keypair();
        let card = encrypt_payload("Z", &a.private_key_hex).unwrap();
        let plain = peel_layer(&card, &a.private_key_hex).unwrap();
        assert_eq!(plain.layers, 0);
        assert!(peel_layer(&plain, &a.private_key_hex).is_err());
    }
}
