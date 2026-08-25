//! Registry value reads from a retained HKEY.

use crate::{Error, RegistryValue, Result, MAX_BUFFER_BYTES, MAX_WIDE_CHARS};
use std::ptr;
use windows_sys::Win32::System::Registry::{RegQueryValueExW, HKEY, REG_VALUE_TYPE};

const ERROR_MORE_DATA: u32 = 234;

pub(super) fn read_value_handle(key: HKEY, value_name: &str) -> Result<RegistryValue> {
    let name = wide_string(value_name)?;
    let mut value_type: REG_VALUE_TYPE = 0;
    let mut length = 0u32;
    let first = unsafe {
        RegQueryValueExW(
            key,
            name.as_ptr(),
            ptr::null(),
            &mut value_type,
            ptr::null_mut(),
            &mut length,
        )
    };
    if first != 0 && first != ERROR_MORE_DATA {
        return Err(Error::Win32(first));
    }
    let length = usize::try_from(length).map_err(|_| Error::BufferTooLarge)?;
    if length > MAX_BUFFER_BYTES {
        return Err(Error::BufferTooLarge);
    }
    let mut data = vec![0u8; length];
    let mut returned_length = u32::try_from(data.len()).map_err(|_| Error::BufferTooLarge)?;
    let second = unsafe {
        RegQueryValueExW(
            key,
            name.as_ptr(),
            ptr::null(),
            &mut value_type,
            data.as_mut_ptr(),
            &mut returned_length,
        )
    };
    if second != 0 {
        return Err(Error::Win32(second));
    }
    let returned_length = usize::try_from(returned_length).map_err(|_| Error::BufferTooLarge)?;
    if returned_length > data.len() {
        return Err(Error::InvalidInput(
            "registry value response exceeds its buffer",
        ));
    }
    data.truncate(returned_length);
    Ok(RegistryValue { value_type, data })
}

fn wide_string(value: &str) -> Result<Vec<u16>> {
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
