//! SCM service security-descriptor observation.

use super::super::super::handles::last_error;
use crate::security;
use crate::{Error, Result, SecurityDescriptorObservation, MAX_BUFFER_BYTES};
use std::ptr;
use windows_sys::Win32::System::Services::{QueryServiceObjectSecurity, SC_HANDLE};

const ERROR_INSUFFICIENT_BUFFER: u32 = 122;
const SECURITY_INFORMATION_OWNER_AND_DACL: u32 = 1 | 4;

pub(super) fn query_service_security(handle: SC_HANDLE) -> Result<SecurityDescriptorObservation> {
    let mut length = 0u32;
    let first = unsafe {
        QueryServiceObjectSecurity(
            handle,
            SECURITY_INFORMATION_OWNER_AND_DACL,
            ptr::null_mut(),
            0,
            &mut length,
        )
    };
    if first != 0 || last_error() != ERROR_INSUFFICIENT_BUFFER {
        return Err(Error::Win32(last_error()));
    }
    let length = usize::try_from(length).map_err(|_| Error::BufferTooLarge)?;
    if length == 0 || length > MAX_BUFFER_BYTES {
        return Err(Error::BufferTooLarge);
    }
    let mut descriptor = vec![0u8; length];
    let mut returned_length = u32::try_from(descriptor.len()).map_err(|_| Error::BufferTooLarge)?;
    if unsafe {
        QueryServiceObjectSecurity(
            handle,
            SECURITY_INFORMATION_OWNER_AND_DACL,
            descriptor.as_mut_ptr() as *mut core::ffi::c_void,
            u32::try_from(descriptor.len()).map_err(|_| Error::BufferTooLarge)?,
            &mut returned_length,
        )
    } == 0
    {
        return Err(Error::Win32(last_error()));
    }
    let returned_length = usize::try_from(returned_length).map_err(|_| Error::BufferTooLarge)?;
    if returned_length == 0 || returned_length > descriptor.len() {
        return Err(Error::InvalidInput(
            "service security response exceeds its buffer",
        ));
    }
    descriptor.truncate(returned_length);
    security::copy_descriptor(descriptor)
}
