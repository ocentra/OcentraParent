//! Bounded digest mechanics for a retained executable image handle.

use super::super::super::handles::last_error;
use crate::{Error, InputFault, Result};
use sha2::{Digest, Sha256};
use std::ptr;
use windows_sys::Win32::Foundation::HANDLE;
use windows_sys::Win32::Storage::FileSystem::{ReadFile, SetFilePointerEx, FILE_BEGIN};

const IMAGE_READ_CHUNK: usize = 64 * 1024;

pub(super) fn read_image_digest(handle: HANDLE, size: u64) -> Result<[u8; 32]> {
    reset_image_cursor(handle)?;
    let read_result = read_image_digest_from_start(handle, size);
    let reset_result = reset_image_cursor(handle);
    match (read_result, reset_result) {
        (Ok(digest), Ok(())) => Ok(digest),
        (Err(error), _) => Err(error),
        (Ok(_), Err(error)) => Err(error),
    }
}

fn read_image_digest_from_start(handle: HANDLE, size: u64) -> Result<[u8; 32]> {
    let mut hasher = Sha256::new();
    let mut remaining = size;
    let mut buffer = vec![0u8; IMAGE_READ_CHUNK];
    while remaining > 0 {
        let requested = usize::try_from(remaining)?.min(buffer.len());
        let mut read = 0u32;
        if unsafe {
            ReadFile(
                handle,
                buffer.as_mut_ptr(),
                u32::try_from(requested)?,
                &mut read,
                ptr::null_mut(),
            )
        } == 0
        {
            return Err(Error::Win32(last_error()));
        }
        let read = usize::try_from(read)?;
        if read == 0 || read > requested {
            return Err(Error::InvalidInput(InputFault::ImageReadSizeInvalid));
        }
        hasher.update(&buffer[..read]);
        remaining = remaining
            .checked_sub(u64::try_from(read)?)
            .ok_or(Error::InvalidInput(InputFault::ImageReadExceededSize))?;
    }
    let digest = hasher.finalize();
    let mut output = [0u8; 32];
    output.copy_from_slice(&digest);
    Ok(output)
}

fn reset_image_cursor(handle: HANDLE) -> Result<()> {
    if unsafe { SetFilePointerEx(handle, 0, ptr::null_mut(), FILE_BEGIN) } == 0 {
        return Err(Error::Win32(last_error()));
    }
    Ok(())
}
