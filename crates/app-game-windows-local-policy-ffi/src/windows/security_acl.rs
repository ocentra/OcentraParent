use windows_sys::Win32::Foundation::{GENERIC_ALL, GENERIC_WRITE};
use windows_sys::Win32::Security::Authorization::{
    BuildTrusteeWithSidW, GetEffectiveRightsFromAclW, TRUSTEE_W,
};
use windows_sys::Win32::Security::{ACL, PSID};
use windows_sys::Win32::Storage::FileSystem::{
    DELETE, FILE_APPEND_DATA, FILE_DELETE_CHILD, FILE_WRITE_ATTRIBUTES, FILE_WRITE_DATA,
    FILE_WRITE_EA, WRITE_DAC, WRITE_OWNER,
};

use super::security_sid;
use crate::{error::AppGameWindowsLocalPolicyError, Result};

const FORBIDDEN_UNPRIVILEGED_RIGHTS: u32 = FILE_WRITE_DATA
    | FILE_APPEND_DATA
    | FILE_WRITE_EA
    | FILE_WRITE_ATTRIBUTES
    | FILE_DELETE_CHILD
    | DELETE
    | WRITE_DAC
    | WRITE_OWNER
    | GENERIC_WRITE
    | GENERIC_ALL;

pub(super) fn reject_unprivileged_write_access(dacl: *const ACL) -> Result<()> {
    for sid in security_sid::unprivileged_sids()? {
        if effective_rights(dacl, sid.as_ptr().cast_mut().cast())? & FORBIDDEN_UNPRIVILEGED_RIGHTS
            != 0
        {
            return Err(AppGameWindowsLocalPolicyError::UntrustedAcl);
        }
    }
    Ok(())
}

fn effective_rights(dacl: *const ACL, sid: PSID) -> Result<u32> {
    let mut trustee = TRUSTEE_W::default();
    unsafe {
        BuildTrusteeWithSidW(&mut trustee, sid);
    }
    let mut rights = 0u32;
    let status = unsafe { GetEffectiveRightsFromAclW(dacl, &trustee, &mut rights) };
    if status != 0 {
        return Err(AppGameWindowsLocalPolicyError::WindowsApi(status));
    }
    Ok(rights)
}
