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
/// is read or mutated. Windows opens use `FILE_FLAG_OPEN_REPARSE_POINT`, and
/// destructive operations retain their guarded target through the operation
/// and verify stable identity before/after path use. Other platforms reopen
/// and compare stable identity before a path is used.
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
        // Open/create the root as part of the retained guard set.  The prior
        // caller-side bootstrap opened a root guard and dropped it before the
        // lock/mutation operation, leaving a substitution window.  Keeping the
        // guard here makes root custody span the complete operation.  Missing
        // roots fail closed in `open_or_create_guard`; this boundary must not
        // create an attacker-swappable path on behalf of a caller.
        let root_guard = super::path_guards_root::ensure_directory_chain(root)?;
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
        let metadata = guard
            .file
            .metadata()
            .map_err(|_error| BrowserManagedProfileStoreError::Io)?;
        let max_bytes = u64::try_from(
            ocentra_parent_agent_protocol::constants::browser::PROFILE_STORE_MAX_METADATA_BYTES,
        )
        .map_err(|_error| BrowserManagedProfileStoreError::Io)?;
        if metadata.len() > max_bytes {
            return Err(BrowserManagedProfileStoreError::MetadataCorrupt);
        }
        let mut file = guard
            .file
            .try_clone()
            .map_err(|_error| BrowserManagedProfileStoreError::Io)?;
        let mut contents = Vec::new();
        file.take(max_bytes.saturating_add(1))
            .read_to_end(&mut contents)
            .map_err(|_error| BrowserManagedProfileStoreError::Io)?;
        if contents.len()
            > ocentra_parent_agent_protocol::constants::browser::PROFILE_STORE_MAX_METADATA_BYTES
        {
            return Err(BrowserManagedProfileStoreError::MetadataCorrupt);
        }
        guard.validate()?;
        self.validate()?;
        String::from_utf8(contents)
            .map(Some)
            .map_err(|_error| BrowserManagedProfileStoreError::MetadataCorrupt)
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

    pub(super) fn rename_directory(
        &self,
        source: &Path,
        target: &Path,
    ) -> Result<(), BrowserManagedProfileStoreError> {
        self.validate()?;
        let source_guard =
            StablePathGuard::open_for_destructive_operation(source, GuardedPathKind::Directory)?;
        source_guard.rename_to(target, &self.root)?;
        self.validate()
    }

    pub(super) fn remove_directory(
        &self,
        path: &Path,
    ) -> Result<(), BrowserManagedProfileStoreError> {
        self.validate()?;
        let guard =
            StablePathGuard::open_for_destructive_operation(path, GuardedPathKind::Directory)?;
        guard.remove_tree()?;
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
