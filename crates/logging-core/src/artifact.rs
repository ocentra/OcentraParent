use std::{
    fs::{create_dir_all, write},
    io,
    path::PathBuf,
};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::path::{path_string, sanitize_segment, timestamp_now};

pub const ARTIFACT_SCHEMA_VERSION: u16 = 1;

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
    pub event_type: String,
    pub artifact_id: String,
    pub run_id: String,
    pub command_id: String,
    pub path: String,
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
        write(&path, content.as_bytes())?;

        let digest = Sha256::digest(content.as_bytes());
        let sha256 = format!("{digest:x}");
        let artifact_id = format!("artifact-{}", &sha256[..12]);

        Ok(ArtifactRef {
            schema_version: ARTIFACT_SCHEMA_VERSION,
            event_type: "artifact".to_owned(),
            artifact_id,
            run_id,
            command_id,
            path: path_string(&path),
            kind,
            sha256,
            byte_length: content.len() as u64,
            line_count: content.lines().count() as u64,
            created_at: timestamp_now(),
        })
    }
}

fn kind_file_name(kind: &ArtifactKind) -> &'static str {
    match kind {
        ArtifactKind::Stdout => "stdout.log",
        ArtifactKind::Stderr => "stderr.log",
        ArtifactKind::Metadata => "metadata.json",
        ArtifactKind::Diagnostic => "diagnostic.log",
    }
}
