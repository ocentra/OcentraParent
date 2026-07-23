use ocentra_family_identity_core::device_trust_registry::{
    DeviceTrustRegistryDecision, DeviceTrustRegistryFailure, DeviceTrustRegistryRejection,
};
use ocentra_family_identity_core::household_authority::HouseholdAuthorityAction;

use super::device_trust_registry::{authority_for, TestResult, TestStore};

#[test]
fn unknown_device_revocation_does_not_reserve_an_identifier() -> TestResult {
    let store = TestStore::new("unknown-revoke")?;
    let registry = store.registry()?;
    assert_eq!(
        registry
            .apply_verified_parent_authority(authority_for(
                &store,
                HouseholdAuthorityAction::RevokeChildDevice,
                "child-device",
            )?)
            .map_err(|error| format!("{error:?}"))?,
        DeviceTrustRegistryDecision::Rejected(DeviceTrustRegistryRejection::UnknownDevice),
    );
    assert!(matches!(
        registry
            .apply_verified_parent_authority(authority_for(
                &store,
                HouseholdAuthorityAction::PairChildDevice,
                "child-device",
            )?)
            .map_err(|error| format!("{error:?}"))?,
        DeviceTrustRegistryDecision::PendingSealing(_)
    ));
    Ok(())
}

#[test]
fn state_reads_are_household_scoped_and_reject_unverifiable_revocation() -> TestResult {
    let store = TestStore::new("scoped-read")?;
    let registry = store.registry()?;
    assert!(matches!(
        registry
            .apply_verified_parent_authority(authority_for(
                &store,
                HouseholdAuthorityAction::PairChildDevice,
                "child-device",
            )?)
            .map_err(|error| format!("{error:?}"))?,
        DeviceTrustRegistryDecision::PendingSealing(_)
    ));
    assert_eq!(
        registry
            .record("other-family", "child-device")
            .map_err(|error| format!("{error:?}"))?,
        None
    );
    assert!(matches!(
        registry
            .apply_verified_parent_authority(authority_for(
                &store,
                HouseholdAuthorityAction::RevokeChildDevice,
                "child-device",
            )?)
            .map_err(|error| format!("{error:?}"))?,
        DeviceTrustRegistryDecision::Revoked(_)
    ));
    assert_eq!(
        registry.record("family", "child-device"),
        Err(DeviceTrustRegistryFailure::StorageIntegrityRejected),
    );
    Ok(())
}
