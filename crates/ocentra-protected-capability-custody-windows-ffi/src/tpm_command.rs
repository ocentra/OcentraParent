//! Bounded TPM2 command encoders.

use super::cursor::SliceCursor;
use super::{
    MAX_AUTH_SESSIONS, TPM_CC_NV_INCREMENT, TPM_CC_NV_READ, TPM_CC_NV_READ_PUBLIC,
    TPM_HEADER_BYTES, TPM_ST_NO_SESSIONS, TPM_ST_SESSIONS,
};
use crate::{Error, Result, MAX_BUFFER_BYTES};

/// Encode a no-session TPM2 `NV_ReadPublic` command.
pub(crate) fn encode_nv_read_public(index: u32) -> Result<Vec<u8>> {
    if index == 0 {
        return Err(Error::InvalidInput("TPM NV index must be non-zero"));
    }
    let mut command = header(TPM_ST_NO_SESSIONS, TPM_CC_NV_READ_PUBLIC, 4)?;
    push_u32(&mut command, index);
    Ok(command)
}

/// Encode a TPM2 `NV_Read` command with an already-encoded authorization area.
pub(crate) fn encode_nv_read(
    auth_handle: u32,
    index: u32,
    authorization_area: &[u8],
    size: u16,
    offset: u16,
) -> Result<Vec<u8>> {
    validate_handles(auth_handle, index)?;
    validate_auth_area(authorization_area)?;
    let parameter_bytes = 12usize
        .checked_add(authorization_area.len())
        .and_then(|value| value.checked_add(4))
        .ok_or(Error::BufferTooLarge)?;
    let mut command = header_with_tag(TPM_ST_SESSIONS, TPM_CC_NV_READ, parameter_bytes)?;
    push_u32(&mut command, auth_handle);
    push_u32(&mut command, index);
    push_u32(
        &mut command,
        u32::try_from(authorization_area.len()).map_err(|_| Error::BufferTooLarge)?,
    );
    command.extend_from_slice(authorization_area);
    push_u16(&mut command, size);
    push_u16(&mut command, offset);
    Ok(command)
}

/// Encode a TPM2 `NV_Increment` command with an authorization area.
pub(crate) fn encode_nv_increment(
    auth_handle: u32,
    index: u32,
    authorization_area: &[u8],
) -> Result<Vec<u8>> {
    validate_handles(auth_handle, index)?;
    validate_auth_area(authorization_area)?;
    let parameter_bytes = 12usize
        .checked_add(authorization_area.len())
        .ok_or(Error::BufferTooLarge)?;
    let mut command = header_with_tag(TPM_ST_SESSIONS, TPM_CC_NV_INCREMENT, parameter_bytes)?;
    push_u32(&mut command, auth_handle);
    push_u32(&mut command, index);
    push_u32(
        &mut command,
        u32::try_from(authorization_area.len()).map_err(|_| Error::BufferTooLarge)?,
    );
    command.extend_from_slice(authorization_area);
    Ok(command)
}

fn validate_handles(auth_handle: u32, index: u32) -> Result<()> {
    if auth_handle == 0 || index == 0 {
        return Err(Error::InvalidInput("TPM NV handles must be non-zero"));
    }
    Ok(())
}

fn validate_auth_area(authorization_area: &[u8]) -> Result<()> {
    if authorization_area.is_empty() || authorization_area.len() > MAX_BUFFER_BYTES {
        return Err(Error::InvalidInput(
            "TPM command authorization area is empty or too large",
        ));
    }
    let mut cursor = SliceCursor::new(authorization_area);
    let mut sessions = 0usize;
    while !cursor.is_empty() {
        cursor.take_u32()?;
        cursor.take_tpm2b()?;
        cursor.take(1)?;
        cursor.take_tpm2b()?;
        sessions = sessions.checked_add(1).ok_or(Error::BufferTooLarge)?;
        if sessions > MAX_AUTH_SESSIONS {
            return Err(Error::BufferTooLarge);
        }
    }
    Ok(())
}

fn header(tag: u16, command_code: u32, body_bytes: usize) -> Result<Vec<u8>> {
    header_with_tag(tag, command_code, body_bytes)
}

fn header_with_tag(tag: u16, command_code: u32, body_bytes: usize) -> Result<Vec<u8>> {
    let size = TPM_HEADER_BYTES
        .checked_add(body_bytes)
        .ok_or(Error::BufferTooLarge)?;
    let size = u32::try_from(size).map_err(|_| Error::BufferTooLarge)?;
    let mut command = Vec::with_capacity(size as usize);
    push_u16(&mut command, tag);
    push_u32(&mut command, size);
    push_u32(&mut command, command_code);
    Ok(command)
}

fn push_u16(output: &mut Vec<u8>, value: u16) {
    output.extend_from_slice(&value.to_be_bytes());
}

fn push_u32(output: &mut Vec<u8>, value: u32) {
    output.extend_from_slice(&value.to_be_bytes());
}
