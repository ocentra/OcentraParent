use std::{
    fs::{self, Metadata},
    os::unix::fs::{MetadataExt, PermissionsExt},
    path::Path,
};

pub(super) fn trusted_directory(path: &Path) -> bool {
    let Some(metadata) = fs::symlink_metadata(path).ok() else {
        return false;
    };
    metadata.file_type().is_dir()
        && metadata.uid() == 0
        && metadata.permissions().mode() & 0o022 == 0
}

pub(super) fn trusted_executable(path: &Path, canonical_root: &Path) -> bool {
    let Some(metadata) = fs::symlink_metadata(path).ok() else {
        return false;
    };
    if !trusted_regular_executable(&metadata) {
        return false;
    }
    let Ok(canonical_path) = fs::canonicalize(path) else {
        return false;
    };
    if canonical_path.parent() != Some(canonical_root) {
        return false;
    }
    fs::symlink_metadata(canonical_path)
        .map(|metadata| trusted_regular_executable(&metadata))
        .unwrap_or(false)
}

fn trusted_regular_executable(metadata: &Metadata) -> bool {
    metadata.file_type().is_file()
        && !metadata.file_type().is_symlink()
        && metadata.uid() == 0
        && metadata.permissions().mode() & 0o022 == 0
        && metadata.permissions().mode() & 0o111 != 0
}
