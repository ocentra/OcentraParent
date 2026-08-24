use std::{
    fs::{self, File},
    path::{Path, PathBuf},
};

use super::super::BrowserManagedProfileStoreError;
use super::path_guard_io::reject_indirection;
use super::path_guards::GuardedPathKind;
use super::path_guards_platform::{
    metadata_is_indirection, open_guarded, stable_file_identity, StableFileIdentity,
};

pub(super) struct StablePathGuard {
    pub(super) path: PathBuf,
    pub(super) file: File,
    identity: StableFileIdentity,
    kind: GuardedPathKind,
}

impl StablePathGuard {
    pub(super) fn open(
        path: &Path,
        kind: GuardedPathKind,
        deny_delete: bool,
    ) -> Result<Self, BrowserManagedProfileStoreError> {
        reject_indirection(path)?;
        let file = open_guarded(path, kind, false, deny_delete)?;
        Self::from_file(path, file, kind)
    }

    pub(super) fn from_file(
        path: &Path,
        file: File,
        kind: GuardedPathKind,
    ) -> Result<Self, BrowserManagedProfileStoreError> {
        let metadata = file
            .metadata()
            .map_err(|_error| BrowserManagedProfileStoreError::Io)?;
        if !kind.matches(&metadata) || metadata_is_indirection(&metadata) {
            return Err(BrowserManagedProfileStoreError::UnsafePath);
        }
        let identity = stable_file_identity(path, &file, &metadata)?;
        Ok(Self {
            path: path.to_path_buf(),
            file,
            identity,
            kind,
        })
    }

    pub(super) fn validate(&self) -> Result<(), BrowserManagedProfileStoreError> {
        reject_indirection(&self.path)?;
        let current = open_guarded(&self.path, self.kind, false, true)?;
        let metadata = current
            .metadata()
            .map_err(|_error| BrowserManagedProfileStoreError::Io)?;
        if !self.kind.matches(&metadata) || metadata_is_indirection(&metadata) {
            return Err(BrowserManagedProfileStoreError::UnsafePath);
        }
        if stable_file_identity(&self.path, &current, &metadata)? != self.identity {
            return Err(BrowserManagedProfileStoreError::UnsafePath);
        }
        Ok(())
    }
}

impl GuardedPathKind {
    pub(super) fn matches(self, metadata: &fs::Metadata) -> bool {
        match self {
            Self::File => metadata.is_file(),
            Self::Directory => metadata.is_dir(),
        }
    }
}
