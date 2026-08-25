//! Registry security descriptor retrieval from retained HKEYs.

use crate::{Error, InputFault, Result, MAX_BUFFER_BYTES};
use std::ptr;
use windows_sys::Win32::System::Registry::{RegGetKeySecurity, HKEY};

const ERROR_INSUFFICIENT_BUFFER: u32 = 122;
const SECURITY_INFORMATION_OWNER_AND_DACL: u32 = 1 | 4;

pub(super) fn query_registry_security(key: HKEY) -> Result<Vec<u8>> {
    let mut length = 0u32;
    let first = unsafe {
        RegGetKeySecurity(
            key,
            SECURITY_INFORMATION_OWNER_AND_DACL,
            ptr::null_mut(),
            &mut length,
        )
    };
    if first != ERROR_INSUFFICIENT_BUFFER {
        return Err(Error::Win32(first));
    }
    let length = usize::try_from(length)?;
    if length == 0 || length > MAX_BUFFER_BYTES {
        return Err(Error::BufferTooLarge);
    }
    let mut descriptor = vec![0u8; length];
    let mut returned_length = u32::try_from(descriptor.len())?;
    let status = unsafe {
        RegGetKeySecurity(
            key,
            SECURITY_INFORMATION_OWNER_AND_DACL,
            descriptor.as_mut_ptr() as *mut core::ffi::c_void,
            &mut returned_length,
        )
    };
    if status != 0 {
        return Err(Error::Win32(status));
    }
    let returned_length = usize::try_from(returned_length)?;
    if returned_length == 0 || returned_length > descriptor.len() {
        return Err(Error::InvalidInput(InputFault::DescriptorLengthInvalid));
    }
    descriptor.truncate(returned_length);
    Ok(descriptor)
}
