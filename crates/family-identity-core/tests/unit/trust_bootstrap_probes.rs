use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, UNIX_EPOCH};

use ocentra_eventing::ids::CorrelationId;
use ocentra_family_identity_core::household_authority::{
    HouseholdAuthorityAction, ParentStepUpAssertionSnapshot,
};
use ocentra_family_identity_core::parent_presence::{
    ParentPresenceChallenge, ParentPresenceObservedAt, ParentPresenceStorageFailureReason,
    ParentPresenceVerificationFailureReason, ParentPresenceVerificationInput,
    ParentPresenceVerificationPort,
};
use ocentra_family_identity_core::trust_bootstrap::{
    current_authority::{
        CurrentParentDeviceTrustAuthority, CurrentParentDeviceTrustAuthorityError,
        CurrentParentDeviceTrustAuthoritySource,
    },
    evaluate_trust_bootstrap, DeviceTrustRef, TrustBootstrapDecision, TrustBootstrapInput,
    TrustBootstrapLifecycleIntent, TrustBootstrapManualRequirementReason,
};
#[cfg(windows)]
use ocentra_storage_custody_core::windows_dpapi_key_sealing::{
    seal_for_current_windows_user, unseal_for_current_windows_user, DpapiSealedKey,
};

use super::open_parent_presence_test_port;

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
    trust_bootstrap_ref: String,
}

struct TestStore {
    root: PathBuf,
    path: PathBuf,
}

impl TestStore {
    fn new(prefix: &str) -> Self {
        let id = NEXT_CASE_ID.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "ocentra-parent-presence-probe-{prefix}-{}-{id}",
            std::process::id()
        ));
        assert!(matches!(fs::create_dir_all(&root), Ok(())));
        let path = root.join("parent-presence.sqlite");
        Self { root, path }
    }

    fn path(&self) -> &Path {
        &self.path
    }

    fn port(&self) -> Result<ParentPresenceVerificationPort, ParentPresenceStorageFailureReason> {
        open_parent_presence_test_port(&self.path)
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
    test_case_for_scope(&scope)
}

fn test_case_for_scope(scope: &str) -> TestCase {
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
        privileged_action: HouseholdAuthorityAction::SealParentDeviceTrust,
        action_device_id: case.action_device_id.clone(),
        action_device_child_profile_id: None,
        target_child_profile_id: None,
        expires_at: expires_at.to_owned(),
    }
}

fn assertion_for(case: &TestCase, expires_at: &str) -> ParentStepUpAssertionSnapshot {
    ParentStepUpAssertionSnapshot {
        family_id: case.family_id.clone(),
        parent_account_id: case.parent_account_id.clone(),
        action_device_id: case.action_device_id.clone(),
        action_device_child_profile_id: None,
        target_child_profile_id: None,
        action: HouseholdAuthorityAction::SealParentDeviceTrust,
        nonce: case.nonce_ref.clone(),
        expires_at: expires_at.to_owned(),
    }
}

struct CurrentAuthority<'a> {
    trusted: bool,
    trust_subject: &'a str,
    device_ref: &'a str,
    lifecycle_generation: u64,
    installation_binding_generation: u64,
}

impl CurrentParentDeviceTrustAuthoritySource for CurrentAuthority<'_> {
    fn current_authorized_parent_device(
        &self,
        trust_subject: &str,
        device_ref: &str,
    ) -> Result<CurrentParentDeviceTrustAuthority, CurrentParentDeviceTrustAuthorityError> {
        (self.trusted && self.trust_subject == trust_subject && self.device_ref == device_ref)
            .then_some(CurrentParentDeviceTrustAuthority {
                lifecycle_generation: self.lifecycle_generation,
                installation_binding_generation: self.installation_binding_generation,
            })
            .ok_or(CurrentParentDeviceTrustAuthorityError::NotTrusted)
    }
}

fn current_parent_authority_for_device<'a>(
    trusted: bool,
    trust_subject: &'a str,
    device_ref: &'a str,
) -> CurrentAuthority<'a> {
    CurrentAuthority {
        trusted,
        trust_subject,
        device_ref,
        lifecycle_generation: 1,
        installation_binding_generation: 1,
    }
}

fn verification_input(
    case: &TestCase,
    expires_at: &str,
) -> Result<ParentPresenceVerificationInput, ParentPresenceStorageFailureReason> {
    Ok(ParentPresenceVerificationInput {
        correlation_id: CorrelationId::parse("parent-presence-probe-correlation")
            .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?,
        challenge_ref: case.challenge_ref.clone(),
        assertion: assertion_for(case, expires_at),
    })
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
    [
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
    ]
    .into_iter()
    .filter(|secret| !secret.is_empty())
    .for_each(|secret| {
        assert!(
            !debug.contains(secret),
            "debug output leaked {secret}: {debug}"
        );
    });
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
    let _ = ParentPresenceVerificationPort::with_clock(
        "parent-presence.sqlite",
        || ParentPresenceObservedAt::from_canonical_utc("2000-01-01T00:00:00.000Z").unwrap(),
    );
}
"#,
        )
        .is_ok(),
        "probe source should be writable"
    );
}

#[test]
fn parent_presence_verification_consumes_once_across_ports() -> TestResult {
    let case = test_case("multi-instance");
    let store = TestStore::new("multi-instance");
    let mut issuer = store.port()?;
    issue_valid_challenge(&mut issuer, &case, ACCEPTED_EXPIRY);
    let mut first = open_parent_presence_test_port(store.path())?;
    let mut second = open_parent_presence_test_port(store.path())?;
    assert_eq!(
        first
            .verify_and_consume(verification_input(&case, ACCEPTED_EXPIRY)?)
            .as_ref()
            .map(|accepted| accepted.assertion_snapshot()),
        Ok(&assertion_for(&case, ACCEPTED_EXPIRY))
    );
    assert_eq!(
        second.verify_and_consume(verification_input(&case, ACCEPTED_EXPIRY)?),
        Err(ParentPresenceVerificationFailureReason::ReplayRejected)
    );
    Ok(())
}

#[test]
fn trust_bootstrap_issues_authorized_capability_for_parent_approved_pairing() -> TestResult {
    let case = test_case("manual-seal");
    let store = TestStore::new("manual-seal");
    let mut port = store.port()?;
    issue_valid_challenge(&mut port, &case, ACCEPTED_EXPIRY);
    let accepted = port.verify_and_consume(verification_input(&case, ACCEPTED_EXPIRY)?);
    assert_eq!(
        accepted
            .as_ref()
            .map(|accepted| accepted.assertion_snapshot()),
        Ok(&assertion_for(&case, ACCEPTED_EXPIRY))
    );
    let accepted =
        accepted.map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;
    assert_redacted_debug(&accepted, &case);
    let decision = evaluate_trust_bootstrap(TrustBootstrapInput {
        trust_bootstrap_ref: case.trust_bootstrap_ref,
        lifecycle_intent: TrustBootstrapLifecycleIntent::SealParentDeviceTrust,
        parent_presence: accepted,
    });

    let TrustBootstrapDecision::AwaitingPlatformKeySealing(request) = decision else {
        return Err(ParentPresenceStorageFailureReason::CustodyUnavailable);
    };
    #[cfg(windows)]
    {
        let authority = current_parent_authority_for_device(
            true,
            case.parent_account_id.as_str(),
            case.action_device_id.as_str(),
        );
        let sealed = seal_for_current_windows_user(request, &authority, b"trust-material")
            .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;
        let context = sealed.context().clone();
        let authority =
            current_parent_authority_for_device(true, &context.trust_subject, &context.device_ref);
        assert_eq!(
            unseal_for_current_windows_user(&sealed, &authority)
                .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?,
            b"trust-material"
        );
        let debug = format!("{sealed:?}");
        assert_eq!(
            debug,
            "DpapiSealedKey { format_version: 1, context: \"[redacted]\", authorization_binding: \"[redacted]\", ciphertext: \"[redacted]\" }"
        );

        assert_eq!(
            unseal_for_current_windows_user(&sealed, &authority),
            Ok(b"trust-material".to_vec())
        );
        let persisted = serde_json::to_vec(&sealed)
            .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;
        let recovered: DpapiSealedKey = serde_json::from_slice(&persisted)
            .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;
        assert_eq!(
            unseal_for_current_windows_user(&recovered, &authority),
            Ok(b"trust-material".to_vec())
        );
        assert_eq!(
            unseal_for_current_windows_user(
                &recovered,
                &CurrentAuthority {
                    trusted: true,
                    trust_subject: &context.trust_subject,
                    device_ref: &context.device_ref,
                    lifecycle_generation: 2,
                    installation_binding_generation: 2,
                },
            ),
            Err(ocentra_storage_custody_core::windows_dpapi_key_sealing::DpapiKeySealingError::CurrentAuthorityRequired)
        );
        assert!(
            !String::from_utf8_lossy(&persisted).contains("MachineGuid"),
            "machine-local binding must not be serialized with the sealed key"
        );
        assert_eq!(
            unseal_for_current_windows_user(&recovered, &current_parent_authority_for_device(true, &context.trust_subject, "another-trusted-parent-device")),
            Err(ocentra_storage_custody_core::windows_dpapi_key_sealing::DpapiKeySealingError::CurrentAuthorityRequired)
        );
        assert_eq!(
            unseal_for_current_windows_user(&recovered, &current_parent_authority_for_device(false, &context.trust_subject, &context.device_ref)),
            Err(ocentra_storage_custody_core::windows_dpapi_key_sealing::DpapiKeySealingError::CurrentAuthorityRequired)
        );
    }
    Ok(())
}

#[test]
fn current_parent_unsealing_authority_rejects_revoked_reset_and_reinstall_lifecycle_states() {
    for trusted in [false, false, false, false] {
        assert_eq!(
            current_parent_authority_for_device(trusted, "parent", "device")
                .current_authorized_parent_device("parent", "device"),
            Err(CurrentParentDeviceTrustAuthorityError::NotTrusted)
        );
    }
}

#[test]
fn trust_bootstrap_never_promotes_matching_low_risk_action_into_sealing() -> TestResult {
    let mut case = test_case("low-risk-manual-seal");
    case.action_device_child_profile_id = None;
    case.target_child_profile_id = None;
    let store = TestStore::new("low-risk-manual-seal");
    let mut port = store.port()?;
    let mut challenge = challenge_for(&case, ACCEPTED_EXPIRY);
    challenge.privileged_action = HouseholdAuthorityAction::ViewChildStatus;
    assert_eq!(port.issue_challenge(challenge), Ok(()));
    let mut assertion = assertion_for(&case, ACCEPTED_EXPIRY);
    assertion.action = HouseholdAuthorityAction::ViewChildStatus;
    let accepted = port
        .verify_and_consume(ParentPresenceVerificationInput {
            correlation_id: CorrelationId::parse("low-risk-sealing-correlation")
                .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?,
            challenge_ref: case.challenge_ref.clone(),
            assertion,
        })
        .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;

    assert!(matches!(
        evaluate_trust_bootstrap(TrustBootstrapInput {
            trust_bootstrap_ref: case.trust_bootstrap_ref,
            lifecycle_intent: TrustBootstrapLifecycleIntent::SealParentDeviceTrust,
            parent_presence: accepted,
        }),
        TrustBootstrapDecision::ManualRequired(requirement)
            if requirement.reason
                == TrustBootstrapManualRequirementReason::AuthorizedChallengeActionUnavailable
    ));
    Ok(())
}

#[test]
fn trust_bootstrap_rejects_a_child_scoped_parent_sealing_ceremony() -> TestResult {
    let case = test_case("child-scoped-sealing");
    let store = TestStore::new("child-scoped-sealing");
    let mut port = store.port()?;
    let mut challenge = challenge_for(&case, ACCEPTED_EXPIRY);
    challenge.privileged_action = HouseholdAuthorityAction::SealParentDeviceTrust;
    assert_eq!(port.issue_challenge(challenge), Ok(()));
    let mut assertion = assertion_for(&case, ACCEPTED_EXPIRY);
    assertion.action = HouseholdAuthorityAction::SealParentDeviceTrust;
    let accepted = port
        .verify_and_consume(ParentPresenceVerificationInput {
            correlation_id: CorrelationId::parse("child-scoped-sealing-correlation")
                .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?,
            challenge_ref: case.challenge_ref.clone(),
            assertion,
        })
        .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;

    assert!(matches!(
        evaluate_trust_bootstrap(TrustBootstrapInput {
            trust_bootstrap_ref: case.trust_bootstrap_ref,
            lifecycle_intent: TrustBootstrapLifecycleIntent::SealParentDeviceTrust,
            parent_presence: accepted,
        }),
        TrustBootstrapDecision::ManualRequired(requirement)
            if requirement.reason
                == TrustBootstrapManualRequirementReason::ChildScopedCeremonyRejected
    ));
    Ok(())
}

#[test]
fn device_trust_reference_is_random_opaque_and_input_independent() -> TestResult {
    let case = test_case("opaque-device-trust-ref");
    let first = DeviceTrustRef::generate()
        .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;
    let second = DeviceTrustRef::generate()
        .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;
    assert_eq!(first.as_str().len(), 64);
    assert!(first
        .as_str()
        .chars()
        .all(|character| character.is_ascii_hexdigit()));
    assert_ne!(first, second);
    let serialized = serde_json::to_value(&first)
        .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;
    assert_eq!(serialized, serde_json::json!(first.as_str()));
    for protected in [
        case.trust_bootstrap_ref.as_str(),
        case.challenge_ref.as_str(),
        case.nonce_ref.as_str(),
        case.family_id.as_str(),
        case.parent_account_id.as_str(),
        case.action_device_id.as_str(),
        case.action_device_child_profile_id
            .as_deref()
            .unwrap_or_default(),
        case.target_child_profile_id.as_deref().unwrap_or_default(),
    ] {
        assert_ne!(first.as_str(), protected);
    }
    Ok(())
}

#[test]
fn trust_bootstrap_operational_debug_redacts_identity_and_capability_material() -> TestResult {
    let case = test_case("operational-debug");
    let store = TestStore::new("operational-debug");
    let mut port = store.port()?;
    issue_valid_challenge(&mut port, &case, ACCEPTED_EXPIRY);
    let accepted = port.verify_and_consume(verification_input(&case, ACCEPTED_EXPIRY)?);
    assert_eq!(accepted.as_ref().map(|_| 1), Ok(1));

    let accepted =
        accepted.map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;
    let receipt_ref = accepted.receipt_ref().as_str().to_owned();
    let decision = evaluate_trust_bootstrap(TrustBootstrapInput {
        trust_bootstrap_ref: case.trust_bootstrap_ref.clone(),
        lifecycle_intent: TrustBootstrapLifecycleIntent::SealParentDeviceTrust,
        parent_presence: accepted,
    });
    let debug = format!("{decision:?}");

    for protected in [
        case.trust_bootstrap_ref.as_str(),
        receipt_ref.as_str(),
        case.family_id.as_str(),
        case.parent_account_id.as_str(),
        case.action_device_id.as_str(),
        case.action_device_child_profile_id
            .as_deref()
            .unwrap_or_default(),
        case.target_child_profile_id.as_deref().unwrap_or_default(),
        case.nonce_ref.as_str(),
        ACCEPTED_EXPIRY,
    ] {
        assert!(
            !debug.contains(protected),
            "operational Debug leaked protected trust material {protected}: {debug}"
        );
    }
    Ok(())
}

#[test]
fn parent_device_sealing_requires_a_binding_complete_authority_receipt() -> TestResult {
    let mut case = test_case("authority-receipt");
    case.action_device_child_profile_id = None;
    case.target_child_profile_id = None;
    let store = TestStore::new("authority-receipt");
    let mut port = store.port()?;
    let mut challenge = challenge_for(&case, ACCEPTED_EXPIRY);
    challenge.privileged_action = HouseholdAuthorityAction::SealParentDeviceTrust;
    port.issue_challenge(challenge.clone())
        .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;
    let mut assertion = assertion_for(&case, ACCEPTED_EXPIRY);
    assertion.action = HouseholdAuthorityAction::SealParentDeviceTrust;
    assertion.action_device_child_profile_id = None;
    assertion.target_child_profile_id = None;
    let accepted = port
        .verify_and_consume(ParentPresenceVerificationInput {
            correlation_id: CorrelationId::parse("authority-receipt-correlation")
                .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?,
            challenge_ref: challenge.challenge_ref,
            assertion,
        })
        .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;

    let direct = evaluate_trust_bootstrap(TrustBootstrapInput {
        trust_bootstrap_ref: case.trust_bootstrap_ref,
        lifecycle_intent: TrustBootstrapLifecycleIntent::SealParentDeviceTrust,
        parent_presence: accepted,
    });
    assert!(matches!(
        direct,
        TrustBootstrapDecision::ManualRequired(requirement)
            if requirement.reason == TrustBootstrapManualRequirementReason::AuthorityReceiptRequired
    ));

    Ok(())
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
