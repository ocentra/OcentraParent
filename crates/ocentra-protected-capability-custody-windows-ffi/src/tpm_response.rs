//! Bounded TPM2 response decoders.

use super::cursor::{ResponseCursor, SliceCursor};
use super::TPM_ST_NO_SESSIONS;
use crate::{Error, Result, TpmNvPublicObservation};
use sha2::{Digest, Sha256};

#[path = "tpm_response_auth.rs"]
pub(crate) mod auth;
#[path = "tpm_response_sessions.rs"]
pub(crate) mod sessions;

/// Decode a strict TPM2 `NV_ReadPublic` response.
pub(crate) fn decode_nv_read_public(
    response: &[u8],
    expected_index: u32,
) -> Result<TpmNvPublicObservation> {
    let mut cursor = ResponseCursor::new(response, TPM_ST_NO_SESSIONS)?;
    cursor.expect_response_code()?;
    let public = cursor.take_tpm2b()?;
    let name = cursor.take_tpm2b()?;
    if !cursor.is_empty() || name.is_empty() {
        return Err(Error::MalformedTpm);
    }
    let mut public_cursor = SliceCursor::new(public);
    let nv_index = public_cursor.take_u32()?;
    let name_algorithm = public_cursor.take_u16()?;
    let attributes = public_cursor.take_u32()?;
    let auth_policy = public_cursor.take_tpm2b()?.to_vec();
    let data_size = public_cursor.take_u16()?;
    if !public_cursor.is_empty() || nv_index != expected_index {
        return Err(Error::MalformedTpm);
    }
    verify_nv_name(public, name, name_algorithm)?;
    Ok(TpmNvPublicObservation {
        nv_index,
        name_algorithm,
        attributes,
        auth_policy,
        data_size,
        name: name.to_vec(),
    })
}
fn verify_nv_name(public: &[u8], name: &[u8], name_algorithm: u16) -> Result<()> {
    const TPM_ALG_SHA256: u16 = 0x000b;
    const SHA256_BYTES: usize = 32;
    if name_algorithm != TPM_ALG_SHA256 || name.len() != 2 + SHA256_BYTES {
        return Err(Error::MalformedTpm);
    }
    let digest = Sha256::digest(public);
    if name[..2] != name_algorithm.to_be_bytes() || name[2..] != digest[..] {
        return Err(Error::MalformedTpm);
    }
    Ok(())
}
