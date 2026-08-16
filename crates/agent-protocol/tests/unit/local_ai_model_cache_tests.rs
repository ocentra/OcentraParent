use super::{
    constants, LocalAiModelCacheHealth, LocalAiModelCacheState, LocalAiModelCacheStatus,
    LocalAiModelCacheUnavailableReason, LocalAiModelDownloadStatus,
    LocalAiModelManifestIntegrityState, LocalAiModelSourcePolicy,
};
use ocentra_eventing::expect_value::ExpectValue;

#[test]
fn local_model_cache_status_serializes_without_parent_visible_paths() {
    let status = LocalAiModelCacheStatus {
        artifact_ref: constants::local_ai_runtime::MODEL_REFERENCE_LOCAL_GGUF_CONFIGURED
            .to_string(),
        manifest_ref: Some(
            constants::local_ai_runtime::MODEL_MANIFEST_REFERENCE_LOCAL_GGUF_CONFIGURED.to_string(),
        ),
        source_policy: LocalAiModelSourcePolicy::ParentInstalled,
        cache_state: LocalAiModelCacheState::CacheDegraded,
        cache_health: LocalAiModelCacheHealth::Degraded,
        manifest_integrity: LocalAiModelManifestIntegrityState::Unchecked,
        download_enabled: false,
        download_status: LocalAiModelDownloadStatus::DownloadDisabled,
        cache_byte_size: 1024,
        checked_at: constants::local_ai_runtime::TEST_CHECKED_AT.to_string(),
        unavailable_reason: Some(LocalAiModelCacheUnavailableReason::IntegrityUnverified),
        storage_error: None,
        corruption_reason: None,
    };

    let serialized = serde_json::to_value(status).expect_value("cache status serializes");

    assert_eq!(
        serialized["artifactRef"],
        constants::local_ai_runtime::MODEL_REFERENCE_LOCAL_GGUF_CONFIGURED
    );
    assert_eq!(
        serialized["manifestRef"],
        constants::local_ai_runtime::MODEL_MANIFEST_REFERENCE_LOCAL_GGUF_CONFIGURED
    );
    assert_eq!(
        serialized["sourcePolicy"],
        constants::local_ai_runtime::SOURCE_POLICY_PARENT_INSTALLED
    );
    assert_eq!(
        serialized["manifestIntegrity"],
        constants::local_ai_runtime::MANIFEST_INTEGRITY_UNCHECKED
    );
    assert_eq!(
        serialized["downloadStatus"],
        constants::local_ai_runtime::DOWNLOAD_STATUS_DISABLED
    );
    assert_eq!(
        serialized["unavailableReason"],
        constants::local_ai_runtime::CACHE_UNAVAILABLE_INTEGRITY_UNVERIFIED
    );
}
