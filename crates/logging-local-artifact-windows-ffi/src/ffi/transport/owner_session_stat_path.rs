use std::path::PathBuf;

use crate::error::ArtifactError;
use crate::owner_paths::DirectoryChain;

use super::MutationSession;

pub(super) fn resolve(
    session: &MutationSession<'_>,
    relative_path: &str,
) -> Result<Option<(DirectoryChain, PathBuf)>, ArtifactError> {
    let (chain, target, _) =
        match crate::owner_paths::parent_and_leaf(&session.owner.root_path, relative_path) {
            Ok(target) => target,
            Err(ArtifactError::NotFound) => return Ok(None),
            Err(error) => return Err(error),
        };
    match session.verify_chain(&chain) {
        Ok(()) => {}
        Err(ArtifactError::NotFound) => return Ok(None),
        Err(error) => return Err(error),
    }
    Ok(Some((chain, target)))
}
