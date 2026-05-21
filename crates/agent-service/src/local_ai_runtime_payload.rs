use ocentra_parent_agent_protocol::{
    constants, LocalModelRuntimeStatus, LocalProviderAdapterProbe, LogFieldValue, LogFields,
};

use crate::fields::fields_from_pairs;

pub fn local_ai_runtime_status_payload(
    status: &LocalModelRuntimeStatus,
    probe: &LocalProviderAdapterProbe,
) -> LogFields {
    let mut pairs = runtime_status_fields(status);
    pairs.extend(adapter_probe_fields(probe));
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
        bool_field(
            constants::field::LOCAL_AI_EXECUTION_ALLOWED,
            probe.execution_allowed,
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
