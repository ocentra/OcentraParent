use crate::household_authority::{
    validate_parent_step_up_assertion, ParentStepUpAssertionSnapshot, ParentStepUpValidationInput,
};
use crate::parent_presence::{
    ParentPresenceChallenge, ParentPresenceObservedAt, ParentPresenceVerificationFailureReason,
};
use crate::trust_bootstrap_validation_map::map_parent_presence_validation_failure_reason;

pub(crate) fn parent_presence_verification_failure_reason(
    challenge: &ParentPresenceChallenge,
    assertion: &ParentStepUpAssertionSnapshot,
    observed_at: &ParentPresenceObservedAt,
) -> Option<ParentPresenceVerificationFailureReason> {
    let challenge_expires_at =
        match ParentPresenceObservedAt::from_canonical_utc(&challenge.expires_at) {
            Ok(value) => value,
            Err(_) => return Some(ParentPresenceVerificationFailureReason::TimestampInvalid),
        };
    let assertion_expires_at =
        match ParentPresenceObservedAt::from_canonical_utc(&assertion.expires_at) {
            Ok(value) => value,
            Err(_) => return Some(ParentPresenceVerificationFailureReason::TimestampInvalid),
        };

    if observed_at.epoch_millis >= challenge_expires_at.epoch_millis {
        return Some(ParentPresenceVerificationFailureReason::Expired);
    }

    if observed_at.epoch_millis >= assertion_expires_at.epoch_millis {
        return Some(ParentPresenceVerificationFailureReason::Expired);
    }

    let validation_input = ParentStepUpValidationInput {
        assertion: Some(assertion.clone()),
        family_id: challenge.family_id.clone(),
        parent_account_id: challenge.parent_account_id.clone(),
        action_device_id: challenge.action_device_id.clone(),
        action_device_child_profile_id: challenge.action_device_child_profile_id.clone(),
        target_child_profile_id: challenge.target_child_profile_id.clone(),
        action: challenge.privileged_action,
        observed_at: observed_at.to_string(),
        expected_nonce: Some(challenge.nonce_ref.clone()),
    };

    validate_parent_step_up_assertion(&validation_input)
        .failure_reason
        .map(|failure_reason| {
            map_parent_presence_validation_failure_reason(failure_reason, challenge, assertion)
        })
}
