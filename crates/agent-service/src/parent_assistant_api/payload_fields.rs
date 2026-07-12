use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;

use super::ParentAssistantPayloadFieldName;
use super::ParentAssistantText;
use super::ParentAssistantTextRef;

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

pub(super) fn string_field_value(value: ParentAssistantTextRef<'_>) -> LogFieldValue {
    LogFieldValue::String(value.into_text().0)
}
