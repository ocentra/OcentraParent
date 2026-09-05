use super::LocalArtifactIdentity;

impl LocalArtifactIdentity {
    #[cfg(windows)]
    pub(super) fn from_native(
        identity: ocentra_parent_logging_local_artifact_windows_ffi::owner::LocalArtifactIdentity,
    ) -> Self {
        Self {
            volume_serial_number: identity.volume_serial_number(),
            file_id: identity.file_id(),
        }
    }

    pub fn volume_serial_number(&self) -> u64 {
        self.volume_serial_number
    }

    pub fn file_id(&self) -> [u8; 16] {
        self.file_id
    }
}
