//! Bounded ACE payload mechanics.

use crate::{AceObservation, Error, InputFault, Result};
use windows_sys::Win32::Security::{GetAce, ACE_HEADER, ACL, PSID};

pub(super) fn copy_ace(
    acl: *const ACL,
    index: usize,
    acl_header_end: usize,
    acl_end: usize,
    acl_size: u16,
) -> Result<AceObservation> {
    let index = u32::try_from(index)?;
    let mut ace: *mut core::ffi::c_void = core::ptr::null_mut();
    let ok = unsafe { GetAce(acl, index, &mut ace) };
    if ok == 0 || ace.is_null() {
        return Err(Error::Win32(super::last_error()));
    }
    let ace_start = ace as usize;
    let ace_header_end = ace_start
        .checked_add(core::mem::size_of::<ACE_HEADER>())
        .ok_or(Error::BufferTooLarge)?;
    if ace_start < acl_header_end || ace_header_end > acl_end {
        return Err(Error::InvalidInput(InputFault::AceHeaderOutsideAcl));
    }
    let ace_header = unsafe { core::ptr::read_unaligned(ace as *const ACE_HEADER) };
    let ace_size = usize::from(ace_header.AceSize);
    let ace_end = ace_start
        .checked_add(ace_size)
        .ok_or(Error::BufferTooLarge)?;
    if ace_size < 8 || ace_size > usize::from(acl_size) || ace_end > acl_end {
        return Err(Error::InvalidInput(InputFault::AceTooSmall));
    }
    let ace_bytes = unsafe { core::slice::from_raw_parts(ace as *const u8, ace_size) };
    let access_mask = u32::from_ne_bytes([ace_bytes[4], ace_bytes[5], ace_bytes[6], ace_bytes[7]]);
    let sid_offset = super::ace_shape::sid_offset_for_ace(ace_header.AceType, ace_bytes)?;
    if sid_offset >= ace_bytes.len() {
        return Err(Error::InvalidInput(InputFault::AceMissingSid));
    }
    let sid_pointer = unsafe { (ace as *mut u8).add(sid_offset) as PSID };
    let sid = super::sid::copy_sid_in_buffer(sid_pointer, ace as *const u8, ace_bytes.len())?;
    Ok(AceObservation {
        ace_type: ace_header.AceType,
        flags: ace_header.AceFlags,
        access_mask,
        sid,
        raw: ace_bytes.to_vec(),
    })
}
