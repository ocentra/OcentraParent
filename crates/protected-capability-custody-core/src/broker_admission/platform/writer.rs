#[cfg(windows)]
use std::fs::{File, OpenOptions};
#[cfg(windows)]
use std::os::windows::fs::{MetadataExt, OpenOptionsExt};
#[cfg(windows)]
use std::path::Path;

#[cfg(windows)]
use fs2::FileExt;
#[cfg(windows)]
use windows_sys::Win32::Storage::FileSystem::{
    FILE_ATTRIBUTE_REPARSE_POINT, FILE_FLAG_OPEN_REPARSE_POINT,
};

#[cfg(windows)]
use super::acl;
#[cfg(windows)]
use crate::platform::PlatformError;

#[cfg(windows)]
const WRITER_LOCK_EXTENSION: &str = "ocentra-custody.lock";

#[cfg(windows)]
pub(super) fn open(canonical_path: &Path) -> Result<File, PlatformError> {
    let lock_path = canonical_path.with_extension(WRITER_LOCK_EXTENSION);
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .custom_flags(FILE_FLAG_OPEN_REPARSE_POINT)
        .open(lock_path)
        .map_err(map_io_error)?;
    if file.metadata().map_err(map_io_error)?.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0
    {
        return Err(PlatformError::Tampered);
    }
    acl::validate_file(&file)?;
    file.try_lock_exclusive().map_err(map_io_error)?;
    Ok(file)
}

#[cfg(windows)]
fn map_io_error(_error: std::io::Error) -> PlatformError {
    PlatformError::Unavailable
}
