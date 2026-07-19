use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use ocentra_family_identity_core::household_authority::{
    HouseholdAuthorityAction, ParentStepUpAssertionSnapshot,
};
use ocentra_family_identity_core::parent_presence::{
    ParentPresenceChallenge, ParentPresenceChallengeIssuanceFailureReason,
    ParentPresenceStorageFailureReason, ParentPresenceVerificationFailureReason,
    ParentPresenceVerificationInput, ParentPresenceVerificationPort,
};

const EXPIRED_EXPIRY: &str = "2000-01-01T00:00:00.000Z";
const ACCEPTED_EXPIRY: &str = "2099-01-01T00:00:00.000Z";

static NEXT_CASE_ID: AtomicU64 = AtomicU64::new(1);

type TestResult = Result<(), ParentPresenceStorageFailureReason>;

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

struct TestStore {
    root: PathBuf,
    path: PathBuf,
}

impl TestStore {
    fn new(prefix: &str) -> Self {
        let id = NEXT_CASE_ID.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "ocentra-parent-presence-{prefix}-{}-{id}",
            std::process::id()
        ));
        assert!(matches!(fs::create_dir_all(&root), Ok(())));
        let path = root.join("parent-presence.sqlite");
        Self { root, path }
    }

    fn port(&self) -> Result<ParentPresenceVerificationPort, ParentPresenceStorageFailureReason> {
        ParentPresenceVerificationPort::open(&self.path)
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TestStore {
    fn drop(&mut self) {
        let _cleanup_result = fs::remove_dir_all(&self.root);
    }
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
fn parent_presence_verification_is_one_time_and_redacted() -> TestResult {
    let case = test_case("one-time");
    let store = TestStore::new("one-time");
    let mut port = store.port()?;
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
    Ok(())
}

#[test]
fn parent_presence_verification_rejects_binding_mismatches_without_consuming() -> TestResult {
    let case = test_case("binding-mismatch");
    let store = TestStore::new("binding-mismatch");
    let mut port = store.port()?;
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
    Ok(())
}

#[test]
fn parent_presence_verification_rejects_duplicate_issuance_without_overwriting_original_binding(
) -> TestResult {
    let case = test_case("duplicate-issuance");
    let store = TestStore::new("duplicate-issuance");
    let mut port = store.port()?;
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
    Ok(())
}

#[test]
fn parent_presence_nonce_identity_is_unique_across_challenge_refs_and_restart() -> TestResult {
    let first = test_case("nonce-first");
    let mut second = test_case("nonce-second");
    second.nonce_ref = first.nonce_ref.clone();
    let store = TestStore::new("nonce-identity");
    let mut issuer = store.port()?;
    issue_valid_challenge(&mut issuer, &first, ACCEPTED_EXPIRY);
    drop(issuer);

    let mut restarted = store.port()?;
    assert_eq!(
        restarted.issue_challenge(challenge_for(&second, ACCEPTED_EXPIRY)),
        Err(ParentPresenceChallengeIssuanceFailureReason::DuplicateNonceRef)
    );
    assert_eq!(
        restarted
            .verify_and_consume(verification_input(&first, ACCEPTED_EXPIRY))
            .as_ref()
            .map(|accepted| accepted.assertion_snapshot()),
        Ok(&assertion_for(&first, ACCEPTED_EXPIRY))
    );
    Ok(())
}

#[test]
fn parent_presence_receipt_is_unique_opaque_and_redacted() -> TestResult {
    let first = test_case("opaque-receipt-first");
    let second = test_case("opaque-receipt-second");
    let store = TestStore::new("opaque-receipt");
    let mut port = store.port()?;
    issue_valid_challenge(&mut port, &first, ACCEPTED_EXPIRY);
    issue_valid_challenge(&mut port, &second, ACCEPTED_EXPIRY);
    let first_accepted = port
        .verify_and_consume(verification_input(&first, ACCEPTED_EXPIRY))
        .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;
    let second_accepted = port
        .verify_and_consume(verification_input(&second, ACCEPTED_EXPIRY))
        .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;
    let first_ref = first_accepted.receipt_ref().to_string();
    let second_ref = second_accepted.receipt_ref().to_string();
    let first_entropy = first_ref.strip_prefix("parent-presence-receipt:");
    assert_eq!(first_entropy.map(str::len), Some(64));
    assert_eq!(
        first_entropy.map(|value| value.chars().all(|character| character.is_ascii_hexdigit())),
        Some(true)
    );
    assert_ne!(first_ref, second_ref);
    assert!(!format!("{first_accepted:?}").contains(&first_ref));
    Ok(())
}

#[test]
fn parent_presence_store_rejects_corruption_without_recreation() {
    let store = TestStore::new("corrupt-store");
    let corrupt = b"not-a-sqlite-database";
    assert!(matches!(fs::write(store.path(), corrupt), Ok(())));
    assert!(matches!(
        ParentPresenceVerificationPort::open(store.path()),
        Err(ParentPresenceStorageFailureReason::CustodyUnavailable)
    ));
    assert!(matches!(
        fs::read(store.path()),
        Ok(content) if content == corrupt
    ));
}

#[test]
fn parent_presence_store_rejects_legacy_receipt_schema_at_open() -> TestResult {
    let store = TestStore::new("legacy-receipt-schema");
    let connection = rusqlite::Connection::open(store.path())
        .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;
    connection
        .execute_batch(
            "CREATE TABLE parent_presence_challenges (challenge_ref TEXT PRIMARY KEY NOT NULL, challenge_json TEXT NOT NULL, privileged_action_json TEXT NOT NULL, expires_at TEXT NOT NULL, nonce_ref TEXT NOT NULL UNIQUE, lifecycle_state TEXT NOT NULL) STRICT;
             CREATE TABLE parent_presence_receipts (receipt_sequence INTEGER PRIMARY KEY AUTOINCREMENT, challenge_ref TEXT NOT NULL UNIQUE) STRICT;",
        )
        .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;
    drop(connection);
    assert!(matches!(
        ParentPresenceVerificationPort::open(store.path()),
        Err(ParentPresenceStorageFailureReason::CustodyUnavailable)
    ));
    Ok(())
}

#[test]
fn parent_presence_store_requires_existing_absolute_caller_custody_parent() {
    let relative = PathBuf::from("parent-presence-relative.sqlite");
    assert!(matches!(
        ParentPresenceVerificationPort::open(&relative),
        Err(ParentPresenceStorageFailureReason::CustodyUnavailable)
    ));
    assert!(!relative.exists());

    let missing_parent = std::env::temp_dir().join(format!(
        "ocentra-parent-presence-missing-parent-{}-{}",
        std::process::id(),
        NEXT_CASE_ID.fetch_add(1, Ordering::Relaxed)
    ));
    let path = missing_parent.join("nested").join("parent-presence.sqlite");
    assert!(matches!(
        ParentPresenceVerificationPort::open(&path),
        Err(ParentPresenceStorageFailureReason::CustodyUnavailable)
    ));
    assert!(!missing_parent.exists());
}

#[test]
fn parent_presence_store_rejects_read_only_database() -> TestResult {
    let store = TestStore::new("read-only-store");
    drop(store.port()?);
    let mut permissions = fs::metadata(store.path())
        .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?
        .permissions();
    permissions.set_readonly(true);
    assert!(matches!(
        fs::set_permissions(store.path(), permissions),
        Ok(())
    ));
    assert!(matches!(
        ParentPresenceVerificationPort::open(store.path()),
        Err(ParentPresenceStorageFailureReason::CustodyUnavailable)
    ));
    Ok(())
}

#[cfg(windows)]
#[test]
fn parent_presence_store_rejects_final_and_ancestor_symbolic_substitution() -> TestResult {
    use std::os::windows::fs::{symlink_dir, symlink_file};

    let store = TestStore::new("symbolic-substitution");
    drop(store.port()?);
    let final_link = store.root.join("linked-parent-presence.sqlite");
    if symlink_file(store.path(), &final_link).is_ok() {
        assert!(matches!(
            ParentPresenceVerificationPort::open(&final_link),
            Err(ParentPresenceStorageFailureReason::CustodyUnavailable)
        ));
    }

    let alias = store.root.with_extension("alias");
    if symlink_dir(&store.root, &alias).is_ok() {
        assert!(matches!(
            ParentPresenceVerificationPort::open(alias.join("parent-presence.sqlite")),
            Err(ParentPresenceStorageFailureReason::CustodyUnavailable)
        ));
        assert!(matches!(fs::remove_dir(&alias), Ok(())));
    }
    Ok(())
}

#[test]
fn parent_presence_verification_rejects_expired_challenges_and_accepts_future_challenges(
) -> TestResult {
    let store = TestStore::new("expiry-clock");
    let expired_case = test_case("expiry-clock-expired");
    let mut expired_port = store.port()?;
    issue_valid_challenge(&mut expired_port, &expired_case, EXPIRED_EXPIRY);
    assert_eq!(
        expired_port.verify_and_consume(verification_input(&expired_case, EXPIRED_EXPIRY)),
        Err(ParentPresenceVerificationFailureReason::Expired)
    );

    let accepted_case = test_case("expiry-clock-accepted");
    let mut accepted_port = store.port()?;
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
    Ok(())
}

#[test]
fn parent_presence_verification_rejects_malformed_noncanonical_and_offset_timestamps() -> TestResult
{
    let store = TestStore::new("timestamp-validation");
    let malformed_case = test_case("malformed");
    let mut malformed_port = store.port()?;
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
    let mut noncanonical_port = store.port()?;
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
    let mut offset_port = store.port()?;
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
    Ok(())
}
