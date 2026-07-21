//! Binary Merkle tree (SHA-256) for hidden placement / board commitments.

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::hashing::sha256;
use crate::hexutil::{from_hex, to_hex};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MerkleSide {
    Left,
    Right,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MerkleProofStep {
    pub side: MerkleSide,
    pub hash_hex: String,
}

fn hash_pair(left: &[u8], right: &[u8]) -> [u8; 32] {
    let mut buf = Vec::with_capacity(left.len() + right.len());
    buf.extend_from_slice(left);
    buf.extend_from_slice(right);
    sha256(&buf)
}

pub fn merkle_root(leaves: &[Vec<u8>]) -> Vec<u8> {
    if leaves.is_empty() {
        return sha256([]).to_vec();
    }
    let mut level: Vec<Vec<u8>> = leaves.to_vec();
    while level.len() > 1 {
        let mut next = Vec::new();
        let mut i = 0;
        while i < level.len() {
            let left = &level[i];
            let right = if i + 1 < level.len() {
                &level[i + 1]
            } else {
                left
            };
            next.push(hash_pair(left, right).to_vec());
            i += 2;
        }
        level = next;
    }
    level[0].clone()
}

pub fn merkle_root_hex(leaves: &[Vec<u8>]) -> String {
    to_hex(merkle_root(leaves))
}

pub fn leaf_from_utf8(s: &str) -> Vec<u8> {
    sha256(s.as_bytes()).to_vec()
}

pub fn merkle_prove(leaves: &[Vec<u8>], leaf_index: usize) -> Result<Vec<MerkleProofStep>> {
    if leaves.is_empty() || leaf_index >= leaves.len() {
        return Err(Error::MerkleFailed);
    }
    let mut index = leaf_index;
    let mut level = leaves.to_vec();
    let mut proof = Vec::new();
    while level.len() > 1 {
        let is_right = index % 2 == 1;
        let sibling_index = if is_right { index - 1 } else { index + 1 };
        let sibling = if sibling_index < level.len() {
            level[sibling_index].clone()
        } else {
            level[index].clone()
        };
        proof.push(MerkleProofStep {
            side: if is_right {
                MerkleSide::Left
            } else {
                MerkleSide::Right
            },
            hash_hex: to_hex(&sibling),
        });
        let mut next = Vec::new();
        let mut i = 0;
        while i < level.len() {
            let left = &level[i];
            let right = if i + 1 < level.len() {
                &level[i + 1]
            } else {
                left
            };
            next.push(hash_pair(left, right).to_vec());
            i += 2;
        }
        level = next;
        index /= 2;
    }
    Ok(proof)
}

pub fn merkle_verify(leaf: &[u8], proof: &[MerkleProofStep], root_hex: &str) -> bool {
    let mut acc = leaf.to_vec();
    for step in proof {
        let Ok(sibling) = from_hex(&step.hash_hex) else {
            return false;
        };
        acc = match step.side {
            MerkleSide::Left => hash_pair(&sibling, &acc).to_vec(),
            MerkleSide::Right => hash_pair(&acc, &sibling).to_vec(),
        };
    }
    to_hex(acc).eq_ignore_ascii_case(root_hex)
}

pub fn assert_merkle_verify(leaf: &[u8], proof: &[MerkleProofStep], root_hex: &str) -> Result<()> {
    if merkle_verify(leaf, proof, root_hex) {
        Ok(())
    } else {
        Err(Error::MerkleFailed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merkle_prove_verify() {
        let leaves: Vec<Vec<u8>> = (0..5).map(|i| leaf_from_utf8(&format!("cell-{i}"))).collect();
        let root = merkle_root_hex(&leaves);
        for i in 0..leaves.len() {
            let proof = merkle_prove(&leaves, i).unwrap();
            assert!(merkle_verify(&leaves[i], &proof, &root));
        }
        let mut bad = leaves[0].clone();
        bad[0] ^= 1;
        let proof = merkle_prove(&leaves, 0).unwrap();
        assert!(!merkle_verify(&bad, &proof, &root));
    }
}
