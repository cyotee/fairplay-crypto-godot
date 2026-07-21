//! Commit–reveal shuffle integrity (anti-cheat, not ZK).

use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

use crate::commitment::{commit_with_nonce, verify_commitment};
use crate::error::{Error, Result};
use crate::hashing::{random_bytes, sha256_hex};
use crate::hexutil::{from_hex, to_hex};
use crate::sra::LayeredCiphertext;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ShuffleCommit {
    pub commitment_hex: String,
    pub input_deck_hash: String,
    pub output_deck_hash: String,
    /// Public only after open — kept in structure for tests/local; wire open is separate
    pub nonce_hex: String,
    pub permutation: Vec<u32>,
}

pub fn generate_permutation(length: usize) -> Vec<u32> {
    let mut perm: Vec<u32> = (0..length as u32).collect();
    perm.shuffle(&mut rand::thread_rng());
    perm
}

pub fn is_valid_permutation(perm: &[u32]) -> bool {
    let n = perm.len() as u32;
    let mut seen = vec![false; perm.len()];
    for &p in perm {
        if p >= n || seen[p as usize] {
            return false;
        }
        seen[p as usize] = true;
    }
    true
}

pub fn apply_permutation(
    deck: &[LayeredCiphertext],
    permutation: &[u32],
) -> Result<Vec<LayeredCiphertext>> {
    if deck.len() != permutation.len() {
        return Err(Error::LengthMismatch);
    }
    if !is_valid_permutation(permutation) {
        return Err(Error::InvalidPermutation);
    }
    let mut result = vec![
        LayeredCiphertext {
            ciphertext: String::new(),
            layers: 0,
        };
        deck.len()
    ];
    for (i, &src) in permutation.iter().enumerate() {
        result[i] = deck[src as usize].clone();
    }
    Ok(result)
}

pub fn serialize_permutation(perm: &[u32]) -> Vec<u8> {
    perm.iter()
        .map(|p| p.to_string())
        .collect::<Vec<_>>()
        .join(",")
        .into_bytes()
}

pub fn hash_deck(deck: &[LayeredCiphertext]) -> String {
    let mut parts = Vec::new();
    for c in deck {
        parts.extend_from_slice(c.ciphertext.to_lowercase().as_bytes());
        parts.push(b'|');
        parts.push(c.layers as u8);
        parts.push(b';');
    }
    sha256_hex(&parts)
}

/// Commit to a permutation applied to `input_deck`. Returns commitment + shuffled deck.
pub fn commit_shuffle(
    input_deck: &[LayeredCiphertext],
    permutation: &[u32],
) -> Result<(ShuffleCommit, Vec<LayeredCiphertext>)> {
    let output = apply_permutation(input_deck, permutation)?;
    let input_hash = hash_deck(input_deck);
    let output_hash = hash_deck(&output);
    let nonce = random_bytes(32);
    let message = serialize_permutation(permutation);
    let c = commit_with_nonce(&message, &nonce);
    Ok((
        ShuffleCommit {
            commitment_hex: c.commitment_hex,
            input_deck_hash: input_hash,
            output_deck_hash: output_hash,
            nonce_hex: to_hex(nonce),
            permutation: permutation.to_vec(),
        },
        output,
    ))
}

/// Public commitment package (no permutation / nonce).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ShuffleCommitmentPublic {
    pub commitment_hex: String,
    pub input_deck_hash: String,
    pub output_deck_hash: String,
}

impl From<&ShuffleCommit> for ShuffleCommitmentPublic {
    fn from(c: &ShuffleCommit) -> Self {
        Self {
            commitment_hex: c.commitment_hex.clone(),
            input_deck_hash: c.input_deck_hash.clone(),
            output_deck_hash: c.output_deck_hash.clone(),
        }
    }
}

/// Open: reveal permutation + nonce; verify against commitment and deck hashes.
pub fn open_and_verify_shuffle(
    public: &ShuffleCommitmentPublic,
    input_deck: &[LayeredCiphertext],
    output_deck: &[LayeredCiphertext],
    permutation: &[u32],
    nonce_hex: &str,
) -> Result<()> {
    if hash_deck(input_deck) != public.input_deck_hash {
        return Err(Error::ShuffleVerifyFailed);
    }
    if hash_deck(output_deck) != public.output_deck_hash {
        return Err(Error::ShuffleVerifyFailed);
    }
    let expected = apply_permutation(input_deck, permutation)?;
    if expected != *output_deck {
        return Err(Error::ShuffleVerifyFailed);
    }
    let nonce = from_hex(nonce_hex)?;
    let message = serialize_permutation(permutation);
    if !verify_commitment(&public.commitment_hex, &message, &nonce) {
        return Err(Error::ShuffleVerifyFailed);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::secp::generate_keypair;
    use crate::sra::encrypt_deck;

    fn sample_deck() -> Vec<LayeredCiphertext> {
        let kp = generate_keypair();
        encrypt_deck(&["A", "B", "C", "D"], &kp.private_key_hex).unwrap()
    }

    #[test]
    fn shuffle_commit_open_ok() {
        let input = sample_deck();
        let perm = generate_permutation(input.len());
        let (commit, output) = commit_shuffle(&input, &perm).unwrap();
        let public = ShuffleCommitmentPublic::from(&commit);
        open_and_verify_shuffle(
            &public,
            &input,
            &output,
            &commit.permutation,
            &commit.nonce_hex,
        )
        .unwrap();
    }

    #[test]
    fn shuffle_tamper_permutation_fails() {
        let input = sample_deck();
        let perm = generate_permutation(input.len());
        let (commit, output) = commit_shuffle(&input, &perm).unwrap();
        let public = ShuffleCommitmentPublic::from(&commit);
        // Wrong perm that still valid as permutation but different
        let mut bad = perm.clone();
        if bad.len() >= 2 {
            bad.swap(0, 1);
        }
        assert!(open_and_verify_shuffle(
            &public,
            &input,
            &output,
            &bad,
            &commit.nonce_hex
        )
        .is_err());
    }

    #[test]
    fn shuffle_wrong_nonce_fails() {
        let input = sample_deck();
        let perm = vec![1, 0, 2, 3];
        let (commit, output) = commit_shuffle(&input, &perm).unwrap();
        let public = ShuffleCommitmentPublic::from(&commit);
        let wrong_nonce = to_hex(random_bytes(32));
        assert!(open_and_verify_shuffle(
            &public,
            &input,
            &output,
            &perm,
            &wrong_nonce
        )
        .is_err());
    }
}
