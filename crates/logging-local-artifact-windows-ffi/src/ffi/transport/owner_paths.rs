//! Lexical containment and retained directory-chain acquisition.

use std::path::{Path, PathBuf};

use crate::error::ArtifactError;

#[path = "owner_paths_chain.rs"]
mod chain;
#[path = "owner_paths_normalize.rs"]
mod normalize;
#[path = "owner_paths_root.rs"]
mod root;
#[path = "owner_paths_validation.rs"]
mod validation;

/// Every handle in this chain remains alive for the operation that uses it.
/// The vector is ordered from the filesystem root towards `leaf`.
#[derive(Debug)]
pub(crate) struct DirectoryChain {
    pub(crate) paths: Vec<PathBuf>,
    pub(crate) handles: Vec<crate::platform::windows::OwnedFile>,
}

impl DirectoryChain {
    pub(crate) fn leaf(&self) -> Result<&crate::platform::windows::OwnedFile, ArtifactError> {
        self.handles.last().ok_or(ArtifactError::InvalidPath(
            crate::constants::ROOT_DIRECTORY_CHAIN_EMPTY,
        ))
    }
}

pub(crate) fn ensure_root_directory(path: &Path) -> Result<DirectoryChain, ArtifactError> {
    root::ensure_root_directory(path)
}

pub(crate) fn open_directory_chain(path: &Path) -> Result<DirectoryChain, ArtifactError> {
    chain::open_directory_chain(path)
}

pub(crate) fn validate_relative(relative: &str) -> Result<PathBuf, ArtifactError> {
    validation::validate_relative(relative)
}

pub(crate) fn validate_directory_relative(relative: &str) -> Result<PathBuf, ArtifactError> {
    validation::validate_directory_relative(relative)
}

pub(crate) fn parent_and_leaf(
    root: &Path,
    relative: &str,
) -> Result<(DirectoryChain, PathBuf, String), ArtifactError> {
    chain::parent_and_leaf(root, relative)
}
