use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use ocentra_eventing::ids::CorrelationId;
use ocentra_family_identity_core::household_authority::{
    HouseholdAuthorityAction, ParentStepUpAssertionSnapshot,
};
use ocentra_family_identity_core::parent_presence::{
    ParentPresenceChallenge, ParentPresenceChallengeIssuanceFailureReason,
    ParentPresenceStorageFailureReason, ParentPresenceVerificationFailureReason,
    ParentPresenceVerificationInput, ParentPresenceVerificationPort,
};

use super::open_parent_presence_test_port;

const EXPIRED_EXPIRY: &str = "2000-01-01T00:00:00.000Z";
const ACCEPTED_EXPIRY: &str = "2099-01-01T00:00:00.000Z";
const LATER_ACCEPTED_EXPIRY: &str = "2099-01-01T00:05:00.000Z";
static NEXT_CASE_ID: AtomicU64 = AtomicU64::new(1);

struct TestCase {
    challenge_ref: String,
    nonce_ref: String,
    family_id: String,
    parent_account_id: String,
    action_device_id: String,
    action_device_child_profile_id: Option<String>,
    target_child_profile_id: Option<String>,
}

fn case(prefix: &str) -> TestCase {
    let id = NEXT_CASE_ID.fetch_add(1, Ordering::Relaxed);
    let scope = format!("{prefix}-{id}");
    TestCase {
        challenge_ref: format!("{scope}-challenge"),
        nonce_ref: format!("{scope}-nonce"),
        family_id: format!("{scope}-family"),
        parent_account_id: format!("{scope}-parent"),
        action_device_id: format!("{scope}-device"),
        action_device_child_profile_id: Some(format!("{scope}-action-child")),
        target_child_profile_id: Some(format!("{scope}-target-child")),
    }
}
fn challenge(case: &TestCase, expires_at: &str) -> ParentPresenceChallenge {
    ParentPresenceChallenge {
        challenge_ref: case.challenge_ref.clone(),
        nonce_ref: case.nonce_ref.clone(),
        family_id: case.family_id.clone(),
        parent_account_id: case.parent_account_id.clone(),
        privileged_action: HouseholdAuthorityAction::PairChildDevice,
        action_device_id: case.action_device_id.clone(),
        action_device_child_profile_id: case.action_device_child_profile_id.clone(),
        target_child_profile_id: case.target_child_profile_id.clone(),
        target_child_device_id: None,
        expires_at: expires_at.to_owned(),
    }
}
fn input(
    case: &TestCase,
    expires_at: &str,
) -> Result<ParentPresenceVerificationInput, ParentPresenceStorageFailureReason> {
    Ok(ParentPresenceVerificationInput {
        correlation_id: CorrelationId::parse("parent-presence-expiry-correlation")
            .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?,
        challenge_ref: case.challenge_ref.clone(),
        assertion: ParentStepUpAssertionSnapshot {
            family_id: case.family_id.clone(),
            parent_account_id: case.parent_account_id.clone(),
            action_device_id: case.action_device_id.clone(),
            action_device_child_profile_id: case.action_device_child_profile_id.clone(),
            target_child_profile_id: case.target_child_profile_id.clone(),
            target_child_device_id: None,
            action: HouseholdAuthorityAction::PairChildDevice,
            nonce: case.nonce_ref.clone(),
            expires_at: expires_at.to_owned(),
        },
    })
}
fn port(
    prefix: &str,
) -> Result<(PathBuf, ParentPresenceVerificationPort), ParentPresenceStorageFailureReason> {
    let root = std::env::temp_dir().join(format!(
        "ocentra-parent-presence-expiry-{prefix}-{}",
        NEXT_CASE_ID.fetch_add(1, Ordering::Relaxed)
    ));
    fs::create_dir_all(&root)
        .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;
    let path = root.join("parent-presence.sqlite");
    open_parent_presence_test_port(&path).map(|port| (root, port))
}

fn port_at(
    prefix: &str,
    observed_at: &str,
) -> Result<(PathBuf, ParentPresenceVerificationPort), ParentPresenceStorageFailureReason> {
    let root = std::env::temp_dir().join(format!(
        "ocentra-parent-presence-expiry-at-{prefix}-{}",
        NEXT_CASE_ID.fetch_add(1, Ordering::Relaxed)
    ));
    fs::create_dir_all(&root)
        .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;
    let path = root.join("parent-presence.sqlite");
    ParentPresenceVerificationPort::open_unsealed_test_custody_at(&path, observed_at)
        .map(|port| (root, port))
}

#[test]
fn parent_presence_issuance_rejects_invalid_expiry_without_reserving_identities(
) -> Result<(), ParentPresenceStorageFailureReason> {
    for invalid in [
        "not-a-timestamp",
        "2099-01-01T00:00:00Z",
        "2099-01-01T00:00:00.000-04:00",
    ] {
        let case = case("invalid-issuance");
        let (root, mut port) = port("invalid-issuance")?;
        assert_eq!(
            port.issue_challenge(challenge(&case, invalid)),
            Err(ParentPresenceChallengeIssuanceFailureReason::TimestampInvalid)
        );
        assert_eq!(
            port.issue_challenge(challenge(&case, ACCEPTED_EXPIRY)),
            Ok(())
        );
        assert!(port
            .verify_and_consume(input(&case, ACCEPTED_EXPIRY)?)
            .is_ok());
        drop(port);
        let _cleanup = fs::remove_dir_all(root);
    }
    assert_eq!(
        serde_json::to_string(&ParentPresenceChallengeIssuanceFailureReason::TimestampInvalid)
            .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?,
        "\"timestamp-invalid\""
    );
    Ok(())
}

#[test]
fn parent_presence_verification_accepts_independently_valid_deadlines_and_rejects_each_expired_deadline(
) -> Result<(), ParentPresenceStorageFailureReason> {
    let earlier_assertion = case("earlier-assertion");
    let (root, mut port) = port("independent-expiry")?;
    assert_eq!(
        port.issue_challenge(challenge(&earlier_assertion, LATER_ACCEPTED_EXPIRY)),
        Ok(())
    );
    assert!(port
        .verify_and_consume(input(&earlier_assertion, ACCEPTED_EXPIRY)?)
        .is_ok());
    let earlier_challenge = case("earlier-challenge");
    assert_eq!(
        port.issue_challenge(challenge(&earlier_challenge, ACCEPTED_EXPIRY)),
        Ok(())
    );
    assert!(port
        .verify_and_consume(input(&earlier_challenge, LATER_ACCEPTED_EXPIRY)?)
        .is_ok());
    let expired_assertion = case("expired-assertion");
    assert_eq!(
        port.issue_challenge(challenge(&expired_assertion, ACCEPTED_EXPIRY)),
        Ok(())
    );
    assert_eq!(
        port.verify_and_consume(input(&expired_assertion, EXPIRED_EXPIRY)?),
        Err(ParentPresenceVerificationFailureReason::Expired)
    );
    assert!(port
        .verify_and_consume(input(&expired_assertion, ACCEPTED_EXPIRY)?)
        .is_ok());
    let expired_challenge = case("expired-challenge");
    assert_eq!(
        port.issue_challenge(challenge(&expired_challenge, EXPIRED_EXPIRY)),
        Ok(())
    );
    assert_eq!(
        port.verify_and_consume(input(&expired_challenge, ACCEPTED_EXPIRY)?),
        Err(ParentPresenceVerificationFailureReason::Expired)
    );
    drop(port);
    let _cleanup = fs::remove_dir_all(root);
    Ok(())
}

#[test]
fn parent_presence_verification_rejects_challenge_at_exact_expiry(
) -> Result<(), ParentPresenceStorageFailureReason> {
    let case = case("challenge-equality");
    let (root, mut port) = port_at("challenge-equality", ACCEPTED_EXPIRY)?;
    assert_eq!(
        port.issue_challenge(challenge(&case, ACCEPTED_EXPIRY)),
        Ok(())
    );
    assert_eq!(
        port.verify_and_consume(input(&case, LATER_ACCEPTED_EXPIRY)?),
        Err(ParentPresenceVerificationFailureReason::Expired)
    );
    drop(port);
    let _cleanup = fs::remove_dir_all(root);
    Ok(())
}

#[test]
fn parent_presence_verification_rejects_assertion_at_exact_expiry(
) -> Result<(), ParentPresenceStorageFailureReason> {
    let case = case("assertion-equality");
    let (root, mut port) = port_at("assertion-equality", ACCEPTED_EXPIRY)?;
    assert_eq!(
        port.issue_challenge(challenge(&case, LATER_ACCEPTED_EXPIRY)),
        Ok(())
    );
    assert_eq!(
        port.verify_and_consume(input(&case, ACCEPTED_EXPIRY)?),
        Err(ParentPresenceVerificationFailureReason::Expired)
    );
    drop(port);
    let _cleanup = fs::remove_dir_all(root);
    Ok(())
}
