//! Bounded process-image path mechanics.

use super::super::super::handles::last_error;
use crate::{Error, Result, MAX_WIDE_CHARS};
use windows_sys::Win32::Foundation::HANDLE;
use windows_sys::Win32::System::Threading::QueryFullProcessImageNameW;

const ERROR_INSUFFICIENT_BUFFER: u32 = 122;

pub(super) fn query_image_path(handle: HANDLE) -> Result<String> {
    let mut buffer = vec![0u16; 1024];
    loop {
        let mut length = u32::try_from(buffer.len()).map_err(|_| Error::BufferTooLarge)?;
        let ok = unsafe { QueryFullProcessImageNameW(handle, 0, buffer.as_mut_ptr(), &mut length) };
        if ok != 0 {
            let length = usize::try_from(length).map_err(|_| Error::BufferTooLarge)?;
            if length > buffer.len() {
                return Err(Error::InvalidInput(
                    "process image response exceeds its buffer",
                ));
            }
            return String::from_utf16(&buffer[..length])
                .map_err(|_| Error::InvalidInput("process image path is not valid UTF-16"));
        }
        let error = last_error();
        if error != ERROR_INSUFFICIENT_BUFFER || buffer.len() >= MAX_WIDE_CHARS {
            return Err(Error::Win32(error));
        }
        buffer.resize((buffer.len() * 2).min(MAX_WIDE_CHARS), 0);
    }
}

pub(super) fn wide_string(value: &str) -> Result<Vec<u16>> {
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
