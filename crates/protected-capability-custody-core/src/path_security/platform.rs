use std::path::Path;

use file_id::FileId;
use same_file::Handle;
use sha2::{Digest, Sha256};

use super::PathSecurityError;

#[cfg(unix)]
pub(super) fn open_guarded(
    path: &Path,
    directory: bool,
) -> Result<(Handle, [u8; 32]), PathSecurityError> {
    use std::fs::File;
    use std::os::unix::fs::MetadataExt;

    use rustix::fs::{open, Mode, OFlags};

    let mut flags = OFlags::RDONLY | OFlags::CLOEXEC | OFlags::NOFOLLOW;
    if directory {
        flags |= OFlags::DIRECTORY;
    }
    let descriptor =
        open(path, flags, Mode::empty()).map_err(|_| PathSecurityError::Unavailable)?;
    let file = File::from(descriptor);
    let metadata = file
        .metadata()
        .map_err(|_| PathSecurityError::Unavailable)?;
    let identity = FileId::new_inode(metadata.dev(), metadata.ino());
    let digest = digest_file_id(identity)?;
    let handle = Handle::from_file(file).map_err(|_| PathSecurityError::Unavailable)?;
    Ok((handle, digest))
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
        .map_err(|_| PathSecurityError::Unavailable)?;
    let metadata = file
        .metadata()
        .map_err(|_| PathSecurityError::Unavailable)?;
    if metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0 {
        return Err(PathSecurityError::UnsafePath);
    }
    let identity =
        file_id::get_high_res_file_id(path).map_err(|_| PathSecurityError::Unavailable)?;
    let digest = digest_file_id(identity)?;
    let handle = Handle::from_file(file).map_err(|_| PathSecurityError::Unavailable)?;
    Ok((handle, digest))
}

#[cfg(not(any(unix, windows)))]
pub(super) fn open_guarded(
    _path: &Path,
    _directory: bool,
) -> Result<(Handle, [u8; 32]), PathSecurityError> {
    Err(PathSecurityError::Unavailable)
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
