use std::{
    fs::{self, File},
    path::{Path, PathBuf},
};

use super::super::BrowserManagedProfileStoreError;
use super::path_guard_io::reject_indirection;
use super::path_guards::GuardedPathKind;
use super::path_guards_mutation::{remove_directory_tree, rename_guarded};
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

    pub(super) fn open_for_destructive_operation(
        path: &Path,
        kind: GuardedPathKind,
    ) -> Result<Self, BrowserManagedProfileStoreError> {
        reject_indirection(path)?;
        // Destructive mutation is fail-closed in the platform helpers.  Keep
        // delete sharing denied even while acquiring this observational guard
        // so no future caller can accidentally widen the substitution window.
        let file = open_guarded(path, kind, false, true)?;
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
        let identity = stable_file_identity(&file, &metadata)?;
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
        if stable_file_identity(&current, &metadata)? != self.identity {
            return Err(BrowserManagedProfileStoreError::UnsafePath);
        }
        Ok(())
    }

    pub(super) fn rename_to(
        &self,
        target: &Path,
        parent: &StablePathGuard,
    ) -> Result<(), BrowserManagedProfileStoreError> {
        self.validate()?;
        parent.validate()?;
        if target.parent() != Some(parent.path.as_path()) {
            return Err(BrowserManagedProfileStoreError::UnsafePath);
        }
        rename_guarded(&self.path, target, &self.file, &parent.file)
    }

    pub(super) fn remove_tree(&self) -> Result<(), BrowserManagedProfileStoreError> {
        self.validate()?;
        remove_directory_tree(&self.path, &self.file)
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
