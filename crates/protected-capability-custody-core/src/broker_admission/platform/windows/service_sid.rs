//! Core interpretation of the retained SCM service-SID observation.

use ocentra_protected_capability_custody_windows_ffi::OwnedService;

use crate::platform::PlatformError;

use super::map_ffi_error;

pub(super) fn observe(service: &OwnedService) -> Result<Vec<u8>, PlatformError> {
    let sid = service.service_sid().map_err(map_ffi_error)?;
    if sid.is_empty() {
        return Err(PlatformError::InvalidAttestation);
    }
    Ok(sid)
}
