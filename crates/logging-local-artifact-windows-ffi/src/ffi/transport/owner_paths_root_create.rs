use super::*;

use std::fs;

use crate::error::io_error;

pub(super) fn directories(
    chain: &mut DirectoryChain,
    missing: &[PathBuf],
) -> Result<(), ArtifactError> {
    for directory in missing.iter().rev() {
        chain.leaf()?.sync_directory()?;
        create_directory(directory)?;
        chain.leaf()?.sync_directory()?;
        let handle = OwnedFile::open_directory(directory)?;
        chain.paths.push(directory.clone());
        chain.handles.push(handle);
    }
    Ok(())
}

fn create_directory(directory: &Path) -> Result<(), ArtifactError> {
    match fs::create_dir(directory) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => Ok(()),
        Err(error) => Err(io_error(error)),
    }
}
