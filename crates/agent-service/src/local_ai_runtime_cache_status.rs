use ocentra_parent_agent_protocol::local_ai_runtime::cache::LocalAiModelCacheHealth;
use ocentra_parent_agent_protocol::local_ai_runtime::cache::LocalAiModelCacheState;
use ocentra_parent_agent_protocol::local_ai_runtime::cache::LocalAiModelDownloadStatus;
use ocentra_parent_agent_protocol::local_ai_runtime::cache::LocalAiModelManifestIntegrityState;
use ocentra_parent_agent_protocol::local_ai_runtime::cache::LocalAiModelSourcePolicy;
use ocentra_parent_agent_protocol::local_ai_runtime::cache_reasons::LocalAiModelCacheUnavailableReason;
use ocentra_parent_agent_protocol::local_ai_runtime::status::LocalAiModelCacheStatus;

use crate::{
    local_ai_runtime_config::LocalAiRuntimeConfigSnapshot,
    local_ai_runtime_config_values::LocalAiRuntimeText,
};

pub(crate) fn local_ai_model_cache_status_from_config(
    checked_at: impl Into<LocalAiRuntimeText>,
    config: &LocalAiRuntimeConfigSnapshot,
) -> LocalAiModelCacheStatus {
    let checked_at = checked_at.into();
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
        artifact_ref: config.artifact_ref().0,
        manifest_ref: config.manifest_ref().map(|value| value.0),
        source_policy: LocalAiModelSourcePolicy::ParentInstalled,
        cache_state: LocalAiModelCacheState::CacheDegraded,
        cache_health: LocalAiModelCacheHealth::Degraded,
        manifest_integrity: LocalAiModelManifestIntegrityState::Unchecked,
        download_enabled: false,
        download_status: LocalAiModelDownloadStatus::DownloadDisabled,
        cache_byte_size: config.model_file().byte_size().unwrap_or(0),
        checked_at: checked_at.0,
        unavailable_reason: Some(LocalAiModelCacheUnavailableReason::IntegrityUnverified),
        storage_error: None,
        corruption_reason: None,
    }
}

fn unavailable_model_cache_status(
    checked_at: impl Into<LocalAiRuntimeText>,
    config: &LocalAiRuntimeConfigSnapshot,
    reason: LocalAiModelCacheUnavailableReason,
) -> LocalAiModelCacheStatus {
    let checked_at = checked_at.into();
    LocalAiModelCacheStatus {
        artifact_ref: config.artifact_ref().0,
        manifest_ref: None,
        source_policy: LocalAiModelSourcePolicy::Unavailable,
        cache_state: LocalAiModelCacheState::Unavailable,
        cache_health: LocalAiModelCacheHealth::Unavailable,
        manifest_integrity: LocalAiModelManifestIntegrityState::Unavailable,
        download_enabled: false,
        download_status: LocalAiModelDownloadStatus::DownloadDisabled,
        cache_byte_size: 0,
        checked_at: checked_at.0,
        unavailable_reason: Some(reason),
        storage_error: None,
        corruption_reason: None,
    }
}
