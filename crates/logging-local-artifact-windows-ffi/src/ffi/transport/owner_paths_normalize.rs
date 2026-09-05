use super::*;

use std::path::Component;

use crate::constants::{
    ROOT_HAS_DOT_COMPONENT, ROOT_NOT_FILESYSTEM_ROOT, ROOT_PATH_NOT_UNICODE, ROOT_SEPARATOR,
};
use crate::error::io_error;

pub(super) fn normalize_root(input: &Path) -> Result<PathBuf, ArtifactError> {
    normalize_absolute_path(input, true)
}

pub(super) fn normalize_absolute_path(
    input: &Path,
    reject_filesystem_root: bool,
) -> Result<PathBuf, ArtifactError> {
    let absolute = if input.is_absolute() {
        input.to_path_buf()
    } else {
        std::env::current_dir().map_err(io_error)?.join(input)
    };
    let mut normalized = PathBuf::new();
    for component in absolute.components() {
        match component {
            Component::Prefix(prefix) => normalized.push(prefix.as_os_str()),
            Component::RootDir => normalized.push(Path::new(ROOT_SEPARATOR)),
            Component::Normal(part) => {
                let text = part
                    .to_str()
                    .ok_or(ArtifactError::InvalidPath(ROOT_PATH_NOT_UNICODE))?;
                super::validation::validate_component(text)?;
                normalized.push(part);
            }
            Component::CurDir | Component::ParentDir => {
                return Err(ArtifactError::InvalidPath(ROOT_HAS_DOT_COMPONENT));
            }
        }
    }
    if reject_filesystem_root
        && normalized
            .parent()
            .map(|parent| parent == normalized.as_path())
            .unwrap_or(true)
    {
        return Err(ArtifactError::InvalidPath(ROOT_NOT_FILESYSTEM_ROOT));
    }
    Ok(normalized)
}
