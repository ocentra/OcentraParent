//! ACE type and SID offset validation.

use crate::{Error, InputFault, Result};

pub(super) fn sid_offset_for_ace(ace_type: u8, ace_bytes: &[u8]) -> Result<usize> {
    match ace_type {
        // ACCESS_*_ACE and SYSTEM_*_ACE payloads place the SID at byte 8.
        // Keep these exact values aligned with ACE_HEADER, including the
        // callback and resource-attribute families.
        0x00..=0x03 | 0x09..=0x0a | 0x0d..=0x0e | 0x11..=0x15 => Ok(8),
        // OBJECT and CALLBACK_OBJECT ACEs carry object flags and optional
        // GUIDs before the SID.
        0x05..=0x08 | 0x0b..=0x0c | 0x0f..=0x10 => object_sid_offset(ace_bytes),
        4 => Err(Error::InvalidInput(InputFault::CompoundAceUnsupported)),
        _ => Err(Error::InvalidInput(InputFault::UnknownAceType)),
    }
}

fn object_sid_offset(ace_bytes: &[u8]) -> Result<usize> {
    if ace_bytes.len() < 12 {
        return Err(Error::InvalidInput(InputFault::ObjectAceTooSmall));
    }
    let object_flags =
        u32::from_ne_bytes([ace_bytes[8], ace_bytes[9], ace_bytes[10], ace_bytes[11]]);
    if object_flags & !0x3 != 0 {
        return Err(Error::InvalidInput(InputFault::ObjectAceFlagsInvalid));
    }
    let object_guid_bytes = if object_flags & 0x1 != 0 { 16 } else { 0 };
    let inherited_guid_bytes = if object_flags & 0x2 != 0 { 16 } else { 0 };
    12usize
        .checked_add(object_guid_bytes)
        .and_then(|value| value.checked_add(inherited_guid_bytes))
        .ok_or(Error::BufferTooLarge)
}
