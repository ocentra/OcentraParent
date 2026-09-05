use super::*;

use std::fs;

use crate::constants::DIRECTORY_ENTRY_NOT_UNICODE;

pub(super) fn remove_tree_contents(
    path: &Path,
    directory: &OwnedFile,
) -> Result<(), ArtifactError> {
    let names = child_names(path)?;
    for name in names {
        remove_child(path, directory, &name)?;
    }
    ensure_empty(path)?;
    directory.sync_directory()
}

fn child_names(path: &Path) -> Result<Vec<String>, ArtifactError> {
    fs::read_dir(path)
        .map_err(crate::error::io_error)?
        .map(|entry| {
            let entry = entry.map_err(crate::error::io_error)?;
            let name = entry
                .file_name()
                .to_str()
                .ok_or(ArtifactError::InvalidPath(DIRECTORY_ENTRY_NOT_UNICODE))?
                .to_owned();
            crate::platform::windows::validate_leaf(&name)?;
            Ok(name)
        })
        .collect()
}

fn remove_child(path: &Path, directory: &OwnedFile, name: &str) -> Result<(), ArtifactError> {
    let child_path = path.join(name);
    match OwnedFile::open_mutation_directory(&child_path) {
        Ok(child) => {
            verify_metadata(&child, true)?;
            remove_tree_contents(&child_path, &child)?;
            child.mark_deleted()?;
            directory.sync_directory()
        }
        Err(ArtifactError::InvalidPath(_)) => {
            let child = OwnedFile::open_existing_mutation_file(&child_path)?;
            verify_metadata(&child, false)?;
            child.mark_deleted()?;
            directory.sync_directory()
        }
        Err(ArtifactError::NotFound) => Err(ArtifactError::RecoveryRequired),
        Err(error) => Err(error),
    }
}

fn ensure_empty(path: &Path) -> Result<(), ArtifactError> {
    let mut entries = fs::read_dir(path).map_err(crate::error::io_error)?;
    if entries
        .next()
        .transpose()
        .map_err(crate::error::io_error)?
        .is_some()
    {
        return Err(ArtifactError::RecoveryRequired);
    }
    Ok(())
}
