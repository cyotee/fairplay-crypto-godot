use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid hex: {0}")]
    InvalidHex(String),
    #[error("invalid private key")]
    InvalidPrivateKey,
    #[error("invalid public key / curve point")]
    InvalidPublicKey,
    #[error("private key does not match public key")]
    KeyMismatch,
    #[error("keychain rejected: {0}")]
    Keychain(String),
    #[error("cannot peel plaintext (layers=0)")]
    CannotPeelPlaintext,
    #[error("expected {expected} layers, got {got}")]
    LayerMismatch { expected: u32, got: u32 },
    #[error("payload not found in lookup")]
    PayloadNotFound,
    #[error("invalid permutation")]
    InvalidPermutation,
    #[error("deck / permutation length mismatch")]
    LengthMismatch,
    #[error("shuffle verification failed")]
    ShuffleVerifyFailed,
    #[error("commitment verification failed")]
    CommitmentFailed,
    #[error("merkle verification failed")]
    MerkleFailed,
    #[error("hash-to-point failed for payload: {0}")]
    HashToPoint(String),
    #[error("wire envelope must not contain private keys")]
    SecretOnWire,
    #[error("crypto error: {0}")]
    Crypto(String),
}
