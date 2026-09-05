use std::fs;

use ocentra_family_identity_core::device_trust_lifecycle::{
    DeviceTrustLifecycleError, DeviceTrustLifecycleRepository,
};
use ocentra_family_identity_core::trust_bootstrap::current_authority::{
    CurrentParentDeviceTrustAuthorityError, CurrentParentDeviceTrustAuthoritySource,
};

struct TestFailure {
    context: &'static str,
    detail: String,
}

impl TestFailure {
    fn new(context: &'static str, error: impl std::fmt::Debug) -> Self {
        Self {
            context,
            detail: format!("{error:?}"),
        }
    }
}

impl std::fmt::Debug for TestFailure {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}: {}", self.context, self.detail)
    }
}

fn repository(name: &str) -> Result<DeviceTrustLifecycleRepository, TestFailure> {
    let path = std::env::temp_dir().join(format!(
        "ocentra-device-trust-lifecycle-{name}-{}.sqlite",
        std::process::id()
    ));
    let _ = fs::remove_file(&path);
    let _ = fs::remove_file(path.with_extension("authority.json"));
    DeviceTrustLifecycleRepository::open(path)
        .map_err(|error| TestFailure::new("open device trust lifecycle repository", error))
}

#[test]
fn a_new_lifecycle_store_is_fail_closed_until_owner_provisioning_exists() -> Result<(), TestFailure>
{
    let repository = repository("empty")?;

    assert!(repository
        .pending_events()
        .map_err(|error| TestFailure::new("read pending lifecycle events", error))?
        .is_empty());
    assert_eq!(
        repository.current_authorized_parent_device("family", "parent", "device"),
        Err(CurrentParentDeviceTrustAuthorityError::NotTrusted)
    );
    assert_eq!(
        repository.current_signer_authority("family", "parent", "device", "child"),
        Err(DeviceTrustLifecycleError::SignerRegistrationMissing)
    );
    Ok(())
}

#[test]
fn malformed_current_authority_bindings_are_rejected_without_touching_storage(
) -> Result<(), TestFailure> {
    let repository = repository("invalid-input")?;

    assert_eq!(
        repository.current_authorized_parent_device(" ", "parent", "device"),
        Err(CurrentParentDeviceTrustAuthorityError::NotTrusted)
    );
    assert_eq!(
        repository.current_signer_authority("family", "parent", "device", ""),
        Err(DeviceTrustLifecycleError::InvalidIdentity)
    );
    assert!(repository
        .pending_events()
        .map_err(|error| TestFailure::new("read pending lifecycle events", error))?
        .is_empty());
    Ok(())
}

#[test]
fn an_existing_uninitialized_database_is_not_repaired_as_a_side_effect() -> Result<(), TestFailure>
{
    let path = std::env::temp_dir().join(format!(
        "ocentra-device-trust-lifecycle-uninitialized-{}.sqlite",
        std::process::id()
    ));
    let _ = fs::remove_file(&path);
    let _ = fs::remove_file(path.with_extension("authority.json"));
    let fixture = fs::File::create(&path)
        .map_err(|error| TestFailure::new("fixture file must be created", error))?;
    drop(fixture);

    assert!(matches!(
        DeviceTrustLifecycleRepository::open(&path),
        Err(DeviceTrustLifecycleError::Unavailable)
    ));

    let _ = fs::remove_file(&path);
    let _ = fs::remove_file(path.with_extension("authority.json"));
    Ok(())
}
