use std::fmt;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use ocentra_eventing::ids::CorrelationId;
use ocentra_family_identity_core::device_trust_authority::verify_parent_device_trust_authority;
use ocentra_family_identity_core::device_trust_registry::{
    DeviceTrustLifecycleState, DeviceTrustRegistry, DeviceTrustRegistryDecision,
    DeviceTrustRegistryRejection,
};
use ocentra_family_identity_core::household_authority::{
    HouseholdAuthorityAction, ParentStepUpAssertionSnapshot,
};
use ocentra_family_identity_core::parent_presence::{
    ParentPresenceChallenge, ParentPresenceVerificationInput,
};

use super::open_parent_presence_test_port;

const ACCEPTED_EXPIRY: &str = "2099-01-01T00:00:00.000Z";
static NEXT_CASE_ID: AtomicU64 = AtomicU64::new(1);
type TestResult = Result<(), TestFailure>;

#[derive(Debug)]
struct TestFailure(String);

impl fmt::Display for TestFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for TestFailure {}

impl From<String> for TestFailure {
    fn from(value: String) -> Self {
        Self(value)
    }
}

struct TestStore {
    root: PathBuf,
    parent_presence_path: PathBuf,
    registry_path: PathBuf,
}

impl TestStore {
    fn new(prefix: &str) -> Result<Self, String> {
        let id = NEXT_CASE_ID.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "ocentra-device-trust-registry-{prefix}-{}-{id}",
            std::process::id()
        ));
        fs::create_dir_all(&root).map_err(|error| error.to_string())?;
        Ok(Self {
            parent_presence_path: root.join("parent-presence.sqlite"),
            registry_path: root.join("device-trust.sqlite"),
            root,
        })
    }

    fn registry(&self) -> Result<DeviceTrustRegistry, String> {
        DeviceTrustRegistry::open(&self.registry_path).map_err(|error| format!("{error:?}"))
    }
}

impl Drop for TestStore {
    fn drop(&mut self) {
        let _cleanup_result = fs::remove_dir_all(&self.root);
    }
}

fn authority_for(
    store: &TestStore,
    action: HouseholdAuthorityAction,
    device_id: &str,
) -> Result<
    ocentra_family_identity_core::device_trust_authority::VerifiedParentDeviceTrustAuthority,
    String,
> {
    let id = NEXT_CASE_ID.fetch_add(1, Ordering::Relaxed);
    let challenge_ref = format!("device-trust-challenge-{id}");
    let nonce_ref = format!("device-trust-nonce-{id}");
    let challenge = ParentPresenceChallenge {
        challenge_ref: challenge_ref.clone(),
        nonce_ref: nonce_ref.clone(),
        family_id: "family".to_owned(),
        parent_account_id: "parent-account".to_owned(),
        privileged_action: action,
        action_device_id: device_id.to_owned(),
        action_device_child_profile_id: None,
        target_child_profile_id: Some("child-profile".to_owned()),
        expires_at: ACCEPTED_EXPIRY.to_owned(),
    };
    let assertion = ParentStepUpAssertionSnapshot {
        family_id: challenge.family_id.clone(),
        parent_account_id: challenge.parent_account_id.clone(),
        action_device_id: challenge.action_device_id.clone(),
        action_device_child_profile_id: None,
        target_child_profile_id: challenge.target_child_profile_id.clone(),
        action,
        nonce: nonce_ref,
        expires_at: ACCEPTED_EXPIRY.to_owned(),
    };
    let mut parent_presence = open_parent_presence_test_port(&store.parent_presence_path)
        .map_err(|error| format!("{error:?}"))?;
    assert_eq!(parent_presence.issue_challenge(challenge), Ok(()));
    let correlation_id =
        CorrelationId::parse("device-trust-registry-test").map_err(|error| format!("{error:?}"))?;
    let accepted = parent_presence
        .verify_and_consume(ParentPresenceVerificationInput {
            correlation_id,
            challenge_ref,
            assertion,
        })
        .map_err(|error| format!("{error:?}"))?;
    verify_parent_device_trust_authority(accepted).map_err(|error| format!("{error:?}"))
}

#[test]
fn verified_parent_pair_persists_pending_sealing_without_a_trusted_transition() -> TestResult {
    let store = TestStore::new("pair-pending")?;
    let registry = store.registry()?;
    let device_id = "child-device";

    assert_eq!(
        registry
            .apply_verified_parent_authority(authority_for(
                &store,
                HouseholdAuthorityAction::PairChildDevice,
                device_id,
            )?)
            .map_err(|error| format!("{error:?}"))?,
        DeviceTrustRegistryDecision::PendingSealing(
            ocentra_family_identity_core::device_trust_registry::DeviceTrustRegistryRecord {
                device_id: device_id.to_owned(),
                state: DeviceTrustLifecycleState::PendingSealing,
            }
        )
    );
    drop(registry);

    assert_eq!(
        store
            .registry()?
            .record(device_id)
            .map_err(|error| format!("{error:?}"))?,
        Some(
            ocentra_family_identity_core::device_trust_registry::DeviceTrustRegistryRecord {
                device_id: device_id.to_owned(),
                state: DeviceTrustLifecycleState::PendingSealing,
            }
        )
    );
    Ok(())
}

#[test]
fn verified_parent_revocation_wins_over_stale_repair_after_restart() -> TestResult {
    let store = TestStore::new("revoke-wins")?;
    let device_id = "child-device";
    let registry = store.registry()?;
    assert!(matches!(
        registry.apply_verified_parent_authority(authority_for(
            &store,
            HouseholdAuthorityAction::PairChildDevice,
            device_id,
        )?),
        Ok(DeviceTrustRegistryDecision::PendingSealing(_))
    ));
    assert!(matches!(
        registry.apply_verified_parent_authority(authority_for(
            &store,
            HouseholdAuthorityAction::RevokeChildDevice,
            device_id,
        )?),
        Ok(DeviceTrustRegistryDecision::Revoked(_))
    ));
    drop(registry);

    assert_eq!(
        store
            .registry()?
            .apply_verified_parent_authority(authority_for(
                &store,
                HouseholdAuthorityAction::PairChildDevice,
                device_id,
            )?)
            .map_err(|error| format!("{error:?}"))?,
        DeviceTrustRegistryDecision::Rejected(
            DeviceTrustRegistryRejection::RevokedDeviceCannotRePair
        )
    );
    Ok(())
}
