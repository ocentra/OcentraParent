//! Exact plain-policy-session execution of one fixed NV counter command.

use super::super::codec_types::auth::{clear_bytes, AuthorizationArea, SecretNonce};
use super::super::codec_types::handles::FixedNvOperation;
use super::super::command::nv::FixedNvCommand;
use super::super::response::auth::ResponseAuthorization;
use super::lifetimes::OwnedTpmSession;
use super::nv_response::{self, DecodedCounterResponse};
use super::CounterOutcome;
use crate::{Error, Result, TpmCounterIncrementUncertainty};

impl OwnedTpmSession<'_> {
    pub(super) fn execute_counter_command(
        &mut self,
        template: &FixedNvCommand,
    ) -> Result<CounterOutcome> {
        if self.command_sequence != 0 || !self.session_key.is_empty() {
            return Err(Error::MalformedTpm);
        }
        let nonce_caller = SecretNonce::from_os_random(self.context.random_nonce()?);
        // This is the final use of the policy session. Clearing continueSession
        // makes successful command completion terminate it inside the TPM.
        let attributes = 0;
        let authorization = AuthorizationArea::policy(self.handle()?, &nonce_caller, attributes)?;
        let mut command = template.encode(&authorization)?;
        let response_result = self.context.submit(&command);
        clear_bytes(command.as_mut_slice());
        let mut response = match response_result {
            Ok(response) => response,
            Err(error) => {
                return self.unverifiable_result(
                    template.operation(),
                    TpmCounterIncrementUncertainty::Transport,
                    error,
                );
            }
        };
        let decoded_result = nv_response::decode(template.operation(), &response);
        clear_bytes(response.as_mut_slice());
        let (outcome, authorization) = match decoded_result {
            DecodedCounterResponse::Accepted(outcome, authorization) => (outcome, authorization),
            DecodedCounterResponse::Rejected(error) => return Err(error),
            DecodedCounterResponse::Unverifiable(error) => {
                return self.unverifiable_result(
                    template.operation(),
                    TpmCounterIncrementUncertainty::MalformedResponse,
                    error,
                );
            }
        };
        if let Err(error) = self.verify_and_terminate(&nonce_caller, authorization) {
            return self.unverifiable_result(
                template.operation(),
                TpmCounterIncrementUncertainty::MalformedResponse,
                error,
            );
        }
        Ok(outcome)
    }

    fn verify_and_terminate(
        &mut self,
        nonce_caller: &SecretNonce,
        response_authorization: ResponseAuthorization,
    ) -> Result<()> {
        if response_authorization.attributes != 0 {
            return Err(Error::MalformedTpm);
        }
        let next_sequence = self
            .command_sequence
            .checked_add(1)
            .ok_or(Error::MalformedTpm)?;
        self.nonce_tpm = response_authorization.nonce_tpm;
        self.nonce_caller = SecretNonce::from_os_random(*nonce_caller.as_bytes());
        self.command_sequence = next_sequence;
        self.mark_terminated();
        Ok(())
    }

    fn unverifiable_result(
        &mut self,
        operation: FixedNvOperation,
        reason: TpmCounterIncrementUncertainty,
        read_error: Error,
    ) -> Result<CounterOutcome> {
        // A success response would already have terminated the session, while
        // a lost response leaves its state unknowable. Never send a raw flush
        // against a handle that could already have been recycled.
        self.abandon_after_unverifiable_response();
        nv_response::unverifiable(operation, reason, read_error)
    }
}
