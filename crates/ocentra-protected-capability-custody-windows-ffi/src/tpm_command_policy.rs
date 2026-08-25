//! Strict StartAuthSession and policy command encoders.

use super::super::codec_types::handles::{CommandCode, NonNullHandle, SessionHandle};
use super::super::codec_types::policy::{PolicyOrDigests, PolicySessionStart, PolicySignature};
use super::super::{
    TPM_ALG_NULL, TPM_ALG_SHA256, TPM_CC_FLUSH_CONTEXT, TPM_CC_POLICY_COMMAND_CODE,
    TPM_CC_POLICY_CPHASH, TPM_CC_POLICY_OR, TPM_CC_POLICY_SIGNED, TPM_CC_START_AUTH_SESSION,
    TPM_SE_POLICY, TPM_ST_NO_SESSIONS,
};
use super::{finish, header, push_i32, push_tpm2b, push_u16, push_u32, validate_tpm2b_input};
use crate::{Error, Result};

pub(crate) fn encode_start_policy_session(request: &PolicySessionStart) -> Result<Vec<u8>> {
    let body_bytes = 8usize
        .checked_add(2 + request.nonce_caller.as_bytes().len())
        .and_then(|value| value.checked_add(2))
        .and_then(|value| value.checked_add(1 + 2 + 2))
        .ok_or(Error::BufferTooLarge)?;
    let mut command = header(TPM_ST_NO_SESSIONS, TPM_CC_START_AUTH_SESSION, body_bytes)?;
    push_u32(&mut command, request.tpm_key.raw());
    push_u32(&mut command, request.bind.raw());
    push_tpm2b(&mut command, request.nonce_caller.as_bytes())?;
    push_tpm2b(&mut command, &[])?;
    command.push(TPM_SE_POLICY);
    push_u16(&mut command, TPM_ALG_NULL);
    push_u16(&mut command, TPM_ALG_SHA256);
    finish(command)
}

pub(crate) fn encode_policy_signed(
    policy_session: &SessionHandle,
    auth_object: NonNullHandle,
    nonce: &[u8],
    cp_hash_a: Option<&[u8; 32]>,
    policy_ref: &[u8],
    expiration: i32,
    signature: &PolicySignature,
) -> Result<Vec<u8>> {
    validate_tpm2b_input(nonce)?;
    validate_tpm2b_input(policy_ref)?;
    let cp_hash_bytes = cp_hash_a.map_or(0, |_| 32);
    let body_bytes = 8usize
        .checked_add(2 + nonce.len())
        .and_then(|value| value.checked_add(2 + cp_hash_bytes))
        .and_then(|value| value.checked_add(2 + policy_ref.len()))
        .and_then(|value| value.checked_add(4))
        .and_then(|value| value.checked_add(signature.as_bytes().len()))
        .ok_or(Error::BufferTooLarge)?;
    let mut command = header(TPM_ST_NO_SESSIONS, TPM_CC_POLICY_SIGNED, body_bytes)?;
    push_u32(&mut command, policy_session.raw());
    push_u32(&mut command, auth_object.raw());
    push_tpm2b(&mut command, nonce)?;
    match cp_hash_a {
        Some(digest) => push_tpm2b(&mut command, digest)?,
        None => push_tpm2b(&mut command, &[])?,
    }
    push_tpm2b(&mut command, policy_ref)?;
    push_i32(&mut command, expiration);
    command.extend_from_slice(signature.as_bytes());
    finish(command)
}

pub(crate) fn encode_policy_cp_hash(
    policy_session: &SessionHandle,
    cp_hash_a: &[u8; 32],
) -> Result<Vec<u8>> {
    let mut command = header(TPM_ST_NO_SESSIONS, TPM_CC_POLICY_CPHASH, 4 + 2 + 32)?;
    push_u32(&mut command, policy_session.raw());
    push_tpm2b(&mut command, cp_hash_a)?;
    finish(command)
}

pub(crate) fn encode_policy_command_code(
    policy_session: &SessionHandle,
    command_code: CommandCode,
) -> Result<Vec<u8>> {
    let mut command = header(TPM_ST_NO_SESSIONS, TPM_CC_POLICY_COMMAND_CODE, 8)?;
    push_u32(&mut command, policy_session.raw());
    push_u32(&mut command, command_code.raw());
    finish(command)
}

pub(crate) fn encode_policy_or(
    policy_session: &SessionHandle,
    digests: &PolicyOrDigests,
) -> Result<Vec<u8>> {
    let body_bytes = 4usize
        .checked_add(4)
        .and_then(|value| value.checked_add(digests.len().checked_mul(2 + 32)?))
        .ok_or(Error::BufferTooLarge)?;
    let mut command = header(TPM_ST_NO_SESSIONS, TPM_CC_POLICY_OR, body_bytes)?;
    push_u32(&mut command, policy_session.raw());
    push_u32(&mut command, u32::try_from(digests.len())?);
    for digest in digests.iter() {
        push_tpm2b(&mut command, digest.as_bytes())?;
    }
    finish(command)
}

pub(crate) fn encode_flush_context(handle: u32) -> Result<Vec<u8>> {
    let handle = NonNullHandle::from_raw(handle)?;
    let mut command = header(TPM_ST_NO_SESSIONS, TPM_CC_FLUSH_CONTEXT, 4)?;
    push_u32(&mut command, handle.raw());
    finish(command)
}
