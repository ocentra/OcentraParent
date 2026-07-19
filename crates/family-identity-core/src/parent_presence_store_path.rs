use std::fs;
use std::path::Path;

use crate::parent_presence_store::ParentPresenceStoreError;

pub(crate) fn validate_caller_custody_path(path: &Path) -> Result<(), ParentPresenceStoreError> {
    if !path.is_absolute() {
        return Err(ParentPresenceStoreError::Unavailable);
    }
    let parent = path.parent().ok_or(ParentPresenceStoreError::Unavailable)?;
    validate_ancestor_chain(parent)?;
    validate_existing_final_path(path)
}

fn validate_ancestor_chain(parent: &Path) -> Result<(), ParentPresenceStoreError> {
    for ancestor in parent.ancestors() {
        let metadata = fs::symlink_metadata(ancestor)
            .map_err(|_error| ParentPresenceStoreError::Unavailable)?;
        if !metadata.is_dir() || metadata.file_type().is_symlink() {
            return Err(ParentPresenceStoreError::Unavailable);
        }
    }
    Ok(())
}

fn validate_existing_final_path(path: &Path) -> Result<(), ParentPresenceStoreError> {
    let Ok(metadata) = fs::symlink_metadata(path) else {
        return Ok(());
    };
    if metadata.file_type().is_symlink() || !metadata.is_file() || metadata.permissions().readonly()
    {
        return Err(ParentPresenceStoreError::Unavailable);
    }
    Ok(())
}
