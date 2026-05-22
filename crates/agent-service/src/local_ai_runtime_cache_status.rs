use ocentra_parent_agent_protocol::{
    LocalAiModelCacheHealth, LocalAiModelCacheState, LocalAiModelCacheStatus,
    LocalAiModelCacheUnavailableReason, LocalAiModelDownloadStatus,
    LocalAiModelManifestIntegrityState, LocalAiModelSourcePolicy,
};

use crate::local_ai_runtime_config::LocalAiRuntimeConfigSnapshot;

pub(crate) fn local_ai_model_cache_status_from_config(
    checked_at: String,
    config: &LocalAiRuntimeConfigSnapshot,
) -> LocalAiModelCacheStatus {
    if !config.model_file().is_configured() {
        return unavailable_model_cache_status(
            checked_at,
            config,
            LocalAiModelCacheUnavailableReason::ModelSourceUnconfigured,
        );
    }

    if !config.model_file().exists() {
        return unavailable_model_cache_status(
            checked_at,
            config,
            LocalAiModelCacheUnavailableReason::ArtifactNotInstalled,
        );
    }

    LocalAiModelCacheStatus {
        artifact_ref: config.artifact_ref().to_string(),
        manifest_ref: config.manifest_ref(),
        source_policy: LocalAiModelSourcePolicy::ParentInstalled,
        cache_state: LocalAiModelCacheState::CacheDegraded,
        cache_health: LocalAiModelCacheHealth::Degraded,
        manifest_integrity: LocalAiModelManifestIntegrityState::Unchecked,
        download_enabled: false,
        download_status: LocalAiModelDownloadStatus::DownloadDisabled,
        cache_byte_size: config.model_file().byte_size().unwrap_or(0),
        checked_at,
        unavailable_reason: Some(LocalAiModelCacheUnavailableReason::IntegrityUnverified),
        storage_error: None,
        corruption_reason: None,
    }
}

fn unavailable_model_cache_status(
    checked_at: String,
    config: &LocalAiRuntimeConfigSnapshot,
    reason: LocalAiModelCacheUnavailableReason,
) -> LocalAiModelCacheStatus {
    LocalAiModelCacheStatus {
        artifact_ref: config.artifact_ref().to_string(),
        manifest_ref: None,
        source_policy: LocalAiModelSourcePolicy::Unavailable,
        cache_state: LocalAiModelCacheState::Unavailable,
        cache_health: LocalAiModelCacheHealth::Unavailable,
        manifest_integrity: LocalAiModelManifestIntegrityState::Unavailable,
        download_enabled: false,
        download_status: LocalAiModelDownloadStatus::DownloadDisabled,
        cache_byte_size: 0,
        checked_at,
        unavailable_reason: Some(reason),
        storage_error: None,
        corruption_reason: None,
    }
}
