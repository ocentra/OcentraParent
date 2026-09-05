use std::path::Path;

use crate::error::ArtifactError;
use crate::owner_paths::open_directory_chain;

use super::*;

#[path = "owner_journal_metadata_directory.rs"]
mod directory;
#[path = "owner_journal_metadata_paths.rs"]
mod paths;

pub(super) fn ensure_metadata_dirs(root: &Path) -> Result<MetadataDirs, ArtifactError> {
    let metadata = paths::from_root(root);
    directory::ensure(&metadata.bridge)?;
    directory::ensure(&metadata.owner)?;
    directory::ensure(&metadata.receipts)?;
    directory::ensure(&metadata.intents)?;
    Ok(MetadataDirs {
        receipts: open_directory_chain(&metadata.receipts)?,
        intents: open_directory_chain(&metadata.intents)?,
    })
}
