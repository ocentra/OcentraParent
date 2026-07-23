use std::fmt;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Barrier};
use std::thread;

use ocentra_eventing::ids::CorrelationId;
use ocentra_family_identity_core::device_trust_authority::{
    verify_parent_device_trust_authority, DeviceTrustAuthorityInput,
};
use ocentra_family_identity_core::device_trust_registry::{
    DeviceTrustLifecycleState, DeviceTrustRegistry, DeviceTrustRegistryDecision,
    DeviceTrustRegistryRejection,
};
use ocentra_family_identity_core::family_identity::{
    ActorAccountState, ChildProfileBindingState, DeviceOwnershipScope, DeviceTrustState,
    HouseholdMembershipState, HouseholdRole, SessionFreshnessState,
};
use ocentra_family_identity_core::household_authority::{
    authorize_device_trust_action, DeviceTrustAuthorizationRequest, HouseholdAuthorityAction,
    HouseholdAuthorityInput, ParentStepUpAssertionSnapshot,
};
use ocentra_family_identity_core::parent_presence::{
    ParentPresenceChallenge, ParentPresenceVerificationInput,
};
use rusqlite::Connection;

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
    authority_for_family(store, action, device_id, "family")
}

fn authority_for_family(
    store: &TestStore,
    action: HouseholdAuthorityAction,
    device_id: &str,
    family_id: &str,
) -> Result<
    ocentra_family_identity_core::device_trust_authority::VerifiedParentDeviceTrustAuthority,
    String,
> {
    authority_for_family_actor(store, action, device_id, family_id, "parent-account")
}

fn authority_for_family_actor(
    store: &TestStore,
    action: HouseholdAuthorityAction,
    device_id: &str,
    family_id: &str,
    parent_account_id: &str,
) -> Result<
    ocentra_family_identity_core::device_trust_authority::VerifiedParentDeviceTrustAuthority,
    String,
> {
    let accepted = accepted_for_identity(store, action, family_id, parent_account_id)?;
    verify_parent_device_trust_authority(DeviceTrustAuthorityInput {
        parent_presence: accepted,
        household_authorization: authorization_grant(
            family_id,
            parent_account_id,
            device_id,
            action,
        )?,
    })
    .map_err(|error| format!("{error:?}"))
}

fn authorization_grant(
    family_id: &str,
    parent_account_id: &str,
    target_child_device_id: &str,
    action: HouseholdAuthorityAction,
) -> Result<
    ocentra_family_identity_core::household_authority::AcceptedDeviceTrustAuthorization,
    String,
> {
    authorize_device_trust_action(DeviceTrustAuthorizationRequest {
        family_id: family_id.to_owned(),
        parent_account_id: parent_account_id.to_owned(),
        target_child_device_id: target_child_device_id.to_owned(),
        authority: authorized_household_authority(action),
    })
    .map_err(|error| format!("{error:?}"))
}

fn accepted_for_family(
    store: &TestStore,
    action: HouseholdAuthorityAction,
    family_id: &str,
) -> Result<ocentra_family_identity_core::parent_presence::ParentPresenceVerificationAccepted, String>
{
    accepted_for_identity(store, action, family_id, "parent-account")
}

fn accepted_for_identity(
    store: &TestStore,
    action: HouseholdAuthorityAction,
    family_id: &str,
    parent_account_id: &str,
) -> Result<ocentra_family_identity_core::parent_presence::ParentPresenceVerificationAccepted, String>
{
    let id = NEXT_CASE_ID.fetch_add(1, Ordering::Relaxed);
    let challenge_ref = format!("device-trust-challenge-{id}");
    let nonce_ref = format!("device-trust-nonce-{id}");
    let challenge = ParentPresenceChallenge {
        challenge_ref: challenge_ref.clone(),
        nonce_ref: nonce_ref.clone(),
        family_id: family_id.to_owned(),
        parent_account_id: parent_account_id.to_owned(),
        privileged_action: action,
        action_device_id: "parent-action-device".to_owned(),
        action_device_child_profile_id: None,
        target_child_profile_id: Some("child-profile".to_owned()),
        target_child_device_id: Some("child-device".to_owned()),
        expires_at: ACCEPTED_EXPIRY.to_owned(),
    };
    let assertion = ParentStepUpAssertionSnapshot {
        family_id: challenge.family_id.clone(),
        parent_account_id: challenge.parent_account_id.clone(),
        action_device_id: challenge.action_device_id.clone(),
        action_device_child_profile_id: None,
        target_child_profile_id: challenge.target_child_profile_id.clone(),
        target_child_device_id: challenge.target_child_device_id.clone(),
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
    Ok(accepted)
}

fn authorized_household_authority(action: HouseholdAuthorityAction) -> HouseholdAuthorityInput {
    HouseholdAuthorityInput {
        actor_role: HouseholdRole::ParentOwner,
        same_family: true,
        actor_account_state: ActorAccountState::Active,
        membership_state: HouseholdMembershipState::Active,
        child_profile_binding_state: ChildProfileBindingState::Bound,
        device_ownership_scope: DeviceOwnershipScope::ChildProfileDevice,
        device_trust_state: DeviceTrustState::Trusted,
        session_freshness_state: SessionFreshnessState::Fresh,
        capability_granted: true,
        controller_lease_state: None,
        action,
    }
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
    assert_eq!(
        store
            .registry()?
            .record("parent-action-device")
            .map_err(|error| format!("{error:?}"))?,
        None,
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
fn household_authorization_grant_rejects_unauthorized_actor_before_device_trust_boundary(
) -> TestResult {
    let store = TestStore::new("authority-rejected")?;
    let accepted =
        accepted_for_family(&store, HouseholdAuthorityAction::PairChildDevice, "family")?;
    let mut unauthorized =
        authorized_household_authority(HouseholdAuthorityAction::PairChildDevice);
    unauthorized.same_family = false;
    let _accepted = accepted;
    assert!(
        authorize_device_trust_action(DeviceTrustAuthorizationRequest {
            family_id: "family".to_owned(),
            parent_account_id: "parent-account".to_owned(),
            target_child_device_id: "child-device".to_owned(),
            authority: unauthorized,
        })
        .is_err()
    );
    Ok(())
}

#[test]
fn device_trust_authority_rejects_a_target_not_bound_to_the_step_up_assertion() -> TestResult {
    let store = TestStore::new("target-device-mismatch")?;
    let accepted =
        accepted_for_family(&store, HouseholdAuthorityAction::PairChildDevice, "family")?;
    assert_eq!(
        verify_parent_device_trust_authority(DeviceTrustAuthorityInput {
            parent_presence: accepted,
            household_authorization: authorization_grant("family", "parent-account", "other-child-device", HouseholdAuthorityAction::PairChildDevice)?,
        }),
        Err(ocentra_family_identity_core::device_trust_authority::DeviceTrustAuthorityVerificationFailure::HouseholdAuthorizationBindingMismatch),
    );
    Ok(())
}

#[test]
fn device_trust_authority_rejects_grants_bound_to_another_family_or_parent() -> TestResult {
    let store = TestStore::new("grant-identity-mismatch")?;
    let accepted =
        accepted_for_family(&store, HouseholdAuthorityAction::PairChildDevice, "family")?;
    assert_eq!(
        verify_parent_device_trust_authority(DeviceTrustAuthorityInput {
            parent_presence: accepted,
            household_authorization: authorization_grant("other-family", "parent-account", "child-device", HouseholdAuthorityAction::PairChildDevice)?,
        }),
        Err(ocentra_family_identity_core::device_trust_authority::DeviceTrustAuthorityVerificationFailure::HouseholdAuthorizationBindingMismatch),
    );
    let accepted =
        accepted_for_family(&store, HouseholdAuthorityAction::PairChildDevice, "family")?;
    assert_eq!(
        verify_parent_device_trust_authority(DeviceTrustAuthorityInput {
            parent_presence: accepted,
            household_authorization: authorization_grant("family", "other-parent", "child-device", HouseholdAuthorityAction::PairChildDevice)?,
        }),
        Err(ocentra_family_identity_core::device_trust_authority::DeviceTrustAuthorityVerificationFailure::HouseholdAuthorizationBindingMismatch),
    );
    Ok(())
}

#[test]
fn copied_unsealed_trusted_row_is_rejected() -> TestResult {
    let store = TestStore::new("unsealed-trusted")?;
    let connection = Connection::open(&store.registry_path).map_err(|error| error.to_string())?;
    connection.execute_batch(
        "CREATE TABLE device_trust_registry (
            device_id TEXT PRIMARY KEY NOT NULL,
            family_id TEXT NOT NULL,
            parent_account_id TEXT NOT NULL,
            state TEXT NOT NULL CHECK (state IN ('pending-sealing', 'trusted', 'revoked', 'reset-required'))
        ) STRICT;
        INSERT INTO device_trust_registry VALUES ('child-device', 'family', 'parent-account', 'trusted');",
    ).map_err(|error| error.to_string())?;
    drop(connection);
    assert_eq!(
        store.registry()?.record("child-device"),
        Err(ocentra_family_identity_core::device_trust_registry::DeviceTrustRegistryFailure::StorageIntegrityRejected),
    );
    Ok(())
}

#[test]
fn household_conflict_cannot_transfer_or_revoke_an_existing_device() -> TestResult {
    let store = TestStore::new("ownership-conflict")?;
    let registry = store.registry()?;
    assert!(matches!(
        registry.apply_verified_parent_authority(authority_for_family(
            &store,
            HouseholdAuthorityAction::PairChildDevice,
            "child-device",
            "family-a",
        )?),
        Ok(DeviceTrustRegistryDecision::PendingSealing(_))
    ));
    assert_eq!(
        registry
            .apply_verified_parent_authority(authority_for_family(
                &store,
                HouseholdAuthorityAction::RevokeChildDevice,
                "child-device",
                "family-b",
            )?)
            .map_err(|error| format!("{error:?}"))?,
        DeviceTrustRegistryDecision::Rejected(DeviceTrustRegistryRejection::OwnershipConflict),
    );
    assert_eq!(
        registry
            .record("child-device")
            .map_err(|error| format!("{error:?}"))?,
        Some(
            ocentra_family_identity_core::device_trust_registry::DeviceTrustRegistryRecord {
                device_id: "child-device".to_owned(),
                state: DeviceTrustLifecycleState::PendingSealing,
            }
        ),
    );
    Ok(())
}

#[test]
fn authorized_coparent_can_revoke_without_reassigning_household_device_ownership() -> TestResult {
    let store = TestStore::new("coparent-revoke")?;
    let registry = store.registry()?;
    assert!(matches!(
        registry.apply_verified_parent_authority(authority_for_family_actor(
            &store,
            HouseholdAuthorityAction::PairChildDevice,
            "child-device",
            "family",
            "parent-owner",
        )?),
        Ok(DeviceTrustRegistryDecision::PendingSealing(_))
    ));
    assert_eq!(
        registry
            .apply_verified_parent_authority(authority_for_family_actor(
                &store,
                HouseholdAuthorityAction::RevokeChildDevice,
                "child-device",
                "family",
                "coparent-guardian",
            )?)
            .map_err(|error| format!("{error:?}"))?,
        DeviceTrustRegistryDecision::Revoked(
            ocentra_family_identity_core::device_trust_registry::DeviceTrustRegistryRecord {
                device_id: "child-device".to_owned(),
                state: DeviceTrustLifecycleState::Revoked,
            }
        ),
    );
    let connection = Connection::open(&store.registry_path).map_err(|error| error.to_string())?;
    let owner = connection
        .query_row(
            "SELECT parent_account_id FROM device_trust_registry WHERE device_id = 'child-device'",
            [],
            |row| row.get::<_, String>(0),
        )
        .map_err(|error| error.to_string())?;
    let actor = connection.query_row("SELECT acting_parent_account_id FROM device_trust_registry_journal WHERE action = 'revoke-child-device'", [], |row| row.get::<_, String>(0)).map_err(|error| error.to_string())?;
    assert_eq!(owner, "parent-owner");
    assert_eq!(actor, "coparent-guardian");
    Ok(())
}

#[test]
fn mutation_journal_is_committed_with_the_registry_state() -> TestResult {
    let store = TestStore::new("mutation-journal")?;
    let registry = store.registry()?;
    assert!(matches!(
        registry.apply_verified_parent_authority(authority_for(
            &store,
            HouseholdAuthorityAction::PairChildDevice,
            "child-device",
        )?),
        Ok(DeviceTrustRegistryDecision::PendingSealing(_))
    ));
    let connection = Connection::open(&store.registry_path).map_err(|error| error.to_string())?;
    let row = connection
        .query_row(
            "SELECT correlation_id, action, outcome, state FROM device_trust_registry_journal",
            [],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                ))
            },
        )
        .map_err(|error| error.to_string())?;
    assert_eq!(
        row,
        (
            "device-trust-registry-test".to_owned(),
            "pair-child-device".to_owned(),
            "accepted".to_owned(),
            "pending-sealing".to_owned()
        )
    );
    Ok(())
}

#[test]
fn concurrent_pair_and_revoke_leave_the_device_revoked() -> TestResult {
    let store = TestStore::new("pair-revoke-race")?;
    let device_id = "child-device";
    let pair_authority =
        authority_for(&store, HouseholdAuthorityAction::PairChildDevice, device_id)?;
    let revoke_authority = authority_for(
        &store,
        HouseholdAuthorityAction::RevokeChildDevice,
        device_id,
    )?;
    let barrier = Arc::new(Barrier::new(3));
    let pair_path = store.registry_path.clone();
    let pair_barrier = Arc::clone(&barrier);
    let pair = thread::spawn(move || {
        pair_barrier.wait();
        DeviceTrustRegistry::open(pair_path)
            .and_then(|registry| registry.apply_verified_parent_authority(pair_authority))
    });
    let revoke_path = store.registry_path.clone();
    let revoke_barrier = Arc::clone(&barrier);
    let revoke = thread::spawn(move || {
        revoke_barrier.wait();
        DeviceTrustRegistry::open(revoke_path)
            .and_then(|registry| registry.apply_verified_parent_authority(revoke_authority))
    });
    barrier.wait();
    pair.join()
        .map_err(|_panic| "pair thread panicked".to_owned())?
        .map_err(|error| format!("{error:?}"))?;
    revoke
        .join()
        .map_err(|_panic| "revoke thread panicked".to_owned())?
        .map_err(|error| format!("{error:?}"))?;
    assert_eq!(
        store
            .registry()?
            .record(device_id)
            .map_err(|error| format!("{error:?}"))?,
        Some(
            ocentra_family_identity_core::device_trust_registry::DeviceTrustRegistryRecord {
                device_id: device_id.to_owned(),
                state: DeviceTrustLifecycleState::Revoked,
            }
        ),
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
