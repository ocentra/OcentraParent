use std::fmt;

use ocentra_family_identity_core::household_authority::{
    HouseholdAuthorityAction, ParentStepUpAssertionSnapshot,
};
use ocentra_family_identity_core::parent_presence::{
    ParentPresenceChallenge, ParentPresenceChallengeIssuanceFailureReason,
    ParentPresenceObservedAt, ParentPresenceVerificationFailureReason,
    ParentPresenceVerificationInput, ParentPresenceVerificationPort,
};
use ocentra_family_identity_core::trust_bootstrap::{
    evaluate_trust_bootstrap, TrustBootstrapDecision, TrustBootstrapInput,
    TrustBootstrapLifecycleIntent,
};

const CHALLENGE_REF: &str = "parent-presence-challenge-1";
const NONCE_REF: &str = "parent-presence-nonce-1";
const BOUNDARY_EXPIRY: &str = "2026-07-18T12:05:00.000Z";
const BEFORE_EXPIRY: &str = "2026-07-18T12:04:00.000Z";
const AFTER_EXPIRY: &str = "2026-07-18T12:05:01.000Z";

fn expected_observed_at() -> ParentPresenceObservedAt {
    ParentPresenceObservedAt::from_canonical_utc("2026-07-18T12:04:00.000Z")
        .expect("expected observed-at instant should be canonical")
}

fn expected_boundary_observed_at() -> ParentPresenceObservedAt {
    ParentPresenceObservedAt::from_canonical_utc(BOUNDARY_EXPIRY)
        .expect("boundary instant should be canonical")
}

fn valid_assertion() -> ParentStepUpAssertionSnapshot {
    ParentStepUpAssertionSnapshot {
        family_id: "family-1".to_owned(),
        parent_account_id: "parent-account-1".to_owned(),
        action_device_id: "parent-device-1".to_owned(),
        action_device_child_profile_id: Some("action-device-child-profile-1".to_owned()),
        target_child_profile_id: Some("child-profile-1".to_owned()),
        action: HouseholdAuthorityAction::PairChildDevice,
        nonce: NONCE_REF.to_owned(),
        expires_at: BOUNDARY_EXPIRY.to_owned(),
    }
}

fn verification_input() -> ParentPresenceVerificationInput {
    ParentPresenceVerificationInput {
        challenge_ref: CHALLENGE_REF.to_owned(),
        assertion: valid_assertion(),
    }
}

fn port_at(now: &'static str) -> ParentPresenceVerificationPort {
    let observed_at = ParentPresenceObservedAt::from_canonical_utc(now)
        .expect("test clock instant should be canonical");
    ParentPresenceVerificationPort::with_clock(move || observed_at.clone())
}

fn parent_presence_challenge() -> ParentPresenceChallenge {
    ParentPresenceChallenge {
        challenge_ref: CHALLENGE_REF.to_owned(),
        nonce_ref: NONCE_REF.to_owned(),
        family_id: "family-1".to_owned(),
        parent_account_id: "parent-account-1".to_owned(),
        privileged_action: HouseholdAuthorityAction::PairChildDevice,
        action_device_id: "parent-device-1".to_owned(),
        action_device_child_profile_id: Some("action-device-child-profile-1".to_owned()),
        target_child_profile_id: Some("child-profile-1".to_owned()),
        expires_at: BOUNDARY_EXPIRY.to_owned(),
    }
}

fn issue_valid_challenge(port: &mut ParentPresenceVerificationPort) {
    port.issue_challenge(parent_presence_challenge())
        .expect("challenge should be issued once");
}

fn assert_debug_redacted<T: fmt::Debug>(value: &T) {
    let debug = format!("{value:?}");
    for secret in [
        "family-1",
        "parent-account-1",
        "parent-device-1",
        "action-device-child-profile-1",
        "child-profile-1",
        "nonce-1",
        "PairChildDevice",
    ] {
        assert!(
            !debug.contains(secret),
            "debug output leaked {secret}: {debug}"
        );
    }
}

#[test]
fn parent_presence_verification_is_one_time_and_redacted() {
    let mut port = port_at(BEFORE_EXPIRY);
    issue_valid_challenge(&mut port);

    let accepted = port
        .verify_and_consume(verification_input())
        .expect("challenge should verify once");

    assert_eq!(
        accepted.receipt_ref().to_string(),
        "parent-presence-receipt:parent-presence-challenge-1"
    );
    assert_eq!(accepted.assertion_snapshot(), &valid_assertion());
    assert_eq!(accepted.observed_at(), expected_observed_at());
    assert_debug_redacted(&accepted);
    assert_debug_redacted(&parent_presence_challenge());

    let replay = port.verify_and_consume(verification_input());
    assert_eq!(
        replay,
        Err(ParentPresenceVerificationFailureReason::ReplayRejected)
    );
}

#[test]
fn parent_presence_verification_rejects_binding_mismatches_without_consuming() {
    let mut port = port_at(BEFORE_EXPIRY);
    issue_valid_challenge(&mut port);

    let household_mismatch = port.verify_and_consume(ParentPresenceVerificationInput {
        challenge_ref: CHALLENGE_REF.to_owned(),
        assertion: ParentStepUpAssertionSnapshot {
            family_id: "family-2".to_owned(),
            ..valid_assertion()
        },
    });
    assert_eq!(
        household_mismatch,
        Err(ParentPresenceVerificationFailureReason::HouseholdMismatch)
    );

    let action_device_child_profile_mismatch =
        port.verify_and_consume(ParentPresenceVerificationInput {
            challenge_ref: CHALLENGE_REF.to_owned(),
            assertion: ParentStepUpAssertionSnapshot {
                action_device_child_profile_id: None,
                ..valid_assertion()
            },
        });
    assert_eq!(
        action_device_child_profile_mismatch,
        Err(ParentPresenceVerificationFailureReason::ActionDeviceChildProfileMismatch)
    );

    let action_device_mismatch = port.verify_and_consume(ParentPresenceVerificationInput {
        challenge_ref: CHALLENGE_REF.to_owned(),
        assertion: ParentStepUpAssertionSnapshot {
            action_device_id: "parent-device-2".to_owned(),
            ..valid_assertion()
        },
    });
    assert_eq!(
        action_device_mismatch,
        Err(ParentPresenceVerificationFailureReason::ActionDeviceMismatch)
    );

    let target_mismatch = port.verify_and_consume(ParentPresenceVerificationInput {
        challenge_ref: CHALLENGE_REF.to_owned(),
        assertion: ParentStepUpAssertionSnapshot {
            target_child_profile_id: Some("child-profile-2".to_owned()),
            ..valid_assertion()
        },
    });
    assert_eq!(
        target_mismatch,
        Err(ParentPresenceVerificationFailureReason::TargetChildProfileMismatch)
    );

    let accepted = port
        .verify_and_consume(verification_input())
        .expect("challenge should still be available after mismatches");
    assert_eq!(
        accepted.receipt_ref().to_string(),
        "parent-presence-receipt:parent-presence-challenge-1"
    );
}

#[test]
fn parent_presence_verification_rejects_duplicate_issuance_without_overwriting_original_binding() {
    let mut port = port_at(BEFORE_EXPIRY);
    issue_valid_challenge(&mut port);

    let mut duplicate = parent_presence_challenge();
    duplicate.family_id = "family-2".to_owned();
    duplicate.nonce_ref = "parent-presence-nonce-duplicate".to_owned();

    let duplicate_issue = port.issue_challenge(duplicate);
    assert_eq!(
        duplicate_issue,
        Err(ParentPresenceChallengeIssuanceFailureReason::DuplicateChallengeRef)
    );

    let accepted = port
        .verify_and_consume(verification_input())
        .expect("original binding should remain authoritative");
    assert_eq!(
        accepted.receipt_ref().to_string(),
        "parent-presence-receipt:parent-presence-challenge-1"
    );
}

#[test]
fn parent_presence_verification_rejects_expired_and_replayed_challenges() {
    let mut expired_port = port_at(AFTER_EXPIRY);
    issue_valid_challenge(&mut expired_port);

    let expired = expired_port.verify_and_consume(verification_input());
    assert_eq!(
        expired,
        Err(ParentPresenceVerificationFailureReason::Expired)
    );

    let mut replay_port = port_at(BEFORE_EXPIRY);
    issue_valid_challenge(&mut replay_port);

    let accepted = replay_port
        .verify_and_consume(verification_input())
        .expect("challenge should be accepted before replay");
    assert_eq!(
        accepted.observed_at(),
        ParentPresenceObservedAt::from_canonical_utc(BEFORE_EXPIRY)
            .expect("clock instant should be canonical")
    );

    let replay = replay_port.verify_and_consume(verification_input());
    assert_eq!(
        replay,
        Err(ParentPresenceVerificationFailureReason::ReplayRejected)
    );
}

#[test]
fn parent_presence_verification_accepts_boundary_equality_and_rejects_after_expiry() {
    let mut boundary_port = port_at(BOUNDARY_EXPIRY);
    issue_valid_challenge(&mut boundary_port);

    let accepted = boundary_port
        .verify_and_consume(verification_input())
        .expect("challenge should be accepted at the expiry boundary");
    assert_eq!(accepted.observed_at(), expected_boundary_observed_at());

    let mut after_boundary_port = port_at(AFTER_EXPIRY);
    issue_valid_challenge(&mut after_boundary_port);
    let expired = after_boundary_port.verify_and_consume(verification_input());
    assert_eq!(
        expired,
        Err(ParentPresenceVerificationFailureReason::Expired)
    );
}

#[test]
fn parent_presence_verification_rejects_malformed_noncanonical_and_offset_timestamps() {
    let mut malformed_port = port_at(BEFORE_EXPIRY);
    issue_valid_challenge(&mut malformed_port);
    let malformed = malformed_port.verify_and_consume(ParentPresenceVerificationInput {
        challenge_ref: CHALLENGE_REF.to_owned(),
        assertion: ParentStepUpAssertionSnapshot {
            expires_at: "not-a-timestamp".to_owned(),
            ..valid_assertion()
        },
    });
    assert_eq!(
        malformed,
        Err(ParentPresenceVerificationFailureReason::TimestampInvalid)
    );

    let mut noncanonical_port = port_at(BEFORE_EXPIRY);
    issue_valid_challenge(&mut noncanonical_port);
    let noncanonical = noncanonical_port.verify_and_consume(ParentPresenceVerificationInput {
        challenge_ref: CHALLENGE_REF.to_owned(),
        assertion: ParentStepUpAssertionSnapshot {
            expires_at: "2026-07-18T12:05:00Z".to_owned(),
            ..valid_assertion()
        },
    });
    assert_eq!(
        noncanonical,
        Err(ParentPresenceVerificationFailureReason::TimestampInvalid)
    );

    let mut offset_port = port_at(BEFORE_EXPIRY);
    issue_valid_challenge(&mut offset_port);
    let offset = offset_port.verify_and_consume(ParentPresenceVerificationInput {
        challenge_ref: CHALLENGE_REF.to_owned(),
        assertion: ParentStepUpAssertionSnapshot {
            expires_at: "2026-07-18T08:05:00.000-04:00".to_owned(),
            ..valid_assertion()
        },
    });
    assert_eq!(
        offset,
        Err(ParentPresenceVerificationFailureReason::TimestampInvalid)
    );
}

#[test]
fn trust_bootstrap_returns_awaiting_platform_key_sealing() {
    let mut port = port_at(BEFORE_EXPIRY);
    issue_valid_challenge(&mut port);
    let accepted = port
        .verify_and_consume(verification_input())
        .expect("parent presence should verify");

    assert_eq!(
        accepted.receipt_ref().to_string(),
        "parent-presence-receipt:parent-presence-challenge-1"
    );
    assert_eq!(accepted.assertion_snapshot(), &valid_assertion());
    assert_eq!(accepted.observed_at(), expected_observed_at());
    assert_debug_redacted(&accepted);

    let decision = evaluate_trust_bootstrap(TrustBootstrapInput {
        trust_bootstrap_ref: "trust-bootstrap-1".to_owned(),
        device_trust_ref: "device-trust-1".to_owned(),
        lifecycle_intent: TrustBootstrapLifecycleIntent::SealParentDeviceTrust,
        parent_presence: accepted,
    });

    assert!(matches!(
        &decision,
        TrustBootstrapDecision::AwaitingPlatformKeySealing(_)
    ));
    if let TrustBootstrapDecision::AwaitingPlatformKeySealing(request) = decision {
        assert_eq!(request.trust_bootstrap_ref, "trust-bootstrap-1");
        assert_eq!(request.device_trust_ref, "device-trust-1");
        assert_eq!(
            request.lifecycle_intent,
            TrustBootstrapLifecycleIntent::SealParentDeviceTrust
        );
    }
}

#[test]
fn parent_presence_verification_rejects_tampered_nonce_via_port() {
    let mut port = port_at(BEFORE_EXPIRY);
    issue_valid_challenge(&mut port);
    let rejected = port.verify_and_consume(ParentPresenceVerificationInput {
        challenge_ref: CHALLENGE_REF.to_owned(),
        assertion: ParentStepUpAssertionSnapshot {
            nonce: "wrong-nonce".to_owned(),
            ..valid_assertion()
        },
    });

    assert_eq!(
        rejected,
        Err(ParentPresenceVerificationFailureReason::NonceMismatch)
    );
}
