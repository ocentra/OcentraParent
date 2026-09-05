use std::fs::File;
use std::os::windows::io::AsRawHandle;
use std::ptr;

use windows_sys::Win32::Foundation::{LocalFree, HANDLE};
use windows_sys::Win32::Security::Authorization::{GetSecurityInfo, SE_FILE_OBJECT};
use windows_sys::Win32::Security::{
    ACL, DACL_SECURITY_INFORMATION, OWNER_SECURITY_INFORMATION, PSECURITY_DESCRIPTOR, PSID,
};

use super::{security_acl, security_sid};
use crate::{error::AppGameWindowsLocalPolicyError, Result};

struct LocalSecurityDescriptor(PSECURITY_DESCRIPTOR);

impl Drop for LocalSecurityDescriptor {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe {
                LocalFree(self.0);
            }
        }
    }
}

pub(super) fn verify_owner_and_acl(file: &File) -> Result<()> {
    let mut owner: PSID = ptr::null_mut();
    let mut dacl: *mut ACL = ptr::null_mut();
    let mut descriptor: PSECURITY_DESCRIPTOR = ptr::null_mut();
    let status = unsafe {
        GetSecurityInfo(
            file.as_raw_handle() as HANDLE,
            SE_FILE_OBJECT,
            OWNER_SECURITY_INFORMATION | DACL_SECURITY_INFORMATION,
            &mut owner,
            ptr::null_mut(),
            &mut dacl,
            ptr::null_mut(),
            &mut descriptor,
        )
    };
    if status != 0 {
        return Err(AppGameWindowsLocalPolicyError::WindowsApi(status));
    }
    let _descriptor = LocalSecurityDescriptor(descriptor);
    validate_security(owner, dacl)
}

fn validate_security(owner: PSID, dacl: *const ACL) -> Result<()> {
    if owner.is_null() || !security_sid::owner_is_trusted(owner)? {
        return Err(AppGameWindowsLocalPolicyError::UntrustedOwner);
    }
    if dacl.is_null() {
        return Err(AppGameWindowsLocalPolicyError::UntrustedAcl);
    }
    security_acl::reject_unprivileged_write_access(dacl)
}
