//! Fixed service-account SID resolution through the Windows authority.

use super::handles::last_error;
use crate::{Error, OwnedService, Result, MAX_BUFFER_BYTES, MAX_WIDE_CHARS};
use std::ptr;
use windows_sys::Win32::Security::{GetLengthSid, IsValidSid, LookupAccountNameW};

const ERROR_INSUFFICIENT_BUFFER: u32 = 122;

impl OwnedService {
    /// Resolves the service's virtual-service SID through LookupAccountNameW.
    /// The account name is derived from the retained SCM service name; callers
    /// cannot provide an SDDL fragment, SID, or alternate path.
    pub fn service_sid(&self) -> Result<Vec<u8>> {
        let account_name = format!("NT SERVICE\\{}", self.service_name.as_str());
        if account_name.encode_utf16().count() >= MAX_WIDE_CHARS {
            return Err(Error::BufferTooLarge);
        }
        let account_name: Vec<u16> = account_name
            .encode_utf16()
            .chain(core::iter::once(0))
            .collect();
        let mut sid_length = 0u32;
        let mut domain_length = 0u32;
        let mut use_type = 0;
        let first_ok = unsafe {
            LookupAccountNameW(
                ptr::null(),
                account_name.as_ptr(),
                ptr::null_mut(),
                &mut sid_length,
                ptr::null_mut(),
                &mut domain_length,
                &mut use_type,
            )
        };
        let first_error = last_error();
        if first_ok != 0 || first_error != ERROR_INSUFFICIENT_BUFFER {
            return Err(Error::Win32(first_error));
        }
        let sid_length = usize::try_from(sid_length)?;
        let domain_length = usize::try_from(domain_length)?;
        if sid_length == 0
            || sid_length > MAX_BUFFER_BYTES
            || domain_length == 0
            || domain_length > MAX_WIDE_CHARS
        {
            return Err(Error::BufferTooLarge);
        }
        let mut sid = vec![0u8; sid_length];
        let mut domain = vec![0u16; domain_length];
        let mut sid_length = u32::try_from(sid.len())?;
        let mut domain_length = u32::try_from(domain.len())?;
        let second_ok = unsafe {
            LookupAccountNameW(
                ptr::null(),
                account_name.as_ptr(),
                sid.as_mut_ptr().cast(),
                &mut sid_length,
                domain.as_mut_ptr(),
                &mut domain_length,
                &mut use_type,
            )
        };
        if second_ok == 0 {
            return Err(Error::Win32(last_error()));
        }
        let sid_length = usize::try_from(sid_length)?;
        let domain_length = usize::try_from(domain_length)?;
        if sid_length == 0 || sid_length > sid.len() || domain_length > domain.len() {
            return Err(Error::BufferTooLarge);
        }
        sid.truncate(sid_length);
        // Reuse the same bounded SID validator as token observations. A SID
        // returned by LookupAccountNameW is not trusted until its complete
        // self-relative shape is checked against the owned buffer.
        if sid.len() < 8 || unsafe { IsValidSid(sid.as_ptr().cast_mut().cast()) } == 0 {
            return Err(Error::InvalidInput(crate::InputFault::TokenSidInvalid));
        }
        let observed_length = unsafe { GetLengthSid(sid.as_ptr().cast_mut().cast()) } as usize;
        if observed_length == 0 || observed_length != sid.len() {
            return Err(Error::InvalidInput(
                crate::InputFault::TokenSidResponseTooLarge,
            ));
        }
        Ok(sid)
    }
}
