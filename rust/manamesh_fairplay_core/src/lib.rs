//! ManaMesh FairPlay pure crypto core (no Godot types).
//!
//! Private keys never belong on the wire — use [`wire`] for public envelopes.

pub mod commitment;
pub mod error;
pub mod hashing;
pub mod hexutil;
pub mod keychain;
pub mod merkle;
pub mod secp;
pub mod shuffle;
pub mod sra;
pub mod transport;
pub mod wire;

pub use commitment::{commit, commit_with_nonce, verify_commitment, Commitment};
pub use error::{Error, Result};
pub use hashing::{sha256, sha256_hex, random_bytes};
pub use keychain::{KeyEntry, Keychain, KeychainPolicy, RejectReason};
pub use merkle::{merkle_prove, merkle_root, merkle_verify, MerkleProofStep, MerkleSide};
pub use secp::{generate_keypair, private_matches_public, validate_public_key, KeyPair};
pub use shuffle::{
    apply_permutation, commit_shuffle, generate_permutation, is_valid_permutation,
    open_and_verify_shuffle, ShuffleCommit,
};
pub use sra::{encrypt_layer, encrypt_payload, peel_layer, peel_to_payload, LayeredCiphertext};
pub use transport::MultiSeatSimulator;
pub use wire::{FairplayEnvelope, WireMessage};
