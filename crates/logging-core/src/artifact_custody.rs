use std::{
    fs::{read, symlink_metadata},
    io,
    path::{Path, PathBuf},
};

use sha2::{Digest, Sha256};

use crate::{
    artifact::{ArtifactKind, ArtifactRef, ARTIFACT_RECORD_TYPE, ARTIFACT_SCHEMA_VERSION},
    artifact_directory::{create_and_sync_directory, create_durable_directory_hierarchy},
    path::path_string,
};

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
    if read(path)? != content.as_bytes() {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "artifact path already contains different content",
        ));
    }

    let sha256 = format!("{:x}", Sha256::digest(content.as_bytes()));
    let metadata_matches = artifact.schema_version == ARTIFACT_SCHEMA_VERSION
        && artifact.record_type == ARTIFACT_RECORD_TYPE
        && artifact.artifact_id == format!("artifact-{}", &sha256[..12])
        && artifact.run_id == run_id
        && artifact.command_id == command_id
        && artifact.artifact_path == path_string(path)
        && &artifact.kind == kind
        && artifact.sha256 == sha256
        && artifact.byte_length == content.len() as u64
        && artifact.line_count == content.lines().count() as u64
        && chrono::DateTime::parse_from_rfc3339(&artifact.created_at).is_ok()
        && artifact.custody_sha256 == custody_sha256(artifact);
    if !metadata_matches {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "artifact metadata does not match replay request",
        ));
    }
    Ok(())
}

pub(crate) fn custody_sha256(artifact: &ArtifactRef) -> String {
    let canonical = serde_json::json!({
        "schemaVersion": artifact.schema_version,
        "eventType": artifact.record_type,
        "artifactId": artifact.artifact_id,
        "runId": artifact.run_id,
        "commandId": artifact.command_id,
        "path": artifact.artifact_path,
        "kind": artifact.kind,
        "sha256": artifact.sha256,
        "byteLength": artifact.byte_length,
        "lineCount": artifact.line_count,
        "createdAt": artifact.created_at,
    });
    format!("{:x}", Sha256::digest(canonical.to_string().as_bytes()))
}

fn ensure_safe_root(root: &Path) -> io::Result<PathBuf> {
    let root = if root.is_absolute() {
        root.to_path_buf()
    } else {
        std::env::current_dir()?.join(root)
    };
    create_durable_directory_hierarchy(&root)?;
    if symlink_metadata(&root)?.file_type().is_symlink() {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "artifact root must not be a symlink",
        ));
    }
    root.canonicalize()
}

fn ensure_safe_directory(root: &Path, directory: &Path) -> io::Result<()> {
    if directory.exists() {
        if symlink_metadata(directory)?.file_type().is_symlink() {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "artifact directory must not be a symlink",
            ));
        }
    } else {
        create_and_sync_directory(directory)?;
    }
    if !directory.canonicalize()?.starts_with(root) {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "artifact directory escapes root",
        ));
    }
    Ok(())
}
