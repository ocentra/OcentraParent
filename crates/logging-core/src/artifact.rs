use std::{io, path::PathBuf};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{
    artifact_custody::{
        artifact_metadata_path, ensure_artifact_directory, read_artifact_ref, validate_replay,
    },
    artifact_publish::publish_immutable,
    artifact_publish_lock::with_publish_lock,
    artifact_publish_platform::sync_parent,
    path::{path_string, sanitize_segment, timestamp_now},
};

pub const ARTIFACT_SCHEMA_VERSION: u16 = 2;
pub const ARTIFACT_RECORD_TYPE: &str = "artifact";

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
    pub custody_sha256: String,
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
        let directory = ensure_artifact_directory(&self.root, &scope, &run_id, &command_id)?;
        let path = directory.join(kind_file_name(&kind));
        let metadata_path = artifact_metadata_path(&path)?;
        with_publish_lock(&path, || {
            if metadata_path.exists() {
                let metadata = read_artifact_ref(&metadata_path)?;
                validate_replay(&metadata, &path, content, &kind, &run_id, &command_id)?;
                sync_parent(&metadata_path)?;
                return Ok(metadata);
            }
            let artifact = build_artifact_ref(&path, content, kind, run_id, command_id)?;
            publish_immutable(&path, content.as_bytes())?;
            let metadata = serde_json::to_vec(&artifact)
                .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
            publish_immutable(&metadata_path, &metadata)?;
            Ok(artifact)
        })
    }
}

fn build_artifact_ref(
    path: &std::path::Path,
    content: &str,
    kind: ArtifactKind,
    run_id: String,
    command_id: String,
) -> io::Result<ArtifactRef> {
    let sha256 = format!("{:x}", Sha256::digest(content.as_bytes()));
    let mut artifact = ArtifactRef {
        schema_version: ARTIFACT_SCHEMA_VERSION,
        record_type: ARTIFACT_RECORD_TYPE.to_owned(),
        artifact_id: format!("artifact-{}", &sha256[..12]),
        run_id,
        command_id,
        artifact_path: path_string(path),
        kind,
        sha256,
        byte_length: content.len() as u64,
        line_count: content.lines().count() as u64,
        created_at: timestamp_now(),
        custody_sha256: String::new(),
    };
    artifact.custody_sha256 = crate::artifact_custody::custody_sha256(&artifact)?;
    Ok(artifact)
}

fn kind_file_name(kind: &ArtifactKind) -> &'static str {
    match kind {
        ArtifactKind::Stdout => "stdout.log",
        ArtifactKind::Stderr => "stderr.log",
        ArtifactKind::Metadata => "metadata.json",
        ArtifactKind::Diagnostic => "diagnostic.log",
    }
}
