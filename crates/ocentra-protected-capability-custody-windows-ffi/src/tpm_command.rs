//! Strict TPM2 command encoders and command-shape helpers.

use super::{FIXED_COUNTER_INDEX, TPM_CC_NV_READ_PUBLIC, TPM_HEADER_BYTES, TPM_ST_NO_SESSIONS};
use crate::{Error, InputFault, Result, MAX_BUFFER_BYTES};

#[path = "tpm_command_nv.rs"]
pub(crate) mod nv;
#[path = "tpm_command_policy.rs"]
pub(crate) mod policy;

/// Encode the no-session public-area observation used during admission.
pub(crate) fn encode_nv_read_public() -> Result<Vec<u8>> {
    let mut command = header(TPM_ST_NO_SESSIONS, TPM_CC_NV_READ_PUBLIC, 4)?;
    push_u32(&mut command, FIXED_COUNTER_INDEX);
    finish(command)
}

pub(super) fn validate_tpm2b_input(bytes: &[u8]) -> Result<()> {
    if bytes.len() > u16::MAX as usize || bytes.len() > MAX_BUFFER_BYTES {
        return Err(Error::InvalidInput(InputFault::TpmCommandShapeInvalid));
    }
    Ok(())
}

pub(super) fn push_tpm2b(output: &mut Vec<u8>, bytes: &[u8]) -> Result<()> {
    validate_tpm2b_input(bytes)?;
    push_u16(output, u16::try_from(bytes.len())?);
    output.extend_from_slice(bytes);
    Ok(())
}

pub(super) fn checked_u32_len(length: usize) -> Result<u32> {
    Ok(u32::try_from(length)?)
}

pub(super) fn header(tag: u16, command_code: u32, body_bytes: usize) -> Result<Vec<u8>> {
    let size = TPM_HEADER_BYTES
        .checked_add(body_bytes)
        .ok_or(Error::BufferTooLarge)?;
    if size > MAX_BUFFER_BYTES {
        return Err(Error::BufferTooLarge);
    }
    let size = u32::try_from(size)?;
    let mut command = Vec::with_capacity(size as usize);
    push_u16(&mut command, tag);
    push_u32(&mut command, size);
    push_u32(&mut command, command_code);
    Ok(command)
}

pub(super) fn finish(command: Vec<u8>) -> Result<Vec<u8>> {
    if command.len() < TPM_HEADER_BYTES {
        return Err(Error::MalformedTpm);
    }
    let declared = u32::from_be_bytes([command[2], command[3], command[4], command[5]]) as usize;
    if declared != command.len() || declared > MAX_BUFFER_BYTES {
        return Err(Error::MalformedTpm);
    }
    Ok(command)
}

pub(super) fn push_u16(output: &mut Vec<u8>, value: u16) {
    output.extend_from_slice(&value.to_be_bytes());
}

pub(super) fn push_i32(output: &mut Vec<u8>, value: i32) {
    output.extend_from_slice(&value.to_be_bytes());
}

pub(super) fn push_u32(output: &mut Vec<u8>, value: u32) {
    output.extend_from_slice(&value.to_be_bytes());
}
