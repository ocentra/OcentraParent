//! Private owned TPM sessions and transient-object lifetimes.
//!
//! A session or object returned by TPM2 is borrowed from its TBS context and
//! cannot outlive it.  Drop always attempts `FlushContext`; cleanup failure is
//! deliberately not converted into success or hidden authority state.

use super::codec_types::auth::AuthorizationArea;
use super::codec_types::auth::ProvisionedAuthValue;
use super::codec_types::handles::{NonNullHandle, NvIndex, SessionHandle, TransientHandle};
use super::codec_types::policy::{ExternalObject, NvPublicDefinition, PolicySessionStart};
use super::{command, response};
use crate::{Error, OwnedTbsContext, Result};

#[path = "tpm_session_nv.rs"]
pub(crate) mod nv;
#[path = "tpm_session_policy.rs"]
pub(crate) mod policy;

pub(crate) struct OwnedTpmSession<'a> {
    context: &'a OwnedTbsContext,
    handle: SessionHandle,
}

pub(crate) struct OwnedTransientObject<'a> {
    context: &'a OwnedTbsContext,
    handle: TransientHandle,
    name: Vec<u8>,
}

impl<'a> OwnedTpmSession<'a> {
    pub(crate) fn start_policy(
        context: &'a OwnedTbsContext,
        request: &PolicySessionStart,
    ) -> Result<(Self, Vec<u8>)> {
        let command = command::policy::encode_start_policy_session(request)?;
        let response = context.submit(&command)?;
        let (handle, nonce_tpm) = response::sessions::decode_start_policy_session(&response)?;
        Ok((Self { context, handle }, nonce_tpm))
    }

    pub(crate) fn authorization_area(
        &self,
        nonce: &[u8],
        attributes: u8,
        hmac: &[u8],
    ) -> Result<AuthorizationArea> {
        let mut wire = Vec::with_capacity(4 + 2 + nonce.len() + 1 + 2 + hmac.len());
        wire.extend_from_slice(&self.handle.raw().to_be_bytes());
        push_tpm2b(&mut wire, nonce)?;
        if attributes & 0x18 != 0 {
            return Err(Error::InvalidInput(
                crate::InputFault::TpmCommandShapeInvalid,
            ));
        }
        wire.push(attributes);
        push_tpm2b(&mut wire, hmac)?;
        AuthorizationArea::from_policy_session_wire(&wire)
    }

    pub(crate) fn handle(&self) -> u32 {
        self.handle.raw()
    }
}

impl Drop for OwnedTpmSession<'_> {
    fn drop(&mut self) {
        if let Ok(command) = command::policy::encode_flush_context(self.handle.raw()) {
            drop(self.context.submit(&command));
        }
    }
}

impl<'a> OwnedTransientObject<'a> {
    pub(crate) fn load_external(
        context: &'a OwnedTbsContext,
        hierarchy: NonNullHandle,
        object: &ExternalObject,
        authorization: &AuthorizationArea,
    ) -> Result<Self> {
        let command = command::nv::encode_load_external(hierarchy, object, authorization)?;
        let response = context.submit(&command)?;
        let (handle, name) = response::sessions::decode_load_external(&response)?;
        Ok(Self {
            context,
            handle,
            name,
        })
    }

    pub(crate) fn handle(&self) -> u32 {
        self.handle.raw()
    }

    pub(crate) fn name(&self) -> &[u8] {
        &self.name
    }
}

impl Drop for OwnedTransientObject<'_> {
    fn drop(&mut self) {
        if let Ok(command) = command::policy::encode_flush_context(self.handle.raw()) {
            drop(self.context.submit(&command));
        }
    }
}

impl OwnedTbsContext {
    pub(crate) fn start_policy_session(
        &self,
        request: &PolicySessionStart,
    ) -> Result<(OwnedTpmSession<'_>, Vec<u8>)> {
        OwnedTpmSession::start_policy(self, request)
    }

    pub(crate) fn define_nv_space(
        &self,
        auth_handle: NonNullHandle,
        index: NvIndex,
        auth_value: &ProvisionedAuthValue,
        public: &NvPublicDefinition,
        authorization: &AuthorizationArea,
    ) -> Result<()> {
        let command = command::nv::encode_nv_define_space(
            auth_handle,
            index,
            auth_value,
            public,
            authorization,
        )?;
        let response = self.submit(&command)?;
        response::sessions::decode_success_with_sessions(&response)
    }

    pub(crate) fn load_external(
        &self,
        hierarchy: NonNullHandle,
        object: &ExternalObject,
        authorization: &AuthorizationArea,
    ) -> Result<OwnedTransientObject<'_>> {
        OwnedTransientObject::load_external(self, hierarchy, object, authorization)
    }
}

fn push_tpm2b(output: &mut Vec<u8>, bytes: &[u8]) -> Result<()> {
    if bytes.len() > u16::MAX as usize {
        return Err(Error::InvalidInput(
            crate::InputFault::TpmCommandShapeInvalid,
        ));
    }
    output.extend_from_slice(&u16::try_from(bytes.len())?.to_be_bytes());
    output.extend_from_slice(bytes);
    Ok(())
}
