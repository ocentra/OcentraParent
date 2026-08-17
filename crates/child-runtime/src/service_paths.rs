use std::fs;
use std::sync::Arc;

use super::{
    trust_binding::ChildAgentTrustBindingSource, ChildAgentServiceError, ChildAgentServicePaths,
    CHILD_AGENT_DATA_DIR_ENV,
};

impl ChildAgentServicePaths {
    pub fn from_root(root: impl Into<std::path::PathBuf>) -> Self {
        let root = root.into();
        Self {
            journal: root.join("child-runtime.ndjson"),
            tombstones: root.join("tombstones"),
            removal: root.join("removal-state.json"),
            trust_binding_source: None,
            root,
        }
    }

    pub fn from_environment() -> Result<Self, ChildAgentServiceError> {
        let root = std::env::var_os(CHILD_AGENT_DATA_DIR_ENV).ok_or_else(|| {
            ChildAgentServiceError::Configuration(format!(
                "{CHILD_AGENT_DATA_DIR_ENV} must identify the child service data directory"
            ))
        })?;
        Ok(Self::from_root(root))
    }

    pub fn root(&self) -> &std::path::Path {
        &self.root
    }

    pub fn journal(&self) -> &std::path::Path {
        &self.journal
    }

    pub fn tombstones(&self) -> &std::path::Path {
        &self.tombstones
    }

    pub fn removal(&self) -> &std::path::Path {
        &self.removal
    }

    pub fn with_trust_binding_source(
        mut self,
        source: Arc<dyn ChildAgentTrustBindingSource>,
    ) -> Self {
        self.trust_binding_source = Some(source);
        self
    }

    pub(super) fn trust_binding_source(&self) -> Option<&dyn ChildAgentTrustBindingSource> {
        self.trust_binding_source.as_deref()
    }

    pub(super) fn current_trust_binding(
        &self,
    ) -> Result<
        ocentra_family_identity_core::device_trust_current_binding::CurrentChildDeviceTrustBinding,
        super::trust_binding::ChildAgentTrustBindingError,
    > {
        self.trust_binding_source()
            .ok_or(super::trust_binding::ChildAgentTrustBindingError::Unavailable)?
            .current_trust_binding()
    }

    pub(super) fn prepare(&self) -> Result<(), ChildAgentServiceError> {
        fs::create_dir_all(&self.root).map_err(ChildAgentServiceError::Storage)?;
        reject_symlink(&self.root, "child service data directory")?;
        reject_symlink(&self.journal, "child service journal")?;
        reject_symlink(&self.removal, "child service removal state")?;
        reject_symlink(&self.tombstones, "child service tombstone directory")?;
        fs::create_dir_all(&self.tombstones).map_err(ChildAgentServiceError::Storage)?;
        reject_symlink(&self.tombstones, "child service tombstone directory")?;
        Ok(())
    }
}

fn reject_symlink(path: &std::path::Path, label: &str) -> Result<(), ChildAgentServiceError> {
    match fs::symlink_metadata(path) {
        Ok(metadata) if metadata.file_type().is_symlink() => {
            Err(ChildAgentServiceError::Storage(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("{label} must not be a symlink"),
            )))
        }
        Ok(_) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(ChildAgentServiceError::Storage(error)),
    }
}
