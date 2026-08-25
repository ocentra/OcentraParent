//! Bounded UTF-16 and MULTI_SZ pointer decoding for SCM responses.

use crate::{Error, Result, MAX_WIDE_CHARS};

pub(crate) fn copy_optional_wide_ptr(
    pointer: windows_sys::core::PWSTR,
    buffer: &[u8],
) -> Result<Option<String>> {
    if pointer.is_null() {
        return Ok(None);
    }
    let slice = bounded_wide_slice(pointer, buffer)?;
    let nul = slice
        .iter()
        .position(|value| *value == 0)
        .ok_or(Error::InvalidInput("service string is not NUL terminated"))?;
    String::from_utf16(&slice[..nul])
        .map(Some)
        .map_err(|_| Error::InvalidInput("service string is not valid UTF-16"))
}

pub(crate) fn copy_multi_sz_ptr(
    pointer: windows_sys::core::PWSTR,
    buffer: &[u8],
) -> Result<Vec<String>> {
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
            return Ok(values);
        }
        values.push(
            String::from_utf16(&slice[value_start..index])
                .map_err(|_| Error::InvalidInput("service multi-string is not valid UTF-16"))?,
        );
        value_start = index + 1;
    }
    Err(Error::InvalidInput(
        "service multi-string is not double-NUL terminated",
    ))
}

fn bounded_wide_slice<'a>(
    pointer: windows_sys::core::PWSTR,
    buffer: &'a [u8],
) -> Result<&'a [u16]> {
    let start = pointer as usize;
    let base = buffer.as_ptr() as usize;
    let end = base
        .checked_add(buffer.len())
        .ok_or(Error::BufferTooLarge)?;
    if start < base || start >= end || start % core::mem::align_of::<u16>() != 0 {
        return Err(Error::InvalidInput(
            "service string points outside the response",
        ));
    }
    let available = (end - start) / core::mem::size_of::<u16>();
    Ok(unsafe { core::slice::from_raw_parts(pointer, available) })
}

pub(crate) fn wide_string(value: &str) -> Result<Vec<u16>> {
    if value.is_empty() || value.contains('\0') {
        return Err(Error::InvalidInput(
            "Windows string is empty or contains NUL",
        ));
    }
    if value.encode_utf16().count() >= MAX_WIDE_CHARS {
        return Err(Error::BufferTooLarge);
    }
    Ok(value.encode_utf16().chain(core::iter::once(0)).collect())
}
