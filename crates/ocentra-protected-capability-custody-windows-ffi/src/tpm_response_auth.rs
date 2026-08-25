//! Strict TPM2 response authorization-area validation.

use super::super::cursor::SliceCursor;
use crate::{Error, Result, MAX_BUFFER_BYTES};

const MAX_RESPONSE_SESSIONS: usize = 3;
const MAX_NONCE_BYTES: usize = 64;
const MAX_HMAC_BYTES: usize = 64;

pub(super) fn validate(bytes: &[u8]) -> Result<()> {
    if bytes.is_empty() || bytes.len() > MAX_BUFFER_BYTES {
        return Err(Error::MalformedTpm);
    }
    let mut cursor = SliceCursor::new(bytes);
    let mut session_count = 0usize;
    while !cursor.is_empty() {
        session_count = session_count.checked_add(1).ok_or(Error::BufferTooLarge)?;
        if session_count > MAX_RESPONSE_SESSIONS {
            return Err(Error::MalformedTpm);
        }
        let nonce = cursor.take_tpm2b()?;
        let attributes = cursor.take_u8()?;
        let hmac = cursor.take_tpm2b()?;
        if nonce.len() > MAX_NONCE_BYTES || hmac.len() > MAX_HMAC_BYTES || attributes & 0x18 != 0 {
            return Err(Error::MalformedTpm);
        }
    }
    Ok(())
}
