use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;

#[path = "local_ai_chat_generation_request/model_id.rs"]
mod model_id;
#[path = "local_ai_chat_generation_request/numeric.rs"]
mod numeric;
#[path = "local_ai_chat_generation_request/prompt.rs"]
mod prompt;

use crate::{
    local_ai_chat_generation_request_input::LocalAiChatGenerationRequest,
    local_ai_runtime_config::LocalAiRuntimeConfigSnapshot,
    local_ai_runtime_config_values::LocalAiUnavailableReason,
};
use model_id::requested_model_id;
use numeric::{numeric_field_u32, numeric_field_u64};
use prompt::prompt_from_command;

pub(crate) fn parse_generation_request(
    command: &AgentCommandEnvelope,
    config: &LocalAiRuntimeConfigSnapshot,
) -> Result<LocalAiChatGenerationRequest, LocalAiUnavailableReason> {
    let prompt = prompt_from_command(command)?;
    let model_id = requested_model_id(command, config)?;

    Ok(LocalAiChatGenerationRequest {
        model_id: model_id.0,
        prompt: prompt.0,
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
