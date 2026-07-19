use std::path::PathBuf;

use crate::parent_presence::{
    ParentPresenceChallenge, ParentPresenceChallengeIssuanceFailureReason,
    ParentPresenceObservedAt, ParentPresenceStorageFailureReason,
    ParentPresenceVerificationAccepted, ParentPresenceVerificationFailureReason,
    ParentPresenceVerificationInput, ParentPresenceVerificationPort,
};
use crate::parent_presence_store::{
    ConsumeChallengeResult, ParentPresenceStore, ParentPresenceStoreError,
    ParentPresenceStoreIssueError,
};
use crate::trust_bootstrap_validation::parent_presence_verification_failure_reason;

impl ParentPresenceVerificationPort {
    pub fn open(
        store_path: impl Into<PathBuf>,
    ) -> Result<Self, ParentPresenceStorageFailureReason> {
        Self::with_clock(store_path, || {
            ParentPresenceObservedAt::from_system_time(std::time::SystemTime::now())
        })
    }

    fn with_clock(
        store_path: impl Into<PathBuf>,
        clock: impl Fn() -> ParentPresenceObservedAt + Send + Sync + 'static,
    ) -> Result<Self, ParentPresenceStorageFailureReason> {
        let store = ParentPresenceStore::open(store_path)
            .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;
        Ok(Self {
            clock: Box::new(clock),
            store,
        })
    }

    pub fn issue_challenge(
        &mut self,
        challenge: ParentPresenceChallenge,
    ) -> Result<(), ParentPresenceChallengeIssuanceFailureReason> {
        self.store
            .issue_challenge(challenge)
            .map_err(|error| match error {
                ParentPresenceStoreIssueError::Duplicate => {
                    ParentPresenceChallengeIssuanceFailureReason::DuplicateChallengeRef
                }
                ParentPresenceStoreIssueError::Store(_error) => {
                    ParentPresenceChallengeIssuanceFailureReason::CustodyUnavailable
                }
            })
    }

    pub fn verify_and_consume(
        &mut self,
        input: ParentPresenceVerificationInput,
    ) -> Result<ParentPresenceVerificationAccepted, ParentPresenceVerificationFailureReason> {
        let ParentPresenceVerificationInput {
            challenge_ref,
            assertion,
        } = input;
        let observed_at = (self.clock)();
        let consumed = self
            .store
            .consume_challenge(&challenge_ref, |challenge| {
                parent_presence_verification_failure_reason(challenge, &assertion, &observed_at)
            })
            .map_err(parent_presence_store_failure_reason)?;

        match consumed {
            ConsumeChallengeResult::Accepted(accepted) => {
                Ok(ParentPresenceVerificationAccepted::new(
                    accepted.receipt_ref,
                    accepted.challenge,
                    assertion,
                    observed_at,
                ))
            }
            ConsumeChallengeResult::Rejected(failure_reason) => Err(failure_reason),
        }
    }
}

fn parent_presence_store_failure_reason(
    error: ParentPresenceStoreError,
) -> ParentPresenceVerificationFailureReason {
    match error {
        ParentPresenceStoreError::Unavailable => {
            ParentPresenceVerificationFailureReason::CustodyUnavailable
        }
        ParentPresenceStoreError::IntegrityRejected => {
            ParentPresenceVerificationFailureReason::CustodyIntegrityRejected
        }
    }
}
