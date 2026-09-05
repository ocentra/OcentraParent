use std::fs::File;
use std::mem::MaybeUninit;
use std::os::windows::ffi::OsStringExt;
use std::os::windows::io::AsRawHandle;
use std::path::PathBuf;

use windows_sys::Win32::Foundation::{FILETIME, HANDLE};
use windows_sys::Win32::System::Threading::{GetProcessTimes, QueryFullProcessImageNameW};

use super::ArtifactError;

const MAX_IMAGE_PATH_CHARS: usize = 32 * 1024;

pub(super) fn query_creation_time(process: &File) -> Result<u64, ArtifactError> {
    let mut creation = MaybeUninit::<FILETIME>::zeroed();
    let mut exit = MaybeUninit::<FILETIME>::zeroed();
    let mut kernel = MaybeUninit::<FILETIME>::zeroed();
    let mut user = MaybeUninit::<FILETIME>::zeroed();
    if unsafe {
        GetProcessTimes(
            process.as_raw_handle() as HANDLE,
            creation.as_mut_ptr(),
            exit.as_mut_ptr(),
            kernel.as_mut_ptr(),
            user.as_mut_ptr(),
        )
    } == 0
    {
        return Err(ArtifactError::OwnershipChanged);
    }
    let creation = unsafe { creation.assume_init() };
    Ok((u64::from(creation.dwHighDateTime) << 32) | u64::from(creation.dwLowDateTime))
}

pub(super) fn query_image_path(process: &File) -> Result<PathBuf, ArtifactError> {
    let mut buffer = vec![0u16; MAX_IMAGE_PATH_CHARS];
    let mut length = u32::try_from(buffer.len()).map_err(|_| ArtifactError::SizeLimit)?;
    if unsafe {
        QueryFullProcessImageNameW(
            process.as_raw_handle() as HANDLE,
            0,
            buffer.as_mut_ptr(),
            &mut length,
        )
    } == 0
    {
        return Err(ArtifactError::OwnershipChanged);
    }
    let length = usize::try_from(length).map_err(|_| ArtifactError::SizeLimit)?;
    if length == 0 || length > buffer.len() {
        return Err(ArtifactError::OwnershipChanged);
    }
    let image = std::ffi::OsString::from_wide(&buffer[..length]);
    let path = PathBuf::from(image);
    if !path.is_absolute() {
        return Err(ArtifactError::OwnershipChanged);
    }
    Ok(path)
}
