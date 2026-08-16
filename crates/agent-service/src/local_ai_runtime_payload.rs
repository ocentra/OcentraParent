use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::local_ai_runtime::status::LocalAiModelCacheStatus;
use ocentra_parent_agent_protocol::local_ai_runtime::status::LocalModelRuntimeStatus;
use ocentra_parent_agent_protocol::local_ai_runtime::status::LocalProviderAdapterProbe;
use ocentra_parent_agent_protocol::local_ai_runtime_provider_proof::LocalAiRuntimeProviderProofReadModel;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::logging::LogFields;

use crate::fields::fields_from_pairs;

#[derive(Clone, Copy, Debug)]
struct LocalAiLogFieldKey(&'static str);

#[derive(Clone, Debug)]
struct LocalAiLogText(String);

#[derive(Clone, Copy, Debug)]
struct LocalAiLogStaticText(&'static str);

#[derive(Clone, Copy, Debug)]
struct LocalAiLogBool(bool);

#[derive(Clone, Copy, Debug)]
struct LocalAiLogNumber(u64);

#[derive(Clone, Debug)]
struct LocalAiLogField {
    key: LocalAiLogFieldKey,
    value: LogFieldValue,
}

pub fn local_ai_runtime_status_payload(
    status: &LocalModelRuntimeStatus,
    probe: &LocalProviderAdapterProbe,
    cache: &LocalAiModelCacheStatus,
    provider_proof: &LocalAiRuntimeProviderProofReadModel,
) -> LogFields {
    let mut pairs = runtime_status_fields(status)
        .into_iter()
        .map(|field| (field.key.0, field.value))
        .collect::<Vec<_>>();
    pairs.extend(
        adapter_probe_fields(probe)
            .into_iter()
            .map(|field| (field.key.0, field.value)),
    );
    pairs.extend(
        model_cache_status_fields(cache)
            .into_iter()
            .map(|field| (field.key.0, field.value)),
    );
    pairs.push((
        constants::field::LOCAL_AI_RUNTIME_PROVIDER_PROOF_READ_MODEL,
        LogFieldValue::String(serde_json::to_string(provider_proof).unwrap_or_default()),
    ));
    fields_from_pairs(pairs)
}

fn runtime_status_fields(status: &LocalModelRuntimeStatus) -> Vec<LocalAiLogField> {
    vec![
        string_field(
            LocalAiLogFieldKey(constants::field::LOCAL_AI_RUNTIME_REFERENCE_ID),
            LocalAiLogText(status.runtime_reference_id.clone()),
        ),
        string_field(
            LocalAiLogFieldKey(constants::field::LOCAL_AI_PROVIDER_ID),
            LocalAiLogText(status.provider_id.clone()),
        ),
        string_field(
            LocalAiLogFieldKey(constants::field::LOCAL_AI_MODEL_ID),
            LocalAiLogText(status.model_id.clone()),
        ),
        string_field(
            LocalAiLogFieldKey(constants::field::LOCAL_AI_MODEL_REFERENCE),
            LocalAiLogText(status.model_reference.clone()),
        ),
        protocol_field(
            LocalAiLogFieldKey(constants::field::LOCAL_AI_PRIVACY_MODE),
            LocalAiLogStaticText(status.privacy_mode.as_protocol_str()),
        ),
        protocol_field(
            LocalAiLogFieldKey(constants::field::LOCAL_AI_ADAPTER_BOUNDARY),
            LocalAiLogStaticText(status.adapter_boundary.as_protocol_str()),
        ),
        protocol_field(
            LocalAiLogFieldKey(constants::field::LOCAL_AI_EXECUTION_STATE),
            LocalAiLogStaticText(status.execution_state.as_protocol_str()),
        ),
        protocol_field(
            LocalAiLogFieldKey(constants::field::LOCAL_AI_PROVIDER_SOURCE),
            LocalAiLogStaticText(status.provider_source.as_protocol_str()),
        ),
        protocol_field(
            LocalAiLogFieldKey(constants::field::LOAD_STATE),
            LocalAiLogStaticText(status.load_state.as_protocol_str()),
        ),
        string_field(
            LocalAiLogFieldKey(constants::field::LOCAL_AI_CAPABILITY_FLAGS),
            capability_flags(status),
        ),
        protocol_field(
            LocalAiLogFieldKey(constants::field::LOCAL_AI_RESOURCE_CLASS),
            LocalAiLogStaticText(status.resource_class.as_protocol_str()),
        ),
        protocol_field(
            LocalAiLogFieldKey(constants::field::LOCAL_AI_DEGRADED_STATE),
            LocalAiLogStaticText(status.degraded_state.as_protocol_str()),
        ),
        string_field(
            LocalAiLogFieldKey(constants::field::CHECKED_AT),
            LocalAiLogText(status.last_checked_at.clone()),
        ),
        LocalAiLogField {
            key: LocalAiLogFieldKey(constants::field::LOCAL_AI_UNAVAILABLE_REASON),
            value: optional_text(
                status
                    .unavailable_reason
                    .as_ref()
                    .map(|value| LocalAiLogText(value.clone())),
            ),
        },
    ]
}

fn adapter_probe_fields(probe: &LocalProviderAdapterProbe) -> Vec<LocalAiLogField> {
    vec![
        protocol_field(
            LocalAiLogFieldKey(constants::field::LOCAL_AI_ADAPTER_PROBE_STATE),
            LocalAiLogStaticText(probe.probe_state.as_protocol_str()),
        ),
        protocol_field(
            LocalAiLogFieldKey(constants::field::LOCAL_AI_PROVIDER_CONFIGURATION_STATE),
            LocalAiLogStaticText(probe.configuration_state.as_protocol_str()),
        ),
        protocol_field(
            LocalAiLogFieldKey(constants::field::LOCAL_AI_ADAPTER_READINESS_STATE),
            LocalAiLogStaticText(probe.readiness_state.as_protocol_str()),
        ),
        bool_field(
            LocalAiLogFieldKey(constants::field::LOCAL_AI_EXECUTION_ALLOWED),
            LocalAiLogBool(probe.execution_allowed),
        ),
    ]
}

fn model_cache_status_fields(cache: &LocalAiModelCacheStatus) -> Vec<LocalAiLogField> {
    vec![
        string_field(
            LocalAiLogFieldKey(constants::field::LOCAL_AI_MODEL_ARTIFACT_REF),
            LocalAiLogText(cache.artifact_ref.clone()),
        ),
        LocalAiLogField {
            key: LocalAiLogFieldKey(constants::field::LOCAL_AI_MODEL_MANIFEST_REF),
            value: optional_text(
                cache
                    .manifest_ref
                    .as_ref()
                    .map(|value| LocalAiLogText(value.clone())),
            ),
        },
        protocol_field(
            LocalAiLogFieldKey(constants::field::LOCAL_AI_MODEL_SOURCE_POLICY),
            LocalAiLogStaticText(cache.source_policy.as_protocol_str()),
        ),
        protocol_field(
            LocalAiLogFieldKey(constants::field::LOCAL_AI_MODEL_CACHE_STATE),
            LocalAiLogStaticText(cache.cache_state.as_protocol_str()),
        ),
        protocol_field(
            LocalAiLogFieldKey(constants::field::LOCAL_AI_MODEL_CACHE_HEALTH),
            LocalAiLogStaticText(cache.cache_health.as_protocol_str()),
        ),
        protocol_field(
            LocalAiLogFieldKey(constants::field::LOCAL_AI_MODEL_MANIFEST_INTEGRITY),
            LocalAiLogStaticText(cache.manifest_integrity.as_protocol_str()),
        ),
        bool_field(
            LocalAiLogFieldKey(constants::field::LOCAL_AI_MODEL_DOWNLOAD_ENABLED),
            LocalAiLogBool(cache.download_enabled),
        ),
        protocol_field(
            LocalAiLogFieldKey(constants::field::LOCAL_AI_MODEL_DOWNLOAD_STATUS),
            LocalAiLogStaticText(cache.download_status.as_protocol_str()),
        ),
        number_field(
            LocalAiLogFieldKey(constants::field::LOCAL_AI_MODEL_CACHE_BYTE_SIZE),
            LocalAiLogNumber(cache.cache_byte_size),
        ),
        string_field(
            LocalAiLogFieldKey(constants::field::CHECKED_AT),
            LocalAiLogText(cache.checked_at.clone()),
        ),
        LocalAiLogField {
            key: LocalAiLogFieldKey(constants::field::LOCAL_AI_MODEL_CACHE_UNAVAILABLE_REASON),
            value: optional_protocol(
                cache
                    .unavailable_reason
                    .as_ref()
                    .map(|value| LocalAiLogStaticText(value.as_protocol_str())),
            ),
        },
        LocalAiLogField {
            key: LocalAiLogFieldKey(constants::field::LOCAL_AI_MODEL_CACHE_STORAGE_ERROR),
            value: optional_protocol(
                cache
                    .storage_error
                    .as_ref()
                    .map(|value| LocalAiLogStaticText(value.as_protocol_str())),
            ),
        },
        LocalAiLogField {
            key: LocalAiLogFieldKey(constants::field::LOCAL_AI_MODEL_CACHE_CORRUPTION_REASON),
            value: optional_protocol(
                cache
                    .corruption_reason
                    .as_ref()
                    .map(|value| LocalAiLogStaticText(value.as_protocol_str())),
            ),
        },
    ]
}

fn string_field(key: LocalAiLogFieldKey, value: LocalAiLogText) -> LocalAiLogField {
    LocalAiLogField {
        key,
        value: LogFieldValue::String(value.0),
    }
}

fn protocol_field(key: LocalAiLogFieldKey, value: LocalAiLogStaticText) -> LocalAiLogField {
    LocalAiLogField {
        key,
        value: LogFieldValue::String(value.0.to_string()),
    }
}

fn bool_field(key: LocalAiLogFieldKey, value: LocalAiLogBool) -> LocalAiLogField {
    LocalAiLogField {
        key,
        value: LogFieldValue::Boolean(value.0),
    }
}

fn number_field(key: LocalAiLogFieldKey, value: LocalAiLogNumber) -> LocalAiLogField {
    LocalAiLogField {
        key,
        value: LogFieldValue::Number(value.0 as f64),
    }
}

fn capability_flags(status: &LocalModelRuntimeStatus) -> LocalAiLogText {
    if status.capability_flags.is_empty() {
        return LocalAiLogText(constants::local_ai_runtime::CAPABILITY_FLAGS_NONE.to_string());
    }

    let separator = constants::delimiter::LIST.to_string();
    LocalAiLogText(
        status
            .capability_flags
            .iter()
            .map(|flag| flag.as_protocol_str())
            .collect::<Vec<_>>()
            .join(&separator),
    )
}

fn optional_text(value: Option<LocalAiLogText>) -> LogFieldValue {
    match value {
        Some(text) => LogFieldValue::String(text.0),
        None => LogFieldValue::Null(()),
    }
}

fn optional_protocol(value: Option<LocalAiLogStaticText>) -> LogFieldValue {
    match value {
        Some(text) => LogFieldValue::String(text.0.to_string()),
        None => LogFieldValue::Null(()),
    }
}
