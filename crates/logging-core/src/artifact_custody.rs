use std::{
    fs::{create_dir, create_dir_all, read},
    io,
    path::{Path, PathBuf},
};

use crate::artifact::{ArtifactKind, ArtifactRef};

pub(crate) fn ensure_artifact_directory(
    root: &Path,
    scope: &str,
    run_id: &str,
    command_id: &str,
) -> io::Result<PathBuf> {
    let root = ensure_safe_root(root)?;
    let mut directory = root.clone();
    for segment in [scope, "artifacts", run_id, command_id] {
        directory.push(segment);
        ensure_safe_directory(&root, &directory)?;
    }
    Ok(directory)
}

pub(crate) fn artifact_metadata_path(path: &Path) -> io::Result<PathBuf> {
    let name = path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "artifact path has no file name",
            )
        })?;
    Ok(path.with_file_name(format!("{name}.metadata.json")))
}

pub(crate) fn read_artifact_ref(path: &Path) -> io::Result<ArtifactRef> {
    serde_json::from_slice(&read(path)?)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))
}

pub(crate) fn validate_replay(
    artifact: &ArtifactRef,
    path: &Path,
    content: &str,
    kind: &ArtifactKind,
    run_id: &str,
    command_id: &str,
) -> io::Result<()> {
    if artifact.run_id != run_id || artifact.command_id != command_id || &artifact.kind != kind {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "artifact metadata does not match replay request",
        ));
    }
    if read(path)? == content.as_bytes() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "artifact path already contains different content",
        ))
    }
}

fn ensure_safe_root(root: &Path) -> io::Result<PathBuf> {
    create_dir_all(root)?;
    if std::fs::symlink_metadata(root)?.file_type().is_symlink() {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "artifact root must not be a symlink",
        ));
    }
    root.canonicalize()
}
fn ensure_safe_directory(root: &Path, directory: &Path) -> io::Result<()> {
    if directory.exists() {
        if std::fs::symlink_metadata(directory)?
            .file_type()
            .is_symlink()
        {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "artifact directory must not be a symlink",
            ));
        }
    } else {
        match create_dir(directory) {
            Ok(()) => {}
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists && directory.exists() => {}
            Err(error) => return Err(error),
        }
    }
    if !directory.canonicalize()?.starts_with(root) {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "artifact directory escapes root",
        ));
    }
    Ok(())
}
