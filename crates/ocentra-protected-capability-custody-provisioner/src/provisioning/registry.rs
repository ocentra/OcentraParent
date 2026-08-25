use ocentra_protected_capability_custody_windows_ffi::{
    Error as FfiError, OwnedRegistryChain, RegistryValueName,
};

use super::constants;
use super::enrollment::{self, EnrollmentSnapshot};
use super::error::{ExternalProvisioningBoundary, ProvisioningError};
use super::{registry_digest, registry_security};

pub(super) fn readback() -> Result<EnrollmentSnapshot, ProvisioningError> {
    observe(map_initial_error)
}

pub(super) fn revalidate(expected: &EnrollmentSnapshot) -> Result<(), ProvisioningError> {
    let current = observe(map_revalidation_error)?;
    if &current != expected {
        return Err(ProvisioningError::ExistingStateRejected);
    }
    Ok(())
}

fn observe(
    error_mapper: fn(FfiError) -> ProvisioningError,
) -> Result<EnrollmentSnapshot, ProvisioningError> {
    let path = constants::enrollment_path().map_err(error_mapper)?;
    let chain = OwnedRegistryChain::open_hklm(&path).map_err(error_mapper)?;
    chain.revalidate().map_err(error_mapper)?;
    let observations = chain.observations().map_err(error_mapper)?;
    registry_security::validate_chain(observations)?;
    let security_digest = registry_digest::security_digest(observations)?;
    let value_name =
        RegistryValueName::try_from_str(constants::ENROLLMENT_VALUE_NAME).map_err(error_mapper)?;
    let value = chain.observe_value(&value_name).map_err(error_mapper)?;
    if value.value().value_type() != constants::REG_BINARY || value.value().data().is_empty() {
        return Err(ProvisioningError::ExistingStateRejected);
    }
    let enrollment = enrollment::parse(value.value().data(), security_digest)?;
    if chain
        .reobserve_value(&value)
        .map_err(map_revalidation_error)?
        != value
    {
        return Err(ProvisioningError::ExistingStateRejected);
    }
    Ok(enrollment)
}

fn map_initial_error(error: FfiError) -> ProvisioningError {
    match error {
        FfiError::UnsupportedPlatform => ProvisioningError::UnsupportedPlatform,
        FfiError::Win32(2 | 3) => ProvisioningError::ExternalProvisioningRequired(
            ExternalProvisioningBoundary::EnrollmentRegistry,
        ),
        FfiError::Win32(_) | FfiError::Crypto(_) | FfiError::Tbs(_) => {
            ProvisioningError::PlatformObservationUnavailable
        }
        FfiError::InvalidInput(_) | FfiError::MalformedTpm | FfiError::BufferTooLarge => {
            ProvisioningError::ExistingStateRejected
        }
        FfiError::Tpm(_) | FfiError::CryptoPropertyViolation => {
            ProvisioningError::ExistingStateRejected
        }
    }
}

fn map_revalidation_error(error: FfiError) -> ProvisioningError {
    if matches!(error, FfiError::Win32(2 | 3)) {
        ProvisioningError::ExistingStateRejected
    } else {
        map_initial_error(error)
    }
}
