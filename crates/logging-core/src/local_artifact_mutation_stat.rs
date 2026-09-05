use super::{LocalArtifactIdentity, LocalArtifactStat};

impl LocalArtifactStat {
    #[cfg(windows)]
    pub(super) fn from_native(
        stat: ocentra_parent_logging_local_artifact_windows_ffi::owner::LocalArtifactStat,
    ) -> Self {
        Self {
            identity: LocalArtifactIdentity::from_native(stat.identity()),
            size: stat.length(),
            links: stat.links(),
            is_directory: stat.is_directory(),
            modified_ms: stat.modified_ms(),
        }
    }

    pub fn identity(&self) -> LocalArtifactIdentity {
        self.identity
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn links(&self) -> u32 {
        self.links
    }

    pub fn is_directory(&self) -> bool {
        self.is_directory
    }

    pub fn modified_ms(&self) -> i64 {
        self.modified_ms
    }
}
