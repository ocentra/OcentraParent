//! Bounded process-image path mechanics.

use super::super::super::handles::last_error;
use crate::{Error, InputFault, Result, WindowsText, MAX_WIDE_CHARS};
use windows_sys::Win32::Foundation::HANDLE;
use windows_sys::Win32::System::Threading::QueryFullProcessImageNameW;

const ERROR_INSUFFICIENT_BUFFER: u32 = 122;

pub(super) fn query_image_path(handle: HANDLE) -> Result<WindowsText> {
    let mut buffer = vec![0u16; 1024];
    loop {
        let mut length = u32::try_from(buffer.len())?;
        let ok = unsafe { QueryFullProcessImageNameW(handle, 0, buffer.as_mut_ptr(), &mut length) };
        if ok != 0 {
            let length = usize::try_from(length)?;
            if length == 0 {
                return Err(Error::InvalidInput(InputFault::ProcessImagePathInvalid));
            }
            if length > buffer.len() {
                return Err(Error::InvalidInput(
                    InputFault::ProcessImageResponseTooLarge,
                ));
            }
            return WindowsText::from_utf16(&buffer[..length], InputFault::ProcessImagePathInvalid);
        }
        let error = last_error();
        if error != ERROR_INSUFFICIENT_BUFFER || buffer.len() >= MAX_WIDE_CHARS {
            return Err(Error::Win32(error));
        }
        buffer.resize((buffer.len() * 2).min(MAX_WIDE_CHARS), 0);
    }
}

pub(super) fn wide_string(value: &WindowsText) -> Result<Vec<u16>> {
    value.wide_nul()
}
