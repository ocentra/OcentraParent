use ocentra_protected_capability_custody_windows_ffi::OwnedTbsContext;

use crate::platform::PlatformError;

use super::enrollment::VerifiedEnrollment;
use super::map_ffi_error;

pub(super) fn preflight(enrollment: &VerifiedEnrollment) -> Result<(), PlatformError> {
    if !OwnedTbsContext::is_tpm_present().map_err(map_ffi_error)? {
        return Err(PlatformError::DeploymentRequired);
    }
    let context = OwnedTbsContext::open().map_err(map_ffi_error)?;
    let expected = enrollment.tpm();
    let first = context
        .observe_fixed_counter_public()
        .map_err(map_ffi_error)?;
    expected.verify(&first)?;
    let second = context
        .observe_fixed_counter_public()
        .map_err(map_ffi_error)?;
    if first != second {
        return Err(PlatformError::Tampered);
    }

    // The FFI boundary deliberately exposes no caller-authenticated NV read or
    // increment. A future installer-owned TPM policy session or non-exportable
    // TPM-bound key/handle must be added before this module can construct a
    // monotonic authority. Registry-held authValue/authorization bytes are not
    // an acceptable substitute, so startup remains fail closed here.
    Err(PlatformError::DeploymentRequired)
}
