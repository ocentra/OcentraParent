use ocentra_family_identity_core::household_authority::{
    requires_parent_step_up, validate_parent_step_up_assertion, HouseholdAuthorityAction,
    ParentStepUpAssertionSnapshot, ParentStepUpValidationFailureReason,
    ParentStepUpValidationInput,
};

const FAMILY_ID: &str = "family-1";
const PARENT_ACCOUNT_ID: &str = "parent-1";
const ACTION_DEVICE_ID: &str = "device-1";
const TARGET_CHILD_PROFILE_ID: &str = "child-1";
const NONCE: &str = "nonce-1";
const ACTION: HouseholdAuthorityAction = HouseholdAuthorityAction::PairChildDevice;

fn assertion() -> ParentStepUpAssertionSnapshot {
    ParentStepUpAssertionSnapshot {
        family_id: FAMILY_ID.to_owned(),
        parent_account_id: PARENT_ACCOUNT_ID.to_owned(),
        action_device_id: ACTION_DEVICE_ID.to_owned(),
        action_device_child_profile_id: None,
        target_child_profile_id: Some(TARGET_CHILD_PROFILE_ID.to_owned()),
        action: ACTION,
        nonce: NONCE.to_owned(),
        expires_at: "2026-08-05T23:04:59.000Z".to_owned(),
    }
}

fn validation_input(
    assertion: Option<ParentStepUpAssertionSnapshot>,
    expected_nonce: Option<&str>,
    observed_at: &str,
) -> ParentStepUpValidationInput {
    ParentStepUpValidationInput {
        assertion,
        family_id: FAMILY_ID.to_owned(),
        parent_account_id: PARENT_ACCOUNT_ID.to_owned(),
        action_device_id: ACTION_DEVICE_ID.to_owned(),
        action_device_child_profile_id: None,
        target_child_profile_id: Some(TARGET_CHILD_PROFILE_ID.to_owned()),
        action: ACTION,
        observed_at: observed_at.to_owned(),
        expected_nonce: expected_nonce.map(str::to_owned),
    }
}

#[test]
fn privileged_actions_require_an_explicit_parent_step_up() {
    assert!(requires_parent_step_up(ACTION));
    assert!(!requires_parent_step_up(
        HouseholdAuthorityAction::ViewChildStatus
    ));
    assert_eq!(
        validate_parent_step_up_assertion(&validation_input(
            None,
            Some(NONCE),
            "2026-08-05T23:00:01.000Z",
        ))
        .failure_reason,
        Some(ParentStepUpValidationFailureReason::Required)
    );
}

#[test]
fn a_matching_unexpired_assertion_is_valid() {
    let decision = validate_parent_step_up_assertion(&validation_input(
        Some(assertion()),
        Some(NONCE),
        "2026-08-05T23:00:01.000Z",
    ));

    assert!(decision.valid);
    assert_eq!(decision.failure_reason, None);
}

#[test]
fn expired_assertions_are_rejected_at_the_exact_boundary() {
    let decision = validate_parent_step_up_assertion(&validation_input(
        Some(assertion()),
        Some(NONCE),
        "2026-08-05T23:04:59.000Z",
    ));

    assert!(!decision.valid);
    assert_eq!(
        decision.failure_reason,
        Some(ParentStepUpValidationFailureReason::Expired)
    );
}

#[test]
fn a_replayed_nonce_is_rejected_before_authorization() {
    let decision = validate_parent_step_up_assertion(&validation_input(
        Some(assertion()),
        Some("different-nonce"),
        "2026-08-05T23:00:01.000Z",
    ));

    assert!(!decision.valid);
    assert_eq!(
        decision.failure_reason,
        Some(ParentStepUpValidationFailureReason::ReplayRejected)
    );
}
