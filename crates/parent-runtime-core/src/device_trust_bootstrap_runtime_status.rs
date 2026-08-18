use crate::device_trust_bootstrap_runtime::{
    ParentDeviceTrustBootstrapError, ParentDeviceTrustCommandError,
};

pub fn command_error_is_manual_required(error: &ParentDeviceTrustCommandError) -> bool {
    matches!(
        error,
        ParentDeviceTrustCommandError::Runtime(ParentDeviceTrustBootstrapError::ManualRequired(_))
            | ParentDeviceTrustCommandError::Runtime(ParentDeviceTrustBootstrapError::Custody(
                ocentra_storage_custody_core::windows_device_trust_custody::Error::Platform,
            ),)
    )
}
