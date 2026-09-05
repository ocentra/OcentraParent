//! Fixed unsalted policy-session and policy command encoders.

use super::super::codec_types::auth::{SecretNonce, Sha256Digest};
use super::super::codec_types::handles::{FixedNvOperation, PermanentHandle, SessionHandle};
use super::super::codec_types::policy::FixedPolicyProfile;
use super::super::codec_types::signer::TpmPolicySignature;
use super::super::{
    TPM_ALG_NULL, TPM_ALG_SHA256, TPM_CC_FLUSH_CONTEXT, TPM_CC_POLICY_COMMAND_CODE,
    TPM_CC_POLICY_OR, TPM_CC_POLICY_SIGNED, TPM_CC_START_AUTH_SESSION, TPM_SE_POLICY,
    TPM_ST_NO_SESSIONS,
};
use super::{finish, header, push_i32, push_tpm2b, push_u16, push_u32};
use crate::{Error, InputFault, Result};

pub(crate) fn encode_start_policy_session(nonce_caller: &SecretNonce) -> Result<Vec<u8>> {
    let body_bytes = 8usize
        .checked_add(2 + nonce_caller.as_bytes().len())
        .and_then(|value| value.checked_add(2))
        .and_then(|value| value.checked_add(1 + 2 + 2))
        .ok_or(Error::BufferTooLarge)?;
    let mut command = header(TPM_ST_NO_SESSIONS, TPM_CC_START_AUTH_SESSION, body_bytes)?;
    push_u32(&mut command, PermanentHandle::null().raw());
    push_u32(&mut command, PermanentHandle::null().raw());
    push_tpm2b(&mut command, nonce_caller.as_bytes())?;
    push_tpm2b(&mut command, &[])?;
    command.push(TPM_SE_POLICY);
    push_u16(&mut command, TPM_ALG_NULL);
    push_u16(&mut command, TPM_ALG_SHA256);
    finish(command)
}

pub(crate) fn encode_policy_signed(
    auth_object: u32,
    policy_session: &SessionHandle,
    nonce_tpm: &SecretNonce,
    cp_hash: &Sha256Digest,
    profile: &FixedPolicyProfile,
    signature: &TpmPolicySignature,
) -> Result<Vec<u8>> {
    let signature = signature.encode()?;
    let body_bytes = 8usize
        .checked_add(2 + nonce_tpm.as_bytes().len())
        .and_then(|value| value.checked_add(2 + cp_hash.as_bytes().len()))
        .and_then(|value| value.checked_add(2 + profile.policy_ref().len()))
        .and_then(|value| value.checked_add(4))
        .and_then(|value| value.checked_add(signature.len()))
        .ok_or(Error::BufferTooLarge)?;
    let mut command = header(TPM_ST_NO_SESSIONS, TPM_CC_POLICY_SIGNED, body_bytes)?;
    // TPM2_PolicySigned handle order is authObject, then policySession.
    push_u32(&mut command, auth_object);
    push_u32(&mut command, policy_session.raw());
    push_tpm2b(&mut command, nonce_tpm.as_bytes())?;
    push_tpm2b(&mut command, cp_hash.as_bytes())?;
    push_tpm2b(&mut command, profile.policy_ref())?;
    push_i32(&mut command, profile.expiration());
    command.extend_from_slice(&signature);
    finish(command)
}

pub(crate) fn encode_policy_command_code(
    policy_session: &SessionHandle,
    operation: FixedNvOperation,
) -> Result<Vec<u8>> {
    let mut command = header(TPM_ST_NO_SESSIONS, TPM_CC_POLICY_COMMAND_CODE, 8)?;
    push_u32(&mut command, policy_session.raw());
    push_u32(&mut command, operation.command_code());
    finish(command)
}

pub(crate) fn encode_policy_or(
    policy_session: &SessionHandle,
    profile: &FixedPolicyProfile,
) -> Result<Vec<u8>> {
    let branches = profile.branches();
    if branches.len() != 2 {
        return Err(Error::InvalidInput(InputFault::TpmCommandShapeInvalid));
    }
    let body_bytes = 4usize
        .checked_add(4)
        .and_then(|value| value.checked_add(branches.len() * (2 + 32)))
        .ok_or(Error::BufferTooLarge)?;
    let mut command = header(TPM_ST_NO_SESSIONS, TPM_CC_POLICY_OR, body_bytes)?;
    push_u32(&mut command, policy_session.raw());
    push_u32(&mut command, u32::try_from(branches.len())?);
    for digest in branches {
        push_tpm2b(&mut command, digest.as_bytes())?;
    }
    finish(command)
}

pub(crate) fn encode_flush_context(handle: u32) -> Result<Vec<u8>> {
    if handle == 0 || handle == u32::MAX {
        return Err(Error::InvalidInput(InputFault::TpmCommandShapeInvalid));
    }
    let mut command = header(TPM_ST_NO_SESSIONS, TPM_CC_FLUSH_CONTEXT, 4)?;
    push_u32(&mut command, handle);
    finish(command)
}
