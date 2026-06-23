use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;

use crate::{
    local_ai_runtime_config::LocalAiRuntimeConfigSnapshot,
    local_ai_runtime_config_values::is_safe_local_ai_model_id,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct LocalAiChatGenerationRequest {
    pub(crate) model_id: String,
    pub(crate) prompt: String,
    pub(crate) max_output_tokens: u32,
    pub(crate) timeout_ms: u64,
}

pub(crate) fn parse_generation_request(
    command: &AgentCommandEnvelope,
    config: &LocalAiRuntimeConfigSnapshot,
) -> Result<LocalAiChatGenerationRequest, &'static str> {
    let prompt = match command.payload.get(constants::field::LOCAL_AI_PROMPT) {
        Some(LogFieldValue::String(value)) => value.trim().to_string(),
        _ => {
            return Err(constants::local_ai_runtime::UNAVAILABLE_REASON_COMMAND_PAYLOAD_INVALID);
        }
    };

    if prompt.is_empty() {
        return Err(constants::local_ai_runtime::UNAVAILABLE_REASON_PROMPT_EMPTY);
    }

    if prompt.chars().count() > constants::local_ai_runtime::MAX_PROMPT_CHARS {
        return Err(constants::local_ai_runtime::UNAVAILABLE_REASON_PROMPT_TOO_LARGE);
    }

    let model_id = requested_model_id(command, config)?;

    Ok(LocalAiChatGenerationRequest {
        model_id,
        prompt,
        max_output_tokens: numeric_field_u32(
            command
                .payload
                .get(constants::field::LOCAL_AI_MAX_OUTPUT_TOKENS),
            config.generation_max_tokens(),
        ),
        timeout_ms: numeric_field_u64(
            command.payload.get(constants::field::LOCAL_AI_TIMEOUT_MS),
            config.generation_timeout_ms(),
        ),
    })
}

fn requested_model_id(
    command: &AgentCommandEnvelope,
    config: &LocalAiRuntimeConfigSnapshot,
) -> Result<String, &'static str> {
    match command.payload.get(constants::field::LOCAL_AI_MODEL_ID) {
        Some(LogFieldValue::String(value)) => {
            let model_id = value.trim();
            if is_safe_local_ai_model_id(model_id) {
                Ok(model_id.to_string())
            } else {
                Err(constants::local_ai_runtime::UNAVAILABLE_REASON_MODEL_ID_INVALID)
            }
        }
        Some(_) => Err(constants::local_ai_runtime::UNAVAILABLE_REASON_MODEL_ID_INVALID),
        None => Ok(config.model_id().to_string()),
    }
}

fn numeric_field_u32(value: Option<&LogFieldValue>, fallback: u32) -> u32 {
    match value {
        Some(LogFieldValue::Number(number)) if number.is_finite() && *number > 0.0 => {
            *number as u32
        }
        _ => fallback,
    }
}

fn numeric_field_u64(value: Option<&LogFieldValue>, fallback: u64) -> u64 {
    match value {
        Some(LogFieldValue::Number(number)) if number.is_finite() && *number > 0.0 => {
            *number as u64
        }
        _ => fallback,
    }
}
