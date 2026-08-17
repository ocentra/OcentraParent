use std::fs;

use super::{ChildAgentServiceError, ChildAgentServicePaths, CHILD_AGENT_DATA_DIR_ENV};

impl ChildAgentServicePaths {
    pub fn from_root(root: impl Into<std::path::PathBuf>) -> Self {
        let root = root.into();
        Self {
            journal: root.join("child-runtime.ndjson"),
            tombstones: root.join("tombstones"),
            removal: root.join("removal-state.json"),
            identity: None,
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

    pub(crate) fn identity(&self) -> Option<&super::ChildAgentServiceIdentity> {
        self.identity.as_ref()
    }

    pub(crate) fn with_identity(mut self, identity: super::ChildAgentServiceIdentity) -> Self {
        self.identity = Some(identity);
        self
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
