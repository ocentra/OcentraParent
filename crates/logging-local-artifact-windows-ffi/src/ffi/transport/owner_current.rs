use super::*;

impl LocalArtifactMutationOwner {
    /// Revalidate the canonical root path and every retained ancestor.
    pub fn verify_current(&self) -> Result<(), ArtifactError> {
        if self.root_chain.handles.len() != self.root_identities.len() {
            return Err(ArtifactError::AncestorIdentityChanged);
        }
        for (handle, expected) in self.root_chain.handles.iter().zip(&self.root_identities) {
            let current = verify_metadata(handle, true)?.identity;
            if current != *expected {
                return Err(ArtifactError::AncestorIdentityChanged);
            }
        }
        let path_handle = OwnedFile::open_sync_directory(&self.root_path)?;
        let path_identity = verify_metadata(&path_handle, true)?.identity;
        if path_identity != self.root_identity {
            return Err(ArtifactError::RootIdentityChanged);
        }
        Ok(())
    }
}
