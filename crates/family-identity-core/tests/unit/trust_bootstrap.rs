use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};

use ocentra_family_identity_core::household_authority::{
    HouseholdAuthorityAction, ParentStepUpAssertionSnapshot,
};
use ocentra_family_identity_core::parent_presence::{
    ParentPresenceChallenge, ParentPresenceChallengeIssuanceFailureReason,
    ParentPresenceVerificationFailureReason, ParentPresenceVerificationInput,
    ParentPresenceVerificationPort,
};

const EXPIRED_EXPIRY: &str = "2000-01-01T00:00:00.000Z";
const ACCEPTED_EXPIRY: &str = "2099-01-01T00:00:00.000Z";

static NEXT_CASE_ID: AtomicU64 = AtomicU64::new(1);

#[derive(Clone)]
struct TestCase {
    challenge_ref: String,
    nonce_ref: String,
    family_id: String,
    parent_account_id: String,
    action_device_id: String,
    action_device_child_profile_id: Option<String>,
    target_child_profile_id: Option<String>,
}

fn test_case(prefix: &str) -> TestCase {
    let id = NEXT_CASE_ID.fetch_add(1, Ordering::Relaxed);
    let scope = format!("{prefix}-{id}");
    TestCase {
        challenge_ref: format!("{scope}-challenge"),
        nonce_ref: format!("{scope}-nonce"),
        family_id: format!("{scope}-family"),
        parent_account_id: format!("{scope}-parent-account"),
        action_device_id: format!("{scope}-device"),
        action_device_child_profile_id: Some(format!("{scope}-action-child")),
        target_child_profile_id: Some(format!("{scope}-target-child")),
    }
}

fn challenge_for(case: &TestCase, expires_at: &str) -> ParentPresenceChallenge {
    ParentPresenceChallenge {
        challenge_ref: case.challenge_ref.clone(),
        nonce_ref: case.nonce_ref.clone(),
        family_id: case.family_id.clone(),
        parent_account_id: case.parent_account_id.clone(),
        privileged_action: HouseholdAuthorityAction::PairChildDevice,
        action_device_id: case.action_device_id.clone(),
        action_device_child_profile_id: case.action_device_child_profile_id.clone(),
        target_child_profile_id: case.target_child_profile_id.clone(),
        expires_at: expires_at.to_owned(),
    }
}

fn assertion_for(case: &TestCase, expires_at: &str) -> ParentStepUpAssertionSnapshot {
    ParentStepUpAssertionSnapshot {
        family_id: case.family_id.clone(),
        parent_account_id: case.parent_account_id.clone(),
        action_device_id: case.action_device_id.clone(),
        action_device_child_profile_id: case.action_device_child_profile_id.clone(),
        target_child_profile_id: case.target_child_profile_id.clone(),
        action: HouseholdAuthorityAction::PairChildDevice,
        nonce: case.nonce_ref.clone(),
        expires_at: expires_at.to_owned(),
    }
}

fn verification_input(case: &TestCase, expires_at: &str) -> ParentPresenceVerificationInput {
    ParentPresenceVerificationInput {
        challenge_ref: case.challenge_ref.clone(),
        assertion: assertion_for(case, expires_at),
    }
}

fn issue_valid_challenge(
    port: &mut ParentPresenceVerificationPort,
    case: &TestCase,
    expires_at: &str,
) {
    assert_eq!(
        port.issue_challenge(challenge_for(case, expires_at)),
        Ok(())
    );
}

fn assert_redacted_debug<T: fmt::Debug>(value: &T, case: &TestCase) {
    let debug = format!("{value:?}");
    for secret in [
        case.challenge_ref.as_str(),
        case.nonce_ref.as_str(),
        case.family_id.as_str(),
        case.parent_account_id.as_str(),
        case.action_device_id.as_str(),
        case.action_device_child_profile_id
            .as_deref()
            .unwrap_or_default(),
        case.target_child_profile_id.as_deref().unwrap_or_default(),
        "PairChildDevice",
    ] {
        assert!(
            !debug.contains(secret),
            "debug output leaked {secret}: {debug}"
        );
    }
}

#[test]
fn parent_presence_verification_input_debug_is_redacted() {
    let case = test_case("input-debug");
    let input = verification_input(&case, ACCEPTED_EXPIRY);
    assert_eq!(
        format!("{input:?}"),
        "ParentPresenceVerificationInput { challenge_ref: \"[redacted]\", assertion: \"[redacted]\" }"
    );
}

#[test]
fn parent_presence_verification_is_one_time_and_redacted() {
    let case = test_case("one-time");
    let mut port = ParentPresenceVerificationPort::new();
    issue_valid_challenge(&mut port, &case, ACCEPTED_EXPIRY);

    let accepted = port.verify_and_consume(verification_input(&case, ACCEPTED_EXPIRY));
    assert_eq!(
        accepted
            .as_ref()
            .map(|accepted| accepted.assertion_snapshot()),
        Ok(&assertion_for(&case, ACCEPTED_EXPIRY))
    );
    if let Ok(accepted) = accepted {
        assert_redacted_debug(&accepted, &case);
    }
    assert_redacted_debug(&challenge_for(&case, ACCEPTED_EXPIRY), &case);

    assert_eq!(
        port.verify_and_consume(verification_input(&case, ACCEPTED_EXPIRY)),
        Err(ParentPresenceVerificationFailureReason::ReplayRejected)
    );
}

#[test]
fn parent_presence_verification_rejects_binding_mismatches_without_consuming() {
    let case = test_case("binding-mismatch");
    let mut port = ParentPresenceVerificationPort::new();
    issue_valid_challenge(&mut port, &case, ACCEPTED_EXPIRY);

    assert_eq!(
        port.verify_and_consume(ParentPresenceVerificationInput {
            challenge_ref: case.challenge_ref.clone(),
            assertion: ParentStepUpAssertionSnapshot {
                family_id: "wrong-family".to_owned(),
                ..assertion_for(&case, ACCEPTED_EXPIRY)
            },
        }),
        Err(ParentPresenceVerificationFailureReason::HouseholdMismatch)
    );

    assert_eq!(
        port.verify_and_consume(ParentPresenceVerificationInput {
            challenge_ref: case.challenge_ref.clone(),
            assertion: ParentStepUpAssertionSnapshot {
                parent_account_id: "wrong-parent".to_owned(),
                ..assertion_for(&case, ACCEPTED_EXPIRY)
            },
        }),
        Err(ParentPresenceVerificationFailureReason::ParentAccountMismatch)
    );

    assert_eq!(
        port.verify_and_consume(ParentPresenceVerificationInput {
            challenge_ref: case.challenge_ref.clone(),
            assertion: ParentStepUpAssertionSnapshot {
                action: HouseholdAuthorityAction::RevokeChildDevice,
                ..assertion_for(&case, ACCEPTED_EXPIRY)
            },
        }),
        Err(ParentPresenceVerificationFailureReason::ActionMismatch)
    );

    assert_eq!(
        port.verify_and_consume(ParentPresenceVerificationInput {
            challenge_ref: case.challenge_ref.clone(),
            assertion: ParentStepUpAssertionSnapshot {
                action_device_id: "wrong-device".to_owned(),
                ..assertion_for(&case, ACCEPTED_EXPIRY)
            },
        }),
        Err(ParentPresenceVerificationFailureReason::ActionDeviceMismatch)
    );

    assert_eq!(
        port.verify_and_consume(ParentPresenceVerificationInput {
            challenge_ref: case.challenge_ref.clone(),
            assertion: ParentStepUpAssertionSnapshot {
                action_device_child_profile_id: None,
                ..assertion_for(&case, ACCEPTED_EXPIRY)
            },
        }),
        Err(ParentPresenceVerificationFailureReason::ActionDeviceChildProfileMismatch)
    );

    assert_eq!(
        port.verify_and_consume(ParentPresenceVerificationInput {
            challenge_ref: case.challenge_ref.clone(),
            assertion: ParentStepUpAssertionSnapshot {
                target_child_profile_id: Some("wrong-target".to_owned()),
                ..assertion_for(&case, ACCEPTED_EXPIRY)
            },
        }),
        Err(ParentPresenceVerificationFailureReason::TargetChildProfileMismatch)
    );

    let accepted = port.verify_and_consume(verification_input(&case, ACCEPTED_EXPIRY));
    assert_eq!(
        accepted
            .as_ref()
            .map(|accepted| accepted.assertion_snapshot()),
        Ok(&assertion_for(&case, ACCEPTED_EXPIRY))
    );
    if let Ok(accepted) = accepted {
        assert_redacted_debug(&accepted, &case);
    }
}

#[test]
fn parent_presence_verification_rejects_duplicate_issuance_without_overwriting_original_binding() {
    let case = test_case("duplicate-issuance");
    let mut port = ParentPresenceVerificationPort::new();
    issue_valid_challenge(&mut port, &case, ACCEPTED_EXPIRY);

    let duplicate = ParentPresenceChallenge {
        family_id: "wrong-family".to_owned(),
        nonce_ref: "wrong-nonce".to_owned(),
        ..challenge_for(&case, ACCEPTED_EXPIRY)
    };

    assert_eq!(
        port.issue_challenge(duplicate),
        Err(ParentPresenceChallengeIssuanceFailureReason::DuplicateChallengeRef)
    );

    let accepted = port.verify_and_consume(verification_input(&case, ACCEPTED_EXPIRY));
    assert_eq!(
        accepted
            .as_ref()
            .map(|accepted| accepted.assertion_snapshot()),
        Ok(&assertion_for(&case, ACCEPTED_EXPIRY))
    );
    if let Ok(accepted) = accepted {
        assert_redacted_debug(&accepted, &case);
    }
}

#[test]
fn parent_presence_verification_rejects_expired_challenges_and_accepts_future_challenges() {
    let expired_case = test_case("expiry-clock-expired");
    let mut expired_port = ParentPresenceVerificationPort::new();
    issue_valid_challenge(&mut expired_port, &expired_case, EXPIRED_EXPIRY);
    assert_eq!(
        expired_port.verify_and_consume(verification_input(&expired_case, EXPIRED_EXPIRY)),
        Err(ParentPresenceVerificationFailureReason::Expired)
    );

    let accepted_case = test_case("expiry-clock-accepted");
    let mut accepted_port = ParentPresenceVerificationPort::new();
    issue_valid_challenge(&mut accepted_port, &accepted_case, ACCEPTED_EXPIRY);
    let accepted =
        accepted_port.verify_and_consume(verification_input(&accepted_case, ACCEPTED_EXPIRY));
    assert_eq!(
        accepted
            .as_ref()
            .map(|accepted| accepted.assertion_snapshot()),
        Ok(&assertion_for(&accepted_case, ACCEPTED_EXPIRY))
    );
    if let Ok(accepted) = accepted {
        assert_redacted_debug(&accepted, &accepted_case);
    }
}

#[test]
fn parent_presence_verification_rejects_malformed_noncanonical_and_offset_timestamps() {
    let malformed_case = test_case("malformed");
    let mut malformed_port = ParentPresenceVerificationPort::new();
    issue_valid_challenge(&mut malformed_port, &malformed_case, ACCEPTED_EXPIRY);
    assert_eq!(
        malformed_port.verify_and_consume(ParentPresenceVerificationInput {
            challenge_ref: malformed_case.challenge_ref.clone(),
            assertion: ParentStepUpAssertionSnapshot {
                expires_at: "not-a-timestamp".to_owned(),
                ..assertion_for(&malformed_case, ACCEPTED_EXPIRY)
            },
        }),
        Err(ParentPresenceVerificationFailureReason::TimestampInvalid)
    );

    let noncanonical_case = test_case("noncanonical");
    let mut noncanonical_port = ParentPresenceVerificationPort::new();
    issue_valid_challenge(&mut noncanonical_port, &noncanonical_case, ACCEPTED_EXPIRY);
    assert_eq!(
        noncanonical_port.verify_and_consume(ParentPresenceVerificationInput {
            challenge_ref: noncanonical_case.challenge_ref.clone(),
            assertion: ParentStepUpAssertionSnapshot {
                expires_at: "2099-01-01T00:00:00Z".to_owned(),
                ..assertion_for(&noncanonical_case, ACCEPTED_EXPIRY)
            },
        }),
        Err(ParentPresenceVerificationFailureReason::TimestampInvalid)
    );

    let offset_case = test_case("offset");
    let mut offset_port = ParentPresenceVerificationPort::new();
    issue_valid_challenge(&mut offset_port, &offset_case, ACCEPTED_EXPIRY);
    assert_eq!(
        offset_port.verify_and_consume(ParentPresenceVerificationInput {
            challenge_ref: offset_case.challenge_ref.clone(),
            assertion: ParentStepUpAssertionSnapshot {
                expires_at: "2099-01-01T00:00:00.000-04:00".to_owned(),
                ..assertion_for(&offset_case, ACCEPTED_EXPIRY)
            },
        }),
        Err(ParentPresenceVerificationFailureReason::TimestampInvalid)
    );
}
