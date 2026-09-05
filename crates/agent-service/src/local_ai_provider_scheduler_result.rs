use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::local_ai_runtime::generation::LocalAiChatGenerationResult;
use ocentra_parent_agent_protocol::local_ai_runtime::lifecycle::{
    LocalAiDegradedState, LocalAiGenerationState, LocalAiModelLoadState, LocalAiResourceClass,
};
use ocentra_parent_agent_protocol::local_ai_runtime::status::LocalModelRuntimeStatus;
use ocentra_parent_agent_protocol::local_ai_runtime_boundary::{
    LocalAiAdapterBoundary, LocalAiExecutionState, LocalAiProviderSource,
};

use crate::local_ai_provider_scheduler_state::LocalAiStatusText;

use super::{
    SCHEDULER_ADAPTER_UNAVAILABLE_REASON, SCHEDULER_CAPABILITY_UNAVAILABLE_REASON,
    SCHEDULER_DEGRADED_RESULT_ID, SCHEDULER_EXECUTION_DISABLED_REASON,
    SCHEDULER_MODEL_NOT_READY_REASON, SCHEDULER_PROVIDER_SOURCE_UNAVAILABLE_REASON,
    SCHEDULER_QUEUE_FULL_REASON, SCHEDULER_RUNTIME_NOT_READY_REASON,
    SCHEDULER_UNAVAILABLE_RESULT_ID,
};

const SCHEDULER_RESULT_ID_SEPARATOR: &str = ":";

pub(super) fn runtime_with_reason(
    runtime: &LocalModelRuntimeStatus,
    reason: LocalAiStatusText,
) -> LocalModelRuntimeStatus {
    let mut unavailable_runtime = runtime.clone();
    unavailable_runtime.unavailable_reason = Some(reason.0);
    unavailable_runtime
}

pub(super) fn runtime_unavailable_reason(
    runtime: &LocalModelRuntimeStatus,
) -> Option<LocalAiStatusText> {
    runtime
        .unavailable_reason
        .clone()
        .map(LocalAiStatusText)
        .or_else(|| {
            [
                (
                    runtime.resource_class == LocalAiResourceClass::RemoteUnavailable,
                    SCHEDULER_RUNTIME_NOT_READY_REASON,
                ),
                (
                    runtime.adapter_boundary != LocalAiAdapterBoundary::LocalAdapterReady,
                    SCHEDULER_ADAPTER_UNAVAILABLE_REASON,
                ),
                (
                    matches!(
                        runtime.execution_state,
                        LocalAiExecutionState::Disabled | LocalAiExecutionState::Failed
                    ),
                    SCHEDULER_EXECUTION_DISABLED_REASON,
                ),
                (
                    runtime.provider_source == LocalAiProviderSource::Unavailable,
                    SCHEDULER_PROVIDER_SOURCE_UNAVAILABLE_REASON,
                ),
                (
                    runtime.load_state != LocalAiModelLoadState::Loaded,
                    SCHEDULER_MODEL_NOT_READY_REASON,
                ),
                (
                    runtime.capability_flags.is_empty(),
                    SCHEDULER_CAPABILITY_UNAVAILABLE_REASON,
                ),
            ]
            .into_iter()
            .find_map(|(unavailable, reason)| {
                unavailable.then(|| LocalAiStatusText(reason.to_string()))
            })
        })
}

pub(super) fn unavailable_generation_result(
    runtime: &LocalModelRuntimeStatus,
) -> LocalAiChatGenerationResult {
    LocalAiChatGenerationResult {
        local_ai_result_id: [
            SCHEDULER_UNAVAILABLE_RESULT_ID,
            SCHEDULER_RESULT_ID_SEPARATOR,
            runtime.runtime_reference_id.as_str(),
        ]
        .concat(),
        runtime_reference_id: runtime.runtime_reference_id.clone(),
        provider_id: runtime.provider_id.clone(),
        model_id: runtime.model_id.clone(),
        model_reference: runtime.model_reference.clone(),
        generation_state: LocalAiGenerationState::Unavailable,
        output_text: None,
        prompt_char_count: 0,
        max_output_tokens: constants::local_ai_runtime::DEFAULT_GENERATION_MAX_TOKENS,
        timeout_ms: constants::local_ai_runtime::DEFAULT_GENERATION_TIMEOUT_MS,
        duration_ms: 0,
        exit_code: None,
        stderr_byte_size: 0,
        unavailable_reason: runtime.unavailable_reason.clone(),
    }
}

pub(super) fn degraded_generation_result(
    runtime: &LocalModelRuntimeStatus,
) -> LocalAiChatGenerationResult {
    LocalAiChatGenerationResult {
        local_ai_result_id: [
            SCHEDULER_DEGRADED_RESULT_ID,
            SCHEDULER_RESULT_ID_SEPARATOR,
            runtime.runtime_reference_id.as_str(),
        ]
        .concat(),
        runtime_reference_id: runtime.runtime_reference_id.clone(),
        provider_id: runtime.provider_id.clone(),
        model_id: runtime.model_id.clone(),
        model_reference: runtime.model_reference.clone(),
        generation_state: LocalAiGenerationState::Failed,
        output_text: None,
        prompt_char_count: 0,
        max_output_tokens: constants::local_ai_runtime::DEFAULT_GENERATION_MAX_TOKENS,
        timeout_ms: constants::local_ai_runtime::DEFAULT_GENERATION_TIMEOUT_MS,
        duration_ms: 0,
        exit_code: None,
        stderr_byte_size: 0,
        unavailable_reason: Some(SCHEDULER_QUEUE_FULL_REASON.to_string()),
    }
}

pub(super) fn degraded_state_for_generation(
    result: &LocalAiChatGenerationResult,
) -> Option<LocalAiDegradedState> {
    match result.generation_state {
        LocalAiGenerationState::Complete | LocalAiGenerationState::Running => None,
        LocalAiGenerationState::Unavailable => Some(LocalAiDegradedState::ProviderUnavailable),
        LocalAiGenerationState::TimedOut => Some(LocalAiDegradedState::Overloaded),
        LocalAiGenerationState::Failed => Some(LocalAiDegradedState::InvalidOutput),
    }
}
