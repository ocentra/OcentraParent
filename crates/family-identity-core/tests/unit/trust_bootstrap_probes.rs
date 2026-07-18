use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Barrier};
use std::thread;
use std::time::{Duration, UNIX_EPOCH};

use ocentra_family_identity_core::household_authority::{
    HouseholdAuthorityAction, ParentStepUpAssertionSnapshot,
};
use ocentra_family_identity_core::parent_presence::{
    ParentPresenceChallenge, ParentPresenceObservedAt, ParentPresenceVerificationFailureReason,
    ParentPresenceVerificationInput, ParentPresenceVerificationPort,
};
use ocentra_family_identity_core::trust_bootstrap::{
    evaluate_trust_bootstrap, TrustBootstrapDecision, TrustBootstrapInput,
    TrustBootstrapLifecycleIntent,
};

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
    trust_bootstrap_ref: String,
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
        trust_bootstrap_ref: format!("{scope}-trust-bootstrap"),
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

fn external_consumer_probe_dir() -> PathBuf {
    let probe_root = std::env::temp_dir().join(format!(
        "ocentra-family-identity-core-with-clock-probe-{}-{}",
        std::process::id(),
        NEXT_CASE_ID.fetch_add(1, Ordering::Relaxed)
    ));
    assert!(
        fs::create_dir_all(probe_root.join("src")).is_ok(),
        "probe directory should be creatable"
    );
    probe_root
}

fn write_external_consumer_probe(probe_root: &Path) {
    let manifest_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let manifest_path = manifest_path.to_string_lossy().replace('\\', "/");

    assert!(
        fs::write(
            probe_root.join("Cargo.toml"),
            format!(
                r#"[package]
name = "ocentra-family-identity-core-with-clock-probe"
version = "0.0.0"
edition = "2021"

[dependencies]
ocentra-family-identity-core = {{ path = "{manifest_path}" }}
"#
            ),
        )
        .is_ok(),
        "probe manifest should be writable"
    );

    assert!(
        fs::write(
            probe_root.join("src/main.rs"),
            r#"use ocentra_family_identity_core::parent_presence::{
    ParentPresenceObservedAt, ParentPresenceVerificationPort,
};

fn main() {
    let _ = ParentPresenceVerificationPort::with_clock(|| {
        ParentPresenceObservedAt::from_canonical_utc("2000-01-01T00:00:00.000Z").unwrap()
    });
}
"#,
        )
        .is_ok(),
        "probe source should be writable"
    );
}

#[test]
fn parent_presence_verification_consumes_once_across_ports_and_threads() {
    let case = test_case("multi-instance");
    let mut issuer = ParentPresenceVerificationPort::new();
    issue_valid_challenge(&mut issuer, &case, ACCEPTED_EXPIRY);

    let barrier = Arc::new(Barrier::new(2));
    let first_case = case.clone();
    let first_barrier = Arc::clone(&barrier);
    let first = thread::spawn(move || {
        let mut port = ParentPresenceVerificationPort::new();
        first_barrier.wait();
        port.verify_and_consume(verification_input(&first_case, ACCEPTED_EXPIRY))
    });

    let second_case = case.clone();
    let second_barrier = Arc::clone(&barrier);
    let second = thread::spawn(move || {
        let mut port = ParentPresenceVerificationPort::new();
        second_barrier.wait();
        port.verify_and_consume(verification_input(&second_case, ACCEPTED_EXPIRY))
    });

    let first = first.join();
    let second = second.join();
    assert_eq!(first.as_ref().map(|_| 1).map_err(|_error| ()), Ok(1));
    assert_eq!(second.as_ref().map(|_| 1).map_err(|_error| ()), Ok(1));

    if let (Ok(first), Ok(second)) = (first, second) {
        let expected_assertion = assertion_for(&case, ACCEPTED_EXPIRY);
        let accepted_count = [first.as_ref(), second.as_ref()]
            .iter()
            .filter(|result| {
                matches!(
                    result,
                    Ok(accepted) if accepted.assertion_snapshot() == &expected_assertion
                )
            })
            .count();
        let replay_count = [first.as_ref(), second.as_ref()]
            .iter()
            .filter(|result| {
                matches!(
                    result,
                    Err(ParentPresenceVerificationFailureReason::ReplayRejected)
                )
            })
            .count();

        assert_eq!(accepted_count, 1);
        assert_eq!(replay_count, 1);
    }
}

#[test]
fn trust_bootstrap_returns_awaiting_platform_key_sealing() {
    let case = test_case("awaiting-seal");
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
        let receipt_ref = accepted.receipt_ref().to_string();
        let decision = evaluate_trust_bootstrap(TrustBootstrapInput {
            trust_bootstrap_ref: case.trust_bootstrap_ref.clone(),
            lifecycle_intent: TrustBootstrapLifecycleIntent::SealParentDeviceTrust,
            parent_presence: accepted,
        });

        assert!(matches!(
            decision,
            TrustBootstrapDecision::AwaitingPlatformKeySealing(_)
        ));
        if let TrustBootstrapDecision::AwaitingPlatformKeySealing(request) = &decision {
            assert_eq!(request.trust_bootstrap_ref, case.trust_bootstrap_ref);
            assert_eq!(
                request.lifecycle_intent,
                TrustBootstrapLifecycleIntent::SealParentDeviceTrust
            );
            assert_eq!(
                request.device_trust_ref,
                format!(
                    "device-trust:{}:{}:{}:{}:{}:{}:{}:{}:{:?}",
                    case.trust_bootstrap_ref,
                    receipt_ref,
                    case.family_id,
                    case.parent_account_id,
                    case.action_device_id,
                    case.action_device_child_profile_id
                        .as_deref()
                        .unwrap_or("-"),
                    case.target_child_profile_id.as_deref().unwrap_or("-"),
                    case.nonce_ref,
                    TrustBootstrapLifecycleIntent::SealParentDeviceTrust
                )
            );
        }
    }
}

#[test]
fn parent_presence_observed_at_supports_canonical_roundtrip_and_pre_unix_system_time() {
    let canonical = ParentPresenceObservedAt::from_canonical_utc(ACCEPTED_EXPIRY);
    assert_eq!(
        canonical.as_ref().map(|value| value.to_string()),
        Ok(ACCEPTED_EXPIRY.to_owned())
    );

    let pre_unix = UNIX_EPOCH.checked_sub(Duration::from_millis(1));
    assert_eq!(pre_unix.as_ref().map(|_| 1), Some(1));
    if let Some(pre_unix) = pre_unix {
        let observed_at = ParentPresenceObservedAt::from_system_time(pre_unix);
        assert_eq!(observed_at.to_string(), "1969-12-31T23:59:59.999Z");
    }
}

#[test]
fn parent_presence_verification_port_with_clock_is_inaccessible_to_external_consumers() {
    let probe_root = external_consumer_probe_dir();
    write_external_consumer_probe(&probe_root);

    let output = Command::new("cargo")
        .arg("check")
        .arg("--quiet")
        .current_dir(&probe_root)
        .output();
    assert_eq!(output.as_ref().map(|_| 1).map_err(|_error| ()), Ok(1));
    let output = match output {
        Ok(output) => output,
        Err(_) => return,
    };

    assert!(!output.status.success());

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("private") && stderr.contains("with_clock"),
        "expected the inaccessible with_clock constructor to be rejected, stderr: {stderr}"
    );
}
