use ocentra_parent_agent_protocol::{constants, LocalModelRuntimeStatus, LogFieldValue, LogFields};

use crate::fields::fields_from_pairs;

pub fn local_ai_runtime_status_payload(status: &LocalModelRuntimeStatus) -> LogFields {
    fields_from_pairs(vec![
        (
            constants::field::LOCAL_AI_RUNTIME_REFERENCE_ID,
            LogFieldValue::String(status.runtime_reference_id.clone()),
        ),
        (
            constants::field::LOCAL_AI_PROVIDER_ID,
            LogFieldValue::String(status.provider_id.clone()),
        ),
        (
            constants::field::LOCAL_AI_MODEL_ID,
            LogFieldValue::String(status.model_id.clone()),
        ),
        (
            constants::field::LOCAL_AI_MODEL_REFERENCE,
            LogFieldValue::String(status.model_reference.clone()),
        ),
        (
            constants::field::LOAD_STATE,
            LogFieldValue::String(status.load_state.as_protocol_str().to_string()),
        ),
        (
            constants::field::LOCAL_AI_CAPABILITY_FLAGS,
            LogFieldValue::String(capability_flags(status)),
        ),
        (
            constants::field::LOCAL_AI_RESOURCE_CLASS,
            LogFieldValue::String(status.resource_class.as_protocol_str().to_string()),
        ),
        (
            constants::field::LOCAL_AI_DEGRADED_STATE,
            LogFieldValue::String(status.degraded_state.as_protocol_str().to_string()),
        ),
        (
            constants::field::CHECKED_AT,
            LogFieldValue::String(status.last_checked_at.clone()),
        ),
        (
            constants::field::LOCAL_AI_UNAVAILABLE_REASON,
            optional_string(&status.unavailable_reason),
        ),
    ])
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
