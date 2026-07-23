use ocentra_eventing::ids::CorrelationId;

use crate::household_authority::ParentStepUpAssertionSnapshot;
use crate::parent_presence::{
    ParentPresenceCustodyDecisionArtifact, ParentPresenceCustodyDecisionResult,
    ParentPresenceObservedAt, ParentPresenceVerificationAccepted,
    ParentPresenceVerificationFailureReason,
};
use crate::parent_presence_store::{ConsumeChallengeResult, ParentPresenceStoreError};

pub(crate) fn finish_parent_presence_verification(
    correlation_id: CorrelationId,
    assertion: ParentStepUpAssertionSnapshot,
    observed_at: ParentPresenceObservedAt,
    consumed: Result<ConsumeChallengeResult, ParentPresenceStoreError>,
    accepted_artifact: ParentPresenceCustodyDecisionArtifact,
) -> (
    ParentPresenceCustodyDecisionArtifact,
    Result<ParentPresenceVerificationAccepted, ParentPresenceVerificationFailureReason>,
) {
    match consumed {
        Ok(ConsumeChallengeResult::Accepted(accepted)) => record_decision(
            accepted_artifact,
            Ok(ParentPresenceVerificationAccepted::new(
                accepted.receipt_ref,
                accepted.challenge,
                assertion,
                observed_at,
            )),
        ),
        Ok(ConsumeChallengeResult::Rejected(failure_reason)) => record_decision(
            ParentPresenceCustodyDecisionArtifact::new(
                correlation_id,
                rejection_artifact_result(failure_reason),
            ),
            Err(failure_reason),
        ),
        Err(error) => record_decision(
            ParentPresenceCustodyDecisionArtifact::new(
                correlation_id,
                store_artifact_result(error),
            ),
            Err(store_failure_reason(error)),
        ),
    }
}

fn record_decision(
    artifact: ParentPresenceCustodyDecisionArtifact,
    result: Result<ParentPresenceVerificationAccepted, ParentPresenceVerificationFailureReason>,
) -> (
    ParentPresenceCustodyDecisionArtifact,
    Result<ParentPresenceVerificationAccepted, ParentPresenceVerificationFailureReason>,
) {
    (artifact, result)
}

fn rejection_artifact_result(
    failure_reason: ParentPresenceVerificationFailureReason,
) -> ParentPresenceCustodyDecisionResult {
    if failure_reason == ParentPresenceVerificationFailureReason::ReplayRejected {
        ParentPresenceCustodyDecisionResult::ReplayRejected
    } else {
        ParentPresenceCustodyDecisionResult::Rejected
    }
}

fn store_artifact_result(error: ParentPresenceStoreError) -> ParentPresenceCustodyDecisionResult {
    match error {
        ParentPresenceStoreError::IntegrityRejected => {
            ParentPresenceCustodyDecisionResult::IntegrityRejected
        }
        ParentPresenceStoreError::Unavailable => {
            ParentPresenceCustodyDecisionResult::CustodyUnavailable
        }
    }
}

fn store_failure_reason(
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
