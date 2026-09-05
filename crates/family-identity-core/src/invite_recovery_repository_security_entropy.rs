use getrandom::fill;
use sha2::{Digest, Sha256};

use super::{InviteRecoveryRepositoryError, INVITE_TOKEN_DIGEST_DOMAIN};

pub(crate) fn opaque_id(prefix: &str) -> Result<String, InviteRecoveryRepositoryError> {
    let mut bytes = [0_u8; 16];
    fill(&mut bytes).map_err(|_error| InviteRecoveryRepositoryError::EntropyUnavailable)?;
    let mut value = String::with_capacity(prefix.len() + bytes.len() * 2);
    value.push_str(prefix);
    for byte in bytes {
        value.push_str(&format!("{byte:02x}"));
    }
    Ok(value)
}

pub(crate) fn digest_token(token: &str) -> String {
    digest(INVITE_TOKEN_DIGEST_DOMAIN, token)
}

fn digest(domain: &[u8], value: &str) -> String {
    let mut hash = Sha256::new();
    hash.update(domain);
    hash.update(value.as_bytes());
    hash.finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}
