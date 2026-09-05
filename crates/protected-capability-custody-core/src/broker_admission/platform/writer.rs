#[cfg(windows)]
use std::fs::{File, OpenOptions};
#[cfg(windows)]
use std::os::windows::fs::{MetadataExt, OpenOptionsExt};
#[cfg(windows)]
use std::path::Path;

#[cfg(windows)]
use fs2::FileExt;
#[cfg(windows)]
use same_file::Handle;
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
    let lock_path = lock_path(canonical_path);
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
pub(super) fn revalidate(canonical_path: &Path, held: &File) -> Result<(), PlatformError> {
    let lock_path = lock_path(canonical_path);
    let current = OpenOptions::new()
        .read(true)
        .write(true)
        .create(false)
        .custom_flags(FILE_FLAG_OPEN_REPARSE_POINT)
        .open(&lock_path)
        .map_err(map_io_error)?;
    if current.metadata().map_err(map_io_error)?.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT
        != 0
    {
        return Err(PlatformError::Tampered);
    }
    acl::validate_file(&current)?;
    let current_handle =
        Handle::from_file(current).map_err(|_handle_error| PlatformError::Unavailable)?;
    let held_handle = Handle::from_file(held.try_clone().map_err(map_io_error)?)
        .map_err(|_handle_error| PlatformError::Unavailable)?;
    if current_handle != held_handle {
        return Err(PlatformError::Conflict);
    }
    Ok(())
}

#[cfg(windows)]
pub(super) fn lock_path(canonical_path: &Path) -> std::path::PathBuf {
    canonical_path.with_extension(WRITER_LOCK_EXTENSION)
}

#[cfg(windows)]
pub(super) fn journal_path(canonical_path: &Path) -> std::path::PathBuf {
    let mut path = std::ffi::OsString::from(canonical_path.as_os_str());
    path.push("-journal");
    std::path::PathBuf::from(path)
}

#[cfg(windows)]
fn map_io_error(_error: std::io::Error) -> PlatformError {
    PlatformError::Unavailable
}
