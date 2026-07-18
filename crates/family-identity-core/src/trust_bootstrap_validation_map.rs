use crate::household_authority::{
    ParentStepUpAssertionSnapshot, ParentStepUpValidationFailureReason,
};
use crate::parent_presence::{ParentPresenceChallenge, ParentPresenceVerificationFailureReason};

pub(crate) fn map_parent_presence_validation_failure_reason(
    failure_reason: ParentStepUpValidationFailureReason,
    challenge: &ParentPresenceChallenge,
    assertion: &ParentStepUpAssertionSnapshot,
) -> ParentPresenceVerificationFailureReason {
    match failure_reason {
        ParentStepUpValidationFailureReason::Expired => {
            ParentPresenceVerificationFailureReason::Expired
        }
        ParentStepUpValidationFailureReason::ReplayRejected => {
            ParentPresenceVerificationFailureReason::NonceMismatch
        }
        ParentStepUpValidationFailureReason::WrongHousehold => {
            ParentPresenceVerificationFailureReason::HouseholdMismatch
        }
        ParentStepUpValidationFailureReason::WrongAccount => {
            ParentPresenceVerificationFailureReason::ParentAccountMismatch
        }
        ParentStepUpValidationFailureReason::WrongAction => {
            ParentPresenceVerificationFailureReason::ActionMismatch
        }
        ParentStepUpValidationFailureReason::WrongDevice => {
            if challenge.action_device_child_profile_id != assertion.action_device_child_profile_id
            {
                ParentPresenceVerificationFailureReason::ActionDeviceChildProfileMismatch
            } else {
                ParentPresenceVerificationFailureReason::ActionDeviceMismatch
            }
        }
        ParentStepUpValidationFailureReason::WrongTarget => {
            ParentPresenceVerificationFailureReason::TargetChildProfileMismatch
        }
        ParentStepUpValidationFailureReason::Required => {
            ParentPresenceVerificationFailureReason::ChallengeNotIssued
        }
    }
}
