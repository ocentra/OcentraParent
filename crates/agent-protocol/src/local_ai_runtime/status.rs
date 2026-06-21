use serde::{Deserialize, Serialize};

use crate::local_ai_runtime_boundary::{
    LocalAiAdapterBoundary, LocalAiAdapterProbeState, LocalAiAdapterReadinessState,
    LocalAiExecutionState, LocalAiProviderConfigurationState, LocalAiProviderPrivacyMode,
    LocalAiProviderSource,
};

use super::{
    cache::{
        LocalAiModelCacheHealth, LocalAiModelCacheState, LocalAiModelDownloadStatus,
        LocalAiModelManifestIntegrityState, LocalAiModelSourcePolicy,
    },
    cache_reasons::{
        LocalAiModelCacheCorruptionReasonCode, LocalAiModelCacheStorageErrorCode,
        LocalAiModelCacheUnavailableReason,
    },
    lifecycle::{
        LocalAiCapabilityFlag, LocalAiDegradedState, LocalAiModelLoadState, LocalAiResourceClass,
    },
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalModelRuntimeStatus {
    pub runtime_reference_id: String,
    pub provider_id: String,
    pub model_id: String,
    pub model_reference: String,
    pub privacy_mode: LocalAiProviderPrivacyMode,
    pub adapter_boundary: LocalAiAdapterBoundary,
    pub execution_state: LocalAiExecutionState,
    pub provider_source: LocalAiProviderSource,
    pub load_state: LocalAiModelLoadState,
    pub capability_flags: Vec<LocalAiCapabilityFlag>,
    pub resource_class: LocalAiResourceClass,
    pub degraded_state: LocalAiDegradedState,
    pub last_checked_at: String,
    pub unavailable_reason: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalProviderAdapterProbe {
    pub provider_id: String,
    pub privacy_mode: LocalAiProviderPrivacyMode,
    pub adapter_boundary: LocalAiAdapterBoundary,
    pub execution_state: LocalAiExecutionState,
    pub provider_source: LocalAiProviderSource,
    pub probe_state: LocalAiAdapterProbeState,
    pub configuration_state: LocalAiProviderConfigurationState,
    pub readiness_state: LocalAiAdapterReadinessState,
    pub execution_allowed: bool,
    pub last_checked_at: String,
    pub unavailable_reason: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalProviderCapability {
    pub provider_id: String,
    pub supported_tasks: Vec<LocalAiCapabilityFlag>,
    pub resource_class: LocalAiResourceClass,
    pub privacy_mode: LocalAiProviderPrivacyMode,
    pub fallback_order: u16,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalAiModelCacheStatus {
    pub artifact_ref: String,
    pub manifest_ref: Option<String>,
    pub source_policy: LocalAiModelSourcePolicy,
    pub cache_state: LocalAiModelCacheState,
    pub cache_health: LocalAiModelCacheHealth,
    pub manifest_integrity: LocalAiModelManifestIntegrityState,
    pub download_enabled: bool,
    pub download_status: LocalAiModelDownloadStatus,
    pub cache_byte_size: u64,
    pub checked_at: String,
    pub unavailable_reason: Option<LocalAiModelCacheUnavailableReason>,
    pub storage_error: Option<LocalAiModelCacheStorageErrorCode>,
    pub corruption_reason: Option<LocalAiModelCacheCorruptionReasonCode>,
}
