use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;

use crate::local_ai_runtime_config_values::LocalAiUnavailableReason;

pub(super) struct PromptText(pub(super) String);

pub(super) fn prompt_from_command(
    command: &AgentCommandEnvelope,
) -> Result<PromptText, LocalAiUnavailableReason> {
    let prompt = match command.payload.get(constants::field::LOCAL_AI_PROMPT) {
        Some(LogFieldValue::String(value)) => value.trim().to_string(),
        _ => {
            return Err(LocalAiUnavailableReason(
                constants::local_ai_runtime::UNAVAILABLE_REASON_COMMAND_PAYLOAD_INVALID,
            ));
        }
    };

    if prompt.is_empty() {
        return Err(LocalAiUnavailableReason(
            constants::local_ai_runtime::UNAVAILABLE_REASON_PROMPT_EMPTY,
        ));
    }

    if prompt.chars().count() > constants::local_ai_runtime::MAX_PROMPT_CHARS {
        return Err(LocalAiUnavailableReason(
            constants::local_ai_runtime::UNAVAILABLE_REASON_PROMPT_TOO_LARGE,
        ));
    }

    Ok(PromptText(prompt))
}
