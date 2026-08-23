use std::path::{Component, Path, PathBuf};

use super::PathSecurityError;

pub(super) fn components(path: &Path) -> Result<(), PathSecurityError> {
    let mut current = PathBuf::new();
    for component in path.components() {
        if matches!(component, Component::CurDir | Component::ParentDir) {
            return Err(PathSecurityError::UnsafePath);
        }
        current.push(component.as_os_str());
        if current.parent().is_none() {
            continue;
        }
        let metadata =
            std::fs::symlink_metadata(&current).map_err(|_| PathSecurityError::Unavailable)?;
        if metadata.file_type().is_symlink() {
            return Err(PathSecurityError::UnsafePath);
        }
    }
    Ok(())
}

pub(super) fn metadata(path: &Path) -> Result<(), PathSecurityError> {
    let metadata = std::fs::symlink_metadata(path).map_err(|_| PathSecurityError::Unavailable)?;
    if !metadata.is_file() || metadata.file_type().is_symlink() {
        return Err(PathSecurityError::UnsafePath);
    }
    let parent = path.parent().ok_or(PathSecurityError::UnsafePath)?;
    let parent_metadata =
        std::fs::symlink_metadata(parent).map_err(|_| PathSecurityError::Unavailable)?;
    if !parent_metadata.is_dir() || parent_metadata.file_type().is_symlink() {
        return Err(PathSecurityError::UnsafePath);
    }
    permissions(&metadata, &parent_metadata)
}

#[cfg(windows)]
pub(super) fn platform_shape(path: &Path) -> Result<(), PathSecurityError> {
    use std::path::Prefix;

    let mut components = path.components();
    let prefix = components.next().ok_or(PathSecurityError::UnsafePath)?;
    if !matches!(prefix, Component::Prefix(value) if matches!(value.kind(), Prefix::Disk(_))) {
        return Err(PathSecurityError::UnsafePath);
    }
    for component in components {
        if let Component::Normal(value) = component {
            let text = value.to_string_lossy();
            if text.contains(':') || text.ends_with('.') || text.ends_with(' ') {
                return Err(PathSecurityError::UnsafePath);
            }
        }
    }
    Ok(())
}

#[cfg(not(windows))]
pub(super) fn platform_shape(_path: &Path) -> Result<(), PathSecurityError> {
    Ok(())
}

#[cfg(unix)]
fn permissions(
    file: &std::fs::Metadata,
    parent: &std::fs::Metadata,
) -> Result<(), PathSecurityError> {
    use std::os::unix::fs::MetadataExt;

    let owner = rustix::process::geteuid().as_raw();
    if file.uid() != owner
        || parent.uid() != owner
        || file.nlink() != 1
        || file.mode() & 0o077 != 0
        || parent.mode() & 0o077 != 0
    {
        return Err(PathSecurityError::UnsafePath);
    }
    Ok(())
}

#[cfg(windows)]
fn permissions(
    file: &std::fs::Metadata,
    parent: &std::fs::Metadata,
) -> Result<(), PathSecurityError> {
    use std::os::windows::fs::MetadataExt;
    use windows_sys::Win32::Storage::FileSystem::FILE_ATTRIBUTE_REPARSE_POINT;

    if file.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0
        || parent.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0
    {
        return Err(PathSecurityError::UnsafePath);
    }
    Ok(())
}

#[cfg(not(any(unix, windows)))]
fn permissions(
    _file: &std::fs::Metadata,
    _parent: &std::fs::Metadata,
) -> Result<(), PathSecurityError> {
    Err(PathSecurityError::Unavailable)
}
