use std::fs;

use ocentra_family_identity_core::device_trust_lifecycle::{
    DeviceTrustLifecycleError, DeviceTrustLifecycleEventKind, DeviceTrustLifecycleRepository,
};
use ocentra_family_identity_core::trust_bootstrap::current_authority::CurrentParentDeviceTrustAuthoritySource;

#[derive(Debug)]
struct TestFailure;

fn repository(name: &str) -> Result<DeviceTrustLifecycleRepository, TestFailure> {
    let path = std::env::temp_dir().join(format!(
        "ocentra-device-trust-lifecycle-{name}-{}.sqlite",
        std::process::id()
    ));
    let _ = fs::remove_file(&path);
    DeviceTrustLifecycleRepository::open(path).map_err(|_error| TestFailure)
}

#[test]
fn revoke_invalidates_current_unseal_authority_and_writes_redacted_event() -> Result<(), TestFailure>
{
    let mut repository = repository("revoke")?;
    repository
        .register_parent_device(
            "parent-opaque",
            "device-opaque",
            1,
            "registration-correlation",
        )
        .map_err(|_error| TestFailure)?;
    repository
        .current_authorized_parent_device("parent-opaque", "device-opaque")
        .map_err(|_error| TestFailure)?;
    repository
        .revoke_or_reset(
            "parent-opaque",
            "device-opaque",
            false,
            "revoke-correlation",
        )
        .map_err(|_error| TestFailure)?;
    assert_eq!(repository.current_authorized_parent_device("parent-opaque", "device-opaque"), Err(ocentra_family_identity_core::trust_bootstrap::current_authority::CurrentParentDeviceTrustAuthorityError::NotTrusted));
    let events = repository.pending_events().map_err(|_error| TestFailure)?;
    assert_eq!(events.len(), 2);
    assert_eq!(events[1].kind, DeviceTrustLifecycleEventKind::Revoked);
    let serialized = serde_json::to_value(&events[1]).map_err(|_error| TestFailure)?;
    assert_eq!(serialized["redaction"], "sensitive-identifiers-omitted");
    assert_eq!(serialized.get("trustSubject"), None);
    assert_eq!(serialized.get("deviceRef"), None);
    Ok(())
}

#[test]
fn re_pair_requires_new_non_restored_installation_generation() -> Result<(), TestFailure> {
    let mut repository = repository("repair")?;
    repository
        .register_parent_device("parent", "device", 7, "registration")
        .map_err(|_error| TestFailure)?;
    repository
        .revoke_or_reset("parent", "device", true, "reset")
        .map_err(|_error| TestFailure)?;
    assert_eq!(
        repository.repair_with_new_installation("parent", "device", 7, "old-installation"),
        Err(DeviceTrustLifecycleError::InvalidGeneration)
    );
    repository
        .repair_with_new_installation("parent", "device", 8, "re-pair")
        .map_err(|_error| TestFailure)?;
    repository
        .current_authorized_parent_device("parent", "device")
        .map_err(|_error| TestFailure)?;
    assert_eq!(
        repository
            .current_authorized_parent_device("parent", "device")
            .map_err(|_error| TestFailure)?,
        ocentra_family_identity_core::trust_bootstrap::current_authority::CurrentParentDeviceTrustAuthority {
            lifecycle_generation: 3,
            installation_binding_generation: 8,
        }
    );
    let events = repository.pending_events().map_err(|_error| TestFailure)?;
    assert_eq!(events[2].kind, DeviceTrustLifecycleEventKind::Repaired);
    assert_eq!(events[2].lifecycle_generation, 3);
    assert_eq!(events[2].installation_binding_generation, 8);
    Ok(())
}
