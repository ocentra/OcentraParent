use std::fs;
use std::path::Path;

use crate::parent_presence_store::ParentPresenceStoreError;

pub(crate) fn validate_caller_custody_path(path: &Path) -> Result<(), ParentPresenceStoreError> {
    if !path.is_absolute() {
        return Err(ParentPresenceStoreError::Unavailable);
    }
    let parent = path.parent().ok_or(ParentPresenceStoreError::Unavailable)?;
    validate_ancestor_chain(parent)
}

fn validate_ancestor_chain(parent: &Path) -> Result<(), ParentPresenceStoreError> {
    for (position, ancestor) in parent.ancestors().enumerate() {
        let metadata = fs::symlink_metadata(ancestor)
            .map_err(|_error| ParentPresenceStoreError::Unavailable)?;
        if !metadata.is_dir() || path_entry_is_link(&metadata) {
            return Err(ParentPresenceStoreError::Unavailable);
        }
        validate_platform_directory_custody(&metadata, position == 0)?;
    }
    Ok(())
}

#[cfg(unix)]
fn path_entry_is_link(metadata: &fs::Metadata) -> bool {
    metadata.file_type().is_symlink()
}

#[cfg(windows)]
fn path_entry_is_link(metadata: &fs::Metadata) -> bool {
    use std::os::windows::fs::MetadataExt;

    const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x0000_0400;
    metadata.file_type().is_symlink()
        || metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0
}

#[cfg(unix)]
fn validate_platform_directory_custody(
    metadata: &fs::Metadata,
    immediate_parent: bool,
) -> Result<(), ParentPresenceStoreError> {
    use std::os::unix::fs::MetadataExt;

    const GROUP_OR_OTHER_WRITE: u32 = 0o022;
    const STICKY: u32 = 0o1000;
    let mode = metadata.mode();
    if immediate_parent && metadata.uid() != rustix::process::geteuid().as_raw() {
        return Err(ParentPresenceStoreError::Unavailable);
    }
    if mode & GROUP_OR_OTHER_WRITE != 0 && mode & STICKY == 0 {
        return Err(ParentPresenceStoreError::Unavailable);
    }
    Ok(())
}

#[cfg(windows)]
fn validate_platform_directory_custody(
    _metadata: &fs::Metadata,
    _immediate_parent: bool,
) -> Result<(), ParentPresenceStoreError> {
    Ok(())
}
