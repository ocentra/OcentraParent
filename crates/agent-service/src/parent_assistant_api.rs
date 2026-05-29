use ocentra_parent_agent_protocol::{
    constants, policy_constants as policy, AgentCommandEnvelope, AgentCommandName,
    AgentEventEnvelope, AgentEventName, LocalAiDegradedState, LocalAiProviderSchedulerJobStatus,
    LocalAiProviderSchedulerLifecycle, LogFieldValue, LogLevel, ParentAssistantActionConfirmResult,
    ParentAssistantActionConfirmState, ParentAssistantActionPreviewKind,
    ParentAssistantBackendState, ParentAssistantEvidenceContext, ParentAssistantProviderState,
    ParentAssistantProviderStatus, ParentAssistantRunCancelResult, ParentAssistantRunCancelState,
    ParentAssistantRunState, ParentAssistantThreadResponse, ParentEvidenceReference,
    ParentEvidenceReferenceKind,
};

use crate::{
    event_builder::build_event,
    fields::fields_from_pairs,
    local_ai_provider_scheduler::local_ai_provider_scheduler,
    local_ai_runtime_config::LocalAiRuntimeConfigSnapshot,
    local_ai_runtime_status::local_ai_runtime_status_for_model_from_config,
    parent_assistant_payload::{
        parent_assistant_action_confirm_payload, parent_assistant_provider_status_payload,
        parent_assistant_run_cancel_payload, parent_assistant_thread_payload,
    },
    time::timestamp_now,
};

pub(crate) mod api_boundary;
pub(crate) mod thread_store;

pub fn build_parent_assistant_scaffold_event(command: AgentCommandEnvelope) -> AgentEventEnvelope {
    match command.command {
        AgentCommandName::AgentParentAssistantThreadList
        | AgentCommandName::AgentParentAssistantThreadCreate
        | AgentCommandName::AgentParentAssistantThreadOpen
        | AgentCommandName::AgentParentAssistantThreadArchive => build_thread_event(command),
        AgentCommandName::AgentParentAssistantProviderStatusGet => {
            build_provider_status_event(command)
        }
        AgentCommandName::AgentParentAssistantRunCancel => build_run_cancel_event(command),
        AgentCommandName::AgentParentAssistantActionConfirm => build_action_confirm_event(command),
        _ => build_scaffold_fallback_event(command),
    }
}

fn build_thread_event(command: AgentCommandEnvelope) -> AgentEventEnvelope {
    let response = thread_response_for_command(&command);
    build_event(
        constants::event_id::PARENT_ASSISTANT_THREAD_UPDATED,
        &command.message_id,
        command.source,
        AgentEventName::AgentParentAssistantThreadUpdated,
        LogLevel::Info,
        parent_assistant_thread_payload(&response),
        None,
    )
}

fn build_provider_status_event(command: AgentCommandEnvelope) -> AgentEventEnvelope {
    let status = provider_status_for_command(&command);
    let severity = if status.provider_state == ParentAssistantProviderState::Configured {
        LogLevel::Info
    } else {
        LogLevel::Warn
    };
    build_event(
        constants::event_id::PARENT_ASSISTANT_PROVIDER_DEGRADED,
        &command.message_id,
        command.source,
        AgentEventName::AgentParentAssistantProviderDegraded,
        severity,
        parent_assistant_provider_status_payload(&status),
        None,
    )
}

fn build_run_cancel_event(command: AgentCommandEnvelope) -> AgentEventEnvelope {
    let result = run_cancel_result_for_command(&command);
    build_event(
        constants::event_id::PARENT_ASSISTANT_ERROR_REPORTED,
        &command.message_id,
        command.source,
        AgentEventName::AgentParentAssistantErrorReported,
        LogLevel::Warn,
        parent_assistant_run_cancel_payload(&result),
        None,
    )
}

fn build_action_confirm_event(command: AgentCommandEnvelope) -> AgentEventEnvelope {
    let result = action_confirm_result_for_command(&command);
    build_event(
        constants::event_id::PARENT_ASSISTANT_ACTION_CONFIRMED,
        &command.message_id,
        command.source,
        AgentEventName::AgentParentAssistantActionConfirmed,
        LogLevel::Warn,
        parent_assistant_action_confirm_payload(&result),
        None,
    )
}

fn build_scaffold_fallback_event(command: AgentCommandEnvelope) -> AgentEventEnvelope {
    build_event(
        constants::event_id::PARENT_ASSISTANT_ERROR_REPORTED,
        &command.message_id,
        command.source,
        AgentEventName::AgentParentAssistantErrorReported,
        LogLevel::Warn,
        fields_from_pairs(vec![
            string_field(
                constants::field::SCHEMA_VERSION,
                policy::CONTRACT_SCHEMA_VERSION_V0_6,
            ),
            string_field(
                constants::field::PARENT_ASSISTANT_BACKEND_STATE,
                constants::parent_assistant::BACKEND_STATE_SCAFFOLD_ONLY,
            ),
            string_field(
                constants::field::REASON,
                constants::parent_assistant::BACKEND_NOT_CONNECTED,
            ),
        ]),
        None,
    )
}

fn thread_response_for_command(command: &AgentCommandEnvelope) -> ParentAssistantThreadResponse {
    thread_store::thread_response_for_command(command)
}

fn provider_status_for_command(command: &AgentCommandEnvelope) -> ParentAssistantProviderStatus {
    let config = LocalAiRuntimeConfigSnapshot::from_environment();
    let model_id = string_payload_field(command, constants::field::LOCAL_AI_MODEL_ID)
        .unwrap_or_else(|| config.model_id().to_string());
    let (runtime, _, _) =
        local_ai_runtime_status_for_model_from_config(timestamp_now(), &config, Some(&model_id));
    let scheduler_status = local_ai_provider_scheduler().status_snapshot();
    let queue_depth = scheduler_status.queue.total();
    let busy = scheduler_status.current_job_class.is_some() || queue_depth > 0;
    let provider_state = if runtime.unavailable_reason.is_some() {
        ParentAssistantProviderState::Unavailable
    } else if busy || scheduler_status.degraded_state != LocalAiDegradedState::None {
        ParentAssistantProviderState::Degraded
    } else {
        ParentAssistantProviderState::Configured
    };
    let scheduler_job_status = if runtime.unavailable_reason.is_some() {
        LocalAiProviderSchedulerJobStatus::Unavailable
    } else {
        match scheduler_status.lifecycle_state {
            LocalAiProviderSchedulerLifecycle::Running => {
                LocalAiProviderSchedulerJobStatus::Running
            }
            LocalAiProviderSchedulerLifecycle::Queued => LocalAiProviderSchedulerJobStatus::Queued,
            LocalAiProviderSchedulerLifecycle::Degraded => {
                LocalAiProviderSchedulerJobStatus::Degraded
            }
            LocalAiProviderSchedulerLifecycle::Unavailable => {
                LocalAiProviderSchedulerJobStatus::Unavailable
            }
            LocalAiProviderSchedulerLifecycle::Idle => LocalAiProviderSchedulerJobStatus::Complete,
        }
    };
    let run_state = if runtime.unavailable_reason.is_some() {
        ParentAssistantRunState::Unavailable
    } else {
        match scheduler_status.lifecycle_state {
            LocalAiProviderSchedulerLifecycle::Running => ParentAssistantRunState::Active,
            LocalAiProviderSchedulerLifecycle::Queued => ParentAssistantRunState::Queued,
            LocalAiProviderSchedulerLifecycle::Degraded => ParentAssistantRunState::Degraded,
            LocalAiProviderSchedulerLifecycle::Unavailable => ParentAssistantRunState::Unavailable,
            LocalAiProviderSchedulerLifecycle::Idle => ParentAssistantRunState::Completed,
        }
    };
    let degraded_state = scheduler_status.degraded_state.clone();
    let unavailable_reason = runtime
        .unavailable_reason
        .or_else(|| scheduler_status.unavailable_reason.clone());
    let citations = [default_evidence_context()];

    ParentAssistantProviderStatus {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        backend_state: ParentAssistantBackendState::RuntimeBacked,
        provider_id: runtime.provider_id,
        model_id: runtime.model_id,
        provider_state,
        run_state,
        scheduler_job_status,
        scheduler_status,
        degraded_state,
        unavailable_reason,
        queue_depth,
        busy,
        api_provider_boundary: api_boundary::api_provider_boundary(&citations),
    }
}

fn run_cancel_result_for_command(command: &AgentCommandEnvelope) -> ParentAssistantRunCancelResult {
    ParentAssistantRunCancelResult {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        backend_state: ParentAssistantBackendState::RuntimeBacked,
        thread_id: string_payload_field(command, constants::parent_assistant::FIELD_THREAD_ID)
            .unwrap_or_else(|| constants::parent_assistant::DEFAULT_THREAD_ID.to_string()),
        run_id: string_payload_field(command, constants::parent_assistant::FIELD_RUN_ID)
            .unwrap_or_else(|| constants::parent_assistant::DEFAULT_RUN_ID.to_string()),
        cancel_state: ParentAssistantRunCancelState::NotRunning,
        run_state: ParentAssistantRunState::Completed,
        provider_state: ParentAssistantProviderState::Unavailable,
        unavailable_reason: Some(constants::parent_assistant::RUN_NOT_RUNNING_REASON.to_string()),
    }
}

fn action_confirm_result_for_command(
    command: &AgentCommandEnvelope,
) -> ParentAssistantActionConfirmResult {
    ParentAssistantActionConfirmResult {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        backend_state: ParentAssistantBackendState::ContractRequired,
        action_intent_id: string_payload_field(
            command,
            constants::parent_assistant::FIELD_ACTION_INTENT_ID,
        )
        .unwrap_or_else(|| constants::parent_assistant::DEFAULT_ACTION_INTENT_ID.to_string()),
        preview_id: Some(constants::parent_assistant::DEFAULT_PREVIEW_ID.to_string()),
        action_kind: ParentAssistantActionPreviewKind::PolicySuggestion,
        confirm_state: ParentAssistantActionConfirmState::ContractRequired,
        requires_controller_lease: true,
        child_agent_contract_required: true,
        enforcement_applied: false,
        policy_written: false,
        reason: constants::parent_assistant::ACTION_CONFIRM_CONTRACT_REQUIRED_REASON.to_string(),
    }
}

fn default_evidence_context() -> ParentAssistantEvidenceContext {
    ParentAssistantEvidenceContext {
        evidence: ParentEvidenceReference {
            evidence_reference_id: constants::field::ACTIVITY_DIGEST.to_string(),
            kind: ParentEvidenceReferenceKind::QueryStoreSummary,
            observed_at: timestamp_now(),
        },
        citation_label: constants::parent_assistant::DEFAULT_CITATION_LABEL.to_string(),
        allowed_summary: constants::parent_assistant::DEFAULT_ALLOWED_SUMMARY.to_string(),
    }
}

fn string_payload_field(command: &AgentCommandEnvelope, key: &str) -> Option<String> {
    match command.payload.get(key) {
        Some(LogFieldValue::String(value)) if !value.trim().is_empty() => {
            Some(value.trim().to_string())
        }
        _ => None,
    }
}

fn string_field(key: &'static str, value: &str) -> (&'static str, LogFieldValue) {
    (key, LogFieldValue::String(value.to_string()))
}
