//! Strict single-session response authorization decoding.

use super::super::codec_types::auth::SecretNonce;
use super::super::cursor::SliceCursor;
use crate::tpm::TPM_SESSION_CONTINUE;
use crate::{Error, Result};

pub(crate) struct ResponseAuthorization {
    pub(crate) nonce_tpm: SecretNonce,
    pub(crate) attributes: u8,
}

pub(super) fn decode_exactly_one(bytes: &[u8]) -> Result<ResponseAuthorization> {
    let mut cursor = SliceCursor::new(bytes);
    let nonce_tpm = SecretNonce::from_tpm(cursor.take_tpm2b()?)?;
    let attributes = cursor.take_u8()?;
    let hmac = cursor.take_tpm2b()?;
    if !cursor.is_empty() || attributes & !TPM_SESSION_CONTINUE != 0 || !hmac.is_empty() {
        return Err(Error::MalformedTpm);
    }
    Ok(ResponseAuthorization {
        nonce_tpm,
        attributes,
    })
}
