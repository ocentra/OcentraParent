//! Fixed policy authorization sequence for one prepared counter command.

use super::super::codec_types::auth::{clear_bytes, Sha256Digest};
use super::super::codec_types::handles::FixedNvOperation;
use super::super::codec_types::policy::FixedPolicyProfile;
use super::super::codec_types::signer::TpmPolicySignature;
use super::super::command::nv::FixedNvCommand;
use super::super::{command, response};
use super::lifetimes::OwnedTpmSession;
use crate::Result;

impl OwnedTpmSession<'_> {
    pub(super) fn policy_signing_digest(
        &self,
        cp_hash: &Sha256Digest,
        profile: &FixedPolicyProfile,
    ) -> Sha256Digest {
        Sha256Digest::hash(&[
            self.nonce_tpm.as_bytes(),
            &profile.expiration().to_be_bytes(),
            cp_hash.as_bytes(),
            profile.policy_ref(),
        ])
    }

    pub(super) fn authorize_operation(
        &self,
        signer_handle: u32,
        nv_command: &FixedNvCommand,
        profile: &FixedPolicyProfile,
        signature: &TpmPolicySignature,
    ) -> Result<()> {
        let cp_hash = nv_command.cp_hash();
        self.policy_signed(signer_handle, &cp_hash, profile, signature)?;
        self.policy_command_code(nv_command.operation())?;
        self.policy_or(profile)
    }

    fn policy_signed(
        &self,
        signer_handle: u32,
        cp_hash: &Sha256Digest,
        profile: &FixedPolicyProfile,
        signature: &TpmPolicySignature,
    ) -> Result<()> {
        let command = command::policy::encode_policy_signed(
            signer_handle,
            self.handle()?,
            &self.nonce_tpm,
            cp_hash,
            profile,
            signature,
        )?;
        let response = self.submit_sensitive(command)?;
        response::sessions::decode_policy_signed(&response)
    }

    fn policy_command_code(&self, operation: FixedNvOperation) -> Result<()> {
        let command = command::policy::encode_policy_command_code(self.handle()?, operation)?;
        let response = self.submit_sensitive(command)?;
        response::sessions::decode_success_no_parameters(&response)
    }

    fn policy_or(&self, profile: &FixedPolicyProfile) -> Result<()> {
        let command = command::policy::encode_policy_or(self.handle()?, profile)?;
        let response = self.submit_sensitive(command)?;
        response::sessions::decode_success_no_parameters(&response)
    }

    fn submit_sensitive(&self, mut command: Vec<u8>) -> Result<Vec<u8>> {
        let result = self.context.submit(&command);
        clear_bytes(command.as_mut_slice());
        result
    }
}
