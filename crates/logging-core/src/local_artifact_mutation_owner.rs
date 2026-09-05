//! Safe facade owner/session delegation.

use super::*;

#[cfg(windows)]
use super::native::error::error_from_native;
#[cfg(windows)]
use super::native::mutation::mutation_to_native;

#[cfg(windows)]
#[path = "local_artifact_mutation_owner_root.rs"]
mod root;

impl<'a> LocalArtifactMutationSession<'a> {
    pub fn root_identity(&self) -> Result<LocalArtifactIdentity, LocalArtifactMutationError> {
        #[cfg(windows)]
        {
            Ok(LocalArtifactIdentity::from_native(
                self.inner.root_identity(),
            ))
        }
        #[cfg(not(windows))]
        {
            Err(LocalArtifactMutationError::UnsupportedPlatform)
        }
    }

    pub fn verify_current(&self) -> Result<(), LocalArtifactMutationError> {
        #[cfg(windows)]
        {
            self.inner.verify_current().map_err(error_from_native)
        }
        #[cfg(not(windows))]
        {
            Err(LocalArtifactMutationError::UnsupportedPlatform)
        }
    }

    pub fn recover(
        &mut self,
    ) -> Result<Vec<LocalArtifactMutationReceipt>, LocalArtifactMutationError> {
        #[cfg(windows)]
        {
            self.inner
                .recover()
                .map(|receipts| {
                    receipts
                        .into_iter()
                        .map(|receipt| LocalArtifactMutationReceipt::from_native(&receipt))
                        .collect()
                })
                .map_err(error_from_native)
        }
        #[cfg(not(windows))]
        {
            Err(LocalArtifactMutationError::UnsupportedPlatform)
        }
    }

    pub fn append(
        &mut self,
        request_id: &str,
        relative_path: &str,
        payload: &[u8],
    ) -> Result<LocalArtifactMutationReceipt, LocalArtifactMutationError> {
        #[cfg(windows)]
        {
            self.inner
                .append(request_id, relative_path, payload)
                .map(|receipt| LocalArtifactMutationReceipt::from_native(&receipt))
                .map_err(error_from_native)
        }
        #[cfg(not(windows))]
        {
            let _ = (request_id, relative_path, payload);
            Err(LocalArtifactMutationError::UnsupportedPlatform)
        }
    }

    pub fn replace(
        &mut self,
        request_id: &str,
        relative_path: &str,
        payload: &[u8],
    ) -> Result<LocalArtifactMutationReceipt, LocalArtifactMutationError> {
        #[cfg(windows)]
        {
            self.inner
                .replace(request_id, relative_path, payload)
                .map(|receipt| LocalArtifactMutationReceipt::from_native(&receipt))
                .map_err(error_from_native)
        }
        #[cfg(not(windows))]
        {
            let _ = (request_id, relative_path, payload);
            Err(LocalArtifactMutationError::UnsupportedPlatform)
        }
    }

    pub fn remove(
        &mut self,
        request_id: &str,
        relative_path: &str,
    ) -> Result<LocalArtifactMutationReceipt, LocalArtifactMutationError> {
        #[cfg(windows)]
        {
            self.inner
                .remove(request_id, relative_path)
                .map(|receipt| LocalArtifactMutationReceipt::from_native(&receipt))
                .map_err(error_from_native)
        }
        #[cfg(not(windows))]
        {
            let _ = (request_id, relative_path);
            Err(LocalArtifactMutationError::UnsupportedPlatform)
        }
    }

    pub fn remove_tree(
        &mut self,
        request_id: &str,
        relative_path: &str,
    ) -> Result<LocalArtifactMutationReceipt, LocalArtifactMutationError> {
        #[cfg(windows)]
        {
            self.inner
                .remove_tree(request_id, relative_path)
                .map(|receipt| LocalArtifactMutationReceipt::from_native(&receipt))
                .map_err(error_from_native)
        }
        #[cfg(not(windows))]
        {
            let _ = (request_id, relative_path);
            Err(LocalArtifactMutationError::UnsupportedPlatform)
        }
    }

    pub fn apply_transaction(
        &mut self,
        request_id: &str,
        mutations: &[LocalArtifactMutation],
    ) -> Result<LocalArtifactMutationReceipt, LocalArtifactMutationError> {
        #[cfg(windows)]
        {
            let native: Vec<_> = mutations.iter().map(mutation_to_native).collect();
            self.inner
                .apply_transaction(request_id, &native)
                .map(|receipt| LocalArtifactMutationReceipt::from_native(&receipt))
                .map_err(error_from_native)
        }
        #[cfg(not(windows))]
        {
            let _ = (request_id, mutations);
            Err(LocalArtifactMutationError::UnsupportedPlatform)
        }
    }

    pub fn stat(
        &self,
        relative_path: &str,
    ) -> Result<Option<LocalArtifactStat>, LocalArtifactMutationError> {
        #[cfg(windows)]
        {
            self.inner
                .stat(relative_path)
                .map(|stat| stat.map(LocalArtifactStat::from_native))
                .map_err(error_from_native)
        }
        #[cfg(not(windows))]
        {
            let _ = relative_path;
            Err(LocalArtifactMutationError::UnsupportedPlatform)
        }
    }

    pub fn read(
        &self,
        relative_path: &str,
        max_bytes: u64,
    ) -> Result<Vec<u8>, LocalArtifactMutationError> {
        #[cfg(windows)]
        {
            self.inner
                .read(relative_path, max_bytes)
                .map_err(error_from_native)
        }
        #[cfg(not(windows))]
        {
            let _ = (relative_path, max_bytes);
            Err(LocalArtifactMutationError::UnsupportedPlatform)
        }
    }

    pub fn read_snapshot(
        &self,
        relative_path: &str,
        max_bytes: u64,
    ) -> Result<Option<LocalArtifactReadSnapshot>, LocalArtifactMutationError> {
        #[cfg(windows)]
        {
            self.inner
                .read_snapshot(relative_path, max_bytes)
                .map(|snapshot| {
                    snapshot.map(|snapshot| LocalArtifactReadSnapshot::from_native(&snapshot))
                })
                .map_err(error_from_native)
        }
        #[cfg(not(windows))]
        {
            let _ = (relative_path, max_bytes);
            Err(LocalArtifactMutationError::UnsupportedPlatform)
        }
    }

    pub fn list(
        &self,
        relative_path: &str,
    ) -> Result<Vec<LocalArtifactDirectoryEntry>, LocalArtifactMutationError> {
        #[cfg(windows)]
        {
            self.inner
                .list(relative_path)
                .map(|entries| {
                    entries
                        .into_iter()
                        .map(|entry| LocalArtifactDirectoryEntry::from_native(&entry))
                        .collect()
                })
                .map_err(error_from_native)
        }
        #[cfg(not(windows))]
        {
            let _ = relative_path;
            Err(LocalArtifactMutationError::UnsupportedPlatform)
        }
    }

    pub fn ensure_directory(
        &self,
        relative_path: &str,
    ) -> Result<LocalArtifactDirectoryDurability, LocalArtifactMutationError> {
        #[cfg(windows)]
        {
            self.inner
                .ensure_directory(relative_path)
                .map(|_| LocalArtifactDirectoryDurability::Synced)
                .map_err(error_from_native)
        }
        #[cfg(not(windows))]
        {
            let _ = relative_path;
            Err(LocalArtifactMutationError::UnsupportedPlatform)
        }
    }

    pub fn sync_directory(
        &self,
        relative_path: &str,
    ) -> Result<LocalArtifactDirectoryDurability, LocalArtifactMutationError> {
        #[cfg(windows)]
        {
            self.inner
                .sync_directory(relative_path)
                .map(|_| LocalArtifactDirectoryDurability::Synced)
                .map_err(error_from_native)
        }
        #[cfg(not(windows))]
        {
            let _ = relative_path;
            Err(LocalArtifactMutationError::UnsupportedPlatform)
        }
    }
}
