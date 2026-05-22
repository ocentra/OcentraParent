use serde::{Deserialize, Serialize};

use crate::constants;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LocalAiModelCacheUnavailableReason {
    #[serde(rename = "model-source-unconfigured")]
    ModelSourceUnconfigured,
    #[serde(rename = "artifact-not-installed")]
    ArtifactNotInstalled,
    #[serde(rename = "manifest-unavailable")]
    ManifestUnavailable,
    #[serde(rename = "download-disabled")]
    DownloadDisabled,
    #[serde(rename = "cache-storage-unavailable")]
    CacheStorageUnavailable,
    #[serde(rename = "integrity-unverified")]
    IntegrityUnverified,
    #[serde(rename = "corruption-detected")]
    CorruptionDetected,
}

impl LocalAiModelCacheUnavailableReason {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::ModelSourceUnconfigured => {
                constants::local_ai_runtime::CACHE_UNAVAILABLE_MODEL_SOURCE_UNCONFIGURED
            }
            Self::ArtifactNotInstalled => {
                constants::local_ai_runtime::CACHE_UNAVAILABLE_ARTIFACT_NOT_INSTALLED
            }
            Self::ManifestUnavailable => {
                constants::local_ai_runtime::CACHE_UNAVAILABLE_MANIFEST_UNAVAILABLE
            }
            Self::DownloadDisabled => {
                constants::local_ai_runtime::CACHE_UNAVAILABLE_DOWNLOAD_DISABLED
            }
            Self::CacheStorageUnavailable => {
                constants::local_ai_runtime::CACHE_UNAVAILABLE_STORAGE_UNAVAILABLE
            }
            Self::IntegrityUnverified => {
                constants::local_ai_runtime::CACHE_UNAVAILABLE_INTEGRITY_UNVERIFIED
            }
            Self::CorruptionDetected => {
                constants::local_ai_runtime::CACHE_UNAVAILABLE_CORRUPTION_DETECTED
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LocalAiModelCacheStorageErrorCode {
    #[serde(rename = "cache-root-unavailable")]
    CacheRootUnavailable,
    #[serde(rename = "manifest-read-failed")]
    ManifestReadFailed,
    #[serde(rename = "artifact-read-failed")]
    ArtifactReadFailed,
    #[serde(rename = "metadata-write-disabled")]
    MetadataWriteDisabled,
    #[serde(rename = "storage-permission-denied")]
    StoragePermissionDenied,
    #[serde(rename = "quota-unavailable")]
    QuotaUnavailable,
}

impl LocalAiModelCacheStorageErrorCode {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::CacheRootUnavailable => {
                constants::local_ai_runtime::CACHE_STORAGE_ROOT_UNAVAILABLE
            }
            Self::ManifestReadFailed => {
                constants::local_ai_runtime::CACHE_STORAGE_MANIFEST_READ_FAILED
            }
            Self::ArtifactReadFailed => {
                constants::local_ai_runtime::CACHE_STORAGE_ARTIFACT_READ_FAILED
            }
            Self::MetadataWriteDisabled => {
                constants::local_ai_runtime::CACHE_STORAGE_METADATA_WRITE_DISABLED
            }
            Self::StoragePermissionDenied => {
                constants::local_ai_runtime::CACHE_STORAGE_PERMISSION_DENIED
            }
            Self::QuotaUnavailable => constants::local_ai_runtime::CACHE_STORAGE_QUOTA_UNAVAILABLE,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LocalAiModelCacheCorruptionReasonCode {
    #[serde(rename = "manifest-missing")]
    ManifestMissing,
    #[serde(rename = "checksum-mismatch")]
    ChecksumMismatch,
    #[serde(rename = "signature-invalid")]
    SignatureInvalid,
    #[serde(rename = "artifact-missing")]
    ArtifactMissing,
    #[serde(rename = "manifest-artifact-mismatch")]
    ManifestArtifactMismatch,
    #[serde(rename = "unknown-integrity")]
    UnknownIntegrity,
}

impl LocalAiModelCacheCorruptionReasonCode {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::ManifestMissing => constants::local_ai_runtime::CACHE_CORRUPTION_MANIFEST_MISSING,
            Self::ChecksumMismatch => {
                constants::local_ai_runtime::CACHE_CORRUPTION_CHECKSUM_MISMATCH
            }
            Self::SignatureInvalid => {
                constants::local_ai_runtime::CACHE_CORRUPTION_SIGNATURE_INVALID
            }
            Self::ArtifactMissing => constants::local_ai_runtime::CACHE_CORRUPTION_ARTIFACT_MISSING,
            Self::ManifestArtifactMismatch => {
                constants::local_ai_runtime::CACHE_CORRUPTION_MANIFEST_ARTIFACT_MISMATCH
            }
            Self::UnknownIntegrity => {
                constants::local_ai_runtime::CACHE_CORRUPTION_UNKNOWN_INTEGRITY
            }
        }
    }
}
