use super::*;

use std::fs;

use crate::constants::ROOT_NOT_DIRECTORY;
use crate::error::io_error;

pub(super) fn collect(normalized: &Path) -> Result<Vec<PathBuf>, ArtifactError> {
    let mut missing = Vec::new();
    let mut cursor = normalized.to_path_buf();
    loop {
        match fs::symlink_metadata(&cursor) {
            Ok(metadata) if metadata.is_dir() => return Ok(missing),
            Ok(_) => return Err(ArtifactError::InvalidPath(ROOT_NOT_DIRECTORY)),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                missing.push(cursor.clone());
                cursor = cursor
                    .parent()
                    .ok_or(ArtifactError::NotFound)?
                    .to_path_buf();
            }
            Err(error) => return Err(io_error(error)),
        }
    }
}
