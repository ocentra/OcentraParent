//! Bounded TokenGroups observation for an already-owned token handle.

use super::super::super::handles::last_error;
use super::sid;
use crate::{Error, InputFault, OwnedToken, Result, MAX_BUFFER_BYTES};
use core::mem::{offset_of, size_of};
use core::ptr;
use windows_sys::Win32::Security::{
    GetTokenInformation, TokenGroups, SID_AND_ATTRIBUTES, TOKEN_GROUPS, TOKEN_INFORMATION_CLASS,
};

const ERROR_INSUFFICIENT_BUFFER: u32 = 122;
const TOKEN_INFORMATION_MAX: usize = 64 * 1024;

impl OwnedToken {
    /// Copies every SID from the OS token's TokenGroups information while the
    /// owning token handle remains live. The caller cannot supply a SID,
    /// buffer, or token handle to this observation boundary.
    pub fn group_sids(&self) -> Result<Vec<Vec<u8>>> {
        let bytes = query_token_groups(self.inner.handle.raw())?;
        if bytes.len() < size_of::<TOKEN_GROUPS>() {
            return Err(Error::InvalidInput(
                InputFault::TokenInformationResponseTooLarge,
            ));
        }
        let groups = unsafe { ptr::read_unaligned(bytes.as_ptr() as *const TOKEN_GROUPS) };
        let count = usize::try_from(groups.GroupCount)?;
        let entry_offset = offset_of!(TOKEN_GROUPS, Groups);
        let entry_bytes = count
            .checked_mul(size_of::<SID_AND_ATTRIBUTES>())
            .ok_or(Error::BufferTooLarge)?;
        let required = entry_offset
            .checked_add(entry_bytes)
            .ok_or(Error::BufferTooLarge)?;
        if required > bytes.len() || required > MAX_BUFFER_BYTES {
            return Err(Error::InvalidInput(
                InputFault::TokenInformationResponseTooLarge,
            ));
        }

        let entries = unsafe {
            bytes
                .as_ptr()
                .add(entry_offset)
                .cast::<SID_AND_ATTRIBUTES>()
        };
        let mut sids = Vec::with_capacity(count);
        for index in 0..count {
            let entry = unsafe { ptr::read_unaligned(entries.add(index)) };
            sids.push(sid::copy_sid_in_buffer(entry.Sid, &bytes)?);
        }
        Ok(sids)
    }
}

fn query_token_groups(handle: windows_sys::Win32::Foundation::HANDLE) -> Result<Vec<u8>> {
    let mut length = 0u32;
    let first_ok = unsafe {
        GetTokenInformation(
            handle,
            TokenGroups as TOKEN_INFORMATION_CLASS,
            ptr::null_mut(),
            0,
            &mut length,
        )
    };
    let first_error = last_error();
    if first_ok != 0 || first_error != ERROR_INSUFFICIENT_BUFFER {
        return Err(Error::Win32(first_error));
    }
    let length = usize::try_from(length)?;
    if length == 0 || length > TOKEN_INFORMATION_MAX {
        return Err(Error::BufferTooLarge);
    }
    let mut buffer = vec![0u8; length];
    let mut returned_length = u32::try_from(buffer.len())?;
    let ok = unsafe {
        GetTokenInformation(
            handle,
            TokenGroups as TOKEN_INFORMATION_CLASS,
            buffer.as_mut_ptr().cast(),
            u32::try_from(buffer.len())?,
            &mut returned_length,
        )
    };
    if ok == 0 {
        return Err(Error::Win32(last_error()));
    }
    let returned_length = usize::try_from(returned_length)?;
    if returned_length == 0 || returned_length > buffer.len() {
        return Err(Error::InvalidInput(
            InputFault::TokenInformationResponseTooLarge,
        ));
    }
    buffer.truncate(returned_length);
    Ok(buffer)
}
