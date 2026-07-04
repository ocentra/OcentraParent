use serde::{Deserialize, Serialize};

use crate::constants;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
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
    const PROTOCOL_STRINGS: [&'static str; 7] = [
        constants::local_ai_runtime::CACHE_UNAVAILABLE_MODEL_SOURCE_UNCONFIGURED,
        constants::local_ai_runtime::CACHE_UNAVAILABLE_ARTIFACT_NOT_INSTALLED,
        constants::local_ai_runtime::CACHE_UNAVAILABLE_MANIFEST_UNAVAILABLE,
        constants::local_ai_runtime::CACHE_UNAVAILABLE_DOWNLOAD_DISABLED,
        constants::local_ai_runtime::CACHE_UNAVAILABLE_STORAGE_UNAVAILABLE,
        constants::local_ai_runtime::CACHE_UNAVAILABLE_INTEGRITY_UNVERIFIED,
        constants::local_ai_runtime::CACHE_UNAVAILABLE_CORRUPTION_DETECTED,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
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
    const PROTOCOL_STRINGS: [&'static str; 6] = [
        constants::local_ai_runtime::CACHE_STORAGE_ROOT_UNAVAILABLE,
        constants::local_ai_runtime::CACHE_STORAGE_MANIFEST_READ_FAILED,
        constants::local_ai_runtime::CACHE_STORAGE_ARTIFACT_READ_FAILED,
        constants::local_ai_runtime::CACHE_STORAGE_METADATA_WRITE_DISABLED,
        constants::local_ai_runtime::CACHE_STORAGE_PERMISSION_DENIED,
        constants::local_ai_runtime::CACHE_STORAGE_QUOTA_UNAVAILABLE,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
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
    const PROTOCOL_STRINGS: [&'static str; 6] = [
        constants::local_ai_runtime::CACHE_CORRUPTION_MANIFEST_MISSING,
        constants::local_ai_runtime::CACHE_CORRUPTION_CHECKSUM_MISMATCH,
        constants::local_ai_runtime::CACHE_CORRUPTION_SIGNATURE_INVALID,
        constants::local_ai_runtime::CACHE_CORRUPTION_ARTIFACT_MISSING,
        constants::local_ai_runtime::CACHE_CORRUPTION_MANIFEST_ARTIFACT_MISMATCH,
        constants::local_ai_runtime::CACHE_CORRUPTION_UNKNOWN_INTEGRITY,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }
}
