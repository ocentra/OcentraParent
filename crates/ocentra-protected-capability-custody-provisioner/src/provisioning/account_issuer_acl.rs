//! Read-only AccountIssuer key and service-ACL ceremony boundary.
//!
//! The installer/OEM owns creation and ACL mutation. This module only opens
//! the fixed key, derives the service SID from the retained SCM service, and
//! requires the CNG capability to revalidate the exact service-only ACL.

use ocentra_protected_capability_custody_windows_ffi::{
    AccountIssuerP256Key, Error as FfiError, OwnedScManager, ServiceName,
};

use super::constants;
use super::error::{ExternalProvisioningBoundary, ProvisioningError};

const NTE_BAD_KEYSET: u32 = 0x8009_0016;

pub(super) fn readback() -> Result<(), ProvisioningError> {
    observe(map_initial_error, map_initial_key_error)
}

pub(super) fn revalidate() -> Result<(), ProvisioningError> {
    observe(map_revalidation_error, map_revalidation_error)
}

fn observe(
    error_mapper: fn(FfiError) -> ProvisioningError,
    key_error_mapper: fn(FfiError) -> ProvisioningError,
) -> Result<(), ProvisioningError> {
    let manager = OwnedScManager::open().map_err(error_mapper)?;
    let service_name =
        ServiceName::try_from_str(constants::FIXED_SERVICE_NAME).map_err(error_mapper)?;
    let service = manager.open_service(&service_name).map_err(error_mapper)?;
    let mut key = AccountIssuerP256Key::open_machine().map_err(key_error_mapper)?;
    key.bind_to_service(&service).map_err(error_mapper)?;
    key.revalidate().map_err(error_mapper)
}

fn map_initial_error(error: FfiError) -> ProvisioningError {
    match error {
        FfiError::UnsupportedPlatform => ProvisioningError::UnsupportedPlatform,
        FfiError::Win32(2 | 3) => ProvisioningError::ExternalProvisioningRequired(
            ExternalProvisioningBoundary::BrokerService,
        ),
        FfiError::InvalidInput(_)
        | FfiError::CryptoPropertyViolation
        | FfiError::BufferTooLarge
        | FfiError::MalformedTpm => ProvisioningError::ExistingStateRejected,
        FfiError::Crypto(_) | FfiError::Win32(_) | FfiError::Tpm(_) | FfiError::Tbs(_) => {
            ProvisioningError::PlatformObservationUnavailable
        }
    }
}

fn map_initial_key_error(error: FfiError) -> ProvisioningError {
    match error {
        FfiError::Crypto(NTE_BAD_KEYSET) | FfiError::Win32(2 | 3) => {
            ProvisioningError::ExternalProvisioningRequired(
                ExternalProvisioningBoundary::AccountIssuerKey,
            )
        }
        other => map_initial_error(other),
    }
}

fn map_revalidation_error(error: FfiError) -> ProvisioningError {
    if matches!(
        error,
        FfiError::Win32(2 | 3) | FfiError::Crypto(NTE_BAD_KEYSET)
    ) {
        ProvisioningError::ExistingStateRejected
    } else {
        map_initial_error(error)
    }
}
