//! Internal one-use preparation for fixed TPM counter ceremonies.

use super::super::codec_types::auth::{constant_time_eq, Sha256Digest};
use super::super::codec_types::handles::FixedNvOperation;
use super::super::codec_types::policy::FixedPolicyProfile;
use super::super::codec_types::signer::{TpmPolicySignature, TpmPolicySignerPublic};
use super::super::command::nv::FixedNvCommand;
use super::super::{
    FIXED_COUNTER_BYTES, FIXED_COUNTER_DEFINED_ATTRIBUTES, FIXED_COUNTER_INDEX,
    FIXED_COUNTER_OBSERVED_ATTRIBUTES, TPM_ALG_SHA256,
};
use super::lifetimes::{OwnedTpmSession, OwnedTransientObject};
use super::CounterOutcome;
use crate::{Error, OwnedTbsContext, Result, TpmNvPublicObservation};

pub(super) struct PreparedCounterOperation<'a> {
    session: OwnedTpmSession<'a>,
    signer: OwnedTransientObject<'a>,
    profile: FixedPolicyProfile,
    command: FixedNvCommand,
    signing_digest: Sha256Digest,
}

impl<'a> PreparedCounterOperation<'a> {
    fn prepare_existing(
        context: &'a OwnedTbsContext,
        signer_public: &TpmPolicySignerPublic,
        command: fn(&[u8]) -> Result<FixedNvCommand>,
        operation: FixedNvOperation,
    ) -> Result<Self> {
        let profile = FixedPolicyProfile::for_signer(signer_public);
        let observation = context.observe_fixed_counter_public()?;
        validate_counter_observation(&observation, &profile, operation)?;
        let signer = OwnedTransientObject::load_external(context, signer_public)?;
        let session = OwnedTpmSession::start_policy(context)?;
        let command = command(observation.name())?;
        Self::finish_prepare(session, signer, profile, command)
    }

    fn finish_prepare(
        session: OwnedTpmSession<'a>,
        signer: OwnedTransientObject<'a>,
        profile: FixedPolicyProfile,
        command: FixedNvCommand,
    ) -> Result<Self> {
        let signing_digest = session.policy_signing_digest(&command.cp_hash(), &profile);
        Ok(Self {
            session,
            signer,
            profile,
            command,
            signing_digest,
        })
    }

    pub(super) fn signing_digest(&self) -> [u8; 32] {
        *self.signing_digest.as_bytes()
    }

    pub(super) fn execute(mut self, signature: &TpmPolicySignature) -> Result<CounterOutcome> {
        let operation_result = self.execute_inner(signature);
        if operation_result.is_err() {
            // No NV effect was accepted on an error returned here. Cleanup is
            // best-effort and Drop retries active handles without replacing
            // the command error.
            let _ = self.session.close_in_place();
            let _ = self.signer.close_in_place();
        }
        operation_result
    }

    fn execute_inner(&mut self, signature: &TpmPolicySignature) -> Result<CounterOutcome> {
        self.session.authorize_operation(
            self.signer.handle()?,
            &self.command,
            &self.profile,
            signature,
        )?;
        // The public-only verifier is no longer needed after PolicySigned.
        // Flush it before a mutating command so cleanup can never turn an
        // accepted increment into an apparent failure.
        self.signer.close_in_place()?;
        self.session.execute_counter_command(&self.command)
    }

    pub(super) fn close(mut self) -> Result<()> {
        let session_close = self.session.close_in_place();
        let signer_close = self.signer.close_in_place();
        session_close.and(signer_close)
    }
}

pub(super) fn prepare_read<'a>(
    context: &'a OwnedTbsContext,
    signer: &TpmPolicySignerPublic,
) -> Result<PreparedCounterOperation<'a>> {
    PreparedCounterOperation::prepare_existing(
        context,
        signer,
        FixedNvCommand::read,
        FixedNvOperation::Read,
    )
}

pub(super) fn prepare_increment<'a>(
    context: &'a OwnedTbsContext,
    signer: &TpmPolicySignerPublic,
) -> Result<PreparedCounterOperation<'a>> {
    PreparedCounterOperation::prepare_existing(
        context,
        signer,
        FixedNvCommand::increment,
        FixedNvOperation::Increment,
    )
}

fn validate_counter_observation(
    observation: &TpmNvPublicObservation,
    profile: &FixedPolicyProfile,
    operation: FixedNvOperation,
) -> Result<()> {
    let attributes_match = match operation {
        FixedNvOperation::Read => observation.attributes == FIXED_COUNTER_OBSERVED_ATTRIBUTES,
        FixedNvOperation::Increment => {
            observation.attributes == FIXED_COUNTER_DEFINED_ATTRIBUTES
                || observation.attributes == FIXED_COUNTER_OBSERVED_ATTRIBUTES
        }
    };
    if observation.nv_index != FIXED_COUNTER_INDEX
        || observation.name_algorithm != TPM_ALG_SHA256
        || !attributes_match
        || observation.data_size != FIXED_COUNTER_BYTES
        || !constant_time_eq(
            &observation.auth_policy,
            profile.counter_policy().as_bytes(),
        )
        || observation.name.len() != 34
    {
        return Err(Error::MalformedTpm);
    }
    Ok(())
}
