use std::{
    fs::{self, File, OpenOptions},
    path::Path,
};

use super::super::BrowserManagedProfileStoreError;
use super::path_guards::GuardedPathKind;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum StableFileIdentity {
    #[cfg(unix)]
    Unix { device: u64, inode: u64 },
    #[cfg(windows)]
    Windows { file_id: file_id::FileId },
}

pub(super) fn metadata_is_indirection(metadata: &fs::Metadata) -> bool {
    if metadata.file_type().is_symlink() {
        return true;
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;

        const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x0000_0400;
        return metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0;
    }
    #[cfg(not(windows))]
    {
        false
    }
}

pub(super) fn stable_file_identity(
    path: &Path,
    file: &File,
    metadata: &fs::Metadata,
) -> Result<StableFileIdentity, BrowserManagedProfileStoreError> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;

        let _ = (path, file);
        return Ok(StableFileIdentity::Unix {
            device: metadata.dev(),
            inode: metadata.ino(),
        });
    }
    #[cfg(windows)]
    {
        let _ = (file, metadata);
        return Ok(StableFileIdentity::Windows {
            file_id: file_id::get_high_res_file_id(path)
                .map_err(|_error| BrowserManagedProfileStoreError::Io)?,
        });
    }
    #[cfg(not(any(unix, windows)))]
    {
        let _ = (path, file, metadata);
        Err(BrowserManagedProfileStoreError::Io)
    }
}

#[cfg(windows)]
pub(super) fn open_guarded(
    path: &Path,
    kind: GuardedPathKind,
    create: bool,
    deny_delete: bool,
) -> Result<File, BrowserManagedProfileStoreError> {
    use std::os::windows::fs::OpenOptionsExt;

    const FILE_FLAG_BACKUP_SEMANTICS: u32 = 0x0200_0000;
    const FILE_FLAG_OPEN_REPARSE_POINT: u32 = 0x0020_0000;
    const FILE_SHARE_DELETE: u32 = 0x0000_0004;
    const FILE_SHARE_READ: u32 = 0x0000_0001;
    const FILE_SHARE_WRITE: u32 = 0x0000_0002;
    let mut options = OpenOptions::new();
    options.read(true);
    if matches!(kind, GuardedPathKind::File) {
        options.write(true);
    }
    if create {
        options.create(true);
    }
    let share_mode =
        FILE_SHARE_READ | FILE_SHARE_WRITE | if deny_delete { 0 } else { FILE_SHARE_DELETE };
    let mut flags = FILE_FLAG_OPEN_REPARSE_POINT;
    if matches!(kind, GuardedPathKind::Directory) {
        flags |= FILE_FLAG_BACKUP_SEMANTICS;
    }
    options.share_mode(share_mode).custom_flags(flags);
    options
        .open(path)
        .map_err(|_error| BrowserManagedProfileStoreError::Io)
}

#[cfg(unix)]
pub(super) fn open_guarded(
    path: &Path,
    kind: GuardedPathKind,
    create: bool,
    _deny_delete: bool,
) -> Result<File, BrowserManagedProfileStoreError> {
    use std::os::unix::fs::OpenOptionsExt;

    let mut options = OpenOptions::new();
    options.read(true);
    if matches!(kind, GuardedPathKind::File) {
        options.write(true);
    }
    if create {
        options.create(true);
    }
    #[cfg(any(target_os = "linux", target_os = "android"))]
    options.custom_flags(0x0002_0000);
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    options.custom_flags(0x0000_0100);
    options
        .open(path)
        .map_err(|_error| BrowserManagedProfileStoreError::Io)
}

#[cfg(not(any(unix, windows)))]
pub(super) fn open_guarded(
    path: &Path,
    kind: GuardedPathKind,
    create: bool,
    _deny_delete: bool,
) -> Result<File, BrowserManagedProfileStoreError> {
    let mut options = OpenOptions::new();
    options.read(true);
    if matches!(kind, GuardedPathKind::File) {
        options.write(true);
    }
    if create {
        options.create(true);
    }
    options
        .open(path)
        .map_err(|_error| BrowserManagedProfileStoreError::Io)
}
