//! Hash-based commit–reveal for arbitrary bytes (dice faces, seeds, etc.).

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::hashing::{random_bytes, sha256_concat};
use crate::hexutil::{from_hex, to_hex};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Commitment {
    /// Hex SHA-256(message || nonce)
    pub commitment_hex: String,
    /// 32-byte nonce (kept secret until open)
    pub nonce_hex: String,
    /// Hex of committed message
    pub message_hex: String,
}

/// Commit to `message` with optional 32-byte nonce (random if omitted).
pub fn commit(message: &[u8]) -> Commitment {
    let nonce = random_bytes(32);
    commit_with_nonce(message, &nonce)
}

pub fn commit_with_nonce(message: &[u8], nonce: &[u8]) -> Commitment {
    let digest = sha256_concat(&[message, nonce]);
    Commitment {
        commitment_hex: to_hex(digest),
        nonce_hex: to_hex(nonce),
        message_hex: to_hex(message),
    }
}

/// Verify a commitment opening. Returns true iff digest matches.
pub fn verify_commitment(commitment_hex: &str, message: &[u8], nonce: &[u8]) -> bool {
    let digest = sha256_concat(&[message, nonce]);
    to_hex(digest).eq_ignore_ascii_case(commitment_hex)
}

pub fn verify_commitment_hex(
    commitment_hex: &str,
    message_hex: &str,
    nonce_hex: &str,
) -> Result<bool> {
    let message = from_hex(message_hex)?;
    let nonce = from_hex(nonce_hex)?;
    Ok(verify_commitment(commitment_hex, &message, &nonce))
}

/// Dice-shaped helper: commit to a die face 1..=6 (or any u8 face).
pub fn commit_die_face(face: u8) -> Commitment {
    commit(&[face])
}

pub fn verify_die_face(commitment_hex: &str, face: u8, nonce_hex: &str) -> Result<bool> {
    let nonce = from_hex(nonce_hex)?;
    if !verify_commitment(commitment_hex, &[face], &nonce) {
        return Ok(false);
    }
    Ok(true)
}

pub fn assert_verify(commitment_hex: &str, message: &[u8], nonce: &[u8]) -> Result<()> {
    if verify_commitment(commitment_hex, message, nonce) {
        Ok(())
    } else {
        Err(Error::CommitmentFailed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dice_commit_reveal_ok() {
        let c = commit_die_face(4);
        assert!(verify_die_face(&c.commitment_hex, 4, &c.nonce_hex).unwrap());
        assert!(!verify_die_face(&c.commitment_hex, 5, &c.nonce_hex).unwrap());
        // wrong nonce
        let other = random_bytes(32);
        assert!(!verify_commitment(
            &c.commitment_hex,
            &[4],
            &other
        ));
    }

    #[test]
    fn multi_die_hand_commit() {
        // Liar's Dice style: commit to five faces
        let faces = [1u8, 3, 3, 5, 6];
        let c = commit(&faces);
        assert!(verify_commitment(
            &c.commitment_hex,
            &faces,
            &from_hex(&c.nonce_hex).unwrap()
        ));
        let mut wrong = faces;
        wrong[0] = 2;
        assert!(!verify_commitment(
            &c.commitment_hex,
            &wrong,
            &from_hex(&c.nonce_hex).unwrap()
        ));
    }
}
