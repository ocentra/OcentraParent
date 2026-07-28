use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use ocentra_eventing::ids::CorrelationId;
use ocentra_family_identity_core::household_authority::{
    HouseholdAuthorityAction, ParentStepUpAssertionSnapshot,
};
use ocentra_family_identity_core::parent_presence::{
    ParentPresenceChallenge, ParentPresenceStorageFailureReason,
    ParentPresenceVerificationFailureReason, ParentPresenceVerificationInput,
};

use super::open_parent_presence_test_port;

const EXPIRY: &str = "2099-01-01T00:00:00.000Z";
static NEXT_ID: AtomicU64 = AtomicU64::new(1);
type TestResult = Result<(), ParentPresenceStorageFailureReason>;

struct Store {
    root: PathBuf,
    path: PathBuf,
}

impl Store {
    fn new(prefix: &str) -> Self {
        let root = std::env::temp_dir().join(format!(
            "ocentra-parent-receipt-integrity-{prefix}-{}-{}",
            std::process::id(),
            NEXT_ID.fetch_add(1, Ordering::Relaxed)
        ));
        assert!(matches!(fs::create_dir_all(&root), Ok(())));
        let path = root.join("parent-presence.sqlite");
        Self { root, path }
    }
}

impl Drop for Store {
    fn drop(&mut self) {
        let _cleanup = fs::remove_dir_all(&self.root);
    }
}

fn challenge(scope: &str) -> ParentPresenceChallenge {
    ParentPresenceChallenge {
        challenge_ref: format!("{scope}-challenge"),
        nonce_ref: format!("{scope}-nonce"),
        family_id: format!("{scope}-family"),
        parent_account_id: format!("{scope}-parent"),
        privileged_action: HouseholdAuthorityAction::PairChildDevice,
        action_device_id: format!("{scope}-device"),
        action_device_child_profile_id: Some(format!("{scope}-action-child")),
        target_child_profile_id: Some(format!("{scope}-target-child")),
        target_child_device_id: None,
        expires_at: EXPIRY.to_owned(),
    }
}

fn input(
    scope: &str,
) -> Result<ParentPresenceVerificationInput, ParentPresenceStorageFailureReason> {
    Ok(ParentPresenceVerificationInput {
        correlation_id: CorrelationId::parse(format!("{scope}-correlation"))
            .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?,
        challenge_ref: format!("{scope}-challenge"),
        assertion: ParentStepUpAssertionSnapshot {
            family_id: format!("{scope}-family"),
            parent_account_id: format!("{scope}-parent"),
            action_device_id: format!("{scope}-device"),
            action_device_child_profile_id: Some(format!("{scope}-action-child")),
            target_child_profile_id: Some(format!("{scope}-target-child")),
            target_child_device_id: None,
            action: HouseholdAuthorityAction::PairChildDevice,
            nonce: format!("{scope}-nonce"),
            expires_at: EXPIRY.to_owned(),
        },
    })
}

#[test]
fn consumed_challenge_with_missing_receipt_is_integrity_rejected() -> TestResult {
    assert_consumed_receipt_tamper_rejected("missing-receipt", |connection, scope| {
        connection.execute(
            "DELETE FROM parent_presence_receipts WHERE challenge_ref = ?1",
            [format!("{scope}-challenge")],
        )
    })
}

#[test]
fn consumed_challenge_with_invalid_receipt_is_integrity_rejected() -> TestResult {
    assert_consumed_receipt_tamper_rejected("invalid-receipt", |connection, scope| {
        connection.execute(
            "UPDATE parent_presence_receipts SET receipt_ref = 'tampered' WHERE challenge_ref = ?1",
            [format!("{scope}-challenge")],
        )
    })
}

#[test]
fn consumed_challenge_rejects_duplicate_receipt_insertion() -> TestResult {
    let scope = "duplicate-receipt";
    let store = Store::new(scope);
    let mut port = open_parent_presence_test_port(&store.path)?;
    assert_eq!(port.issue_challenge(challenge(scope)), Ok(()));
    port.verify_and_consume(input(scope)?)
        .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;
    let connection = rusqlite::Connection::open(&store.path)
        .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;
    let duplicate = connection.execute(
        "INSERT INTO parent_presence_receipts(challenge_ref, receipt_ref) VALUES (?1, ?2)",
        rusqlite::params![
            format!("{scope}-challenge"),
            "parent-presence-receipt:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
        ],
    );
    assert!(matches!(
        duplicate,
        Err(rusqlite::Error::SqliteFailure(failure, _))
            if failure.code == rusqlite::ErrorCode::ConstraintViolation
    ));
    assert_eq!(
        port.verify_and_consume(input(scope)?),
        Err(ParentPresenceVerificationFailureReason::ReplayRejected)
    );
    Ok(())
}

fn assert_consumed_receipt_tamper_rejected(
    scope: &str,
    tamper: impl FnOnce(&rusqlite::Connection, &str) -> rusqlite::Result<usize>,
) -> TestResult {
    let store = Store::new(scope);
    let mut port = open_parent_presence_test_port(&store.path)?;
    assert_eq!(port.issue_challenge(challenge(scope)), Ok(()));
    port.verify_and_consume(input(scope)?)
        .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;
    let connection = rusqlite::Connection::open(&store.path)
        .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;
    assert_eq!(
        tamper(&connection, scope)
            .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?,
        1
    );
    assert_eq!(
        port.verify_and_consume(input(scope)?),
        Err(ParentPresenceVerificationFailureReason::CustodyIntegrityRejected)
    );
    Ok(())
}
