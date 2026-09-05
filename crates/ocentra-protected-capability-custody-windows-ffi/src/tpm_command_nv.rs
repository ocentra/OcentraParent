//! Fixed NV counter command templates and public-only `LoadExternal`.

use super::super::codec_types::auth::{AuthorizationArea, Sha256Digest};
use super::super::codec_types::handles::{FixedNvOperation, NvIndex, PermanentHandle};
use super::super::codec_types::signer::TpmPolicySignerPublic;
use super::super::{
    FIXED_COUNTER_BYTES, TPM_CC_LOAD_EXTERNAL, TPM_ST_NO_SESSIONS, TPM_ST_SESSIONS,
};
use super::{checked_u32_len, finish, header, push_tpm2b, push_u16, push_u32};
use crate::{Error, Result};

pub(crate) struct FixedNvCommand {
    operation: FixedNvOperation,
    handles: Vec<u32>,
    names: Vec<Vec<u8>>,
    parameters: Vec<u8>,
}

impl FixedNvCommand {
    pub(crate) fn read(counter_name: &[u8]) -> Result<Self> {
        let index = NvIndex::fixed_counter();
        let mut parameters = Vec::with_capacity(4);
        push_u16(&mut parameters, FIXED_COUNTER_BYTES);
        push_u16(&mut parameters, 0);
        Self::new(
            FixedNvOperation::Read,
            vec![index.raw(), index.raw()],
            vec![counter_name.to_vec(), counter_name.to_vec()],
            parameters,
        )
    }

    pub(crate) fn increment(counter_name: &[u8]) -> Result<Self> {
        let index = NvIndex::fixed_counter();
        Self::new(
            FixedNvOperation::Increment,
            vec![index.raw(), index.raw()],
            vec![counter_name.to_vec(), counter_name.to_vec()],
            Vec::new(),
        )
    }

    fn new(
        operation: FixedNvOperation,
        handles: Vec<u32>,
        names: Vec<Vec<u8>>,
        parameters: Vec<u8>,
    ) -> Result<Self> {
        if handles.is_empty() || handles.len() != names.len() || names.iter().any(Vec::is_empty) {
            return Err(Error::MalformedTpm);
        }
        Ok(Self {
            operation,
            handles,
            names,
            parameters,
        })
    }

    pub(crate) fn operation(&self) -> FixedNvOperation {
        self.operation
    }

    pub(crate) fn cp_hash(&self) -> Sha256Digest {
        let command_code = self.operation.command_code().to_be_bytes();
        let mut parts = Vec::with_capacity(1 + self.names.len() + 1);
        parts.push(command_code.as_slice());
        for name in &self.names {
            parts.push(name.as_slice());
        }
        parts.push(self.parameters.as_slice());
        Sha256Digest::hash(&parts)
    }

    pub(crate) fn encode(&self, authorization: &AuthorizationArea) -> Result<Vec<u8>> {
        let handle_bytes = self
            .handles
            .len()
            .checked_mul(4)
            .ok_or(Error::BufferTooLarge)?;
        let body_bytes = handle_bytes
            .checked_add(4)
            .and_then(|value| value.checked_add(authorization.as_bytes().len()))
            .and_then(|value| value.checked_add(self.parameters.len()))
            .ok_or(Error::BufferTooLarge)?;
        let mut command = header(TPM_ST_SESSIONS, self.operation.command_code(), body_bytes)?;
        for handle in &self.handles {
            push_u32(&mut command, *handle);
        }
        push_u32(
            &mut command,
            checked_u32_len(authorization.as_bytes().len())?,
        );
        command.extend_from_slice(authorization.as_bytes());
        command.extend_from_slice(&self.parameters);
        finish(command)
    }
}

pub(crate) fn encode_load_external(signer: &TpmPolicySignerPublic) -> Result<Vec<u8>> {
    let public = signer.public();
    let body_bytes = 2usize
        .checked_add(2 + public.len())
        .and_then(|value| value.checked_add(4))
        .ok_or(Error::BufferTooLarge)?;
    let mut command = header(TPM_ST_NO_SESSIONS, TPM_CC_LOAD_EXTERNAL, body_bytes)?;
    push_tpm2b(&mut command, &[])?;
    push_tpm2b(&mut command, public)?;
    push_u32(&mut command, PermanentHandle::null().raw());
    finish(command)
}
