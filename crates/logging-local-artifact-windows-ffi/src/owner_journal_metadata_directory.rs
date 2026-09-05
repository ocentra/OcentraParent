use std::fs;
use std::path::Path;

use crate::error::{io_error, ArtifactError};
use crate::owner_paths::open_directory_chain;

const NOT_DIRECTORY: &str = "owner metadata path is not a directory";
const NO_PARENT: &str = "metadata directory has no parent";
const PARENT_ESCAPE: &str = "metadata directory parent is unavailable";

pub(super) fn ensure(path: &Path) -> Result<(), ArtifactError> {
    match fs::symlink_metadata(path) {
        Ok(metadata) => existing(metadata),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => create(path),
        Err(error) => Err(io_error(error)),
    }
}

fn existing(metadata: fs::Metadata) -> Result<(), ArtifactError> {
    if metadata.is_dir() {
        Ok(())
    } else {
        Err(ArtifactError::InvalidPath(NOT_DIRECTORY))
    }
}

fn create(path: &Path) -> Result<(), ArtifactError> {
    let parent = path.parent().ok_or(ArtifactError::InvalidPath(NO_PARENT))?;
    let parent_chain =
        open_directory_chain(parent).map_err(|_| ArtifactError::InvalidPath(PARENT_ESCAPE))?;
    parent_chain.leaf()?.sync_directory()?;
    create_child(path)?;
    parent_chain.leaf()?.sync_directory()?;
    open_directory_chain(path)?.leaf()?.sync_directory()
}

fn create_child(path: &Path) -> Result<(), ArtifactError> {
    match fs::create_dir(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
            let metadata = fs::symlink_metadata(path).map_err(io_error)?;
            if metadata.is_dir() {
                Ok(())
            } else {
                Err(ArtifactError::LinkOrReparseDetected)
            }
        }
        Err(error) => Err(io_error(error)),
    }
}
