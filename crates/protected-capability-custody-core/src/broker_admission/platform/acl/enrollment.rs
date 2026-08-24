use windows_acl::acl::{ACLEntry, AceType, ACL};

use crate::platform::PlatformError;

use super::{
    current_user_sid, grants_write, is_allow, INHERIT_ONLY_ACE, SYSTEM_SID, TRUSTED_INSTALLER_SID,
};

pub(crate) fn validate_enrollment_store(key: &winreg::RegKey) -> Result<(), PlatformError> {
    let acl =
        ACL::from_registry_handle(key.raw_handle().cast(), false, false).map_err(map_acl_error)?;
    validate_enrollment_entries(acl.all().map_err(map_acl_error)?)
}

fn validate_enrollment_entries(entries: Vec<ACLEntry>) -> Result<(), PlatformError> {
    if entries.is_empty() {
        return Err(PlatformError::Tampered);
    }
    // The broker is the only runtime reader. The installer identity is
    // represented by TrustedInstaller; no administrator, LocalService,
    // owner, or interactive user may rewrite enrollment.
    if current_user_sid()? != SYSTEM_SID {
        return Err(PlatformError::Tampered);
    }
    let mut system_read = false;
    for entry in entries {
        if entry.entry_type == AceType::Unknown {
            return Err(PlatformError::Tampered);
        }
        if !is_allow(&entry) || entry.flags & INHERIT_ONLY_ACE != 0 {
            continue;
        }
        if entry.string_sid == SYSTEM_SID && !grants_write(entry.mask) {
            system_read = true;
            continue;
        }
        if entry.string_sid == TRUSTED_INSTALLER_SID && grants_write(entry.mask) {
            continue;
        }
        return Err(PlatformError::Tampered);
    }
    if !system_read {
        return Err(PlatformError::Tampered);
    }
    Ok(())
}

fn map_acl_error(_error: u32) -> PlatformError {
    PlatformError::Unavailable
}
