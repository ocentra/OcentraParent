use serde::{Deserialize, Serialize};

use crate::constants;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LocalAiModelSourcePolicy {
    #[serde(rename = "bundled")]
    Bundled,
    #[serde(rename = "parent-installed")]
    ParentInstalled,
    #[serde(rename = "local-cache")]
    LocalCache,
    #[serde(rename = "unavailable")]
    Unavailable,
}

impl LocalAiModelSourcePolicy {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::Bundled => constants::local_ai_runtime::SOURCE_POLICY_BUNDLED,
            Self::ParentInstalled => constants::local_ai_runtime::SOURCE_POLICY_PARENT_INSTALLED,
            Self::LocalCache => constants::local_ai_runtime::SOURCE_POLICY_LOCAL_CACHE,
            Self::Unavailable => constants::local_ai_runtime::SOURCE_POLICY_UNAVAILABLE,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LocalAiModelCacheState {
    #[serde(rename = "unavailable")]
    Unavailable,
    #[serde(rename = "not-cached")]
    NotCached,
    #[serde(rename = "cache-ready")]
    CacheReady,
    #[serde(rename = "cache-degraded")]
    CacheDegraded,
    #[serde(rename = "cache-corrupted")]
    CacheCorrupted,
    #[serde(rename = "storage-error")]
    StorageError,
}

impl LocalAiModelCacheState {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::Unavailable => constants::local_ai_runtime::CACHE_STATE_UNAVAILABLE,
            Self::NotCached => constants::local_ai_runtime::CACHE_STATE_NOT_CACHED,
            Self::CacheReady => constants::local_ai_runtime::CACHE_STATE_READY,
            Self::CacheDegraded => constants::local_ai_runtime::CACHE_STATE_DEGRADED,
            Self::CacheCorrupted => constants::local_ai_runtime::CACHE_STATE_CORRUPTED,
            Self::StorageError => constants::local_ai_runtime::CACHE_STATE_STORAGE_ERROR,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LocalAiModelCacheHealth {
    #[serde(rename = "healthy")]
    Healthy,
    #[serde(rename = "degraded")]
    Degraded,
    #[serde(rename = "unavailable")]
    Unavailable,
    #[serde(rename = "download-disabled")]
    DownloadDisabled,
    #[serde(rename = "corrupted")]
    Corrupted,
    #[serde(rename = "storage-error")]
    StorageError,
}

impl LocalAiModelCacheHealth {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::Healthy => constants::local_ai_runtime::CACHE_HEALTH_HEALTHY,
            Self::Degraded => constants::local_ai_runtime::CACHE_HEALTH_DEGRADED,
            Self::Unavailable => constants::local_ai_runtime::CACHE_HEALTH_UNAVAILABLE,
            Self::DownloadDisabled => constants::local_ai_runtime::CACHE_HEALTH_DOWNLOAD_DISABLED,
            Self::Corrupted => constants::local_ai_runtime::CACHE_HEALTH_CORRUPTED,
            Self::StorageError => constants::local_ai_runtime::CACHE_HEALTH_STORAGE_ERROR,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LocalAiModelManifestIntegrityState {
    #[serde(rename = "unavailable")]
    Unavailable,
    #[serde(rename = "unchecked")]
    Unchecked,
    #[serde(rename = "verified")]
    Verified,
    #[serde(rename = "manifest-missing")]
    ManifestMissing,
    #[serde(rename = "checksum-mismatch")]
    ChecksumMismatch,
    #[serde(rename = "signature-invalid")]
    SignatureInvalid,
    #[serde(rename = "corrupted")]
    Corrupted,
}

impl LocalAiModelManifestIntegrityState {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::Unavailable => constants::local_ai_runtime::MANIFEST_INTEGRITY_UNAVAILABLE,
            Self::Unchecked => constants::local_ai_runtime::MANIFEST_INTEGRITY_UNCHECKED,
            Self::Verified => constants::local_ai_runtime::MANIFEST_INTEGRITY_VERIFIED,
            Self::ManifestMissing => constants::local_ai_runtime::MANIFEST_INTEGRITY_MISSING,
            Self::ChecksumMismatch => {
                constants::local_ai_runtime::MANIFEST_INTEGRITY_CHECKSUM_MISMATCH
            }
            Self::SignatureInvalid => {
                constants::local_ai_runtime::MANIFEST_INTEGRITY_SIGNATURE_INVALID
            }
            Self::Corrupted => constants::local_ai_runtime::MANIFEST_INTEGRITY_CORRUPTED,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LocalAiModelDownloadStatus {
    #[serde(rename = "download-disabled")]
    DownloadDisabled,
    #[serde(rename = "download-not-requested")]
    DownloadNotRequested,
    #[serde(rename = "download-in-progress")]
    DownloadInProgress,
    #[serde(rename = "download-complete")]
    DownloadComplete,
    #[serde(rename = "download-failed")]
    DownloadFailed,
}

impl LocalAiModelDownloadStatus {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::DownloadDisabled => constants::local_ai_runtime::DOWNLOAD_STATUS_DISABLED,
            Self::DownloadNotRequested => {
                constants::local_ai_runtime::DOWNLOAD_STATUS_NOT_REQUESTED
            }
            Self::DownloadInProgress => constants::local_ai_runtime::DOWNLOAD_STATUS_IN_PROGRESS,
            Self::DownloadComplete => constants::local_ai_runtime::DOWNLOAD_STATUS_COMPLETE,
            Self::DownloadFailed => constants::local_ai_runtime::DOWNLOAD_STATUS_FAILED,
        }
    }
}
