use super::*;

pub(super) fn reject_existing_sibling(
    root: &Path,
    chain: &DirectoryChain,
    name: &str,
) -> Result<(), ArtifactError> {
    let parent = chain
        .paths
        .last()
        .cloned()
        .ok_or(ArtifactError::RecoveryRequired)?;
    let path = sibling_path(parent.as_path(), root, name);
    match OwnedFile::open_directory(&path) {
        Ok(_) => Err(ArtifactError::RecoveryRequired),
        Err(ArtifactError::InvalidPath(_)) => sibling_file_exists(&path),
        Err(ArtifactError::NotFound) => Ok(()),
        Err(error) => Err(error),
    }
}

fn sibling_path(parent: &Path, root: &Path, name: &str) -> PathBuf {
    if parent == root {
        root.join(name)
    } else {
        parent.join(name)
    }
}

fn sibling_file_exists(path: &Path) -> Result<(), ArtifactError> {
    if optional_mutation_file(path)?.is_some() {
        return Err(ArtifactError::RecoveryRequired);
    }
    Ok(())
}
