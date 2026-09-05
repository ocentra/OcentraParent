use super::{LocalArtifactReadSnapshot, LocalArtifactStat};

impl LocalArtifactReadSnapshot {
    #[cfg(windows)]
    pub(super) fn from_native(
        snapshot: &ocentra_parent_logging_local_artifact_windows_ffi::owner::LocalArtifactReadSnapshot,
    ) -> Self {
        Self {
            stat: LocalArtifactStat::from_native(snapshot.stat()),
            bytes: snapshot.bytes().to_vec(),
        }
    }

    pub fn stat(&self) -> LocalArtifactStat {
        self.stat
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}
