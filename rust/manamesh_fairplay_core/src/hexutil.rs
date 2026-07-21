use crate::error::{Error, Result};

pub fn to_hex(bytes: impl AsRef<[u8]>) -> String {
    hex::encode(bytes.as_ref())
}

pub fn from_hex(s: &str) -> Result<Vec<u8>> {
    let s = s.strip_prefix("0x").unwrap_or(s);
    hex::decode(s).map_err(|e| Error::InvalidHex(e.to_string()))
}

pub fn from_hex_32(s: &str) -> Result<[u8; 32]> {
    let v = from_hex(s)?;
    if v.len() != 32 {
        return Err(Error::InvalidHex(format!("expected 32 bytes, got {}", v.len())));
    }
    let mut out = [0u8; 32];
    out.copy_from_slice(&v);
    Ok(out)
}
