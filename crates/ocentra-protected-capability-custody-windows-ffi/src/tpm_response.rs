//! Bounded TPM2 response decoders.

use super::cursor::{ResponseCursor, SliceCursor};
use super::{MAX_AUTH_SESSIONS, TPM_ST_NO_SESSIONS, TPM_ST_SESSIONS};
use crate::{Error, NvPublic, Result, MAX_BUFFER_BYTES};

/// Decode a strict TPM2 `NV_ReadPublic` response.
pub(crate) fn decode_nv_read_public(response: &[u8], expected_index: u32) -> Result<NvPublic> {
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
    Ok(NvPublic {
        nv_index,
        name_algorithm,
        attributes,
        auth_policy,
        data_size,
    })
}

/// Decode the parameter area of a TPM2 `NV_Read` response.
pub(crate) fn decode_nv_read(response: &[u8]) -> Result<Vec<u8>> {
    let mut cursor = ResponseCursor::new(response, TPM_ST_SESSIONS)?;
    cursor.expect_response_code()?;
    let parameter_size = cursor.take_u32()? as usize;
    if parameter_size > MAX_BUFFER_BYTES || parameter_size > cursor.remaining() {
        return Err(Error::MalformedTpm);
    }
    let parameters = cursor.take(parameter_size)?;
    let mut parameter_cursor = SliceCursor::new(parameters);
    let data = parameter_cursor.take_tpm2b()?.to_vec();
    if !parameter_cursor.is_empty() {
        return Err(Error::MalformedTpm);
    }
    validate_auth_response_area(cursor.take(cursor.remaining())?)?;
    Ok(data)
}

/// Decode the strict no-parameter TPM2 `NV_Increment` response.
pub(crate) fn decode_nv_increment(response: &[u8]) -> Result<()> {
    let mut cursor = ResponseCursor::new(response, TPM_ST_SESSIONS)?;
    cursor.expect_response_code()?;
    let parameter_size = cursor.take_u32()?;
    if parameter_size != 0 {
        return Err(Error::MalformedTpm);
    }
    validate_auth_response_area(cursor.take(cursor.remaining())?)?;
    Ok(())
}

fn validate_auth_response_area(authorization_area: &[u8]) -> Result<()> {
    if authorization_area.is_empty() || authorization_area.len() > MAX_BUFFER_BYTES {
        return Err(Error::MalformedTpm);
    }
    let mut cursor = SliceCursor::new(authorization_area);
    let mut sessions = 0usize;
    while !cursor.is_empty() {
        cursor.take_tpm2b()?;
        cursor.take(1)?;
        cursor.take_tpm2b()?;
        sessions = sessions.checked_add(1).ok_or(Error::MalformedTpm)?;
        if sessions > MAX_AUTH_SESSIONS {
            return Err(Error::MalformedTpm);
        }
    }
    Ok(())
}
