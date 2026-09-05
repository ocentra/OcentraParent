use super::*;

use std::path::Component;

use crate::constants::{
    BRIDGE_DIRECTORY, COMPONENT_ALIASING_SUFFIX, COMPONENT_TOO_LONG, METADATA_NOT_TARGET,
    MUTATION_OWNER_DIRECTORY, PATH_EMPTY, PATH_NOT_RELATIVE, PATH_NOT_UNICODE,
    PATH_UNSAFE_COMPONENT, RELATIVE_PATH_EMPTY_OR_LONG,
};
use crate::platform::windows::{validate_leaf, MAX_COMPONENT_CHARS, MAX_RELATIVE_PATH_CHARS};

pub(super) fn validate_relative(relative: &str) -> Result<PathBuf, ArtifactError> {
    if relative.is_empty() || relative.len() > MAX_RELATIVE_PATH_CHARS {
        return Err(ArtifactError::InvalidPath(RELATIVE_PATH_EMPTY_OR_LONG));
    }
    let path = Path::new(relative);
    if path.is_absolute() {
        return Err(ArtifactError::InvalidPath(PATH_NOT_RELATIVE));
    }
    let mut normalized = PathBuf::new();
    let mut component_names = Vec::new();
    for component in path.components() {
        match component {
            Component::Normal(part) => {
                let text = part
                    .to_str()
                    .ok_or(ArtifactError::InvalidPath(PATH_NOT_UNICODE))?;
                validate_component(text)?;
                component_names.push(text.to_owned());
                normalized.push(part);
            }
            Component::CurDir
            | Component::ParentDir
            | Component::Prefix(_)
            | Component::RootDir => {
                return Err(ArtifactError::InvalidPath(PATH_UNSAFE_COMPONENT));
            }
        }
    }
    if normalized.as_os_str().is_empty() {
        return Err(ArtifactError::InvalidPath(PATH_EMPTY));
    }
    if is_internal_metadata_path(&component_names) {
        return Err(ArtifactError::InvalidPath(METADATA_NOT_TARGET));
    }
    Ok(normalized)
}

pub(super) fn validate_directory_relative(relative: &str) -> Result<PathBuf, ArtifactError> {
    if relative.is_empty() {
        return Ok(PathBuf::new());
    }
    validate_relative(relative)
}

pub(super) fn validate_component(value: &str) -> Result<(), ArtifactError> {
    validate_leaf(value)?;
    if value.encode_utf16().count() > MAX_COMPONENT_CHARS {
        return Err(ArtifactError::InvalidPath(COMPONENT_TOO_LONG));
    }
    if value.ends_with('.') || value.ends_with(' ') {
        return Err(ArtifactError::InvalidPath(COMPONENT_ALIASING_SUFFIX));
    }
    Ok(())
}

fn is_internal_metadata_path(names: &[String]) -> bool {
    names
        .first()
        .is_some_and(|name| name.eq_ignore_ascii_case(BRIDGE_DIRECTORY))
        && (names.len() == 1
            || names
                .get(1)
                .is_some_and(|name| name.eq_ignore_ascii_case(MUTATION_OWNER_DIRECTORY)))
}
