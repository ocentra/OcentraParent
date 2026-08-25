//! Token SID bounds and identity mechanics.

use crate::{Error, Result, MAX_BUFFER_BYTES};
use windows_sys::Win32::Security::{
    GetLengthSid, GetSidSubAuthority, GetSidSubAuthorityCount, IsValidSid, PSID,
};

pub(super) fn integrity_rid_in_buffer(sid: PSID, buffer: &[u8]) -> Result<u32> {
    let sid = validate_sid_in_buffer(sid, buffer)?;
    let count = unsafe { *GetSidSubAuthorityCount(sid) };
    if count == 0 {
        return Err(Error::InvalidInput("token integrity SID has no RID"));
    }
    Ok(unsafe { *GetSidSubAuthority(sid, u32::from(count - 1)) })
}

pub(super) fn copy_sid_in_buffer(sid: PSID, buffer: &[u8]) -> Result<Vec<u8>> {
    let sid = validate_sid_in_buffer(sid, buffer)?;
    let sid_length = unsafe { GetLengthSid(sid) } as usize;
    if sid_length == 0 || sid_length > MAX_BUFFER_BYTES {
        return Err(Error::BufferTooLarge);
    }
    let buffer_end = (buffer.as_ptr() as usize)
        .checked_add(buffer.len())
        .ok_or(Error::BufferTooLarge)?;
    let sid_end = (sid as usize)
        .checked_add(sid_length)
        .ok_or(Error::BufferTooLarge)?;
    if sid_end > buffer_end {
        return Err(Error::InvalidInput("token SID exceeds its response buffer"));
    }
    Ok(unsafe { core::slice::from_raw_parts(sid as *const u8, sid_length) }.to_vec())
}

fn validate_sid_in_buffer(sid: PSID, buffer: &[u8]) -> Result<PSID> {
    let start = buffer.as_ptr() as usize;
    let end = start
        .checked_add(buffer.len())
        .ok_or(Error::BufferTooLarge)?;
    let sid_start = sid as usize;
    let header_end = sid_start.checked_add(8).ok_or(Error::BufferTooLarge)?;
    if sid.is_null() || sid_start < start || header_end > end {
        return Err(Error::InvalidInput("token SID is outside its response"));
    }
    let count = unsafe { *((sid as *const u8).add(1)) } as usize;
    let sid_size = 8usize
        .checked_add(count.checked_mul(4).ok_or(Error::BufferTooLarge)?)
        .ok_or(Error::BufferTooLarge)?;
    if sid_start
        .checked_add(sid_size)
        .ok_or(Error::BufferTooLarge)?
        > end
        || unsafe { IsValidSid(sid) } == 0
    {
        return Err(Error::InvalidInput("token SID is invalid or truncated"));
    }
    Ok(sid)
}
