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
    let _ = fs::remove_file(path.with_extension("authority.json"));
    DeviceTrustLifecycleRepository::open(path).map_err(|_error| TestFailure)
}

#[test]
fn revoke_invalidates_current_unseal_authority_and_writes_redacted_event() -> Result<(), TestFailure>
{
    let mut repository = repository("revoke")?;
    repository
        .register_parent_device(
            "family-opaque",
            "parent-opaque",
            "device-opaque",
            1,
            "registration-correlation",
        )
        .map_err(|_error| TestFailure)?;
    repository
        .current_authorized_parent_device("family-opaque", "parent-opaque", "device-opaque")
        .expect_err("pending registration must not be current authority");
    repository
        .activate_after_sealing(
            "family-opaque",
            "parent-opaque",
            "device-opaque",
            "activation",
        )
        .map_err(|_error| TestFailure)?;
    repository
        .current_authorized_parent_device("family-opaque", "parent-opaque", "device-opaque")
        .map_err(|_error| TestFailure)?;
    repository
        .revoke_or_reset(
            "family-opaque",
            "parent-opaque",
            "device-opaque",
            false,
            "revoke-correlation",
        )
        .map_err(|_error| TestFailure)?;
    assert_eq!(repository.current_authorized_parent_device("family-opaque", "parent-opaque", "device-opaque"), Err(ocentra_family_identity_core::trust_bootstrap::current_authority::CurrentParentDeviceTrustAuthorityError::NotTrusted));
    let events = repository.pending_events().map_err(|_error| TestFailure)?;
    assert_eq!(events.len(), 3);
    assert_eq!(events[2].kind, DeviceTrustLifecycleEventKind::Revoked);
    assert_ne!(events[0].event_id, events[2].event_id);
    assert_ne!(events[2].device_binding, "");
    let serialized = serde_json::to_value(&events[2]).map_err(|_error| TestFailure)?;
    assert_eq!(serialized["redaction"], "sensitive-identifiers-omitted");
    assert_eq!(serialized.get("trustSubject"), None);
    assert_eq!(serialized.get("deviceRef"), None);
    Ok(())
}

#[test]
fn re_pair_requires_new_non_restored_installation_generation() -> Result<(), TestFailure> {
    let mut repository = repository("repair")?;
    repository
        .register_parent_device("family", "parent", "device", 7, "registration")
        .map_err(|_error| TestFailure)?;
    repository
        .revoke_or_reset("family", "parent", "device", true, "reset")
        .map_err(|_error| TestFailure)?;
    assert_eq!(
        repository.repair_with_new_installation(
            "family",
            "parent",
            "device",
            7,
            "old-installation"
        ),
        Err(DeviceTrustLifecycleError::InvalidGeneration)
    );
    repository
        .repair_with_new_installation("family", "parent", "device", 8, "re-pair")
        .map_err(|_error| TestFailure)?;
    repository
        .current_authorized_parent_device("family", "parent", "device")
        .map_err(|_error| TestFailure)?;
    assert_eq!(
        repository
            .current_authorized_parent_device("family", "parent", "device")
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

#[test]
fn revoked_devices_cannot_be_repaired_without_reset_state() -> Result<(), TestFailure> {
    let mut repository = repository("revoked-repair")?;
    repository
        .register_parent_device("family", "parent", "device", 1, "registration")
        .map_err(|_error| TestFailure)?;
    repository
        .revoke_or_reset("family", "parent", "device", false, "revoke")
        .map_err(|_error| TestFailure)?;
    assert_eq!(
        repository.repair_with_new_installation("family", "parent", "device", 2, "repair-revoked",),
        Err(DeviceTrustLifecycleError::RevokedDevice)
    );
    Ok(())
}

#[test]
fn restored_database_without_platform_authority_is_unavailable() -> Result<(), TestFailure> {
    let source_path = std::env::temp_dir().join(format!(
        "ocentra-device-trust-lifecycle-restore-source-{}.sqlite",
        std::process::id()
    ));
    let restored_path = std::env::temp_dir().join(format!(
        "ocentra-device-trust-lifecycle-restore-copy-{}.sqlite",
        std::process::id()
    ));
    let _ = fs::remove_file(&source_path);
    let _ = fs::remove_file(source_path.with_extension("authority.json"));
    let _ = fs::remove_file(&restored_path);
    let _ = fs::remove_file(restored_path.with_extension("authority.json"));
    let mut repository =
        DeviceTrustLifecycleRepository::open(&source_path).map_err(|_error| TestFailure)?;
    repository
        .register_parent_device("family", "parent", "device", 1, "registration")
        .map_err(|_error| TestFailure)?;
    repository
        .activate_after_sealing("family", "parent", "device", "activation")
        .map_err(|_error| TestFailure)?;
    drop(repository);
    fs::copy(&source_path, &restored_path).map_err(|_error| TestFailure)?;
    assert!(matches!(
        DeviceTrustLifecycleRepository::open(&restored_path),
        Err(DeviceTrustLifecycleError::Unavailable)
    ));
    let _ = fs::remove_file(&source_path);
    let _ = fs::remove_file(source_path.with_extension("authority.json"));
    let _ = fs::remove_file(&restored_path);
    let _ = fs::remove_file(restored_path.with_extension("authority.json"));
    Ok(())
}
