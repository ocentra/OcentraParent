//! Extended SCM launch-protection observation.

use super::config;
use crate::{Error, InputFault, Result};
use std::ptr;
use windows_sys::Win32::System::Services::{
    SC_HANDLE, SERVICE_CONFIG_LAUNCH_PROTECTED, SERVICE_LAUNCH_PROTECTED_INFO,
};

pub(super) fn query_launch_protected(handle: SC_HANDLE) -> Result<u32> {
    let buffer = config::query_service_config2(handle, SERVICE_CONFIG_LAUNCH_PROTECTED)?;
    if buffer.len() != core::mem::size_of::<SERVICE_LAUNCH_PROTECTED_INFO>() {
        return Err(Error::InvalidInput(
            InputFault::ServiceLaunchProtectionSizeInvalid,
        ));
    }
    Ok(
        unsafe { ptr::read_unaligned(buffer.as_ptr() as *const SERVICE_LAUNCH_PROTECTED_INFO) }
            .dwLaunchProtected,
    )
}
