use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use ocentra_eventing::ids::CorrelationId;
use ocentra_family_identity_core::household_authority::{
    HouseholdAuthorityAction, ParentStepUpAssertionSnapshot,
};
use ocentra_family_identity_core::parent_presence::{
    ParentPresenceChallenge, ParentPresenceChallengeIssuanceFailureReason,
    ParentPresenceCustodyDecisionBoundary, ParentPresenceCustodyDecisionDelivery,
    ParentPresenceCustodyDecisionOwner, ParentPresenceCustodyDecisionRedaction,
    ParentPresenceCustodyDecisionResult, ParentPresenceStorageFailureReason,
    ParentPresenceVerificationFailureReason, ParentPresenceVerificationInput,
    ParentPresenceVerificationPort,
};

use super::open_parent_presence_test_port;

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
        open_parent_presence_test_port(&self.path)
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

fn verification_input(
    case: &TestCase,
    expires_at: &str,
) -> Result<ParentPresenceVerificationInput, ParentPresenceStorageFailureReason> {
    Ok(ParentPresenceVerificationInput {
        correlation_id: correlation_id()?,
        challenge_ref: case.challenge_ref.clone(),
        assertion: assertion_for(case, expires_at),
    })
}

fn correlation_id() -> Result<CorrelationId, ParentPresenceStorageFailureReason> {
    CorrelationId::parse("parent-presence-unit-correlation")
        .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)
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
fn parent_presence_verification_input_debug_is_redacted() -> TestResult {
    let case = test_case("input-debug");
    let input = verification_input(&case, ACCEPTED_EXPIRY)?;
    assert_eq!(
        format!("{input:?}"),
        "ParentPresenceVerificationInput { correlation_id: CorrelationId(\"parent-presence-unit-correlation\"), challenge_ref: \"[redacted]\", assertion: \"[redacted]\" }"
    );
    Ok(())
}

#[test]
fn parent_presence_verification_is_one_time_and_redacted() -> TestResult {
    let case = test_case("one-time");
    let store = TestStore::new("one-time");
    let mut port = store.port()?;
    issue_valid_challenge(&mut port, &case, ACCEPTED_EXPIRY);

    let accepted = port.verify_and_consume(verification_input(&case, ACCEPTED_EXPIRY)?);
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
        port.verify_and_consume(verification_input(&case, ACCEPTED_EXPIRY)?),
        Err(ParentPresenceVerificationFailureReason::ReplayRejected)
    );
    Ok(())
}

#[test]
fn custody_decisions_return_correlated_redacted_eventing_artifacts() -> TestResult {
    let accepted_case = test_case("custody-artifact-accepted");
    let accepted_store = TestStore::new("custody-artifact-accepted");
    let mut accepted_port = accepted_store.port()?;
    issue_valid_challenge(&mut accepted_port, &accepted_case, ACCEPTED_EXPIRY);
    assert!(accepted_port
        .verify_and_consume(verification_input(&accepted_case, ACCEPTED_EXPIRY)?)
        .is_ok());
    assert_custody_artifact(
        &mut accepted_port,
        ParentPresenceCustodyDecisionResult::Accepted,
        &accepted_case,
    )?;
    assert_eq!(
        accepted_port.verify_and_consume(verification_input(&accepted_case, ACCEPTED_EXPIRY)?),
        Err(ParentPresenceVerificationFailureReason::ReplayRejected)
    );
    assert_custody_artifact(
        &mut accepted_port,
        ParentPresenceCustodyDecisionResult::ReplayRejected,
        &accepted_case,
    )?;

    let integrity_case = test_case("custody-artifact-integrity");
    let integrity_store = TestStore::new("custody-artifact-integrity");
    let mut integrity_port = integrity_store.port()?;
    issue_valid_challenge(&mut integrity_port, &integrity_case, ACCEPTED_EXPIRY);
    let connection = rusqlite::Connection::open(integrity_store.path())
        .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;
    connection
        .execute(
            "UPDATE parent_presence_challenges SET challenge_json = '{}' WHERE challenge_ref = ?1",
            [&integrity_case.challenge_ref],
        )
        .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;
    drop(connection);
    assert_eq!(
        integrity_port.verify_and_consume(verification_input(&integrity_case, ACCEPTED_EXPIRY)?),
        Err(ParentPresenceVerificationFailureReason::CustodyIntegrityRejected)
    );
    assert_custody_artifact(
        &mut integrity_port,
        ParentPresenceCustodyDecisionResult::IntegrityRejected,
        &integrity_case,
    )?;
    Ok(())
}

fn assert_custody_artifact(
    port: &mut ParentPresenceVerificationPort,
    expected_result: ParentPresenceCustodyDecisionResult,
    case: &TestCase,
) -> TestResult {
    let artifact = port
        .take_custody_artifact()
        .ok_or(ParentPresenceStorageFailureReason::CustodyUnavailable)?;
    assert_eq!(artifact.correlation_id, correlation_id()?);
    assert_eq!(
        artifact.owner,
        ParentPresenceCustodyDecisionOwner::FamilyIdentityCore
    );
    assert_eq!(
        artifact.boundary,
        ParentPresenceCustodyDecisionBoundary::VerifyAndConsume
    );
    assert_eq!(artifact.result, expected_result);
    assert!(!artifact.decision_id.as_str().is_empty());
    assert_eq!(
        artifact.delivery,
        ParentPresenceCustodyDecisionDelivery::EventingJournal
    );
    assert_eq!(
        artifact.redaction,
        ParentPresenceCustodyDecisionRedaction::SensitiveInputsOmitted
    );
    let serialized = serde_json::to_value(&artifact)
        .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;
    assert_eq!(
        serialized,
        serde_json::json!({
            "decisionId": artifact.decision_id.as_str(),
            "correlationId": "parent-presence-unit-correlation",
            "owner": "family-identity-core",
            "boundary": "verify-and-consume",
            "result": expected_result,
            "delivery": "eventing-journal",
            "redaction": "sensitive-inputs-omitted"
        })
    );
    assert_redacted_debug(&artifact, case);
    assert!(port.take_custody_artifact().is_none());
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
            correlation_id: correlation_id()?,
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
            correlation_id: correlation_id()?,
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
            correlation_id: correlation_id()?,
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
            correlation_id: correlation_id()?,
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
            correlation_id: correlation_id()?,
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
            correlation_id: correlation_id()?,
            challenge_ref: case.challenge_ref.clone(),
            assertion: ParentStepUpAssertionSnapshot {
                target_child_profile_id: Some("wrong-target".to_owned()),
                ..assertion_for(&case, ACCEPTED_EXPIRY)
            },
        }),
        Err(ParentPresenceVerificationFailureReason::TargetChildProfileMismatch)
    );

    let accepted = port.verify_and_consume(verification_input(&case, ACCEPTED_EXPIRY)?);
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

    let accepted = port.verify_and_consume(verification_input(&case, ACCEPTED_EXPIRY)?);
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
            .verify_and_consume(verification_input(&first, ACCEPTED_EXPIRY)?)
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
        .verify_and_consume(verification_input(&first, ACCEPTED_EXPIRY)?)
        .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;
    let second_accepted = port
        .verify_and_consume(verification_input(&second, ACCEPTED_EXPIRY)?)
        .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;
    let first_ref = first_accepted.receipt_ref().as_str().to_owned();
    let second_ref = second_accepted.receipt_ref().as_str().to_owned();
    let first_entropy = first_ref.strip_prefix("parent-presence-receipt:");
    assert_eq!(first_entropy.map(str::len), Some(64));
    assert_eq!(
        first_entropy.map(|value| value.chars().all(|character| character.is_ascii_hexdigit())),
        Some(true)
    );
    assert_ne!(first_ref, second_ref);
    assert_eq!(first_accepted.receipt_ref().to_string(), "[redacted]");
    assert_eq!(format!("{}", first_accepted.receipt_ref()), "[redacted]");
    assert!(!format!("{first_accepted:?}").contains(&first_ref));
    Ok(())
}

#[test]
fn parent_presence_store_requires_existing_absolute_caller_custody_parent() {
    let relative = PathBuf::from("parent-presence-relative.sqlite");
    assert!(matches!(
        open_parent_presence_test_port(&relative),
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
        open_parent_presence_test_port(&path),
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
        open_parent_presence_test_port(store.path()),
        Err(ParentPresenceStorageFailureReason::CustodyUnavailable)
    ));
    Ok(())
}

#[cfg(windows)]
#[test]
fn parent_presence_store_rejects_final_and_ancestor_symbolic_substitution(
) -> Result<(), Box<dyn std::error::Error>> {
    use std::os::windows::fs::{symlink_dir, symlink_file};

    let store = TestStore::new("symbolic-substitution");
    drop(store.port().map_err(|_error| {
        std::io::Error::other("failed to initialize the symbolic-substitution fixture")
    })?);
    let final_link = store.root.join("linked-parent-presence.sqlite");
    symlink_file(store.path(), &final_link).map_err(|error| {
        std::io::Error::new(
            error.kind(),
            format!("Windows final-file symbolic-link coverage could not be exercised: {error}"),
        )
    })?;
    assert!(matches!(
        open_parent_presence_test_port(&final_link),
        Err(ParentPresenceStorageFailureReason::CustodyUnavailable)
    ));

    let alias = store.root.with_extension("alias");
    symlink_dir(&store.root, &alias).map_err(|error| {
        std::io::Error::new(
            error.kind(),
            format!(
                "Windows ancestor-directory symbolic-link coverage could not be exercised: {error}"
            ),
        )
    })?;
    assert!(matches!(
        open_parent_presence_test_port(alias.join("parent-presence.sqlite")),
        Err(ParentPresenceStorageFailureReason::CustodyUnavailable)
    ));
    fs::remove_dir(&alias)?;
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
        expired_port.verify_and_consume(verification_input(&expired_case, EXPIRED_EXPIRY)?),
        Err(ParentPresenceVerificationFailureReason::Expired)
    );

    let accepted_case = test_case("expiry-clock-accepted");
    let mut accepted_port = store.port()?;
    issue_valid_challenge(&mut accepted_port, &accepted_case, ACCEPTED_EXPIRY);
    let accepted =
        accepted_port.verify_and_consume(verification_input(&accepted_case, ACCEPTED_EXPIRY)?);
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
fn parent_presence_verification_accepts_second_precision_and_rejects_malformed_or_offset_timestamps(
) -> TestResult {
    let store = TestStore::new("timestamp-validation");
    let malformed_case = test_case("malformed");
    let mut malformed_port = store.port()?;
    issue_valid_challenge(&mut malformed_port, &malformed_case, ACCEPTED_EXPIRY);
    assert_eq!(
        malformed_port.verify_and_consume(ParentPresenceVerificationInput {
            correlation_id: correlation_id()?,
            challenge_ref: malformed_case.challenge_ref.clone(),
            assertion: ParentStepUpAssertionSnapshot {
                expires_at: "not-a-timestamp".to_owned(),
                ..assertion_for(&malformed_case, ACCEPTED_EXPIRY)
            },
        }),
        Err(ParentPresenceVerificationFailureReason::TimestampInvalid)
    );
    assert!(malformed_port
        .verify_and_consume(verification_input(&malformed_case, ACCEPTED_EXPIRY)?)
        .is_ok());

    let second_precision_case = test_case("second-precision");
    let mut second_precision_port = store.port()?;
    issue_valid_challenge(
        &mut second_precision_port,
        &second_precision_case,
        ACCEPTED_EXPIRY,
    );
    assert!(second_precision_port
        .verify_and_consume(ParentPresenceVerificationInput {
            correlation_id: correlation_id()?,
            challenge_ref: second_precision_case.challenge_ref.clone(),
            assertion: ParentStepUpAssertionSnapshot {
                expires_at: "2099-01-01T00:00:00Z".to_owned(),
                ..assertion_for(&second_precision_case, ACCEPTED_EXPIRY)
            },
        })
        .is_ok());

    let offset_case = test_case("offset");
    let mut offset_port = store.port()?;
    issue_valid_challenge(&mut offset_port, &offset_case, ACCEPTED_EXPIRY);
    assert_eq!(
        offset_port.verify_and_consume(ParentPresenceVerificationInput {
            correlation_id: correlation_id()?,
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
