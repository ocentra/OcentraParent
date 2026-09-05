#![cfg(test)]

use super::{preflight_service_start, PlatformError};

#[cfg(windows)]
#[test]
fn monotonic_provider_remains_deployment_required_without_an_installer_owned_authority() {
    assert!(matches!(
        super::anti_rollback::provider_available(),
        Err(PlatformError::DeploymentRequired)
    ));
}

#[cfg(windows)]
#[test]
fn malformed_registry_identity_is_rejected_before_tpm_provider_access() {
    assert!(matches!(
        preflight_service_start("invalid"),
        Err(PlatformError::InvalidAttestation)
    ));
}

#[cfg(not(windows))]
#[test]
fn unsupported_platform_is_typed_before_tpm_provider_access() {
    assert!(matches!(
        preflight_service_start("any"),
        Err(PlatformError::Unavailable)
    ));
}
