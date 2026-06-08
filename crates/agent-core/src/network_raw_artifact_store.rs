use std::{
    fs::{create_dir_all, metadata, read, remove_file, File},
    io::Write,
    path::{Path, PathBuf},
};

use ocentra_parent_agent_protocol::constants;
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetworkRawArtifactStoreConfig {
    pub root: PathBuf,
    pub max_artifact_bytes: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetworkRawCaptureArtifactInput {
    pub artifact_id: String,
    pub captured_at: String,
    pub source_event_id: String,
    pub custody_label: String,
    pub bytes: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetworkRawCaptureArtifactRecord {
    pub artifact_id: String,
    pub artifact_path: PathBuf,
    pub byte_len: usize,
    pub hash_algorithm: String,
    pub sha256_hex: String,
    pub captured_at: String,
    pub source_event_id: String,
    pub custody_label: String,
    pub state: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetworkRawCaptureArtifactExport {
    pub record: NetworkRawCaptureArtifactRecord,
    pub bytes: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkRawArtifactStoreError {
    EmptyRoot,
    EmptyArtifactId,
    UnsafeArtifactId,
    EmptyCapturedAt,
    EmptySourceEventId,
    EmptyCustodyLabel,
    EmptyBytes,
    ArtifactTooLarge,
    ArtifactDeleted,
    ArtifactMissing,
    Io,
}

pub struct NetworkRawArtifactStore {
    root: PathBuf,
    max_artifact_bytes: usize,
}

impl NetworkRawArtifactStore {
    pub fn open(
        config: NetworkRawArtifactStoreConfig,
    ) -> Result<Self, NetworkRawArtifactStoreError> {
        if config.root.as_os_str().is_empty() {
            return Err(NetworkRawArtifactStoreError::EmptyRoot);
        }

        create_dir_all(&config.root).map_err(|_| NetworkRawArtifactStoreError::Io)?;

        Ok(Self {
            root: config.root,
            max_artifact_bytes: config.max_artifact_bytes,
        })
    }

    pub fn write_artifact(
        &self,
        input: NetworkRawCaptureArtifactInput,
    ) -> Result<NetworkRawCaptureArtifactRecord, NetworkRawArtifactStoreError> {
        validate_artifact_id(&input.artifact_id)?;
        validate_non_empty(
            &input.captured_at,
            NetworkRawArtifactStoreError::EmptyCapturedAt,
        )?;
        validate_non_empty(
            &input.source_event_id,
            NetworkRawArtifactStoreError::EmptySourceEventId,
        )?;
        validate_non_empty(
            &input.custody_label,
            NetworkRawArtifactStoreError::EmptyCustodyLabel,
        )?;

        if input.bytes.is_empty() {
            return Err(NetworkRawArtifactStoreError::EmptyBytes);
        }
        if input.bytes.len() > self.max_artifact_bytes {
            return Err(NetworkRawArtifactStoreError::ArtifactTooLarge);
        }

        let artifact_path = self.artifact_path(&input.artifact_id);
        let mut file =
            File::create(&artifact_path).map_err(|_| NetworkRawArtifactStoreError::Io)?;
        file.write_all(&input.bytes)
            .map_err(|_| NetworkRawArtifactStoreError::Io)?;
        file.sync_data()
            .map_err(|_| NetworkRawArtifactStoreError::Io)?;

        Ok(NetworkRawCaptureArtifactRecord {
            artifact_id: input.artifact_id,
            artifact_path,
            byte_len: input.bytes.len(),
            hash_algorithm: constants::network_raw_artifact::HASH_ALGORITHM_SHA256.to_string(),
            sha256_hex: sha256_hex(&input.bytes),
            captured_at: input.captured_at,
            source_event_id: input.source_event_id,
            custody_label: input.custody_label,
            state: constants::network_raw_artifact::STATE_ACTIVE.to_string(),
        })
    }

    pub fn read_artifact(
        &self,
        record: &NetworkRawCaptureArtifactRecord,
    ) -> Result<NetworkRawCaptureArtifactExport, NetworkRawArtifactStoreError> {
        reject_deleted(record)?;
        let bytes = read_existing_artifact(&record.artifact_path)?;

        Ok(NetworkRawCaptureArtifactExport {
            record: record.clone(),
            bytes,
        })
    }

    pub fn export_artifact(
        &self,
        record: &NetworkRawCaptureArtifactRecord,
        export_root: &Path,
    ) -> Result<NetworkRawCaptureArtifactRecord, NetworkRawArtifactStoreError> {
        reject_deleted(record)?;
        if export_root.as_os_str().is_empty() {
            return Err(NetworkRawArtifactStoreError::EmptyRoot);
        }
        let bytes = read_existing_artifact(&record.artifact_path)?;
        create_dir_all(export_root).map_err(|_| NetworkRawArtifactStoreError::Io)?;
        let export_path = export_path(export_root, &record.artifact_id);
        let mut file = File::create(&export_path).map_err(|_| NetworkRawArtifactStoreError::Io)?;
        file.write_all(&bytes)
            .map_err(|_| NetworkRawArtifactStoreError::Io)?;
        file.sync_data()
            .map_err(|_| NetworkRawArtifactStoreError::Io)?;

        Ok(NetworkRawCaptureArtifactRecord {
            artifact_path: export_path,
            ..record.clone()
        })
    }

    pub fn delete_artifact(
        &self,
        record: &NetworkRawCaptureArtifactRecord,
    ) -> Result<NetworkRawCaptureArtifactRecord, NetworkRawArtifactStoreError> {
        reject_deleted(record)?;
        if !record.artifact_path.exists() {
            return Err(NetworkRawArtifactStoreError::ArtifactMissing);
        }
        remove_file(&record.artifact_path).map_err(|_| NetworkRawArtifactStoreError::Io)?;

        Ok(NetworkRawCaptureArtifactRecord {
            state: constants::network_raw_artifact::STATE_DELETED.to_string(),
            ..record.clone()
        })
    }

    fn artifact_path(&self, artifact_id: &str) -> PathBuf {
        let mut file_name = String::from(constants::network_raw_artifact::ARTIFACT_FILE_PREFIX);
        file_name.push_str(artifact_id);

        let mut path = self.root.clone();
        path.push(file_name);
        path.set_extension(constants::network_raw_artifact::FILE_EXTENSION);
        path
    }
}

fn export_path(root: &Path, artifact_id: &str) -> PathBuf {
    let mut file_name = String::from(constants::network_raw_artifact::EXPORT_FILE_PREFIX);
    file_name.push_str(artifact_id);

    let mut path = root.to_path_buf();
    path.push(file_name);
    path.set_extension(constants::network_raw_artifact::FILE_EXTENSION);
    path
}

fn read_existing_artifact(path: &Path) -> Result<Vec<u8>, NetworkRawArtifactStoreError> {
    if metadata(path).is_err() {
        return Err(NetworkRawArtifactStoreError::ArtifactMissing);
    }
    read(path).map_err(|_| NetworkRawArtifactStoreError::Io)
}

fn reject_deleted(
    record: &NetworkRawCaptureArtifactRecord,
) -> Result<(), NetworkRawArtifactStoreError> {
    if record.state == constants::network_raw_artifact::STATE_DELETED {
        return Err(NetworkRawArtifactStoreError::ArtifactDeleted);
    }
    Ok(())
}

fn validate_non_empty(
    value: &str,
    error: NetworkRawArtifactStoreError,
) -> Result<(), NetworkRawArtifactStoreError> {
    if value.is_empty() {
        Err(error)
    } else {
        Ok(())
    }
}

fn validate_artifact_id(artifact_id: &str) -> Result<(), NetworkRawArtifactStoreError> {
    if artifact_id.is_empty() {
        return Err(NetworkRawArtifactStoreError::EmptyArtifactId);
    }

    if artifact_id.chars().all(is_safe_artifact_id_char) {
        Ok(())
    } else {
        Err(NetworkRawArtifactStoreError::UnsafeArtifactId)
    }
}

fn is_safe_artifact_id_char(character: char) -> bool {
    character.is_ascii_alphanumeric()
        || character == constants::delimiter::HYPHEN
        || character == constants::delimiter::UNDERSCORE
}

fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}
