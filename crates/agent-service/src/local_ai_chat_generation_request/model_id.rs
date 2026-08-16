use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;

use crate::{
    local_ai_runtime_config::LocalAiRuntimeConfigSnapshot,
    local_ai_runtime_config_values::validation::is_safe_local_ai_model_id,
    local_ai_runtime_config_values::{LocalAiRuntimeText, LocalAiUnavailableReason},
};

pub(super) fn requested_model_id(
    command: &AgentCommandEnvelope,
    config: &LocalAiRuntimeConfigSnapshot,
) -> Result<LocalAiRuntimeText, LocalAiUnavailableReason> {
    match command.payload.get(constants::field::LOCAL_AI_MODEL_ID) {
        Some(LogFieldValue::String(value)) => {
            let model_id = LocalAiRuntimeText(value.trim().to_string());
            if is_safe_local_ai_model_id(&model_id) {
                Ok(model_id)
            } else {
                Err(LocalAiUnavailableReason(
                    constants::local_ai_runtime::UNAVAILABLE_REASON_MODEL_ID_INVALID,
                ))
            }
        }
        Some(_) => Err(LocalAiUnavailableReason(
            constants::local_ai_runtime::UNAVAILABLE_REASON_MODEL_ID_INVALID,
        )),
        None => Ok(config.model_id()),
    }
}
