use ocentra_parent_agent_protocol::activity::policy::ParentActorReference;
use ocentra_parent_agent_protocol::activity::policy::ParentActorRole;
use ocentra_parent_agent_protocol::activity_surface::ActivityHistoricalReportList;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::local_ai_runtime::lifecycle::LocalAiDegradedState;
use ocentra_parent_agent_protocol::local_ai_runtime::scheduler::LocalAiProviderSchedulerJobClass;
use ocentra_parent_agent_protocol::local_ai_runtime::scheduler::LocalAiProviderSchedulerJobStatus;
use ocentra_parent_agent_protocol::logging::LogLevel;
use ocentra_parent_agent_protocol::parent_assistant::ParentAssistantActionPreview;
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

#[path = "parent_assistant_runtime/action_preview.rs"]
mod action_preview;
#[path = "parent_assistant_runtime/answer_factory.rs"]
mod answer_factory;
#[path = "parent_assistant_runtime/command_fields.rs"]
mod command_fields;

use crate::{
    activity_surface_store::{local_store_snapshot, ActivitySurfaceStoreSnapshot},
    event_builder::build_event,
    local_ai_chat_generation_request::LocalAiChatGenerationRequest,
    local_ai_chat_generation_runner::run_local_ai_chat_generation,
    local_ai_provider_scheduler::{local_ai_provider_scheduler, LocalAiProviderSchedulerRuntime},
    local_ai_runtime_config::LocalAiRuntimeConfigSnapshot,
    local_ai_runtime_config_values::LocalAiRuntimeText,
    local_ai_runtime_status::local_ai_runtime_status_for_model_from_config,
    parent_assistant_api::{api_boundary, thread_store},
    parent_assistant_evidence_context::evidence_contexts_from_command,
    parent_assistant_payload::parent_assistant_answer_payload,
    parent_assistant_report_history::activity_report_history_from_command,
    time::timestamp_now,
};

#[derive(Clone, Debug, Eq, PartialEq)]
struct ParentAssistantText(String);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ParentAssistantTextRef<'a>(&'a str);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ParentAssistantPayloadFieldName(&'static str);

impl ParentAssistantTextRef<'_> {
    fn into_text(self) -> ParentAssistantText {
        ParentAssistantText(self.0.to_string())
    }
}

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
    thread_store::record_message_for_thread(thread_store::ParentAssistantThreadId(
        answer.thread_id.clone(),
    ));
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
    provider_id: ParentAssistantText,
    model_id: ParentAssistantText,
    provider_state: ParentAssistantProviderState,
    answer_state: ParentAssistantAnswerState,
    run_state: ParentAssistantRunState,
    scheduler_job_status: LocalAiProviderSchedulerJobStatus,
    degraded_state: LocalAiDegradedState,
    unavailable_reason: Option<ParentAssistantText>,
    local_ai_result_id: Option<ParentAssistantText>,
    answer_text: Option<ParentAssistantText>,
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
        .map(ParentAssistantText)
        .unwrap_or_else(|| ParentAssistantText(config.model_id().0));
    let (runtime, _, _) = local_ai_runtime_status_for_model_from_config(
        timestamp_now::<String>(),
        config,
        Some(LocalAiRuntimeText(model_id.0.clone())),
    );
    if runtime.unavailable_reason.is_some() {
        scheduler
            .record_unavailable_job(&runtime, LocalAiProviderSchedulerJobClass::ParentAssistant);
        return answer_with_api_boundary(
            command,
            answer_factory::unavailable_answer(request, &runtime),
        );
    }
    if scheduler.status_snapshot().current_job_class.is_some() {
        scheduler.record_queued_job(&runtime, LocalAiProviderSchedulerJobClass::ParentAssistant);
        return answer_with_api_boundary(
            command,
            answer_factory::degraded_busy_answer(request, &runtime),
        );
    }

    let generation_request = generation_request_from_parent_request(&request, config, model_id);
    let result = scheduler
        .run_generation_job(
            LocalAiProviderSchedulerJobClass::ParentAssistant,
            runtime,
            || {
                run_local_ai_chat_generation(
                    command.message_id.as_str(),
                    generation_request,
                    config,
                )
            },
        )
        .await;
    answer_with_api_boundary(
        command,
        answer_factory::answer_from_generation_result(request, result),
    )
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
    let asked_at: String = timestamp_now();
    let evidence_context = evidence_contexts_from_command(
        command,
        activity_snapshot,
        stored_report_history,
        asked_at.clone(),
    );
    ParentAssistantGenerateRequest {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        request_id: command_fields::string_payload_field(
            command,
            ParentAssistantPayloadFieldName(constants::field::PARENT_ASSISTANT_REQUEST_ID),
        )
        .unwrap_or_else(|| {
            ParentAssistantTextRef(constants::parent_assistant::DEFAULT_REQUEST_ID).into_text()
        })
        .0,
        thread_id: command_fields::string_payload_field(
            command,
            ParentAssistantPayloadFieldName(constants::parent_assistant::FIELD_THREAD_ID),
        )
        .unwrap_or_else(|| {
            ParentAssistantTextRef(constants::parent_assistant::DEFAULT_THREAD_ID).into_text()
        })
        .0,
        message_id: command_fields::string_payload_field(
            command,
            ParentAssistantPayloadFieldName(constants::parent_assistant::FIELD_MESSAGE_ID),
        )
        .unwrap_or_else(|| {
            ParentAssistantTextRef(constants::parent_assistant::DEFAULT_MESSAGE_ID).into_text()
        })
        .0,
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
        question: command_fields::string_payload_field(
            command,
            ParentAssistantPayloadFieldName(constants::field::PARENT_ASSISTANT_QUESTION),
        )
        .unwrap_or_else(|| {
            ParentAssistantTextRef(constants::parent_assistant::DEFAULT_QUESTION).into_text()
        })
        .0,
        evidence_context,
        model_id: command_fields::string_payload_field(
            command,
            ParentAssistantPayloadFieldName(constants::field::LOCAL_AI_MODEL_ID),
        )
        .map(|value| value.0)
        .or_else(|| Some(config.model_id().0)),
        max_output_tokens: command_fields::numeric_field_u32(
            command
                .payload
                .get(constants::field::LOCAL_AI_MAX_OUTPUT_TOKENS),
            config.generation_max_tokens(),
        ),
        timeout_ms: command_fields::numeric_field_u64(
            command.payload.get(constants::field::LOCAL_AI_TIMEOUT_MS),
            config.generation_timeout_ms(),
        ),
    }
}

fn generation_request_from_parent_request(
    request: &ParentAssistantGenerateRequest,
    config: &LocalAiRuntimeConfigSnapshot,
    model_id: ParentAssistantText,
) -> LocalAiChatGenerationRequest {
    LocalAiChatGenerationRequest {
        model_id: model_id.0,
        prompt: parent_prompt(request).0,
        max_output_tokens: request
            .max_output_tokens
            .min(config.generation_max_tokens()),
        timeout_ms: request.timeout_ms.min(config.generation_timeout_ms()),
    }
}

fn parent_prompt(request: &ParentAssistantGenerateRequest) -> ParentAssistantText {
    let evidence = request
        .evidence_context
        .first()
        .map(|context| context.allowed_summary.as_str())
        .unwrap_or(constants::parent_assistant::DEFAULT_ALLOWED_SUMMARY);
    ParentAssistantText(
        [
            constants::parent_assistant::PROMPT_SYSTEM,
            constants::parent_assistant::PROMPT_QUESTION_LABEL,
            request.question.as_str(),
            constants::parent_assistant::PROMPT_EVIDENCE_LABEL,
            evidence,
        ]
        .join(constants::parent_assistant::PROMPT_SEPARATOR),
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
        provider_id: parts.provider_id.0,
        model_id: parts.model_id.0,
        provider_state: parts.provider_state,
        answer_state: parts.answer_state,
        run_state: parts.run_state,
        scheduler_job_status: parts.scheduler_job_status,
        degraded_state: parts.degraded_state,
        unavailable_reason: parts.unavailable_reason.map(|value| value.0),
        local_ai_result_id: parts.local_ai_result_id.map(|value| value.0),
        answer_text: parts.answer_text.map(|value| value.0),
        citations: request.evidence_context.clone(),
        action_preview: preview_only_action(&ParentAssistantText(request.question)),
        api_provider_boundary,
        provider_route,
        prompt_version: constants::parent_assistant::PROMPT_VERSION_LOCAL_V1.to_string(),
    }
}

fn preview_only_action(question: &ParentAssistantText) -> ParentAssistantActionPreview {
    action_preview::preview_only_action(question)
}
