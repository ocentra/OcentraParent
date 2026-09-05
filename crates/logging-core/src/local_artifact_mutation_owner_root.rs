use super::*;

#[cfg(windows)]
impl LocalArtifactMutationOwner {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, LocalArtifactMutationError> {
        ocentra_parent_logging_local_artifact_windows_ffi::owner::LocalArtifactMutationOwner::open(
            path,
        )
        .map(|inner| Self { inner })
        .map_err(error_from_native)
    }

    pub fn root_path(&self) -> &Path {
        self.inner.root_path()
    }

    pub fn root_identity(&self) -> Result<LocalArtifactIdentity, LocalArtifactMutationError> {
        Ok(LocalArtifactIdentity::from_native(
            self.inner.root_identity(),
        ))
    }

    pub fn verify_current(&self) -> Result<(), LocalArtifactMutationError> {
        self.inner.verify_current().map_err(error_from_native)
    }

    pub fn session(&self) -> Result<LocalArtifactMutationSession<'_>, LocalArtifactMutationError> {
        self.inner
            .session()
            .map(|inner| LocalArtifactMutationSession { inner })
            .map_err(error_from_native)
    }
}
