use std::path::{Path, PathBuf};

use same_file::Handle;
use sha2::{Digest, Sha256};
use thiserror::Error;

use crate::platform::DatabaseIdentity;

mod platform;
mod validation;

pub(crate) struct PendingSecuredPath {
    canonical: PathBuf,
    file_handle: Handle,
    parent_handle: Handle,
    canonical_path_digest: [u8; 32],
    physical_file_digest: [u8; 32],
}

pub(crate) struct SecuredPath {
    canonical: PathBuf,
    file_handle: Handle,
    parent_handle: Handle,
    physical_file_digest: [u8; 32],
    identity: DatabaseIdentity,
}

impl PendingSecuredPath {
    pub(crate) fn open(path: &Path) -> Result<Self, PathSecurityError> {
        reject_unsafe_shape(path)?;
        validation::components(path)?;
        let canonical = dunce::canonicalize(path).map_err(|_| PathSecurityError::Unavailable)?;
        if !canonical.is_absolute() {
            return Err(PathSecurityError::UnsafePath);
        }
        validation::components(&canonical)?;
        validation::metadata(&canonical)?;
        let parent = canonical.parent().ok_or(PathSecurityError::UnsafePath)?;
        let (file_handle, physical_file_digest) = platform::open_guarded(&canonical, false)?;
        let (parent_handle, _) = platform::open_guarded(parent, true)?;
        let value = Self {
            canonical_path_digest: digest_path(&canonical)?,
            canonical,
            file_handle,
            parent_handle,
            physical_file_digest,
        };
        value.revalidate()?;
        Ok(value)
    }

    pub(crate) fn revalidate(&self) -> Result<(), PathSecurityError> {
        revalidate(
            &self.canonical,
            &self.file_handle,
            &self.parent_handle,
            self.physical_file_digest,
        )
    }

    pub(crate) fn canonical(&self) -> &Path {
        &self.canonical
    }

    pub(crate) fn bind_instance(
        self,
        database_instance_id: [u8; 32],
    ) -> Result<SecuredPath, PathSecurityError> {
        self.revalidate()?;
        let identity = DatabaseIdentity::from_parts(
            self.canonical_path_digest,
            self.physical_file_digest,
            database_instance_id,
        )
        .map_err(|_| PathSecurityError::Unavailable)?;
        Ok(SecuredPath {
            canonical: self.canonical,
            file_handle: self.file_handle,
            parent_handle: self.parent_handle,
            physical_file_digest: self.physical_file_digest,
            identity,
        })
    }
}

impl SecuredPath {
    pub(crate) fn revalidate(&self) -> Result<(), PathSecurityError> {
        revalidate(
            &self.canonical,
            &self.file_handle,
            &self.parent_handle,
            self.physical_file_digest,
        )
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
    #[error("database path is unavailable")]
    Unavailable,
    #[error("database path is unsafe")]
    UnsafePath,
    #[error("database path was replaced")]
    Replaced,
}

fn revalidate(
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
