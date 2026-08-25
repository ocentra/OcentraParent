//! Bounded ACL mechanics.

use crate::{AceObservation, Error, Result, MAX_ACES, MAX_BUFFER_BYTES};
use windows_sys::Win32::Security::{IsValidAcl, ACL};

pub(super) fn copy_aces(
    acl: *const ACL,
    descriptor: *const u8,
    descriptor_length: usize,
) -> Result<Vec<AceObservation>> {
    let (acl_header_end, acl_end, header) = bounded_acl(acl, descriptor, descriptor_length)?;
    if unsafe { IsValidAcl(acl) } == 0 {
        return Err(Error::InvalidInput("Windows returned an invalid ACL"));
    }
    let count = usize::from(header.AceCount);
    if count > MAX_ACES {
        return Err(Error::BufferTooLarge);
    }
    let mut observations = Vec::with_capacity(count);
    for index in 0..count {
        observations.push(super::ace::copy_ace(
            acl,
            index,
            acl_header_end,
            acl_end,
            header.AclSize,
        )?);
    }
    Ok(observations)
}

fn bounded_acl(
    acl: *const ACL,
    descriptor: *const u8,
    descriptor_length: usize,
) -> Result<(usize, usize, ACL)> {
    let descriptor_start = descriptor as usize;
    let descriptor_end = descriptor_start
        .checked_add(descriptor_length)
        .ok_or(Error::BufferTooLarge)?;
    let acl_start = acl as usize;
    let acl_header_end = acl_start
        .checked_add(core::mem::size_of::<ACL>())
        .ok_or(Error::BufferTooLarge)?;
    if acl_start < descriptor_start || acl_header_end > descriptor_end {
        return Err(Error::InvalidInput(
            "DACL pointer is outside its descriptor",
        ));
    }
    let header = unsafe { core::ptr::read_unaligned(acl) };
    let acl_size = usize::from(header.AclSize);
    let acl_end = acl_start
        .checked_add(acl_size)
        .ok_or(Error::BufferTooLarge)?;
    if acl_size < core::mem::size_of::<ACL>()
        || acl_size > MAX_BUFFER_BYTES
        || acl_end > descriptor_end
    {
        return Err(Error::InvalidInput("ACL size is outside the bounded shape"));
    }
    Ok((acl_header_end, acl_end, header))
}
