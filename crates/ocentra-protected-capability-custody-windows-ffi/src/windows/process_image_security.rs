//! Security observation from a retained file or directory handle.

use super::super::super::handles::last_error;
use crate::security;
use crate::{Error, InputFault, Result, SecurityDescriptorObservation, MAX_BUFFER_BYTES};
use std::ptr;
use windows_sys::Win32::Foundation::HANDLE;
use windows_sys::Win32::Security::GetKernelObjectSecurity;

const ERROR_INSUFFICIENT_BUFFER: u32 = 122;
const OWNER_AND_DACL_SECURITY_INFORMATION: u32 = 1 | 4;

pub(super) fn query_file_security(handle: HANDLE) -> Result<SecurityDescriptorObservation> {
    let mut length = 0u32;
    let first = unsafe {
        GetKernelObjectSecurity(
            handle,
            OWNER_AND_DACL_SECURITY_INFORMATION,
            ptr::null_mut(),
            0,
            &mut length,
        )
    };
    if first != 0 || last_error() != ERROR_INSUFFICIENT_BUFFER {
        return Err(Error::Win32(last_error()));
    }
    let length = usize::try_from(length)?;
    if length == 0 || length > MAX_BUFFER_BYTES {
        return Err(Error::BufferTooLarge);
    }
    let mut descriptor = vec![0u8; length];
    let mut returned_length = u32::try_from(descriptor.len())?;
    if unsafe {
        GetKernelObjectSecurity(
            handle,
            OWNER_AND_DACL_SECURITY_INFORMATION,
            descriptor.as_mut_ptr() as *mut core::ffi::c_void,
            u32::try_from(descriptor.len())?,
            &mut returned_length,
        )
    } == 0
    {
        return Err(Error::Win32(last_error()));
    }
    let returned_length = usize::try_from(returned_length)?;
    if returned_length == 0 || returned_length > descriptor.len() {
        return Err(Error::InvalidInput(InputFault::DescriptorLengthInvalid));
    }
    descriptor.truncate(returned_length);
    security::copy_descriptor(descriptor)
}
