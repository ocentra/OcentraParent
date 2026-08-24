use std::ffi::OsString;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use same_file::Handle;

use super::PathSecurityError;

pub(super) struct JournalGuard {
    path: PathBuf,
    handle: Handle,
    physical_digest: [u8; 32],
}

impl JournalGuard {
    pub(super) fn secure(database: &Path) -> Result<Self, PathSecurityError> {
        reject_untracked_sidecars(database)?;
        let path = sidecar(database, "-journal");
        let (handle, physical_digest) = match std::fs::symlink_metadata(&path) {
            Ok(_) => {
                super::validation::metadata(&path)?;
                super::platform::open_guarded(&path, false)?
            }
            Err(error) if error.kind() == ErrorKind::NotFound => {
                super::platform::create_guarded(&path)?
            }
            Err(_) => return Err(PathSecurityError::Unavailable),
        };
        let value = Self {
            path,
            handle,
            physical_digest,
        };
        value.revalidate(database)?;
        Ok(value)
    }

    pub(super) fn digest(&self) -> [u8; 32] {
        self.physical_digest
    }

    pub(super) fn revalidate(&self, database: &Path) -> Result<(), PathSecurityError> {
        reject_untracked_sidecars(database)?;
        if self.path != sidecar(database, "-journal") {
            return Err(PathSecurityError::Replaced);
        }
        super::validation::metadata(&self.path)?;
        let (current, digest) = super::platform::open_guarded(&self.path, false)?;
        if current != self.handle || digest != self.physical_digest {
            return Err(PathSecurityError::Replaced);
        }
        Ok(())
    }

    pub(super) fn validate_empty(&self) -> Result<(), PathSecurityError> {
        let metadata = self
            .handle
            .as_file()
            .metadata()
            .map_err(|_| PathSecurityError::Unavailable)?;
        if metadata.len() != 0 {
            return Err(PathSecurityError::UnsafePath);
        }
        Ok(())
    }
}

pub(super) fn reject_untracked_sidecars(database: &Path) -> Result<(), PathSecurityError> {
    for suffix in ["-wal", "-shm"] {
        match std::fs::symlink_metadata(sidecar(database, suffix)) {
            Ok(_) => return Err(PathSecurityError::UnsafePath),
            Err(error) if error.kind() == ErrorKind::NotFound => {}
            Err(_) => return Err(PathSecurityError::Unavailable),
        }
    }
    Ok(())
}

pub(super) fn sidecar(database: &Path, suffix: &str) -> PathBuf {
    let mut value = OsString::from(database.as_os_str());
    value.push(suffix);
    PathBuf::from(value)
}
