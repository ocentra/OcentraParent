use crate::device_trust_bootstrap_runtime::{
    ParentDeviceTrustBootstrapError, ParentDeviceTrustCommandError,
};

use ocentra_storage_custody_core::windows_device_trust_custody::Error as CustodyError;

pub fn command_error_is_manual_required(error: &ParentDeviceTrustCommandError) -> bool {
    matches!(
        error,
        ParentDeviceTrustCommandError::Runtime(ParentDeviceTrustBootstrapError::ManualRequired(_))
            | ParentDeviceTrustCommandError::Runtime(ParentDeviceTrustBootstrapError::Custody(
                CustodyError::Platform,
            ),)
    )
}

pub fn startup_error_is_manual_required(error: &ParentDeviceTrustCommandError) -> bool {
    matches!(
        error,
        ParentDeviceTrustCommandError::Runtime(ParentDeviceTrustBootstrapError::ManualRequired(_))
    ) || startup_platform_unavailable_is_manual_required(error)
}

#[cfg(not(windows))]
fn startup_platform_unavailable_is_manual_required(error: &ParentDeviceTrustCommandError) -> bool {
    matches!(
        error,
        ParentDeviceTrustCommandError::Runtime(ParentDeviceTrustBootstrapError::Custody(
            CustodyError::Platform,
        ))
    )
}

#[cfg(windows)]
fn startup_platform_unavailable_is_manual_required(_error: &ParentDeviceTrustCommandError) -> bool {
    false
}
