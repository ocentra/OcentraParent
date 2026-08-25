//! Bounded SID copying.

use crate::{Error, InputFault, Result, MAX_BUFFER_BYTES};
use windows_sys::Win32::Security::{GetLengthSid, IsValidSid, PSID};

pub(super) fn copy_sid_in_buffer(sid: PSID, buffer: *const u8, length: usize) -> Result<Vec<u8>> {
    let start = buffer as usize;
    let end = start.checked_add(length).ok_or(Error::BufferTooLarge)?;
    let sid_start = sid as usize;
    let header_end = sid_start.checked_add(8).ok_or(Error::BufferTooLarge)?;
    if sid.is_null() || sid_start < start || header_end > end {
        return Err(Error::InvalidInput(InputFault::SecuritySidOutsideBuffer));
    }
    let count = unsafe { *((sid as *const u8).add(1)) } as usize;
    let expected_length = 8usize
        .checked_add(count.checked_mul(4).ok_or(Error::BufferTooLarge)?)
        .ok_or(Error::BufferTooLarge)?;
    let sid_end = sid_start
        .checked_add(expected_length)
        .ok_or(Error::BufferTooLarge)?;
    if expected_length > MAX_BUFFER_BYTES || sid_end > end || unsafe { IsValidSid(sid) } == 0 {
        return Err(Error::InvalidInput(InputFault::SecuritySidInvalid));
    }
    let actual_length = unsafe { GetLengthSid(sid) } as usize;
    if actual_length != expected_length || actual_length > MAX_BUFFER_BYTES {
        return Err(Error::InvalidInput(InputFault::SecuritySidLengthInvalid));
    }
    let bytes = unsafe { core::slice::from_raw_parts(sid as *const u8, actual_length) };
    Ok(bytes.to_vec())
}
