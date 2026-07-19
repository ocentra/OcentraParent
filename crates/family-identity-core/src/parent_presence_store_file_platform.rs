use std::fs::{self, File, OpenOptions};
use std::path::Path;

#[cfg(windows)]
use std::path::PathBuf;

use crate::parent_presence_store::ParentPresenceStoreError;
#[cfg(windows)]
use crate::parent_presence_store_file_temporary::{
    reserve_private_temporary_artifact, TemporaryStoreArtifact,
};

#[cfg(unix)]
pub(crate) fn configure_private_creation(options: &mut OpenOptions) {
    use std::os::unix::fs::OpenOptionsExt;

    options.mode(0o600);
}

#[cfg(windows)]
pub(crate) fn configure_private_creation(options: &mut OpenOptions) {
    use std::os::windows::fs::OpenOptionsExt;

    const FILE_SHARE_READ: u32 = 0x0000_0001;
    const FILE_SHARE_WRITE: u32 = 0x0000_0002;
    options.share_mode(FILE_SHARE_READ | FILE_SHARE_WRITE);
}

#[cfg(unix)]
pub(crate) fn open_guard_file(path: &Path) -> Result<File, ParentPresenceStoreError> {
    use rustix::fs::{open, Mode, OFlags};

    let descriptor = open(
        path,
        OFlags::RDWR | OFlags::NOFOLLOW | OFlags::CLOEXEC,
        Mode::empty(),
    )
    .map_err(|_error| ParentPresenceStoreError::Unavailable)?;
    Ok(descriptor.into())
}

#[cfg(windows)]
pub(crate) fn open_guard_file(path: &Path) -> Result<File, ParentPresenceStoreError> {
    use std::os::windows::fs::OpenOptionsExt;

    const FILE_SHARE_READ: u32 = 0x0000_0001;
    const FILE_SHARE_WRITE: u32 = 0x0000_0002;
    const FILE_FLAG_OPEN_REPARSE_POINT: u32 = 0x0020_0000;
    OpenOptions::new()
        .read(true)
        .write(true)
        .share_mode(FILE_SHARE_READ | FILE_SHARE_WRITE)
        .custom_flags(FILE_FLAG_OPEN_REPARSE_POINT)
        .open(path)
        .map_err(|_error| ParentPresenceStoreError::Unavailable)
}

#[cfg(unix)]
pub(crate) fn open_ancestor_guards(_path: &Path) -> Result<Vec<File>, ParentPresenceStoreError> {
    Ok(Vec::new())
}

#[cfg(windows)]
pub(crate) fn open_ancestor_guards(path: &Path) -> Result<Vec<File>, ParentPresenceStoreError> {
    use std::os::windows::fs::{MetadataExt, OpenOptionsExt};

    const FILE_SHARE_READ: u32 = 0x0000_0001;
    const FILE_SHARE_WRITE: u32 = 0x0000_0002;
    const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x0000_0400;
    const FILE_FLAG_BACKUP_SEMANTICS: u32 = 0x0200_0000;
    const FILE_FLAG_OPEN_REPARSE_POINT: u32 = 0x0020_0000;
    let parent = path.parent().ok_or(ParentPresenceStoreError::Unavailable)?;
    let mut guards = Vec::new();
    for ancestor in parent.ancestors() {
        let directory = OpenOptions::new()
            .read(true)
            .share_mode(FILE_SHARE_READ | FILE_SHARE_WRITE)
            .custom_flags(FILE_FLAG_BACKUP_SEMANTICS | FILE_FLAG_OPEN_REPARSE_POINT)
            .open(ancestor)
            .map_err(|_error| ParentPresenceStoreError::Unavailable)?;
        let metadata = directory
            .metadata()
            .map_err(|_error| ParentPresenceStoreError::Unavailable)?;
        if !metadata.is_dir() || metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0 {
            return Err(ParentPresenceStoreError::IntegrityRejected);
        }
        guards.push(directory);
    }
    Ok(guards)
}

#[cfg(unix)]
pub(crate) fn validate_delete_sharing_capability(
    _path: &Path,
) -> Result<(), ParentPresenceStoreError> {
    Ok(())
}

#[cfg(windows)]
pub(crate) fn validate_delete_sharing_capability(
    path: &Path,
) -> Result<(), ParentPresenceStoreError> {
    let probe = reserve_private_temporary_artifact(path)?;
    let probe_path = probe.path().to_path_buf();
    let mut moved_name = probe_path.as_os_str().to_owned();
    moved_name.push(".moved");
    let moved_path = PathBuf::from(moved_name);
    let result = fs::rename(&probe_path, &moved_path);
    match result {
        Err(error) if matches!(error.raw_os_error(), Some(5 | 32)) => Ok(()),
        Ok(()) => reject_supported_rename(probe, &moved_path),
        Err(_error) => Err(ParentPresenceStoreError::Unavailable),
    }
}

#[cfg(windows)]
fn reject_supported_rename(
    probe: TemporaryStoreArtifact,
    moved_path: &Path,
) -> Result<(), ParentPresenceStoreError> {
    drop(probe);
    let _cleanup = fs::remove_file(moved_path);
    Err(ParentPresenceStoreError::Unavailable)
}

pub(crate) fn validate_private_store_metadata(
    metadata: &fs::Metadata,
) -> Result<(), ParentPresenceStoreError> {
    if !metadata.is_file() || metadata.file_type().is_symlink() || metadata.permissions().readonly()
    {
        return Err(ParentPresenceStoreError::IntegrityRejected);
    }
    validate_platform_private_metadata(metadata)
}

#[cfg(unix)]
fn validate_platform_private_metadata(
    metadata: &fs::Metadata,
) -> Result<(), ParentPresenceStoreError> {
    use std::os::unix::fs::MetadataExt;

    let mode = metadata.mode();
    if metadata.uid() != rustix::process::geteuid().as_raw()
        || mode & 0o600 != 0o600
        || mode & 0o077 != 0
    {
        return Err(ParentPresenceStoreError::IntegrityRejected);
    }
    Ok(())
}

#[cfg(windows)]
fn validate_platform_private_metadata(
    metadata: &fs::Metadata,
) -> Result<(), ParentPresenceStoreError> {
    use std::os::windows::fs::MetadataExt;

    const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x0000_0400;
    if metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0 {
        return Err(ParentPresenceStoreError::IntegrityRejected);
    }
    Ok(())
}
