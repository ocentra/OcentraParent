use super::*;

use crate::platform::windows::OwnedFile;
#[path = "owner_paths_root_create.rs"]
mod create;
#[path = "owner_paths_root_missing.rs"]
mod missing;

pub(super) fn ensure_root_directory(path: &Path) -> Result<DirectoryChain, ArtifactError> {
    let normalized = super::normalize::normalize_root(path)?;
    match OwnedFile::open_directory(&normalized) {
        Ok(_) => return super::chain::open_directory_chain(&normalized),
        Err(ArtifactError::NotFound) => {}
        Err(error) => return Err(error),
    }
    let missing = missing::collect(&normalized)?;
    let cursor = missing
        .last()
        .and_then(|path| path.parent())
        .ok_or(ArtifactError::NotFound)?
        .to_path_buf();
    let mut chain = super::chain::open_directory_chain(&cursor)?;
    create::directories(&mut chain, &missing)?;
    if chain.paths.last() != Some(&normalized) {
        return Err(ArtifactError::RootIdentityChanged);
    }
    Ok(chain)
}
