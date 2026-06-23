use ocentra_parent_agent_protocol::activity::policy::ParentActorReference;
use ocentra_parent_agent_protocol::activity::policy::ParentActorRole;
use ocentra_parent_agent_protocol::activity_surface::ActivityHistoricalReportList;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::local_ai_runtime::lifecycle::LocalAiDegradedState;
use ocentra_parent_agent_protocol::local_ai_runtime::lifecycle::LocalAiGenerationState;
use ocentra_parent_agent_protocol::local_ai_runtime::scheduler::LocalAiProviderSchedulerJobClass;
use ocentra_parent_agent_protocol::local_ai_runtime::scheduler::LocalAiProviderSchedulerJobStatus;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::logging::LogLevel;
use ocentra_parent_agent_protocol::parent_assistant::ParentAssistantActionPreview;
use ocentra_parent_agent_protocol::parent_assistant::ParentAssistantActionPreviewKind;
use ocentra_parent_agent_protocol::parent_assistant::ParentAssistantAnswer;
use ocentra_parent_agent_protocol::parent_assistant::ParentAssistantAnswerState;
use ocentra_parent_agent_protocol::parent_assistant::ParentAssistantGenerateRequest;
use ocentra_parent_agent_protocol::parent_assistant::ParentAssistantProviderState;
use ocentra_parent_agent_protocol::parent_assistant::ParentAssistantRunState;
use ocentra_parent_agent_protocol::parent_assistant::ParentAssistantScope;
use ocentra_parent_agent_protocol::policy_constants as policy;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventName;

use crate::{
    activity_surface_store::{local_store_snapshot, ActivitySurfaceStoreSnapshot},
    event_builder::build_event,
    local_ai_chat_generation_request::LocalAiChatGenerationRequest,
    local_ai_chat_generation_runner::run_local_ai_chat_generation,
    local_ai_provider_scheduler::{local_ai_provider_scheduler, LocalAiProviderSchedulerRuntime},
    local_ai_runtime_config::LocalAiRuntimeConfigSnapshot,
    local_ai_runtime_status::local_ai_runtime_status_for_model_from_config,
    parent_assistant_api::{api_boundary, thread_store},
    parent_assistant_evidence_context::evidence_contexts_from_command,
    parent_assistant_payload::parent_assistant_answer_payload,
    parent_assistant_report_history::activity_report_history_from_command,
    time::timestamp_now,
};

pub async fn build_parent_assistant_answer_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let config = tokio::task::spawn_blocking(LocalAiRuntimeConfigSnapshot::from_environment)
        .await
        .unwrap_or_else(|_| LocalAiRuntimeConfigSnapshot::unconfigured());
    let snapshot = local_store_snapshot().await;
    let stored_report_history = activity_report_history_from_command(&command).await;
    let request = request_from_command(&command, &config, snapshot, stored_report_history);
    let answer = generate_parent_assistant_answer(&command, request, &config).await;
    thread_store::record_message_for_thread(&answer.thread_id);
    let severity = if answer.answer_state == ParentAssistantAnswerState::Answered {
        LogLevel::Info
    } else {
        LogLevel::Warn
    };

    build_event(
        constants::event_id::PARENT_ASSISTANT_ANSWER_REPORTED,
        &command.message_id,
        command.source,
        AgentEventName::AgentParentAssistantAnswerReported,
        severity,
        parent_assistant_answer_payload(&answer),
        None,
    )
}

struct ParentAssistantAnswerParts {
    provider_id: String,
    model_id: String,
    provider_state: ParentAssistantProviderState,
    answer_state: ParentAssistantAnswerState,
    run_state: ParentAssistantRunState,
    scheduler_job_status: LocalAiProviderSchedulerJobStatus,
    degraded_state: LocalAiDegradedState,
    unavailable_reason: Option<String>,
    local_ai_result_id: Option<String>,
    answer_text: Option<String>,
}

pub(crate) async fn generate_parent_assistant_answer(
    command: &AgentCommandEnvelope,
    request: ParentAssistantGenerateRequest,
    config: &LocalAiRuntimeConfigSnapshot,
) -> ParentAssistantAnswer {
    generate_parent_assistant_answer_with_scheduler(
        command,
        request,
        config,
        local_ai_provider_scheduler(),
    )
    .await
}

pub(crate) async fn generate_parent_assistant_answer_with_scheduler(
    command: &AgentCommandEnvelope,
    request: ParentAssistantGenerateRequest,
    config: &LocalAiRuntimeConfigSnapshot,
    scheduler: &LocalAiProviderSchedulerRuntime,
) -> ParentAssistantAnswer {
    let model_id = request
        .model_id
        .clone()
        .unwrap_or_else(|| config.model_id().to_string());
    let (runtime, _, _) =
        local_ai_runtime_status_for_model_from_config(timestamp_now(), config, Some(&model_id));
    if runtime.unavailable_reason.is_some() {
        scheduler
            .record_unavailable_job(&runtime, LocalAiProviderSchedulerJobClass::ParentAssistant);
        return answer_with_api_boundary(command, unavailable_answer(request, &runtime));
    }
    if scheduler.status_snapshot().current_job_class.is_some() {
        scheduler.record_queued_job(&runtime, LocalAiProviderSchedulerJobClass::ParentAssistant);
        return answer_with_api_boundary(command, degraded_busy_answer(request, &runtime));
    }

    let generation_request = generation_request_from_parent_request(&request, config, model_id);
    let result = scheduler
        .run_generation_job(
            LocalAiProviderSchedulerJobClass::ParentAssistant,
            runtime,
            || run_local_ai_chat_generation(&command.message_id, generation_request, config),
        )
        .await;
    answer_with_api_boundary(command, answer_from_generation_result(request, result))
}

fn answer_with_api_boundary(
    command: &AgentCommandEnvelope,
    mut answer: ParentAssistantAnswer,
) -> ParentAssistantAnswer {
    answer.api_provider_boundary =
        api_boundary::api_provider_boundary_for_command(command, &answer.citations);
    answer.provider_route =
        api_boundary::provider_route(answer.provider_state, &answer.api_provider_boundary);
    answer
}

pub(crate) fn request_from_command(
    command: &AgentCommandEnvelope,
    config: &LocalAiRuntimeConfigSnapshot,
    activity_snapshot: Option<ActivitySurfaceStoreSnapshot>,
    stored_report_history: Option<ActivityHistoricalReportList>,
) -> ParentAssistantGenerateRequest {
    let asked_at = timestamp_now();
    let evidence_context = evidence_contexts_from_command(
        command,
        activity_snapshot,
        stored_report_history,
        asked_at.clone(),
    );
    ParentAssistantGenerateRequest {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        request_id: string_payload_field(command, constants::field::PARENT_ASSISTANT_REQUEST_ID)
            .unwrap_or_else(|| constants::parent_assistant::DEFAULT_REQUEST_ID.to_string()),
        thread_id: string_payload_field(command, constants::parent_assistant::FIELD_THREAD_ID)
            .unwrap_or_else(|| constants::parent_assistant::DEFAULT_THREAD_ID.to_string()),
        message_id: string_payload_field(command, constants::parent_assistant::FIELD_MESSAGE_ID)
            .unwrap_or_else(|| constants::parent_assistant::DEFAULT_MESSAGE_ID.to_string()),
        asked_at,
        actor: ParentActorReference {
            actor_id: constants::parent_assistant::DEFAULT_PARENT_ACTOR_ID.to_string(),
            role: ParentActorRole::Parent,
        },
        scope: ParentAssistantScope {
            family: ocentra_parent_agent_protocol::activity::policy_context::FamilyReference {
                family_id: constants::parent_assistant::DEFAULT_FAMILY_ID.to_string(),
            },
            device: None,
        },
        question: string_payload_field(command, constants::field::PARENT_ASSISTANT_QUESTION)
            .unwrap_or_else(|| constants::parent_assistant::DEFAULT_QUESTION.to_string()),
        evidence_context,
        model_id: string_payload_field(command, constants::field::LOCAL_AI_MODEL_ID)
            .or_else(|| Some(config.model_id().to_string())),
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
    }
}

fn generation_request_from_parent_request(
    request: &ParentAssistantGenerateRequest,
    config: &LocalAiRuntimeConfigSnapshot,
    model_id: String,
) -> LocalAiChatGenerationRequest {
    LocalAiChatGenerationRequest {
        model_id,
        prompt: parent_prompt(request),
        max_output_tokens: request
            .max_output_tokens
            .min(config.generation_max_tokens()),
        timeout_ms: request.timeout_ms.min(config.generation_timeout_ms()),
    }
}

fn parent_prompt(request: &ParentAssistantGenerateRequest) -> String {
    let evidence = request
        .evidence_context
        .first()
        .map(|context| context.allowed_summary.as_str())
        .unwrap_or(constants::parent_assistant::DEFAULT_ALLOWED_SUMMARY);
    [
        constants::parent_assistant::PROMPT_SYSTEM,
        constants::parent_assistant::PROMPT_QUESTION_LABEL,
        request.question.as_str(),
        constants::parent_assistant::PROMPT_EVIDENCE_LABEL,
        evidence,
    ]
    .join(constants::parent_assistant::PROMPT_SEPARATOR)
}

fn answer_from_generation_result(
    request: ParentAssistantGenerateRequest,
    result: ocentra_parent_agent_protocol::local_ai_runtime::generation::LocalAiChatGenerationResult,
) -> ParentAssistantAnswer {
    match result.generation_state {
        LocalAiGenerationState::Complete => configured_answer(request, result),
        LocalAiGenerationState::Unavailable => unavailable_result_answer(request, result),
        _ => degraded_result_answer(request, result),
    }
}

fn configured_answer(
    request: ParentAssistantGenerateRequest,
    result: ocentra_parent_agent_protocol::local_ai_runtime::generation::LocalAiChatGenerationResult,
) -> ParentAssistantAnswer {
    base_answer(
        request,
        ParentAssistantAnswerParts {
            provider_id: result.provider_id,
            model_id: result.model_id,
            provider_state: ParentAssistantProviderState::Configured,
            answer_state: ParentAssistantAnswerState::Answered,
            run_state: ParentAssistantRunState::Completed,
            scheduler_job_status: LocalAiProviderSchedulerJobStatus::Complete,
            degraded_state: LocalAiDegradedState::None,
            unavailable_reason: result.unavailable_reason,
            local_ai_result_id: Some(result.local_ai_result_id),
            answer_text: result.output_text,
        },
    )
}

fn degraded_result_answer(
    request: ParentAssistantGenerateRequest,
    result: ocentra_parent_agent_protocol::local_ai_runtime::generation::LocalAiChatGenerationResult,
) -> ParentAssistantAnswer {
    base_answer(
        request,
        ParentAssistantAnswerParts {
            provider_id: result.provider_id,
            model_id: result.model_id,
            provider_state: ParentAssistantProviderState::Degraded,
            answer_state: ParentAssistantAnswerState::Degraded,
            run_state: match result.generation_state {
                LocalAiGenerationState::Failed | LocalAiGenerationState::TimedOut => {
                    ParentAssistantRunState::Failed
                }
                _ => ParentAssistantRunState::Degraded,
            },
            scheduler_job_status: LocalAiProviderSchedulerJobStatus::Degraded,
            degraded_state: LocalAiDegradedState::InvalidOutput,
            unavailable_reason: result
                .unavailable_reason
                .or_else(|| Some(constants::parent_assistant::LOCAL_PROVIDER_DEGRADED.to_string())),
            local_ai_result_id: Some(result.local_ai_result_id),
            answer_text: None,
        },
    )
}

fn unavailable_result_answer(
    request: ParentAssistantGenerateRequest,
    result: ocentra_parent_agent_protocol::local_ai_runtime::generation::LocalAiChatGenerationResult,
) -> ParentAssistantAnswer {
    base_answer(
        request,
        ParentAssistantAnswerParts {
            provider_id: result.provider_id,
            model_id: result.model_id,
            provider_state: ParentAssistantProviderState::Unavailable,
            answer_state: ParentAssistantAnswerState::Unavailable,
            run_state: ParentAssistantRunState::Unavailable,
            scheduler_job_status: LocalAiProviderSchedulerJobStatus::Unavailable,
            degraded_state: LocalAiDegradedState::ProviderUnavailable,
            unavailable_reason: result.unavailable_reason,
            local_ai_result_id: Some(result.local_ai_result_id),
            answer_text: None,
        },
    )
}

fn unavailable_answer(
    request: ParentAssistantGenerateRequest,
    runtime: &ocentra_parent_agent_protocol::local_ai_runtime::status::LocalModelRuntimeStatus,
) -> ParentAssistantAnswer {
    base_answer(
        request,
        ParentAssistantAnswerParts {
            provider_id: runtime.provider_id.clone(),
            model_id: runtime.model_id.clone(),
            provider_state: ParentAssistantProviderState::Unavailable,
            answer_state: ParentAssistantAnswerState::Unavailable,
            run_state: ParentAssistantRunState::Unavailable,
            scheduler_job_status: LocalAiProviderSchedulerJobStatus::Unavailable,
            degraded_state: LocalAiDegradedState::ProviderUnavailable,
            unavailable_reason: runtime.unavailable_reason.clone(),
            local_ai_result_id: None,
            answer_text: None,
        },
    )
}

fn degraded_busy_answer(
    request: ParentAssistantGenerateRequest,
    runtime: &ocentra_parent_agent_protocol::local_ai_runtime::status::LocalModelRuntimeStatus,
) -> ParentAssistantAnswer {
    base_answer(
        request,
        ParentAssistantAnswerParts {
            provider_id: runtime.provider_id.clone(),
            model_id: runtime.model_id.clone(),
            provider_state: ParentAssistantProviderState::Degraded,
            answer_state: ParentAssistantAnswerState::Queued,
            run_state: ParentAssistantRunState::Queued,
            scheduler_job_status: LocalAiProviderSchedulerJobStatus::Queued,
            degraded_state: LocalAiDegradedState::Overloaded,
            unavailable_reason: Some(constants::parent_assistant::LOCAL_PROVIDER_BUSY.to_string()),
            local_ai_result_id: None,
            answer_text: None,
        },
    )
}

fn base_answer(
    request: ParentAssistantGenerateRequest,
    parts: ParentAssistantAnswerParts,
) -> ParentAssistantAnswer {
    let api_provider_boundary = api_boundary::api_provider_boundary(&request.evidence_context);
    let provider_route = api_boundary::provider_route(parts.provider_state, &api_provider_boundary);
    ParentAssistantAnswer {
        schema_version: request.schema_version,
        request_id: request.request_id,
        thread_id: request.thread_id,
        message_id: request.message_id,
        answered_at: timestamp_now(),
        provider_id: parts.provider_id,
        model_id: parts.model_id,
        provider_state: parts.provider_state,
        answer_state: parts.answer_state,
        run_state: parts.run_state,
        scheduler_job_status: parts.scheduler_job_status,
        degraded_state: parts.degraded_state,
        unavailable_reason: parts.unavailable_reason,
        local_ai_result_id: parts.local_ai_result_id,
        answer_text: parts.answer_text,
        citations: request.evidence_context.clone(),
        action_preview: preview_only_action(&request.question),
        api_provider_boundary,
        provider_route,
        prompt_version: constants::parent_assistant::PROMPT_VERSION_LOCAL_V1.to_string(),
    }
}

pub(crate) fn preview_only_action(question: &str) -> ParentAssistantActionPreview {
    let normalized_question = question.to_ascii_lowercase();
    let (action_kind, summary) = if normalized_question
        .contains(constants::parent_assistant::QUESTION_POLICY_HINT)
        || normalized_question.contains(constants::parent_assistant::QUESTION_RULE_HINT)
    {
        (
            ParentAssistantActionPreviewKind::PolicySuggestion,
            constants::parent_assistant::ACTION_PREVIEW_POLICY_SUMMARY,
        )
    } else if normalized_question.contains(constants::parent_assistant::QUESTION_SCHEDULE_HINT)
        || normalized_question.contains(constants::parent_assistant::QUESTION_BEDTIME_HINT)
    {
        (
            ParentAssistantActionPreviewKind::ScheduleChange,
            constants::parent_assistant::ACTION_PREVIEW_SCHEDULE_SUMMARY,
        )
    } else if normalized_question.contains(constants::parent_assistant::QUESTION_TIME_LIMIT_HINT)
        || normalized_question.contains(constants::parent_assistant::QUESTION_LIMIT_HINT)
    {
        (
            ParentAssistantActionPreviewKind::TimeLimitChange,
            constants::parent_assistant::ACTION_PREVIEW_TIME_LIMIT_SUMMARY,
        )
    } else {
        (
            ParentAssistantActionPreviewKind::None,
            constants::parent_assistant::ACTION_PREVIEW_NONE_SUMMARY,
        )
    };

    ParentAssistantActionPreview {
        preview_id: Some(constants::parent_assistant::DEFAULT_PREVIEW_ID.to_string()),
        action_kind,
        summary: Some(summary.to_string()),
        action_reference: None,
        requires_controller_lease: action_kind != ParentAssistantActionPreviewKind::None,
        child_agent_contract_required: true,
        enforcement_applied: false,
    }
}

fn string_payload_field(
    command: &AgentCommandEnvelope,
    payload_field_name: &str,
) -> Option<String> {
    match command.payload.get(payload_field_name) {
        Some(LogFieldValue::String(value)) if !value.trim().is_empty() => {
            Some(value.trim().to_string())
        }
        _ => None,
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
