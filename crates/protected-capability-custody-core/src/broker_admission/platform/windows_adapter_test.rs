use super::{preflight_service_start, PlatformError};

#[cfg(not(windows))]
use crate::platform::{identity::PhysicalDatabaseIdentity, PlatformCustodyOwner};

#[test]
fn service_preflight_rejects_an_untrusted_registry_identifier_before_admission() {
    let result = preflight_service_start("not-a-canonical-enrollment-id");

    #[cfg(windows)]
    assert!(matches!(result, Err(PlatformError::InvalidAttestation)));
    #[cfg(not(windows))]
    assert!(matches!(result, Err(PlatformError::Unavailable)));
}

#[test]
fn service_preflight_never_reports_ready_on_an_empty_registry_identifier() {
    let result = preflight_service_start("");

    #[cfg(windows)]
    assert!(matches!(result, Err(PlatformError::InvalidAttestation)));
    #[cfg(not(windows))]
    assert!(matches!(result, Err(PlatformError::Unavailable)));
}

#[cfg(not(windows))]
#[test]
fn database_owner_does_not_open_a_local_substitute_without_the_platform_adapter() {
    let identity = PhysicalDatabaseIdentity::from_parts([1_u8; 32], [2_u8; 32], [3_u8; 32]);
    assert!(identity.is_ok());
    let result = identity.and_then(|identity| {
        super::BrokerPlatformOwner::new().acquire_database(
            std::path::Path::new("/var/lib/ocentra/custody.sqlite"),
            identity,
        )
    });
    assert!(matches!(result, Err(PlatformError::Unavailable)));
}
