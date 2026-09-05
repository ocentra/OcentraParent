//! Strict response decoding for fixed policy-session commands.

use super::super::codec_types::auth::SecretNonce;
use super::super::codec_types::handles::{SessionHandle, TransientHandle};
use super::super::cursor::{ResponseCursor, SliceCursor};
use super::super::{
    FIXED_COUNTER_BYTES, TPM_RH_NULL, TPM_ST_AUTH_SIGNED, TPM_ST_NO_SESSIONS, TPM_ST_SESSIONS,
};
use super::auth::{self, ResponseAuthorization};
use crate::{Error, Result, MAX_BUFFER_BYTES};

pub(crate) struct SessionResponse {
    pub(crate) parameters: Vec<u8>,
    pub(crate) authorization: ResponseAuthorization,
}

pub(crate) fn decode_nv_read(response: &[u8]) -> Result<(u64, SessionResponse)> {
    let decoded = decode_session_response(response)?;
    let mut parameters = SliceCursor::new(&decoded.parameters);
    let data = parameters.take_tpm2b()?;
    if data.len() != usize::from(FIXED_COUNTER_BYTES) || !parameters.is_empty() {
        return Err(Error::MalformedTpm);
    }
    let value =
        u64::from_be_bytes(<[u8; 8]>::try_from(data).map_err(|_error| Error::MalformedTpm)?);
    Ok((value, decoded))
}

pub(crate) fn decode_success_with_session(response: &[u8]) -> Result<SessionResponse> {
    let decoded = decode_session_response(response)?;
    if !decoded.parameters.is_empty() {
        return Err(Error::MalformedTpm);
    }
    Ok(decoded)
}

fn decode_session_response(response: &[u8]) -> Result<SessionResponse> {
    let mut cursor = ResponseCursor::new(response, TPM_ST_SESSIONS)?;
    let parameter_size = usize::try_from(cursor.take_u32()?)?;
    if parameter_size > cursor.remaining_len() || parameter_size > MAX_BUFFER_BYTES {
        return Err(Error::MalformedTpm);
    }
    let parameters = cursor.take(parameter_size)?.to_vec();
    let authorization = auth::decode_exactly_one(cursor.take_remaining())?;
    if !cursor.is_empty() {
        return Err(Error::MalformedTpm);
    }
    Ok(SessionResponse {
        parameters,
        authorization,
    })
}

pub(crate) fn decode_success_no_parameters(response: &[u8]) -> Result<()> {
    let cursor = ResponseCursor::new(response, TPM_ST_NO_SESSIONS)?;
    if !cursor.is_empty() {
        return Err(Error::MalformedTpm);
    }
    Ok(())
}

pub(crate) fn decode_policy_signed(response: &[u8]) -> Result<()> {
    let mut cursor = ResponseCursor::new(response, TPM_ST_NO_SESSIONS)?;
    let timeout = cursor.take_tpm2b()?;
    let ticket_tag = cursor.take_u16()?;
    let hierarchy = cursor.take_u32()?;
    let digest = cursor.take_tpm2b()?;
    if !cursor.is_empty()
        || !timeout.is_empty()
        || ticket_tag != TPM_ST_AUTH_SIGNED
        || hierarchy != TPM_RH_NULL
        || !digest.is_empty()
    {
        return Err(Error::MalformedTpm);
    }
    Ok(())
}

pub(crate) fn decode_start_policy_session(response: &[u8]) -> Result<(SessionHandle, SecretNonce)> {
    let mut cursor = ResponseCursor::new(response, TPM_ST_NO_SESSIONS)?;
    let session = SessionHandle::from_policy_response(cursor.take_u32()?)?;
    let nonce_tpm = SecretNonce::from_tpm(cursor.take_tpm2b()?)?;
    if !cursor.is_empty() {
        return Err(Error::MalformedTpm);
    }
    Ok((session, nonce_tpm))
}

pub(crate) fn decode_load_external(response: &[u8]) -> Result<(TransientHandle, Vec<u8>)> {
    let mut cursor = ResponseCursor::new(response, TPM_ST_NO_SESSIONS)?;
    let transient = TransientHandle::from_response(cursor.take_u32()?)?;
    let name = cursor.take_tpm2b()?.to_vec();
    if name.len() != 34 || !cursor.is_empty() {
        return Err(Error::MalformedTpm);
    }
    Ok((transient, name))
}
