//! Strict NV_DefineSpace, NV_Read, NV_Increment, NV_UndefineSpace, and
//! LoadExternal command encoders.

use super::super::codec_types::auth::{AuthorizationArea, ProvisionedAuthValue};
use super::super::codec_types::handles::{NonNullHandle, NvIndex};
use super::super::codec_types::policy::{ExternalObject, NvPublicDefinition};
use super::super::{
    TPM_ALG_SHA256, TPM_CC_LOAD_EXTERNAL, TPM_CC_NV_DEFINE_SPACE, TPM_CC_NV_INCREMENT,
    TPM_CC_NV_READ, TPM_CC_NV_UNDEFINE_SPACE, TPM_ST_SESSIONS,
};
use super::{checked_u32_len, finish, header, push_tpm2b, push_u16, push_u32};
use crate::{Error, InputFault, Result};

pub(crate) fn encode_nv_define_space(
    auth_handle: NonNullHandle,
    index: NvIndex,
    auth_value: &ProvisionedAuthValue,
    public: &NvPublicDefinition,
    authorization: &AuthorizationArea,
) -> Result<Vec<u8>> {
    if public.index.raw() != index.raw() {
        return Err(Error::InvalidInput(InputFault::TpmCommandShapeInvalid));
    }
    let auth_area = authorization.as_bytes();
    let public_area = encode_nv_public(&index, public)?;
    let body_bytes = 4usize
        .checked_add(4)
        .and_then(|value| value.checked_add(auth_area.len()))
        .and_then(|value| value.checked_add(2 + auth_value.as_bytes().len()))
        .and_then(|value| value.checked_add(public_area.len()))
        .ok_or(Error::BufferTooLarge)?;
    let mut command = header(TPM_ST_SESSIONS, TPM_CC_NV_DEFINE_SPACE, body_bytes)?;
    push_u32(&mut command, auth_handle.raw());
    push_u32(&mut command, checked_u32_len(auth_area.len())?);
    command.extend_from_slice(auth_area);
    push_tpm2b(&mut command, auth_value.as_bytes())?;
    command.extend_from_slice(&public_area);
    finish(command)
}

pub(crate) fn encode_nv_read(
    auth_handle: NonNullHandle,
    index: NvIndex,
    authorization: &AuthorizationArea,
    size: u16,
    offset: u16,
) -> Result<Vec<u8>> {
    let auth_area = authorization.as_bytes();
    let body_bytes = 8usize
        .checked_add(4)
        .and_then(|value| value.checked_add(auth_area.len()))
        .and_then(|value| value.checked_add(4))
        .ok_or(Error::BufferTooLarge)?;
    let mut command = header(TPM_ST_SESSIONS, TPM_CC_NV_READ, body_bytes)?;
    push_u32(&mut command, auth_handle.raw());
    push_u32(&mut command, index.raw());
    push_u32(&mut command, checked_u32_len(auth_area.len())?);
    command.extend_from_slice(auth_area);
    push_u16(&mut command, size);
    push_u16(&mut command, offset);
    finish(command)
}

pub(crate) fn encode_nv_increment(
    auth_handle: NonNullHandle,
    index: NvIndex,
    authorization: &AuthorizationArea,
) -> Result<Vec<u8>> {
    let auth_area = authorization.as_bytes();
    let body_bytes = 8usize
        .checked_add(4)
        .and_then(|value| value.checked_add(auth_area.len()))
        .ok_or(Error::BufferTooLarge)?;
    let mut command = header(TPM_ST_SESSIONS, TPM_CC_NV_INCREMENT, body_bytes)?;
    push_u32(&mut command, auth_handle.raw());
    push_u32(&mut command, index.raw());
    push_u32(&mut command, checked_u32_len(auth_area.len())?);
    command.extend_from_slice(auth_area);
    finish(command)
}

pub(crate) fn encode_nv_undefine_space(
    auth_handle: NonNullHandle,
    index: NvIndex,
    authorization: &AuthorizationArea,
) -> Result<Vec<u8>> {
    let auth_area = authorization.as_bytes();
    let body_bytes = 8usize
        .checked_add(4)
        .and_then(|value| value.checked_add(auth_area.len()))
        .ok_or(Error::BufferTooLarge)?;
    let mut command = header(TPM_ST_SESSIONS, TPM_CC_NV_UNDEFINE_SPACE, body_bytes)?;
    push_u32(&mut command, auth_handle.raw());
    push_u32(&mut command, index.raw());
    push_u32(&mut command, checked_u32_len(auth_area.len())?);
    command.extend_from_slice(auth_area);
    finish(command)
}

pub(crate) fn encode_load_external(
    hierarchy: NonNullHandle,
    object: &ExternalObject,
    authorization: &AuthorizationArea,
) -> Result<Vec<u8>> {
    let auth_area = authorization.as_bytes();
    let body_bytes = 4usize
        .checked_add(4)
        .and_then(|value| value.checked_add(auth_area.len()))
        .and_then(|value| value.checked_add(2 + object.sensitive().len()))
        .and_then(|value| value.checked_add(2 + object.public().len()))
        .ok_or(Error::BufferTooLarge)?;
    let mut command = header(TPM_ST_SESSIONS, TPM_CC_LOAD_EXTERNAL, body_bytes)?;
    push_u32(&mut command, checked_u32_len(auth_area.len())?);
    command.extend_from_slice(auth_area);
    push_tpm2b(&mut command, object.sensitive())?;
    push_tpm2b(&mut command, object.public())?;
    push_u32(&mut command, hierarchy.raw());
    finish(command)
}

fn encode_nv_public(index: &NvIndex, public: &NvPublicDefinition) -> Result<Vec<u8>> {
    if public.index.raw() != index.raw() {
        return Err(Error::InvalidInput(InputFault::TpmCommandShapeInvalid));
    }
    let mut value = Vec::with_capacity(48);
    push_u32(&mut value, index.raw());
    push_u16(&mut value, TPM_ALG_SHA256);
    push_u32(&mut value, public.attributes);
    push_tpm2b(&mut value, public.auth_policy.as_bytes())?;
    push_u16(&mut value, public.data_size);
    let mut encoded = Vec::with_capacity(value.len() + 2);
    push_tpm2b(&mut encoded, &value)?;
    Ok(encoded)
}
