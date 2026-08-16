use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::local_ai_runtime::generation::LocalAiChatGenerationResult;
use ocentra_parent_agent_protocol::local_ai_runtime::lifecycle::LocalAiDegradedState;
use ocentra_parent_agent_protocol::local_ai_runtime::lifecycle::LocalAiGenerationState;
use ocentra_parent_agent_protocol::local_ai_runtime::scheduler::LocalAiProviderSchedulerJobStatus;
use ocentra_parent_agent_protocol::local_ai_runtime::status::LocalModelRuntimeStatus;
use ocentra_parent_agent_protocol::parent_assistant::ParentAssistantAnswer;
use ocentra_parent_agent_protocol::parent_assistant::ParentAssistantAnswerState;
use ocentra_parent_agent_protocol::parent_assistant::ParentAssistantGenerateRequest;
use ocentra_parent_agent_protocol::parent_assistant::ParentAssistantProviderState;
use ocentra_parent_agent_protocol::parent_assistant::ParentAssistantRunState;

use super::base_answer;
use super::ParentAssistantAnswerParts;
use super::ParentAssistantText;
use super::ParentAssistantTextRef;

pub(super) fn answer_from_generation_result(
    request: ParentAssistantGenerateRequest,
    result: LocalAiChatGenerationResult,
) -> ParentAssistantAnswer {
    match result.generation_state {
        LocalAiGenerationState::Complete => configured_answer(request, result),
        LocalAiGenerationState::Unavailable => unavailable_result_answer(request, result),
        _ => degraded_result_answer(request, result),
    }
}

pub(super) fn unavailable_answer(
    request: ParentAssistantGenerateRequest,
    runtime: &LocalModelRuntimeStatus,
) -> ParentAssistantAnswer {
    base_answer(
        request,
        ParentAssistantAnswerParts {
            provider_id: ParentAssistantText(runtime.provider_id.clone()),
            model_id: ParentAssistantText(runtime.model_id.clone()),
            provider_state: ParentAssistantProviderState::Unavailable,
            answer_state: ParentAssistantAnswerState::Unavailable,
            run_state: ParentAssistantRunState::Unavailable,
            scheduler_job_status: LocalAiProviderSchedulerJobStatus::Unavailable,
            degraded_state: LocalAiDegradedState::ProviderUnavailable,
            unavailable_reason: runtime.unavailable_reason.clone().map(ParentAssistantText),
            local_ai_result_id: None,
            answer_text: None,
        },
    )
}

pub(super) fn degraded_busy_answer(
    request: ParentAssistantGenerateRequest,
    runtime: &LocalModelRuntimeStatus,
) -> ParentAssistantAnswer {
    base_answer(
        request,
        ParentAssistantAnswerParts {
            provider_id: ParentAssistantText(runtime.provider_id.clone()),
            model_id: ParentAssistantText(runtime.model_id.clone()),
            provider_state: ParentAssistantProviderState::Degraded,
            answer_state: ParentAssistantAnswerState::Queued,
            run_state: ParentAssistantRunState::Queued,
            scheduler_job_status: LocalAiProviderSchedulerJobStatus::Queued,
            degraded_state: LocalAiDegradedState::Overloaded,
            unavailable_reason: Some(
                ParentAssistantTextRef(constants::parent_assistant::LOCAL_PROVIDER_BUSY)
                    .into_text(),
            ),
            local_ai_result_id: None,
            answer_text: None,
        },
    )
}

fn configured_answer(
    request: ParentAssistantGenerateRequest,
    result: LocalAiChatGenerationResult,
) -> ParentAssistantAnswer {
    base_answer(
        request,
        ParentAssistantAnswerParts {
            provider_id: ParentAssistantText(result.provider_id),
            model_id: ParentAssistantText(result.model_id),
            provider_state: ParentAssistantProviderState::Configured,
            answer_state: ParentAssistantAnswerState::Answered,
            run_state: ParentAssistantRunState::Completed,
            scheduler_job_status: LocalAiProviderSchedulerJobStatus::Complete,
            degraded_state: LocalAiDegradedState::None,
            unavailable_reason: result.unavailable_reason.map(ParentAssistantText),
            local_ai_result_id: Some(ParentAssistantText(result.local_ai_result_id)),
            answer_text: result.output_text.map(ParentAssistantText),
        },
    )
}

fn degraded_result_answer(
    request: ParentAssistantGenerateRequest,
    result: LocalAiChatGenerationResult,
) -> ParentAssistantAnswer {
    base_answer(
        request,
        ParentAssistantAnswerParts {
            provider_id: ParentAssistantText(result.provider_id),
            model_id: ParentAssistantText(result.model_id),
            provider_state: ParentAssistantProviderState::Degraded,
            answer_state: ParentAssistantAnswerState::Degraded,
            run_state: degraded_run_state(&result.generation_state),
            scheduler_job_status: LocalAiProviderSchedulerJobStatus::Degraded,
            degraded_state: LocalAiDegradedState::InvalidOutput,
            unavailable_reason: degraded_unavailable_reason(
                result.unavailable_reason.map(ParentAssistantText),
            ),
            local_ai_result_id: Some(ParentAssistantText(result.local_ai_result_id)),
            answer_text: None,
        },
    )
}

fn unavailable_result_answer(
    request: ParentAssistantGenerateRequest,
    result: LocalAiChatGenerationResult,
) -> ParentAssistantAnswer {
    base_answer(
        request,
        ParentAssistantAnswerParts {
            provider_id: ParentAssistantText(result.provider_id),
            model_id: ParentAssistantText(result.model_id),
            provider_state: ParentAssistantProviderState::Unavailable,
            answer_state: ParentAssistantAnswerState::Unavailable,
            run_state: ParentAssistantRunState::Unavailable,
            scheduler_job_status: LocalAiProviderSchedulerJobStatus::Unavailable,
            degraded_state: LocalAiDegradedState::ProviderUnavailable,
            unavailable_reason: result.unavailable_reason.map(ParentAssistantText),
            local_ai_result_id: Some(ParentAssistantText(result.local_ai_result_id)),
            answer_text: None,
        },
    )
}

fn degraded_run_state(generation_state: &LocalAiGenerationState) -> ParentAssistantRunState {
    match generation_state {
        LocalAiGenerationState::Failed | LocalAiGenerationState::TimedOut => {
            ParentAssistantRunState::Failed
        }
        _ => ParentAssistantRunState::Degraded,
    }
}

fn degraded_unavailable_reason(
    unavailable_reason: Option<ParentAssistantText>,
) -> Option<ParentAssistantText> {
    unavailable_reason.or_else(|| {
        Some(
            ParentAssistantTextRef(constants::parent_assistant::LOCAL_PROVIDER_DEGRADED)
                .into_text(),
        )
    })
}
