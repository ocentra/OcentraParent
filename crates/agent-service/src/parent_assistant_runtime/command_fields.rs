use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;

use super::ParentAssistantPayloadFieldName;
use super::ParentAssistantText;

pub(super) fn string_payload_field(
    command: &AgentCommandEnvelope,
    payload_field_name: ParentAssistantPayloadFieldName,
) -> Option<ParentAssistantText> {
    match command.payload.get(payload_field_name.0) {
        Some(LogFieldValue::String(value)) if !value.trim().is_empty() => {
            Some(ParentAssistantText(value.trim().to_string()))
        }
        _ => None,
    }
}

pub(super) fn numeric_field_u32(value: Option<&LogFieldValue>, fallback: u32) -> u32 {
    match value {
        Some(LogFieldValue::Number(number)) if number.is_finite() && *number > 0.0 => {
            *number as u32
        }
        _ => fallback,
    }
}

pub(super) fn numeric_field_u64(value: Option<&LogFieldValue>, fallback: u64) -> u64 {
    match value {
        Some(LogFieldValue::Number(number)) if number.is_finite() && *number > 0.0 => {
            *number as u64
        }
        _ => fallback,
    }
}
