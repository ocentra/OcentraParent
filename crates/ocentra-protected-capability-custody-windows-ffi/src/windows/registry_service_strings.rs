//! Bounded UTF-16 and MULTI_SZ pointer decoding for SCM responses.

use crate::{Error, InputFault, Result, WindowsText};

pub(crate) fn copy_optional_wide_ptr(
    pointer: windows_sys::core::PWSTR,
    buffer: &[u8],
) -> Result<Option<WindowsText>> {
    if pointer.is_null() {
        return Ok(None);
    }
    let slice = bounded_wide_slice(pointer, buffer)?;
    let nul = slice
        .iter()
        .position(|value| *value == 0)
        .ok_or(Error::InvalidInput(InputFault::ServiceStringUnterminated))?;
    WindowsText::from_utf16(&slice[..nul], InputFault::ServiceStringInvalid).map(Some)
}

pub(crate) fn copy_multi_sz_ptr(
    pointer: windows_sys::core::PWSTR,
    buffer: &[u8],
) -> Result<Vec<WindowsText>> {
    if pointer.is_null() {
        return Ok(Vec::new());
    }
    let slice = bounded_wide_slice(pointer, buffer)?;
    let mut values = Vec::new();
    let mut value_start = 0usize;
    for (index, value) in slice.iter().enumerate() {
        if *value != 0 {
            continue;
        }
        if index == value_start {
            if index == 0 && slice.get(1).copied() != Some(0) {
                return Err(Error::InvalidInput(InputFault::ServiceMultiStringInvalid));
            }
            return Ok(values);
        }
        values.push(WindowsText::from_utf16(
            &slice[value_start..index],
            InputFault::ServiceMultiStringInvalid,
        )?);
        value_start = index + 1;
    }
    Err(Error::InvalidInput(InputFault::ServiceMultiStringInvalid))
}

fn bounded_wide_slice(pointer: windows_sys::core::PWSTR, buffer: &[u8]) -> Result<&[u16]> {
    let start = pointer as usize;
    let base = buffer.as_ptr() as usize;
    let end = base
        .checked_add(buffer.len())
        .ok_or(Error::BufferTooLarge)?;
    if start < base || start >= end || !start.is_multiple_of(core::mem::align_of::<u16>()) {
        return Err(Error::InvalidInput(InputFault::ServiceStringOutsideBuffer));
    }
    let available = (end - start) / core::mem::size_of::<u16>();
    Ok(unsafe { core::slice::from_raw_parts(pointer, available) })
}
