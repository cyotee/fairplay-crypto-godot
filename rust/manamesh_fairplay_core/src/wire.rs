//! Public-only wire DTOs. Private keys must never appear here.

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::shuffle::ShuffleCommitmentPublic;
use crate::sra::LayeredCiphertext;

/// Top-level envelope for fair-play multiplayer messages.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FairplayEnvelope {
    pub from_seat: String,
    pub message: WireMessage,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WireMessage {
    PublicKey {
        owner_id: String,
        public_key: String,
    },
    Commitment {
        commitment_hex: String,
        /// Optional label e.g. "dice_hand"
        label: String,
    },
    /// Opening: revealed message + nonce (still not a private key)
    CommitmentOpen {
        commitment_hex: String,
        message_hex: String,
        nonce_hex: String,
        label: String,
    },
    Ciphertext {
        card: LayeredCiphertext,
    },
    PeelShare {
        /// Peeled ciphertext after local layer removal (public)
        card: LayeredCiphertext,
    },
    ShuffleCommit {
        commit: ShuffleCommitmentPublic,
    },
    ShuffleOpen {
        commit: ShuffleCommitmentPublic,
        permutation: Vec<u32>,
        nonce_hex: String,
    },
    MerkleRoot {
        root_hex: String,
    },
    MerkleProof {
        leaf_hex: String,
        root_hex: String,
        proof_json: String,
    },
}

impl FairplayEnvelope {
    pub fn to_json(&self) -> Result<String> {
        let s = serde_json::to_string(self).map_err(|e| Error::Crypto(e.to_string()))?;
        assert_no_private_key_fields(&s)?;
        Ok(s)
    }

    pub fn from_json(s: &str) -> Result<Self> {
        assert_no_private_key_fields(s)?;
        serde_json::from_str(s).map_err(|e| Error::Crypto(e.to_string()))
    }
}

/// Reject serialized payloads that look like they leak private keys.
pub fn assert_no_private_key_fields(json: &str) -> Result<()> {
    let lower = json.to_lowercase();
    // Field names that must never appear on the wire for this product
    const FORBIDDEN: &[&str] = &[
        "\"private_key\"",
        "\"privatekey\"",
        "\"secret_key\"",
        "\"secretkey\"",
        "\"sk_hex\"",
        "\"privkey\"",
        "\"private_key_hex\"",
    ];
    for f in FORBIDDEN {
        if lower.contains(f) {
            return Err(Error::SecretOnWire);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commitment::commit_die_face;

    #[test]
    fn envelope_public_ok() {
        let c = commit_die_face(3);
        let env = FairplayEnvelope {
            from_seat: "0".into(),
            message: WireMessage::Commitment {
                commitment_hex: c.commitment_hex.clone(),
                label: "die".into(),
            },
        };
        let json = env.to_json().unwrap();
        assert!(json.contains("commitment"));
        let back = FairplayEnvelope::from_json(&json).unwrap();
        assert_eq!(back.from_seat, "0");
    }

    #[test]
    fn secret_field_rejected() {
        let bad = r#"{"from_seat":"0","message":{"type":"public_key","owner_id":"0","public_key":"x","private_key":"deadbeef"}}"#;
        assert!(matches!(
            assert_no_private_key_fields(bad),
            Err(Error::SecretOnWire)
        ));
    }

    #[test]
    fn open_is_not_private_key() {
        let c = commit_die_face(6);
        let env = FairplayEnvelope {
            from_seat: "1".into(),
            message: WireMessage::CommitmentOpen {
                commitment_hex: c.commitment_hex,
                message_hex: c.message_hex,
                nonce_hex: c.nonce_hex,
                label: "die".into(),
            },
        };
        // Openings are intentional public reveals of committed material, not sk
        env.to_json().unwrap();
    }
}
