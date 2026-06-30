use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::local_ai_runtime::status::LocalAiModelCacheStatus;
use ocentra_parent_agent_protocol::local_ai_runtime::status::LocalModelRuntimeStatus;
use ocentra_parent_agent_protocol::local_ai_runtime::status::LocalProviderAdapterProbe;
use ocentra_parent_agent_protocol::local_ai_runtime_provider_proof::LocalAiRuntimeProviderProofReadModel;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::logging::LogFields;

use crate::fields::fields_from_pairs;

pub fn local_ai_runtime_status_payload(
    status: &LocalModelRuntimeStatus,
    probe: &LocalProviderAdapterProbe,
    cache: &LocalAiModelCacheStatus,
    provider_proof: &LocalAiRuntimeProviderProofReadModel,
) -> LogFields {
    let mut pairs = runtime_status_fields(status);
    pairs.extend(adapter_probe_fields(probe));
    pairs.extend(model_cache_status_fields(cache));
    pairs.push((
        constants::field::LOCAL_AI_RUNTIME_PROVIDER_PROOF_READ_MODEL,
        LogFieldValue::String(serde_json::to_string(provider_proof).unwrap_or_default()),
    ));
    fields_from_pairs(pairs)
}

fn runtime_status_fields(status: &LocalModelRuntimeStatus) -> Vec<(&'static str, LogFieldValue)> {
    vec![
        string_field(
            constants::field::LOCAL_AI_RUNTIME_REFERENCE_ID,
            status.runtime_reference_id.clone(),
        ),
        string_field(
            constants::field::LOCAL_AI_PROVIDER_ID,
            status.provider_id.clone(),
        ),
        string_field(constants::field::LOCAL_AI_MODEL_ID, status.model_id.clone()),
        string_field(
            constants::field::LOCAL_AI_MODEL_REFERENCE,
            status.model_reference.clone(),
        ),
        protocol_field(
            constants::field::LOCAL_AI_PRIVACY_MODE,
            status.privacy_mode.as_protocol_str(),
        ),
        protocol_field(
            constants::field::LOCAL_AI_ADAPTER_BOUNDARY,
            status.adapter_boundary.as_protocol_str(),
        ),
        protocol_field(
            constants::field::LOCAL_AI_EXECUTION_STATE,
            status.execution_state.as_protocol_str(),
        ),
        protocol_field(
            constants::field::LOCAL_AI_PROVIDER_SOURCE,
            status.provider_source.as_protocol_str(),
        ),
        protocol_field(
            constants::field::LOAD_STATE,
            status.load_state.as_protocol_str(),
        ),
        string_field(
            constants::field::LOCAL_AI_CAPABILITY_FLAGS,
            capability_flags(status),
        ),
        protocol_field(
            constants::field::LOCAL_AI_RESOURCE_CLASS,
            status.resource_class.as_protocol_str(),
        ),
        protocol_field(
            constants::field::LOCAL_AI_DEGRADED_STATE,
            status.degraded_state.as_protocol_str(),
        ),
        string_field(constants::field::CHECKED_AT, status.last_checked_at.clone()),
        (
            constants::field::LOCAL_AI_UNAVAILABLE_REASON,
            optional_string(&status.unavailable_reason),
        ),
    ]
}

fn adapter_probe_fields(probe: &LocalProviderAdapterProbe) -> Vec<(&'static str, LogFieldValue)> {
    vec![
        protocol_field(
            constants::field::LOCAL_AI_ADAPTER_PROBE_STATE,
            probe.probe_state.as_protocol_str(),
        ),
        protocol_field(
            constants::field::LOCAL_AI_PROVIDER_CONFIGURATION_STATE,
            probe.configuration_state.as_protocol_str(),
        ),
        protocol_field(
            constants::field::LOCAL_AI_ADAPTER_READINESS_STATE,
            probe.readiness_state.as_protocol_str(),
        ),
        bool_field(
            constants::field::LOCAL_AI_EXECUTION_ALLOWED,
            probe.execution_allowed,
        ),
    ]
}

fn model_cache_status_fields(
    cache: &LocalAiModelCacheStatus,
) -> Vec<(&'static str, LogFieldValue)> {
    vec![
        string_field(
            constants::field::LOCAL_AI_MODEL_ARTIFACT_REF,
            cache.artifact_ref.clone(),
        ),
        (
            constants::field::LOCAL_AI_MODEL_MANIFEST_REF,
            optional_string(&cache.manifest_ref),
        ),
        protocol_field(
            constants::field::LOCAL_AI_MODEL_SOURCE_POLICY,
            cache.source_policy.as_protocol_str(),
        ),
        protocol_field(
            constants::field::LOCAL_AI_MODEL_CACHE_STATE,
            cache.cache_state.as_protocol_str(),
        ),
        protocol_field(
            constants::field::LOCAL_AI_MODEL_CACHE_HEALTH,
            cache.cache_health.as_protocol_str(),
        ),
        protocol_field(
            constants::field::LOCAL_AI_MODEL_MANIFEST_INTEGRITY,
            cache.manifest_integrity.as_protocol_str(),
        ),
        bool_field(
            constants::field::LOCAL_AI_MODEL_DOWNLOAD_ENABLED,
            cache.download_enabled,
        ),
        protocol_field(
            constants::field::LOCAL_AI_MODEL_DOWNLOAD_STATUS,
            cache.download_status.as_protocol_str(),
        ),
        number_field(
            constants::field::LOCAL_AI_MODEL_CACHE_BYTE_SIZE,
            cache.cache_byte_size,
        ),
        string_field(constants::field::CHECKED_AT, cache.checked_at.clone()),
        (
            constants::field::LOCAL_AI_MODEL_CACHE_UNAVAILABLE_REASON,
            optional_protocol(
                cache
                    .unavailable_reason
                    .as_ref()
                    .map(|value| value.as_protocol_str()),
            ),
        ),
        (
            constants::field::LOCAL_AI_MODEL_CACHE_STORAGE_ERROR,
            optional_protocol(
                cache
                    .storage_error
                    .as_ref()
                    .map(|value| value.as_protocol_str()),
            ),
        ),
        (
            constants::field::LOCAL_AI_MODEL_CACHE_CORRUPTION_REASON,
            optional_protocol(
                cache
                    .corruption_reason
                    .as_ref()
                    .map(|value| value.as_protocol_str()),
            ),
        ),
    ]
}

fn string_field(key: &'static str, value: String) -> (&'static str, LogFieldValue) {
    (key, LogFieldValue::String(value))
}

fn protocol_field(key: &'static str, value: &'static str) -> (&'static str, LogFieldValue) {
    (key, LogFieldValue::String(value.to_string()))
}

fn bool_field(key: &'static str, value: bool) -> (&'static str, LogFieldValue) {
    (key, LogFieldValue::Boolean(value))
}

fn number_field(key: &'static str, value: u64) -> (&'static str, LogFieldValue) {
    (key, LogFieldValue::Number(value as f64))
}

fn capability_flags(status: &LocalModelRuntimeStatus) -> String {
    if status.capability_flags.is_empty() {
        return constants::local_ai_runtime::CAPABILITY_FLAGS_NONE.to_string();
    }

    let separator = constants::delimiter::LIST.to_string();
    status
        .capability_flags
        .iter()
        .map(|flag| flag.as_protocol_str())
        .collect::<Vec<_>>()
        .join(&separator)
}

fn optional_string(value: &Option<String>) -> LogFieldValue {
    match value {
        Some(text) => LogFieldValue::String(text.clone()),
        None => LogFieldValue::Null(()),
    }
}

fn optional_protocol(value: Option<&'static str>) -> LogFieldValue {
    match value {
        Some(text) => LogFieldValue::String(text.to_string()),
        None => LogFieldValue::Null(()),
    }
}
