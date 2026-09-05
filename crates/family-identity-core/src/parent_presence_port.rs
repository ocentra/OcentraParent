use std::path::PathBuf;

use crate::parent_presence::{
    ParentPresenceChallenge, ParentPresenceChallengeIssuanceFailureReason,
    ParentPresenceCustodyDecisionArtifact, ParentPresenceObservedAt,
    ParentPresenceStorageFailureReason, ParentPresenceVerificationAccepted,
    ParentPresenceVerificationFailureReason, ParentPresenceVerificationInput,
    ParentPresenceVerificationPort,
};
use crate::parent_presence_event_delivery::ParentPresenceDecisionDelivery;
use crate::parent_presence_port_decision::finish_parent_presence_verification;
use crate::parent_presence_store::{
    ConsumeChallengeResult, ParentPresenceStore, ParentPresenceStoreIssueError,
};
use crate::trust_bootstrap_validation::parent_presence_verification_failure_reason;

#[path = "parent_presence_port_step_up.rs"]
mod step_up;

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

    #[cfg(debug_assertions)]
    pub fn open_unsealed_test_custody_at(
        store_path: impl Into<PathBuf>,
        observed_at: &str,
    ) -> Result<Self, ParentPresenceStorageFailureReason> {
        let observed_at = ParentPresenceObservedAt::from_canonical_utc(observed_at)
            .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;
        Self::with_clock(store_path, move || observed_at.clone())
    }

    fn with_clock(
        store_path: impl Into<PathBuf>,
        clock: impl Fn() -> ParentPresenceObservedAt + Send + Sync + 'static,
    ) -> Result<Self, ParentPresenceStorageFailureReason> {
        let store_path = store_path.into();
        let mut store = ParentPresenceStore::open(&store_path)
            .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;
        let decision_delivery = ParentPresenceDecisionDelivery::for_store_path(&store_path);
        drain_pending_decisions(&mut store, &decision_delivery)?;
        Ok(Self {
            clock: Box::new(clock),
            store,
            custody_artifact: None,
            decision_delivery,
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
        self.verify_and_consume_inner(input, None)
    }

    fn verify_and_consume_inner(
        &mut self,
        input: ParentPresenceVerificationInput,
        verified_credential: Option<(&str, i32, u32)>,
    ) -> Result<ParentPresenceVerificationAccepted, ParentPresenceVerificationFailureReason> {
        let ParentPresenceVerificationInput {
            correlation_id,
            challenge_ref,
            assertion,
        } = input;
        let observed_at = (self.clock)();
        let accepted_artifact = ParentPresenceCustodyDecisionArtifact::new(
            correlation_id.clone(),
            crate::parent_presence::ParentPresenceCustodyDecisionResult::Accepted,
        )
        .map_err(|_error| ParentPresenceVerificationFailureReason::CustodyUnavailable)?;
        let accepted_pending = self
            .decision_delivery
            .prepare(&accepted_artifact, &observed_at)
            .map_err(|_error| ParentPresenceVerificationFailureReason::CustodyUnavailable)?;
        let decision_observed_at = observed_at.clone();
        let consumed = self.store.consume_challenge(
            &challenge_ref,
            &accepted_pending,
            |challenge| {
                parent_presence_verification_failure_reason(challenge, &assertion, &observed_at)
            },
            verified_credential,
        );

        let accepted = matches!(&consumed, Ok(ConsumeChallengeResult::Accepted(_)));
        let (artifact, result) = finish_parent_presence_verification(
            correlation_id,
            assertion,
            observed_at,
            consumed,
            accepted_artifact,
        )?;
        if !accepted {
            let pending = self
                .decision_delivery
                .prepare(&artifact, &decision_observed_at)
                .map_err(|_error| ParentPresenceVerificationFailureReason::CustodyUnavailable)?;
            if self.store.enqueue_decision(&pending).is_err() {
                self.custody_artifact = None;
                return Err(ParentPresenceVerificationFailureReason::CustodyUnavailable);
            }
        }
        if drain_pending_decisions(&mut self.store, &self.decision_delivery).is_err() {
            self.custody_artifact = None;
            return Err(ParentPresenceVerificationFailureReason::CustodyUnavailable);
        }
        self.custody_artifact = Some(artifact);
        result
    }

    pub fn take_custody_artifact(&mut self) -> Option<ParentPresenceCustodyDecisionArtifact> {
        self.custody_artifact.take()
    }

    #[cfg(debug_assertions)]
    pub fn custody_decision_journal_path(&self) -> &std::path::Path {
        self.decision_delivery.journal_path()
    }

    #[cfg(debug_assertions)]
    pub fn inject_next_custody_journal_sync_failure_for_debug(&self) {
        self.decision_delivery.inject_next_sync_failure_for_debug();
    }

    #[cfg(debug_assertions)]
    pub fn inject_next_custody_journal_partial_write_failure_for_debug(&self) {
        self.decision_delivery
            .inject_next_partial_write_failure_for_debug();
    }

    #[cfg(debug_assertions)]
    pub fn inject_next_custody_journal_directory_sync_failure_for_debug(&self) {
        self.decision_delivery
            .inject_next_directory_sync_failure_for_debug();
    }
}

fn drain_pending_decisions(
    store: &mut ParentPresenceStore,
    delivery: &ParentPresenceDecisionDelivery,
) -> Result<(), ParentPresenceStorageFailureReason> {
    store
        .deliver_pending(|decision| delivery.append_pending(decision))
        .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)
}
