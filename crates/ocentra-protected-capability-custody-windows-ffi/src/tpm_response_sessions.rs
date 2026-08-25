//! Strict response decoders for session-tagged and policy commands.

use super::super::codec_types::handles::{SessionHandle, TransientHandle};
use super::super::cursor::{ResponseCursor, SliceCursor};
use super::super::{TPM_ST_NO_SESSIONS, TPM_ST_SESSIONS};
use super::auth;
use crate::{Error, Result, MAX_BUFFER_BYTES};

pub(crate) fn decode_nv_read(response: &[u8]) -> Result<Vec<u8>> {
    let mut cursor = ResponseCursor::new(response, TPM_ST_SESSIONS)?;
    let parameter_size = usize::try_from(cursor.take_u32()?)?;
    if parameter_size > cursor.remaining_len() || parameter_size > MAX_BUFFER_BYTES {
        return Err(Error::MalformedTpm);
    }
    let parameters = cursor.take(parameter_size)?;
    let mut parameter_cursor = SliceCursor::new(parameters);
    let data = parameter_cursor.take_tpm2b()?.to_vec();
    if !parameter_cursor.is_empty() {
        return Err(Error::MalformedTpm);
    }
    auth::validate(cursor.take_remaining())?;
    if !cursor.is_empty() {
        return Err(Error::MalformedTpm);
    }
    Ok(data)
}

pub(crate) fn decode_success_no_parameters(response: &[u8]) -> Result<()> {
    let cursor = ResponseCursor::new(response, TPM_ST_NO_SESSIONS)?;
    if !cursor.is_empty() {
        return Err(Error::MalformedTpm);
    }
    Ok(())
}

pub(crate) fn decode_success_with_sessions(response: &[u8]) -> Result<()> {
    let mut cursor = ResponseCursor::new(response, TPM_ST_SESSIONS)?;
    if cursor.take_u32()? != 0 {
        return Err(Error::MalformedTpm);
    }
    auth::validate(cursor.take_remaining())?;
    if !cursor.is_empty() {
        return Err(Error::MalformedTpm);
    }
    Ok(())
}

pub(crate) fn decode_start_policy_session(response: &[u8]) -> Result<(SessionHandle, Vec<u8>)> {
    let mut cursor = ResponseCursor::new(response, TPM_ST_NO_SESSIONS)?;
    let session = SessionHandle::from_response(cursor.take_u32()?)?;
    let nonce_tpm = cursor.take_tpm2b()?.to_vec();
    if nonce_tpm.is_empty() || nonce_tpm.len() > 64 {
        return Err(Error::MalformedTpm);
    }
    if !cursor.is_empty() {
        return Err(Error::MalformedTpm);
    }
    Ok((session, nonce_tpm))
}

pub(crate) fn decode_load_external(response: &[u8]) -> Result<(TransientHandle, Vec<u8>)> {
    let mut cursor = ResponseCursor::new(response, TPM_ST_SESSIONS)?;
    let transient = TransientHandle::from_response(cursor.take_u32()?)?;
    let parameter_size = usize::try_from(cursor.take_u32()?)?;
    let parameters = cursor.take(parameter_size)?;
    let mut parameter_cursor = SliceCursor::new(parameters);
    let name = parameter_cursor.take_tpm2b()?.to_vec();
    if name.is_empty() || name.len() > MAX_BUFFER_BYTES || !parameter_cursor.is_empty() {
        return Err(Error::MalformedTpm);
    }
    auth::validate(cursor.take_remaining())?;
    if !cursor.is_empty() {
        return Err(Error::MalformedTpm);
    }
    Ok((transient, name))
}
