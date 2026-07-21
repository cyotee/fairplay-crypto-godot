use rand::RngCore;
use sha2::{Digest, Sha256};

pub fn sha256(data: impl AsRef<[u8]>) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(data.as_ref());
    h.finalize().into()
}

pub fn sha256_concat(parts: &[&[u8]]) -> [u8; 32] {
    let mut h = Sha256::new();
    for p in parts {
        h.update(p);
    }
    h.finalize().into()
}

pub fn sha256_hex(data: impl AsRef<[u8]>) -> String {
    crate::hexutil::to_hex(sha256(data))
}

pub fn random_bytes(len: usize) -> Vec<u8> {
    let mut buf = vec![0u8; len];
    rand::thread_rng().fill_bytes(&mut buf);
    buf
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sha256_empty_known() {
        // e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
        assert_eq!(
            sha256_hex([]),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn random_bytes_nonzero_len() {
        assert_eq!(random_bytes(32).len(), 32);
    }
}
