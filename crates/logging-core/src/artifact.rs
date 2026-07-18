use std::{
    fs::{create_dir_all, hard_link, read, remove_file, OpenOptions},
    io::{self, Write},
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::path::{path_string, sanitize_segment, timestamp_now};

pub const ARTIFACT_SCHEMA_VERSION: u16 = 1;

static TEMPORARY_SEQUENCE: AtomicU64 = AtomicU64::new(0);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ArtifactKind {
    Stdout,
    Stderr,
    Metadata,
    Diagnostic,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtifactRef {
    pub schema_version: u16,
    #[serde(rename = "eventType")]
    pub record_type: String,
    pub artifact_id: String,
    pub run_id: String,
    pub command_id: String,
    #[serde(rename = "path")]
    pub artifact_path: String,
    pub kind: ArtifactKind,
    pub sha256: String,
    pub byte_length: u64,
    pub line_count: u64,
    pub created_at: String,
}

pub struct ArtifactWriter {
    root: PathBuf,
}

impl ArtifactWriter {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    pub fn write_text_artifact(
        &self,
        scope: &str,
        run_id: &str,
        command_id: &str,
        kind: ArtifactKind,
        content: &str,
    ) -> io::Result<ArtifactRef> {
        let scope = sanitize_segment(scope)?;
        let run_id = sanitize_segment(run_id)?;
        let command_id = sanitize_segment(command_id)?;
        let mut path = self
            .root
            .join(scope)
            .join("artifacts")
            .join(&run_id)
            .join(&command_id);
        create_dir_all(&path)?;
        path.push(kind_file_name(&kind));
        publish_immutable(&path, content.as_bytes())?;

        let digest = Sha256::digest(content.as_bytes());
        let sha256 = format!("{digest:x}");
        let artifact_id = format!("artifact-{}", &sha256[..12]);

        Ok(ArtifactRef {
            schema_version: ARTIFACT_SCHEMA_VERSION,
            record_type: "artifact".to_owned(),
            artifact_id,
            run_id,
            command_id,
            artifact_path: path_string(&path),
            kind,
            sha256,
            byte_length: content.len() as u64,
            line_count: content.lines().count() as u64,
            created_at: timestamp_now(),
        })
    }
}

fn publish_immutable(path: &std::path::Path, content: &[u8]) -> io::Result<()> {
    if let Ok(existing) = read(path) {
        return compare_existing(&existing, content);
    }

    let temporary = temporary_path(path)?;
    let write_result = write_temporary(&temporary, content);
    if let Err(error) = write_result {
        let _ = remove_file(&temporary);
        return Err(error);
    }

    let publish_result = hard_link(&temporary, path);
    let cleanup_result = remove_file(&temporary);
    match publish_result {
        Ok(()) => cleanup_result,
        Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {
            let existing = read(path)?;
            compare_existing(&existing, content)
        }
        Err(error) => Err(error),
    }
}

fn compare_existing(existing: &[u8], content: &[u8]) -> io::Result<()> {
    if existing == content {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "artifact path already contains different content",
        ))
    }
}

fn temporary_path(path: &std::path::Path) -> io::Result<PathBuf> {
    let name = path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "artifact path has no file name",
            )
        })?;
    let sequence = TEMPORARY_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    Ok(path.with_file_name(format!(".{name}.{}.{}.tmp", std::process::id(), sequence)))
}

fn write_temporary(path: &std::path::Path, content: &[u8]) -> io::Result<()> {
    let mut file = OpenOptions::new().write(true).create_new(true).open(path)?;
    file.write_all(content)?;
    file.sync_all()
}

fn kind_file_name(kind: &ArtifactKind) -> &'static str {
    match kind {
        ArtifactKind::Stdout => "stdout.log",
        ArtifactKind::Stderr => "stderr.log",
        ArtifactKind::Metadata => "metadata.json",
        ArtifactKind::Diagnostic => "diagnostic.log",
    }
}
