//! Policy-session operations kept separate from lifetime/transport ownership.

use super::super::codec_types::handles::{CommandCode, NonNullHandle};
use super::super::codec_types::policy::{PolicyOrDigests, PolicySignature};
use super::super::{command, response};
use super::OwnedTpmSession;
use crate::Result;

impl OwnedTpmSession<'_> {
    pub(crate) fn policy_signed(
        &self,
        auth_object: NonNullHandle,
        nonce: &[u8],
        cp_hash_a: Option<&[u8; 32]>,
        policy_ref: &[u8],
        expiration: i32,
        signature: &PolicySignature,
    ) -> Result<()> {
        let command = command::policy::encode_policy_signed(
            &self.handle,
            auth_object,
            nonce,
            cp_hash_a,
            policy_ref,
            expiration,
            signature,
        )?;
        let response = self.context.submit(&command)?;
        response::sessions::decode_success_no_parameters(&response)
    }

    pub(crate) fn policy_cp_hash(&self, cp_hash_a: &[u8; 32]) -> Result<()> {
        let command = command::policy::encode_policy_cp_hash(&self.handle, cp_hash_a)?;
        let response = self.context.submit(&command)?;
        response::sessions::decode_success_no_parameters(&response)
    }

    pub(crate) fn policy_command_code(&self, command_code: CommandCode) -> Result<()> {
        let command = command::policy::encode_policy_command_code(&self.handle, command_code)?;
        let response = self.context.submit(&command)?;
        response::sessions::decode_success_no_parameters(&response)
    }

    pub(crate) fn policy_or(&self, digests: &PolicyOrDigests) -> Result<()> {
        let command = command::policy::encode_policy_or(&self.handle, digests)?;
        let response = self.context.submit(&command)?;
        response::sessions::decode_success_no_parameters(&response)
    }
}
