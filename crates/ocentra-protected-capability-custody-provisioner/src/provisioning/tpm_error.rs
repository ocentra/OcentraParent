use ocentra_protected_capability_custody_windows_ffi::Error as FfiError;

use super::error::{ExternalProvisioningBoundary, ProvisioningError};

const TPM_RC_HANDLE_FIXED_NV_INDEX: u32 = 0x0000_018b;

pub(super) fn operation(error: FfiError) -> ProvisioningError {
    match error {
        FfiError::UnsupportedPlatform => ProvisioningError::UnsupportedPlatform,
        FfiError::Tpm(_)
        | FfiError::CryptoPropertyViolation
        | FfiError::InvalidInput(_)
        | FfiError::MalformedTpm
        | FfiError::BufferTooLarge => ProvisioningError::ExistingStateRejected,
        FfiError::Tbs(_) | FfiError::Win32(_) | FfiError::Crypto(_) => {
            ProvisioningError::PlatformObservationUnavailable
        }
    }
}

pub(super) fn tbs(error: FfiError) -> ProvisioningError {
    operation(error)
}

pub(super) fn public_observation(error: FfiError) -> ProvisioningError {
    match error {
        FfiError::Tpm(TPM_RC_HANDLE_FIXED_NV_INDEX) => {
            ProvisioningError::ExternalProvisioningRequired(
                ExternalProvisioningBoundary::FixedTpmCounter,
            )
        }
        FfiError::Tpm(_) => ProvisioningError::PlatformObservationUnavailable,
        other => operation(other),
    }
}

pub(super) fn public_revalidation(error: FfiError) -> ProvisioningError {
    if matches!(error, FfiError::Tpm(TPM_RC_HANDLE_FIXED_NV_INDEX)) {
        ProvisioningError::ExistingStateRejected
    } else {
        public_observation(error)
    }
}
