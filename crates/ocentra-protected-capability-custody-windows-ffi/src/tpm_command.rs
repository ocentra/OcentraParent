//! Bounded TPM2 command encoders.

use super::{TPM_CC_NV_READ_PUBLIC, TPM_HEADER_BYTES, TPM_ST_NO_SESSIONS};
use crate::{Error, InputFault, Result};

/// Encode a no-session TPM2 `NV_ReadPublic` command.
pub(crate) fn encode_nv_read_public(index: u32) -> Result<Vec<u8>> {
    if index == 0 {
        return Err(Error::InvalidInput(InputFault::TpmNvIndexInvalid));
    }
    let mut command = header(TPM_ST_NO_SESSIONS, TPM_CC_NV_READ_PUBLIC, 4)?;
    push_u32(&mut command, index);
    Ok(command)
}

fn header(tag: u16, command_code: u32, body_bytes: usize) -> Result<Vec<u8>> {
    header_with_tag(tag, command_code, body_bytes)
}

fn header_with_tag(tag: u16, command_code: u32, body_bytes: usize) -> Result<Vec<u8>> {
    let size = TPM_HEADER_BYTES
        .checked_add(body_bytes)
        .ok_or(Error::BufferTooLarge)?;
    let size = u32::try_from(size)?;
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
