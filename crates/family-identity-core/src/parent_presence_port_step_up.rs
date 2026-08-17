use crate::parent_presence::{
    ParentPresenceChallenge, ParentPresenceChallengeIssuanceFailureReason, ParentPresenceReceiptRef,
};
use crate::parent_presence_port::ParentPresenceVerificationPort;
use crate::parent_presence_store::{
    ParentPresenceStoreError, ParentPresenceStoreIssueError, StoredParentStepUpIntent,
};

impl ParentPresenceVerificationPort {
    pub(crate) fn issue_challenge_with_parent_step_up_intent(
        &mut self,
        challenge: ParentPresenceChallenge,
        intent: StoredParentStepUpIntent,
    ) -> Result<(), ParentPresenceChallengeIssuanceFailureReason> {
        self.store
            .issue_challenge_with_parent_step_up_intent(challenge, intent)
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

    pub(crate) fn parent_step_up_intent(
        &self,
        challenge_ref: &str,
    ) -> Result<Option<StoredParentStepUpIntent>, ParentPresenceStoreError> {
        self.store.parent_step_up_intent(challenge_ref)
    }

    pub(crate) fn complete_parent_step_up_registration(
        &mut self,
        challenge_ref: &str,
    ) -> Result<(), ParentPresenceStoreError> {
        self.store
            .complete_parent_step_up_registration(challenge_ref)
    }

    pub(crate) fn consumed_receipt_ref(
        &self,
        challenge_ref: &str,
    ) -> Result<Option<ParentPresenceReceiptRef>, ParentPresenceStoreError> {
        self.store.consumed_receipt_ref(challenge_ref)
    }
}
