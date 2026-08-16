use sha2::{Digest, Sha256};

pub fn stored_key(value: &str) -> String {
    format!("{:x}", Sha256::digest(Sha256::digest(value.as_bytes())))
}
