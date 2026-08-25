//! Registry value reads from a retained HKEY.

use crate::{Error, InputFault, RegistryValue, RegistryValueName, Result, MAX_BUFFER_BYTES};
use std::ptr;
use windows_sys::Win32::System::Registry::{RegQueryValueExW, HKEY, REG_VALUE_TYPE};

const ERROR_MORE_DATA: u32 = 234;

pub(super) fn read_value_handle(
    key: HKEY,
    value_name: &RegistryValueName,
) -> Result<RegistryValue> {
    let name = value_name.wide_nul()?;
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
    let length = usize::try_from(length)?;
    if length > MAX_BUFFER_BYTES {
        return Err(Error::BufferTooLarge);
    }
    let mut data = vec![0u8; length];
    let mut returned_length = u32::try_from(data.len())?;
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
    let returned_length = usize::try_from(returned_length)?;
    if returned_length > data.len() {
        return Err(Error::InvalidInput(
            InputFault::RegistryValueResponseTooLarge,
        ));
    }
    data.truncate(returned_length);
    Ok(RegistryValue { value_type, data })
}
