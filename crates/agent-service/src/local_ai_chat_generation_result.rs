use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::local_ai_runtime::generation::LocalAiChatGenerationResult;
use ocentra_parent_agent_protocol::local_ai_runtime::lifecycle::LocalAiGenerationState;

use crate::{
    local_ai_chat_generation_request::LocalAiChatGenerationRequest,
    local_ai_runtime_config::LocalAiRuntimeConfigSnapshot,
    local_ai_runtime_config_values::{LocalAiRuntimeText, LocalAiUnavailableReason},
    local_ai_runtime_model_selection::model_reference_for_request,
    local_ai_runtime_status::local_ai_runtime_status_from_config,
    time::timestamp_now,
};

pub(crate) fn unavailable_result(
    message_id: impl Into<LocalAiRuntimeText>,
    config: &LocalAiRuntimeConfigSnapshot,
    request: &LocalAiChatGenerationRequest,
    reason: LocalAiUnavailableReason,
) -> LocalAiChatGenerationResult {
    let message_id = message_id.into();
    let (status, _, _) = local_ai_runtime_status_from_config(timestamp_now::<String>(), config);
    LocalAiChatGenerationResult {
        local_ai_result_id: result_id(message_id).0,
        runtime_reference_id: status.runtime_reference_id,
        provider_id: status.provider_id,
        model_id: request.model_id.clone(),
        model_reference: model_reference_for_request(
            config,
            &LocalAiRuntimeText(request.model_id.clone()),
        )
        .0,
        generation_state: LocalAiGenerationState::Unavailable,
        output_text: None,
        prompt_char_count: request.prompt.chars().count() as u64,
        max_output_tokens: request.max_output_tokens,
        timeout_ms: request.timeout_ms,
        duration_ms: 0,
        exit_code: None,
        stderr_byte_size: 0,
        unavailable_reason: Some(reason.0.to_string()),
    }
}

pub(crate) struct LocalAiFailedGeneration {
    pub(crate) duration_ms: u64,
    pub(crate) exit_code: Option<i32>,
    pub(crate) stderr_byte_size: u64,
    pub(crate) generation_state: LocalAiGenerationState,
    pub(crate) reason: &'static str,
}

pub(crate) fn failed_result(
    message_id: impl Into<LocalAiRuntimeText>,
    config: &LocalAiRuntimeConfigSnapshot,
    request: &LocalAiChatGenerationRequest,
    failure: &LocalAiFailedGeneration,
) -> LocalAiChatGenerationResult {
    let message_id = message_id.into();
    LocalAiChatGenerationResult {
        local_ai_result_id: result_id(message_id).0,
        runtime_reference_id: constants::local_ai_runtime::RUNTIME_REFERENCE_LOCAL_LLAMA_CLI
            .to_string(),
        provider_id: constants::local_ai_runtime::PROVIDER_ID_LOCAL_LLAMA_CLI.to_string(),
        model_id: request.model_id.clone(),
        model_reference: model_reference_for_request(
            config,
            &LocalAiRuntimeText(request.model_id.clone()),
        )
        .0,
        generation_state: failure.generation_state,
        output_text: None,
        prompt_char_count: request.prompt.chars().count() as u64,
        max_output_tokens: request.max_output_tokens,
        timeout_ms: request.timeout_ms,
        duration_ms: failure.duration_ms,
        exit_code: failure.exit_code,
        stderr_byte_size: failure.stderr_byte_size,
        unavailable_reason: Some(failure.reason.to_string()),
    }
}

pub(crate) fn result_id(message_id: impl Into<LocalAiRuntimeText>) -> LocalAiRuntimeText {
    let message_id = message_id.into();
    let mut value = constants::local_ai_runtime::RESULT_ID_PREFIX.to_string();
    value.push_str(message_id.0.as_str());
    LocalAiRuntimeText(value)
}
