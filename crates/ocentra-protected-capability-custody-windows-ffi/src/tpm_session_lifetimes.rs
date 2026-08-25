//! TPM session and transient-object ownership with fallible flush.

use super::super::codec_types::auth::{
    clear_bytes, constant_time_eq, SecretNonce, SecretSessionKey,
};
use super::super::codec_types::handles::SessionHandle;
use super::super::codec_types::signer::TpmPolicySignerPublic;
use super::super::{command, response};
use super::close::{SessionHandleState, TransientHandleState};
use crate::{Error, OwnedTbsContext, Result};

pub(super) struct OwnedTpmSession<'a> {
    pub(super) context: &'a OwnedTbsContext,
    pub(super) handle: SessionHandleState,
    pub(super) nonce_tpm: SecretNonce,
    pub(super) nonce_caller: SecretNonce,
    pub(super) session_key: SecretSessionKey,
    pub(super) command_sequence: u64,
}

pub(super) struct OwnedTransientObject<'a> {
    pub(super) context: &'a OwnedTbsContext,
    pub(super) handle: TransientHandleState,
    name: Vec<u8>,
}

impl<'a> OwnedTpmSession<'a> {
    pub(super) fn start_policy(context: &'a OwnedTbsContext) -> Result<Self> {
        let nonce_caller = SecretNonce::from_os_random(context.random_nonce()?);
        let mut command = command::policy::encode_start_policy_session(&nonce_caller)?;
        let response_result = context.submit(&command);
        clear_bytes(command.as_mut_slice());
        let mut response = response_result?;
        let decoded = response::sessions::decode_start_policy_session(&response);
        clear_bytes(response.as_mut_slice());
        let (handle, nonce_tpm) = decoded?;
        Ok(Self {
            context,
            handle: SessionHandleState::Active(handle),
            nonce_tpm,
            nonce_caller,
            session_key: SecretSessionKey::unsalted_unbound(),
            command_sequence: 0,
        })
    }

    pub(super) fn handle(&self) -> Result<&SessionHandle> {
        match &self.handle {
            SessionHandleState::Active(handle) => Ok(handle),
            SessionHandleState::Terminated | SessionHandleState::Abandoned => {
                Err(Error::MalformedTpm)
            }
        }
    }

    pub(super) fn mark_terminated(&mut self) {
        self.handle = SessionHandleState::Terminated;
    }

    pub(super) fn abandon_after_unverifiable_response(&mut self) {
        self.handle = SessionHandleState::Abandoned;
    }
}

impl<'a> OwnedTransientObject<'a> {
    pub(super) fn load_external(
        context: &'a OwnedTbsContext,
        signer: &TpmPolicySignerPublic,
    ) -> Result<Self> {
        let command = command::nv::encode_load_external(signer)?;
        let response = context.submit(&command)?;
        let (handle, name) = response::sessions::decode_load_external(&response)?;
        let mut owned = Self {
            context,
            handle: TransientHandleState::Active(handle),
            name,
        };
        if !constant_time_eq(&owned.name, signer.name()) {
            let _ = owned.close_in_place();
            return Err(Error::MalformedTpm);
        }
        Ok(owned)
    }

    pub(super) fn handle(&self) -> Result<u32> {
        match &self.handle {
            TransientHandleState::Active(handle) => Ok(handle.raw()),
            TransientHandleState::Terminated | TransientHandleState::Abandoned => {
                Err(Error::MalformedTpm)
            }
        }
    }
}
