use std::{
    fs::{self, File},
    io::Read,
    path::Path,
};

use super::super::{BrowserManagedProfileStoreError, BrowserManagedProfileStorePaths};
use super::path_guard::StablePathGuard;
use super::path_guard_io::{open_optional, open_or_create};

/// Handles held for the whole store operation.
///
/// The root, every root ancestor, and the lock file stay open while a record
/// is read or mutated. Windows opens deny delete/rename and use
/// `FILE_FLAG_OPEN_REPARSE_POINT`; every platform reopens and compares stable
/// file identity before a path is used. This closes the check-then-use gap
/// without importing a sibling runtime's private custody implementation.
pub(super) struct ProfileStorePathGuards {
    root: StablePathGuard,
    ancestors: Vec<StablePathGuard>,
    lock: StablePathGuard,
}

#[derive(Clone, Copy)]
pub(super) enum GuardedPathKind {
    File,
    Directory,
}

impl ProfileStorePathGuards {
    pub(super) fn open(
        paths: &BrowserManagedProfileStorePaths,
    ) -> Result<Self, BrowserManagedProfileStoreError> {
        let root = paths
            .profile_dir
            .parent()
            .ok_or(BrowserManagedProfileStoreError::UnsafePath)?;
        let root_guard = StablePathGuard::open(root, GuardedPathKind::Directory, true)?;
        let ancestors = open_ancestor_guards(root)?;
        let lock = open_or_create(&paths.lock_path, true)?;
        let guards = Self {
            root: root_guard,
            ancestors,
            lock,
        };
        guards.validate()?;
        Ok(guards)
    }

    pub(super) fn lock_file(&self) -> &File {
        &self.lock.file
    }

    pub(super) fn validate(&self) -> Result<(), BrowserManagedProfileStoreError> {
        self.root.validate()?;
        for ancestor in &self.ancestors {
            ancestor.validate()?;
        }
        self.lock.validate()
    }

    pub(super) fn read_text(
        &self,
        path: &Path,
    ) -> Result<Option<String>, BrowserManagedProfileStoreError> {
        self.validate()?;
        let Some(guard) = open_optional(path, GuardedPathKind::File, true)? else {
            return Ok(None);
        };
        let mut contents = String::new();
        let mut file = guard
            .file
            .try_clone()
            .map_err(|_error| BrowserManagedProfileStoreError::Io)?;
        file.read_to_string(&mut contents)
            .map_err(|_error| BrowserManagedProfileStoreError::Io)?;
        guard.validate()?;
        self.validate()?;
        Ok(Some(contents))
    }

    pub(super) fn directory_exists(
        &self,
        path: &Path,
    ) -> Result<bool, BrowserManagedProfileStoreError> {
        self.validate()?;
        let present = open_optional(path, GuardedPathKind::Directory, true)?.is_some();
        self.validate()?;
        Ok(present)
    }

    pub(super) fn validate_path(
        &self,
        path: &Path,
        kind: GuardedPathKind,
    ) -> Result<(), BrowserManagedProfileStoreError> {
        self.validate()?;
        StablePathGuard::open(path, kind, true)?.validate()?;
        self.validate()
    }

    pub(super) fn remove_directory(
        &self,
        path: &Path,
    ) -> Result<(), BrowserManagedProfileStoreError> {
        self.validate()?;
        let guard = StablePathGuard::open(path, GuardedPathKind::Directory, false)?;
        guard.validate()?;
        fs::remove_dir_all(path).map_err(|_error| BrowserManagedProfileStoreError::Io)?;
        if fs::symlink_metadata(path).is_ok() {
            return Err(BrowserManagedProfileStoreError::UnsafePath);
        }
        self.validate()
    }
}

pub(super) fn guarded_directory_path_kind() -> GuardedPathKind {
    GuardedPathKind::Directory
}

fn open_ancestor_guards(
    root: &Path,
) -> Result<Vec<StablePathGuard>, BrowserManagedProfileStoreError> {
    let parent = root
        .parent()
        .ok_or(BrowserManagedProfileStoreError::UnsafePath)?;
    parent
        .ancestors()
        .map(|ancestor| StablePathGuard::open(ancestor, GuardedPathKind::Directory, true))
        .collect()
}
