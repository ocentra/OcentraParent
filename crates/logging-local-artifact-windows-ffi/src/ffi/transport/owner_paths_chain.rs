use super::*;

use std::path::PathBuf;

use crate::constants::{TARGET_NOT_UNICODE, TARGET_NO_PARENT};
use crate::platform::windows::{validate_leaf, OwnedFile};

pub(super) fn open_directory_chain(path: &Path) -> Result<DirectoryChain, ArtifactError> {
    let normalized = super::normalize::normalize_absolute_path(path, false)?;
    let mut paths: Vec<PathBuf> = normalized
        .ancestors()
        .filter(|ancestor| !ancestor.as_os_str().is_empty())
        .map(PathBuf::from)
        .collect();
    paths.reverse();
    let handles = open_chain_handles(&paths)?;
    Ok(DirectoryChain { paths, handles })
}

fn open_chain_handles(paths: &[PathBuf]) -> Result<Vec<OwnedFile>, ArtifactError> {
    let mut handles = Vec::with_capacity(paths.len());
    for (index, directory) in paths.iter().enumerate() {
        let handle = if index + 1 == paths.len() {
            OwnedFile::open_sync_directory(directory)?
        } else {
            OwnedFile::open_directory(directory)?
        };
        handles.push(handle);
    }
    Ok(handles)
}

pub(super) fn parent_and_leaf(
    root: &Path,
    relative: &str,
) -> Result<(DirectoryChain, PathBuf, String), ArtifactError> {
    let relative_path = super::validation::validate_relative(relative)?;
    let target = root.join(&relative_path);
    let parent = target
        .parent()
        .ok_or(ArtifactError::InvalidPath(TARGET_NO_PARENT))?;
    let leaf = target
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or(ArtifactError::InvalidPath(TARGET_NOT_UNICODE))?;
    validate_leaf(leaf)?;
    let leaf = leaf.to_owned();
    Ok((open_directory_chain(parent)?, target, leaf))
}
