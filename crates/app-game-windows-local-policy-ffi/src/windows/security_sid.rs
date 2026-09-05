use std::os::windows::ffi::OsStrExt;
use std::ptr;

use windows_sys::Win32::Foundation::{GetLastError, ERROR_INSUFFICIENT_BUFFER};
use windows_sys::Win32::Security::{
    CreateWellKnownSid, EqualSid, LookupAccountNameW, WinAuthenticatedUserSid,
    WinBuiltinAdministratorsSid, WinBuiltinUsersSid, WinLocalSystemSid, WinWorldSid, PSID,
    SECURITY_MAX_SID_SIZE,
};

use crate::{error::AppGameWindowsLocalPolicyError, Result};

const TRUSTED_INSTALLER: &str = "NT SERVICE\\TrustedInstaller";

pub(super) fn owner_is_trusted(owner: PSID) -> Result<bool> {
    for sid_type in [WinLocalSystemSid, WinBuiltinAdministratorsSid] {
        let sid = well_known_sid(sid_type)?;
        if unsafe { EqualSid(owner, sid.as_ptr().cast_mut().cast()) } != 0 {
            return Ok(true);
        }
    }
    let trusted_installer = trusted_installer_sid()?;
    Ok(unsafe { EqualSid(owner, trusted_installer.as_ptr().cast_mut().cast()) } != 0)
}

pub(super) fn unprivileged_sids() -> Result<[Vec<u8>; 3]> {
    Ok([
        well_known_sid(WinWorldSid)?,
        well_known_sid(WinBuiltinUsersSid)?,
        well_known_sid(WinAuthenticatedUserSid)?,
    ])
}

fn well_known_sid(sid_type: i32) -> Result<Vec<u8>> {
    let mut sid = vec![0u8; SECURITY_MAX_SID_SIZE as usize];
    let mut size = SECURITY_MAX_SID_SIZE;
    let ok = unsafe {
        CreateWellKnownSid(
            sid_type,
            ptr::null_mut(),
            sid.as_mut_ptr().cast(),
            &mut size,
        )
    };
    if ok == 0 {
        return Err(AppGameWindowsLocalPolicyError::WindowsApi(unsafe {
            GetLastError()
        }));
    }
    let size = usize::try_from(size)
        .map_err(|_size_error| AppGameWindowsLocalPolicyError::UntrustedAcl)?;
    if size == 0 || size > sid.len() {
        return Err(AppGameWindowsLocalPolicyError::UntrustedAcl);
    }
    sid.truncate(size);
    Ok(sid)
}

fn trusted_installer_sid() -> Result<Vec<u8>> {
    let account: Vec<u16> = std::ffi::OsStr::new(TRUSTED_INSTALLER)
        .encode_wide()
        .chain(core::iter::once(0))
        .collect();
    let mut sid_size = 0u32;
    let mut domain_size = 0u32;
    let mut sid_use = 0i32;
    unsafe {
        LookupAccountNameW(
            ptr::null(),
            account.as_ptr(),
            ptr::null_mut(),
            &mut sid_size,
            ptr::null_mut(),
            &mut domain_size,
            &mut sid_use,
        );
    }
    if unsafe { GetLastError() } != ERROR_INSUFFICIENT_BUFFER || sid_size == 0 {
        return Err(AppGameWindowsLocalPolicyError::UntrustedOwner);
    }
    lookup_account_sid(&account, sid_size, domain_size, &mut sid_use)
}

fn lookup_account_sid(
    account: &[u16],
    mut sid_size: u32,
    mut domain_size: u32,
    sid_use: &mut i32,
) -> Result<Vec<u8>> {
    let mut sid = vec![0u8; sid_size as usize];
    let mut domain = vec![0u16; domain_size as usize];
    let ok = unsafe {
        LookupAccountNameW(
            ptr::null(),
            account.as_ptr(),
            sid.as_mut_ptr().cast(),
            &mut sid_size,
            domain.as_mut_ptr(),
            &mut domain_size,
            sid_use,
        )
    };
    if ok == 0 {
        return Err(AppGameWindowsLocalPolicyError::WindowsApi(unsafe {
            GetLastError()
        }));
    }
    Ok(sid)
}
