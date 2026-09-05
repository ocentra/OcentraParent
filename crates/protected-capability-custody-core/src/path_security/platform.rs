use std::path::Path;

use file_id::FileId;
use same_file::Handle;
use sha2::{Digest, Sha256};

use super::PathSecurityError;

#[cfg(unix)]
pub(super) fn stable_sqlite_paths_supported() -> bool {
    false
}

#[cfg(windows)]
pub(super) fn stable_sqlite_paths_supported() -> bool {
    true
}

#[cfg(not(any(unix, windows)))]
pub(super) fn stable_sqlite_paths_supported() -> bool {
    false
}

#[cfg(unix)]
pub(super) fn open_guarded(
    _path: &Path,
    _directory: bool,
) -> Result<(Handle, [u8; 32]), PathSecurityError> {
    Err(PathSecurityError::UnsupportedPlatform)
}

#[cfg(windows)]
pub(super) fn open_guarded(
    path: &Path,
    directory: bool,
) -> Result<(Handle, [u8; 32]), PathSecurityError> {
    use std::fs::OpenOptions;
    use std::os::windows::fs::{MetadataExt, OpenOptionsExt};

    use windows_sys::Win32::Storage::FileSystem::{
        FILE_ATTRIBUTE_REPARSE_POINT, FILE_FLAG_BACKUP_SEMANTICS, FILE_FLAG_OPEN_REPARSE_POINT,
        FILE_SHARE_READ, FILE_SHARE_WRITE,
    };

    let mut flags = FILE_FLAG_OPEN_REPARSE_POINT;
    if directory {
        flags |= FILE_FLAG_BACKUP_SEMANTICS;
    }
    let file = OpenOptions::new()
        .access_mode(0)
        .share_mode(FILE_SHARE_READ | FILE_SHARE_WRITE)
        .custom_flags(flags)
        .open(path)
        .map_err(|_file_open_error| PathSecurityError::Unavailable)?;
    let metadata = file
        .metadata()
        .map_err(|_file_metadata_error| PathSecurityError::Unavailable)?;
    if metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0 {
        return Err(PathSecurityError::UnsafePath);
    }
    // The no-follow handle denies delete/rename sharing before the ID lookup;
    // the retained handle is also compared on every revalidation.  The path
    // lookup therefore cannot silently bind a replacement between validation
    // and identity capture.
    let identity = file_id::get_high_res_file_id(path)
        .map_err(|_file_identity_error| PathSecurityError::Unavailable)?;
    let digest = digest_file_id(identity)?;
    let handle =
        Handle::from_file(file).map_err(|_handle_creation_error| PathSecurityError::Unavailable)?;
    Ok((handle, digest))
}

#[cfg(windows)]
pub(super) fn create_guarded(path: &Path) -> Result<(Handle, [u8; 32]), PathSecurityError> {
    use std::fs::OpenOptions;
    use std::os::windows::fs::OpenOptionsExt;

    use windows_sys::Win32::Storage::FileSystem::{
        FILE_FLAG_OPEN_REPARSE_POINT, FILE_SHARE_READ, FILE_SHARE_WRITE,
    };

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create_new(true)
        .share_mode(FILE_SHARE_READ | FILE_SHARE_WRITE)
        .custom_flags(FILE_FLAG_OPEN_REPARSE_POINT)
        .open(path)
        .map_err(|_file_create_error| PathSecurityError::Unavailable)?;
    let identity = file_id::get_high_res_file_id(path)
        .map_err(|_file_identity_error| PathSecurityError::Unavailable)?;
    let digest = digest_file_id(identity)?;
    let handle =
        Handle::from_file(file).map_err(|_handle_creation_error| PathSecurityError::Unavailable)?;
    Ok((handle, digest))
}

#[cfg(not(windows))]
pub(super) fn create_guarded(_path: &Path) -> Result<(Handle, [u8; 32]), PathSecurityError> {
    Err(PathSecurityError::UnsupportedPlatform)
}

#[cfg(not(any(unix, windows)))]
pub(super) fn open_guarded(
    _path: &Path,
    _directory: bool,
) -> Result<(Handle, [u8; 32]), PathSecurityError> {
    Err(PathSecurityError::UnsupportedPlatform)
}

fn digest_file_id(identity: FileId) -> Result<[u8; 32], PathSecurityError> {
    let mut hasher = Sha256::new();
    hasher.update(b"ocentra.database-physical-file.v1");
    match identity {
        FileId::Inode {
            device_id,
            inode_number,
        } => {
            hasher.update(b"unix-inode");
            hasher.update(device_id.to_be_bytes());
            hasher.update(inode_number.to_be_bytes());
        }
        FileId::HighRes {
            volume_serial_number,
            file_id,
        } => {
            hasher.update(b"windows-file-id-128");
            hasher.update(volume_serial_number.to_be_bytes());
            hasher.update(file_id.to_be_bytes());
        }
        FileId::LowRes { .. } => return Err(PathSecurityError::Unavailable),
    }
    Ok(hasher.finalize().into())
}
