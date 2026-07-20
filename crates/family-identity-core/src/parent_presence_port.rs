use std::path::PathBuf;

use crate::parent_presence::{
    ParentPresenceChallenge, ParentPresenceChallengeIssuanceFailureReason,
    ParentPresenceCustodyDecisionArtifact, ParentPresenceObservedAt,
    ParentPresenceStorageFailureReason, ParentPresenceVerificationAccepted,
    ParentPresenceVerificationFailureReason, ParentPresenceVerificationInput,
    ParentPresenceVerificationPort,
};
use crate::parent_presence_port_decision::finish_parent_presence_verification;
use crate::parent_presence_store::{ParentPresenceStore, ParentPresenceStoreIssueError};
use crate::trust_bootstrap_validation::parent_presence_verification_failure_reason;

impl ParentPresenceVerificationPort {
    pub fn open(
        store_path: impl Into<PathBuf>,
    ) -> Result<Self, ParentPresenceStorageFailureReason> {
        let _unsupported_path = store_path.into();
        Err(ParentPresenceStorageFailureReason::CustodyUnavailable)
    }

    #[cfg(debug_assertions)]
    pub fn open_unsealed_test_custody(
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
            custody_artifact: None,
        })
    }

    pub fn issue_challenge(
        &mut self,
        challenge: ParentPresenceChallenge,
    ) -> Result<(), ParentPresenceChallengeIssuanceFailureReason> {
        self.store
            .issue_challenge(challenge)
            .map_err(|error| match error {
                ParentPresenceStoreIssueError::TimestampInvalid => {
                    ParentPresenceChallengeIssuanceFailureReason::TimestampInvalid
                }
                ParentPresenceStoreIssueError::DuplicateChallenge => {
                    ParentPresenceChallengeIssuanceFailureReason::DuplicateChallengeRef
                }
                ParentPresenceStoreIssueError::DuplicateNonce => {
                    ParentPresenceChallengeIssuanceFailureReason::DuplicateNonceRef
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
            correlation_id,
            challenge_ref,
            assertion,
        } = input;
        let observed_at = (self.clock)();
        let consumed = self.store.consume_challenge(&challenge_ref, |challenge| {
            parent_presence_verification_failure_reason(challenge, &assertion, &observed_at)
        });

        finish_parent_presence_verification(
            &mut self.custody_artifact,
            correlation_id,
            assertion,
            observed_at,
            consumed,
        )
    }

    pub fn take_custody_artifact(&mut self) -> Option<ParentPresenceCustodyDecisionArtifact> {
        self.custody_artifact.take()
    }
}
