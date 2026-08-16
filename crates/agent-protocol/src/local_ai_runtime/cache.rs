use serde::{Deserialize, Serialize};

use crate::constants;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
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
    const PROTOCOL_STRINGS: [&'static str; 4] = [
        constants::local_ai_runtime::SOURCE_POLICY_BUNDLED,
        constants::local_ai_runtime::SOURCE_POLICY_PARENT_INSTALLED,
        constants::local_ai_runtime::SOURCE_POLICY_LOCAL_CACHE,
        constants::local_ai_runtime::SOURCE_POLICY_UNAVAILABLE,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
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
    const PROTOCOL_STRINGS: [&'static str; 6] = [
        constants::local_ai_runtime::CACHE_STATE_UNAVAILABLE,
        constants::local_ai_runtime::CACHE_STATE_NOT_CACHED,
        constants::local_ai_runtime::CACHE_STATE_READY,
        constants::local_ai_runtime::CACHE_STATE_DEGRADED,
        constants::local_ai_runtime::CACHE_STATE_CORRUPTED,
        constants::local_ai_runtime::CACHE_STATE_STORAGE_ERROR,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
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
    const PROTOCOL_STRINGS: [&'static str; 6] = [
        constants::local_ai_runtime::CACHE_HEALTH_HEALTHY,
        constants::local_ai_runtime::CACHE_HEALTH_DEGRADED,
        constants::local_ai_runtime::CACHE_HEALTH_UNAVAILABLE,
        constants::local_ai_runtime::CACHE_HEALTH_DOWNLOAD_DISABLED,
        constants::local_ai_runtime::CACHE_HEALTH_CORRUPTED,
        constants::local_ai_runtime::CACHE_HEALTH_STORAGE_ERROR,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
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
    const PROTOCOL_STRINGS: [&'static str; 7] = [
        constants::local_ai_runtime::MANIFEST_INTEGRITY_UNAVAILABLE,
        constants::local_ai_runtime::MANIFEST_INTEGRITY_UNCHECKED,
        constants::local_ai_runtime::MANIFEST_INTEGRITY_VERIFIED,
        constants::local_ai_runtime::MANIFEST_INTEGRITY_MISSING,
        constants::local_ai_runtime::MANIFEST_INTEGRITY_CHECKSUM_MISMATCH,
        constants::local_ai_runtime::MANIFEST_INTEGRITY_SIGNATURE_INVALID,
        constants::local_ai_runtime::MANIFEST_INTEGRITY_CORRUPTED,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
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
    const PROTOCOL_STRINGS: [&'static str; 5] = [
        constants::local_ai_runtime::DOWNLOAD_STATUS_DISABLED,
        constants::local_ai_runtime::DOWNLOAD_STATUS_NOT_REQUESTED,
        constants::local_ai_runtime::DOWNLOAD_STATUS_IN_PROGRESS,
        constants::local_ai_runtime::DOWNLOAD_STATUS_COMPLETE,
        constants::local_ai_runtime::DOWNLOAD_STATUS_FAILED,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }
}
