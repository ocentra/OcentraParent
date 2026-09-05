use ocentra_protected_capability_custody_windows_ffi::Error as FfiError;

use super::error::{ExternalProvisioningBoundary, ProvisioningError};

pub(super) fn initial(error: FfiError) -> ProvisioningError {
    match error {
        FfiError::UnsupportedPlatform => ProvisioningError::UnsupportedPlatform,
        FfiError::Win32(2 | 3 | 1060) => ProvisioningError::ExternalProvisioningRequired(
            ExternalProvisioningBoundary::BrokerService,
        ),
        FfiError::Win32(_) | FfiError::Tpm(_) | FfiError::Tbs(_) | FfiError::Crypto(_) => {
            ProvisioningError::PlatformObservationUnavailable
        }
        FfiError::InvalidInput(_)
        | FfiError::MalformedTpm
        | FfiError::BufferTooLarge
        | FfiError::CryptoPropertyViolation => ProvisioningError::ExistingStateRejected,
    }
}

pub(super) fn revalidation(error: FfiError) -> ProvisioningError {
    if matches!(error, FfiError::Win32(2 | 3 | 1060)) {
        ProvisioningError::ExistingStateRejected
    } else {
        initial(error)
    }
}
