use super::*;

#[path = "owner_session_ensure.rs"]
mod ensure;

impl<'a> MutationSession<'a> {
    pub fn sync_directory(
        &self,
        relative_path: &str,
    ) -> Result<DirectoryDurability, ArtifactError> {
        let path = self.directory_path(relative_path)?;
        let chain = open_directory_chain(&path)?;
        self.verify_chain(&chain)?;
        chain.leaf()?.sync_directory()?;
        Ok(DirectoryDurability::Synced)
    }

    pub fn ensure_directory(
        &self,
        relative_path: &str,
    ) -> Result<DirectoryDurability, ArtifactError> {
        if relative_path.is_empty() {
            self.owner.verify_current()?;
            self.owner.root_chain.leaf()?.sync_directory()?;
            return Ok(DirectoryDurability::Synced);
        }
        let relative = validate_relative(relative_path)?;
        let target = self.owner.root_path.join(relative);
        let mut current = self.owner.root_path.clone();
        for component in target
            .strip_prefix(&self.owner.root_path)
            .map_err(|_| ArtifactError::InvalidPath(DIRECTORY_ESCAPED_ROOT))?
            .components()
        {
            let name = normal_component(component)?;
            let parent_chain = open_directory_chain(&current)?;
            self.verify_chain(&parent_chain)?;
            parent_chain.leaf()?.sync_directory()?;
            current.push(name);
            let created = ensure::ensure_child_directory(&current)?;
            if created {
                parent_chain.leaf()?.sync_directory()?;
            }
            let chain = open_directory_chain(&current)?;
            self.verify_chain(&chain)?;
            chain.leaf()?.sync_directory()?;
        }
        self.owner.verify_current()?;
        Ok(DirectoryDurability::Synced)
    }

    pub(crate) fn verify_chain(&self, chain: &DirectoryChain) -> Result<(), ArtifactError> {
        self.owner.verify_current()?;
        for handle in &chain.handles {
            verify_metadata(handle, true)?;
        }
        Ok(())
    }

    pub(crate) fn directory_path(&self, relative_path: &str) -> Result<PathBuf, ArtifactError> {
        if relative_path.is_empty() {
            return Ok(self.owner.root_path.clone());
        }
        Ok(self.owner.root_path.join(validate_relative(relative_path)?))
    }
}

fn normal_component(component: Component<'_>) -> Result<&std::ffi::OsStr, ArtifactError> {
    match component {
        Component::Normal(name) => Ok(name),
        _ => Err(ArtifactError::InvalidPath(DIRECTORY_UNSAFE_COMPONENT)),
    }
}
