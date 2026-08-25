use super::{MAX_AUTH_VALUE_BYTES, MAX_TPM_AUTH_SESSIONS, MAX_TPM_NONCE_BYTES};
use crate::tpm::codec_types::handles::SessionHandle;
use crate::{Error, InputFault, Result};

pub(super) fn validate_authorization_area(bytes: &[u8]) -> Result<()> {
    let mut cursor = 0usize;
    let mut sessions = 0usize;
    while cursor < bytes.len() {
        validate_authorization_entry(bytes, &mut cursor)?;
        sessions = sessions.checked_add(1).ok_or(Error::BufferTooLarge)?;
        if sessions > MAX_TPM_AUTH_SESSIONS {
            return Err(Error::InvalidInput(InputFault::TpmCommandShapeInvalid));
        }
    }
    if sessions == 0 || cursor != bytes.len() {
        return Err(Error::InvalidInput(InputFault::TpmCommandShapeInvalid));
    }
    Ok(())
}

fn validate_authorization_entry(bytes: &[u8], cursor: &mut usize) -> Result<()> {
    let session_handle = take_u32(bytes, cursor)?;
    SessionHandle::from_response(session_handle)
        .map_err(|_| Error::InvalidInput(InputFault::TpmCommandShapeInvalid))?;
    let nonce = take_tpm2b(bytes, cursor)?;
    let attributes = take_u8(bytes, cursor)?;
    if attributes & 0x18 != 0 {
        return Err(Error::InvalidInput(InputFault::TpmCommandShapeInvalid));
    }
    let hmac = take_tpm2b(bytes, cursor)?;
    if nonce.len() > MAX_TPM_NONCE_BYTES || hmac.len() > MAX_AUTH_VALUE_BYTES {
        return Err(Error::InvalidInput(InputFault::TpmCommandShapeInvalid));
    }
    Ok(())
}

fn take_u8(bytes: &[u8], cursor: &mut usize) -> Result<u8> {
    let end = cursor.checked_add(1).ok_or(Error::MalformedTpm)?;
    if end > bytes.len() {
        return Err(Error::MalformedTpm);
    }
    let value = bytes[*cursor];
    *cursor = end;
    Ok(value)
}

fn take_u16(bytes: &[u8], cursor: &mut usize) -> Result<u16> {
    let end = cursor.checked_add(2).ok_or(Error::MalformedTpm)?;
    if end > bytes.len() {
        return Err(Error::MalformedTpm);
    }
    let value = u16::from_be_bytes([bytes[*cursor], bytes[*cursor + 1]]);
    *cursor = end;
    Ok(value)
}

fn take_u32(bytes: &[u8], cursor: &mut usize) -> Result<u32> {
    let end = cursor.checked_add(4).ok_or(Error::MalformedTpm)?;
    if end > bytes.len() {
        return Err(Error::MalformedTpm);
    }
    let value = u32::from_be_bytes([
        bytes[*cursor],
        bytes[*cursor + 1],
        bytes[*cursor + 2],
        bytes[*cursor + 3],
    ]);
    *cursor = end;
    Ok(value)
}

fn take_tpm2b<'a>(bytes: &'a [u8], cursor: &mut usize) -> Result<&'a [u8]> {
    let length = usize::from(take_u16(bytes, cursor)?);
    let end = cursor.checked_add(length).ok_or(Error::MalformedTpm)?;
    if end > bytes.len() {
        return Err(Error::MalformedTpm);
    }
    let value = &bytes[*cursor..end];
    *cursor = end;
    Ok(value)
}
