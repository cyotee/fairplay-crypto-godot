//! Public-key keychain admission with strict policies.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::hashing::sha256_hex;
use crate::secp::{private_matches_public, public_key_from_hex, public_key_to_hex};

#[derive(Clone, Debug)]
pub struct KeychainPolicy {
    pub require_valid_curve: bool,
    pub unique_ids: bool,
    pub unique_public_keys: bool,
    pub allow_replace: bool,
}

impl Default for KeychainPolicy {
    fn default() -> Self {
        Self::strict()
    }
}

impl KeychainPolicy {
    pub fn strict() -> Self {
        Self {
            require_valid_curve: true,
            unique_ids: true,
            unique_public_keys: true,
            allow_replace: false,
        }
    }

    pub fn mental_poker() -> Self {
        Self::strict()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct KeyEntry {
    pub owner_id: String,
    pub public_key: String,
    pub fingerprint: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RejectReason {
    Ok,
    InvalidCurve,
    DuplicateId,
    DuplicateKey,
    ReplaceForbidden,
    EmptyId,
    EmptyKey,
}

#[derive(Clone, Debug, Default)]
pub struct Keychain {
    entries: BTreeMap<String, KeyEntry>,
    policy: KeychainPolicy,
}

impl Keychain {
    pub fn new(policy: KeychainPolicy) -> Self {
        Self {
            entries: BTreeMap::new(),
            policy,
        }
    }

    pub fn empty_strict() -> Self {
        Self::new(KeychainPolicy::strict())
    }

    pub fn entries(&self) -> &BTreeMap<String, KeyEntry> {
        &self.entries
    }

    pub fn get(&self, owner_id: &str) -> Option<&KeyEntry> {
        self.entries.get(owner_id)
    }

    pub fn admit(&mut self, owner_id: &str, public_key: &str) -> Result<KeyEntry> {
        if owner_id.is_empty() {
            return Err(Error::Keychain("empty id".into()));
        }
        if public_key.is_empty() {
            return Err(Error::Keychain("empty key".into()));
        }

        let normalized = if self.policy.require_valid_curve {
            let pk = public_key_from_hex(public_key).map_err(|_| {
                Error::Keychain(format!("{:?}", RejectReason::InvalidCurve))
            })?;
            public_key_to_hex(&pk)
        } else {
            public_key.to_string()
        };

        if self.policy.unique_ids && self.entries.contains_key(owner_id) && !self.policy.allow_replace
        {
            return Err(Error::Keychain(format!("{:?}", RejectReason::DuplicateId)));
        }

        if self.policy.unique_public_keys {
            for (id, e) in &self.entries {
                if e.public_key.eq_ignore_ascii_case(&normalized) && id != owner_id {
                    return Err(Error::Keychain(format!(
                        "{:?}",
                        RejectReason::DuplicateKey
                    )));
                }
            }
        }

        let fingerprint = sha256_hex(normalized.as_bytes());
        let entry = KeyEntry {
            owner_id: owner_id.to_string(),
            public_key: normalized,
            fingerprint,
        };
        self.entries.insert(owner_id.to_string(), entry.clone());
        Ok(entry)
    }

    /// Ensure local private key matches an admitted public key for `owner_id`.
    pub fn bind_local_sk(&self, owner_id: &str, private_key_hex: &str) -> Result<()> {
        let entry = self
            .get(owner_id)
            .ok_or_else(|| Error::Keychain("owner not admitted".into()))?;
        if private_matches_public(private_key_hex, &entry.public_key) {
            Ok(())
        } else {
            Err(Error::KeyMismatch)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::secp::generate_keypair;

    #[test]
    fn admit_and_reject_duplicate() {
        let mut kc = Keychain::empty_strict();
        let a = generate_keypair();
        let b = generate_keypair();
        kc.admit("p0", &a.public_key_hex).unwrap();
        assert!(kc.admit("p0", &b.public_key_hex).is_err());
        assert!(kc.admit("p1", &a.public_key_hex).is_err()); // duplicate key
        kc.admit("p1", &b.public_key_hex).unwrap();
    }

    #[test]
    fn reject_invalid_curve() {
        let mut kc = Keychain::empty_strict();
        assert!(kc.admit("x", "00").is_err());
        assert!(kc.admit("x", "not-hex").is_err());
    }

    #[test]
    fn bind_sk_pk() {
        let mut kc = Keychain::empty_strict();
        let kp = generate_keypair();
        kc.admit("seat0", &kp.public_key_hex).unwrap();
        kc.bind_local_sk("seat0", &kp.private_key_hex).unwrap();
        let other = generate_keypair();
        assert!(kc.bind_local_sk("seat0", &other.private_key_hex).is_err());
    }
}
