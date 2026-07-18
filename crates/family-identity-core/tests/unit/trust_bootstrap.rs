use ocentra_family_identity_core::household_authority::{
    HouseholdAuthorityAction, ParentStepUpAssertionSnapshot,
};
use ocentra_family_identity_core::parent_presence::{
    ParentPresenceChallenge, ParentPresenceObservedAt, ParentPresenceReceiptRef,
    ParentPresenceVerificationFailureReason, ParentPresenceVerificationInput,
    ParentPresenceVerificationPort,
};
use ocentra_family_identity_core::trust_bootstrap::{
    evaluate_trust_bootstrap, TrustBootstrapDecision, TrustBootstrapInput,
    TrustBootstrapLifecycleIntent,
};

const CHALLENGE_REF: &str = "parent-presence-challenge-1";
const NONCE_REF: &str = "parent-presence-nonce-1";

fn expected_receipt_ref() -> ParentPresenceReceiptRef {
    ParentPresenceReceiptRef::from_static(b"parent-presence-receipt:parent-presence-challenge-1")
}

fn expected_observed_at() -> ParentPresenceObservedAt {
    ParentPresenceObservedAt::from_static(b"2026-07-18T12:04:00.000Z")
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
        expires_at: "2026-07-18T12:05:00.000Z".to_owned(),
    }
}

fn issued_port() -> ParentPresenceVerificationPort {
    let mut port = ParentPresenceVerificationPort::new();
    port.issue_challenge(parent_presence_challenge());
    port
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
        expires_at: "2026-07-18T12:05:00.000Z".to_owned(),
    }
}

#[test]
fn parent_presence_verification_is_one_time_and_redacted() {
    let mut port = issued_port();

    let accepted = port
        .verify_and_consume(ParentPresenceVerificationInput {
            challenge_ref: CHALLENGE_REF.to_owned(),
            assertion: valid_assertion(),
            observed_at: "2026-07-18T12:04:00.000Z".to_owned(),
        })
        .expect("challenge should verify once");

    assert_eq!(accepted.receipt_ref(), expected_receipt_ref());
    assert_eq!(accepted.assertion_snapshot(), &valid_assertion());
    assert_eq!(accepted.observed_at(), expected_observed_at());

    let replay = port.verify_and_consume(ParentPresenceVerificationInput {
        challenge_ref: CHALLENGE_REF.to_owned(),
        assertion: valid_assertion(),
        observed_at: "2026-07-18T12:04:30.000Z".to_owned(),
    });

    assert_eq!(
        replay,
        Err(ParentPresenceVerificationFailureReason::ReplayRejected)
    );
}

#[test]
fn parent_presence_verification_rejects_binding_mismatches_without_consuming() {
    let mut port = issued_port();

    let household_mismatch = port.verify_and_consume(ParentPresenceVerificationInput {
        challenge_ref: CHALLENGE_REF.to_owned(),
        assertion: ParentStepUpAssertionSnapshot {
            family_id: "family-2".to_owned(),
            ..valid_assertion()
        },
        observed_at: "2026-07-18T12:04:00.000Z".to_owned(),
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
            observed_at: "2026-07-18T12:04:00.000Z".to_owned(),
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
        observed_at: "2026-07-18T12:04:00.000Z".to_owned(),
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
        observed_at: "2026-07-18T12:04:00.000Z".to_owned(),
    });
    assert_eq!(
        target_mismatch,
        Err(ParentPresenceVerificationFailureReason::TargetChildProfileMismatch)
    );

    let accepted = port
        .verify_and_consume(ParentPresenceVerificationInput {
            challenge_ref: CHALLENGE_REF.to_owned(),
            assertion: valid_assertion(),
            observed_at: "2026-07-18T12:04:00.000Z".to_owned(),
        })
        .expect("challenge should still be available after mismatches");
    assert_eq!(accepted.receipt_ref(), expected_receipt_ref());
}

#[test]
fn parent_presence_verification_rejects_expired_and_replayed_challenges() {
    let mut port = issued_port();

    let expired = port.verify_and_consume(ParentPresenceVerificationInput {
        challenge_ref: CHALLENGE_REF.to_owned(),
        assertion: ParentStepUpAssertionSnapshot {
            expires_at: "2026-07-18T12:05:00.000Z".to_owned(),
            ..valid_assertion()
        },
        observed_at: "2026-07-18T12:05:01.000Z".to_owned(),
    });
    assert_eq!(
        expired,
        Err(ParentPresenceVerificationFailureReason::Expired)
    );

    let accepted = port
        .verify_and_consume(ParentPresenceVerificationInput {
            challenge_ref: CHALLENGE_REF.to_owned(),
            assertion: valid_assertion(),
            observed_at: "2026-07-18T12:04:00.000Z".to_owned(),
        })
        .expect("challenge should be accepted before replay");
    assert_eq!(accepted.receipt_ref(), expected_receipt_ref());

    let replay = port.verify_and_consume(ParentPresenceVerificationInput {
        challenge_ref: CHALLENGE_REF.to_owned(),
        assertion: valid_assertion(),
        observed_at: "2026-07-18T12:04:30.000Z".to_owned(),
    });
    assert_eq!(
        replay,
        Err(ParentPresenceVerificationFailureReason::ReplayRejected)
    );
}

#[test]
fn trust_bootstrap_returns_awaiting_platform_key_sealing() {
    let mut port = issued_port();
    let accepted = port
        .verify_and_consume(ParentPresenceVerificationInput {
            challenge_ref: CHALLENGE_REF.to_owned(),
            assertion: valid_assertion(),
            observed_at: "2026-07-18T12:04:00.000Z".to_owned(),
        })
        .expect("parent presence should verify");

    assert_eq!(accepted.receipt_ref(), expected_receipt_ref());
    assert_eq!(accepted.assertion_snapshot(), &valid_assertion());
    assert_eq!(accepted.observed_at(), expected_observed_at());

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
    let mut port = issued_port();
    let rejected = port.verify_and_consume(ParentPresenceVerificationInput {
        challenge_ref: CHALLENGE_REF.to_owned(),
        assertion: ParentStepUpAssertionSnapshot {
            nonce: "wrong-nonce".to_owned(),
            ..valid_assertion()
        },
        observed_at: "2026-07-18T12:04:00.000Z".to_owned(),
    });

    assert_eq!(
        rejected,
        Err(ParentPresenceVerificationFailureReason::NonceMismatch)
    );
}
