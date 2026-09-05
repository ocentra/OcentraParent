//! Core interpretation of the OS-owned TokenGroups observation.

use ocentra_protected_capability_custody_windows_ffi::OwnedToken;

use crate::platform::PlatformError;

use super::map_ffi_error;

pub(super) fn require_member(token: &OwnedToken, expected_sid: &[u8]) -> Result<(), PlatformError> {
    if expected_sid.is_empty() {
        return Err(PlatformError::InvalidAttestation);
    }
    let groups = token.group_sids().map_err(map_ffi_error)?;
    if groups.iter().any(|sid| sid == expected_sid) {
        Ok(())
    } else {
        Err(PlatformError::WrongBinding)
    }
}
