use std::path::{Path, PathBuf};

use same_file::Handle;
use sha2::{Digest, Sha256};
use thiserror::Error;

use crate::platform::identity::{DatabaseIdentity, PhysicalDatabaseIdentity};

pub(crate) mod identity;
mod journal;
pub(crate) mod journal_identity;
mod platform;
mod validation;

#[cfg(test)]
mod path_security_test;

pub(crate) struct PendingSecuredPath {
    canonical: PathBuf,
    file_handle: Handle,
    parent_handle: Handle,
    canonical_path_digest: [u8; 32],
    physical_file_digest: [u8; 32],
    rollback_journal: Option<journal::JournalGuard>,
}

pub(crate) struct SecuredPath {
    canonical: PathBuf,
    file_handle: Handle,
    parent_handle: Handle,
    physical_file_digest: [u8; 32],
    rollback_journal: journal::JournalGuard,
    identity: DatabaseIdentity,
}

impl PendingSecuredPath {
    pub(crate) fn open(path: &Path) -> Result<Self, PathSecurityError> {
        if !platform::stable_sqlite_paths_supported() {
            return Err(PathSecurityError::UnsupportedPlatform);
        }
        reject_unsafe_shape(path)?;
        validation::components(path)?;
        // Open the caller-supplied lexical path before canonicalization.  The
        // canonical path is only accepted when it names the same no-follow
        // file and parent handles.  This closes the validation/canonicalize
        // window instead of binding the broker to whichever object a later
        // path lookup happens to resolve.
        let lexical_parent = path.parent().ok_or(PathSecurityError::UnsafePath)?;
        let (lexical_file_handle, lexical_file_digest) = platform::open_guarded(path, false)?;
        let (lexical_parent_handle, _) = platform::open_guarded(lexical_parent, true)?;
        let canonical = dunce::canonicalize(path).map_err(|_| PathSecurityError::Unavailable)?;
        if !canonical.is_absolute() {
            return Err(PathSecurityError::UnsafePath);
        }
        validation::components(&canonical)?;
        validation::metadata(&canonical)?;
        let parent = canonical.parent().ok_or(PathSecurityError::UnsafePath)?;
        let (file_handle, physical_file_digest) = platform::open_guarded(&canonical, false)?;
        let (parent_handle, _) = platform::open_guarded(parent, true)?;
        if lexical_file_handle != file_handle
            || lexical_parent_handle != parent_handle
            || lexical_file_digest != physical_file_digest
        {
            return Err(PathSecurityError::Replaced);
        }
        let value = Self {
            canonical_path_digest: digest_path(&canonical)?,
            canonical,
            file_handle,
            parent_handle,
            physical_file_digest,
            rollback_journal: None,
        };
        value.revalidate()?;
        Ok(value)
    }

    pub(crate) fn revalidate(&self) -> Result<(), PathSecurityError> {
        revalidate_main(
            &self.canonical,
            &self.file_handle,
            &self.parent_handle,
            self.physical_file_digest,
        )?;
        if let Some(journal) = &self.rollback_journal {
            journal.revalidate(&self.canonical)?;
        } else {
            journal::reject_untracked_sidecars(&self.canonical)?;
        }
        Ok(())
    }

    pub(crate) fn revalidate_quiescent(&self) -> Result<(), PathSecurityError> {
        self.revalidate()?;
        self.rollback_journal
            .as_ref()
            .ok_or(PathSecurityError::Unavailable)?
            .validate_empty()
    }

    pub(crate) fn canonical(&self) -> &Path {
        &self.canonical
    }

    /// Identity of the exact main file, rollback journal, and canonical path
    /// pinned before SQLite is allowed to open either file.
    pub(crate) fn physical_identity(&self) -> Result<PhysicalDatabaseIdentity, PathSecurityError> {
        self.revalidate_quiescent()?;
        let rollback_journal = self
            .rollback_journal
            .as_ref()
            .ok_or(PathSecurityError::Unavailable)?;
        PhysicalDatabaseIdentity::from_parts(
            self.canonical_path_digest,
            self.physical_file_digest,
            rollback_journal.digest(),
        )
        .map_err(|_| PathSecurityError::Unavailable)
    }

    pub(crate) fn secure_rollback_journal(&mut self) -> Result<(), PathSecurityError> {
        if self.rollback_journal.is_some() {
            return Err(PathSecurityError::UnsafePath);
        }
        self.revalidate()?;
        self.rollback_journal = Some(journal::JournalGuard::secure(&self.canonical)?);
        self.revalidate()
    }

    pub(crate) fn bind_instance(
        self,
        database_instance_id: [u8; 32],
    ) -> Result<SecuredPath, PathSecurityError> {
        self.revalidate_quiescent()?;
        let rollback_journal = self
            .rollback_journal
            .ok_or(PathSecurityError::Unavailable)?;
        let physical_identity = PhysicalDatabaseIdentity::from_parts(
            self.canonical_path_digest,
            self.physical_file_digest,
            rollback_journal.digest(),
        )
        .map_err(|_| PathSecurityError::Unavailable)?;
        let identity = DatabaseIdentity::from_parts(physical_identity, database_instance_id)
            .map_err(|_| PathSecurityError::Unavailable)?;
        Ok(SecuredPath {
            canonical: self.canonical,
            file_handle: self.file_handle,
            parent_handle: self.parent_handle,
            physical_file_digest: self.physical_file_digest,
            rollback_journal,
            identity,
        })
    }
}

impl SecuredPath {
    pub(crate) fn revalidate(&self) -> Result<(), PathSecurityError> {
        revalidate_main(
            &self.canonical,
            &self.file_handle,
            &self.parent_handle,
            self.physical_file_digest,
        )?;
        self.rollback_journal.revalidate(&self.canonical)?;
        self.rollback_journal.validate_empty()
    }

    pub(crate) fn canonical(&self) -> &Path {
        &self.canonical
    }

    pub(crate) fn identity(&self) -> DatabaseIdentity {
        self.identity
    }
}

#[derive(Debug, Error)]
pub(crate) enum PathSecurityError {
    #[error("stable custody paths are unsupported on this platform")]
    UnsupportedPlatform,
    #[error("database path is unavailable")]
    Unavailable,
    #[error("database path is unsafe")]
    UnsafePath,
    #[error("database path was replaced")]
    Replaced,
}

fn revalidate_main(
    canonical: &Path,
    file_handle: &Handle,
    parent_handle: &Handle,
    physical_file_digest: [u8; 32],
) -> Result<(), PathSecurityError> {
    validation::components(canonical)?;
    validation::metadata(canonical)?;
    let parent = canonical.parent().ok_or(PathSecurityError::UnsafePath)?;
    let (current_file, current_digest) = platform::open_guarded(canonical, false)?;
    let (current_parent, _) = platform::open_guarded(parent, true)?;
    if current_file != *file_handle
        || current_parent != *parent_handle
        || current_digest != physical_file_digest
    {
        return Err(PathSecurityError::Replaced);
    }
    Ok(())
}

fn reject_unsafe_shape(path: &Path) -> Result<(), PathSecurityError> {
    if !path.is_absolute() {
        return Err(PathSecurityError::UnsafePath);
    }
    let text = path.as_os_str().to_string_lossy();
    let lower = text.to_ascii_lowercase();
    if lower == ":memory:"
        || lower.starts_with("file:")
        || lower.contains("mode=memory")
        || text.contains('?')
        || text.contains('#')
    {
        return Err(PathSecurityError::UnsafePath);
    }
    validation::platform_shape(path)
}

fn digest_path(path: &Path) -> Result<[u8; 32], PathSecurityError> {
    let bytes = path_bytes(path)?;
    let mut hasher = Sha256::new();
    hasher.update(b"ocentra.database-path.v1");
    hasher.update((bytes.len() as u32).to_be_bytes());
    hasher.update(bytes);
    Ok(hasher.finalize().into())
}

#[cfg(unix)]
fn path_bytes(path: &Path) -> Result<Vec<u8>, PathSecurityError> {
    use std::os::unix::ffi::OsStrExt;

    Ok(path.as_os_str().as_bytes().to_vec())
}

#[cfg(windows)]
fn path_bytes(path: &Path) -> Result<Vec<u8>, PathSecurityError> {
    use std::os::windows::ffi::OsStrExt;

    Ok(path
        .as_os_str()
        .encode_wide()
        .flat_map(u16::to_le_bytes)
        .collect())
}

#[cfg(not(any(unix, windows)))]
fn path_bytes(_path: &Path) -> Result<Vec<u8>, PathSecurityError> {
    Err(PathSecurityError::Unavailable)
}
