use ocentra_parent_agent_protocol::{
    constants, policy_constants as policy, AgentCommandEnvelope, AgentCommandName,
    AgentEventEnvelope, AgentEventName, LocalAiDegradedState, LocalAiProviderSchedulerJobStatus,
    LogFieldValue, LogLevel, ParentAssistantActionConfirmResult, ParentAssistantActionConfirmState,
    ParentAssistantActionPreviewKind, ParentAssistantApiAuthorizationState,
    ParentAssistantApiProviderBoundary, ParentAssistantBackendState,
    ParentAssistantEvidenceContext, ParentAssistantProviderState, ParentAssistantProviderStatus,
    ParentAssistantRunCancelResult, ParentAssistantRunCancelState, ParentAssistantThreadResponse,
    ParentEvidenceReference, ParentEvidenceReferenceKind,
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
    let scheduler = local_ai_provider_scheduler().status_snapshot();
    let queue_depth = scheduler.queue.total();
    let busy = scheduler.current_job_class.is_some() || queue_depth > 0;
    let provider_state = if runtime.unavailable_reason.is_some() {
        ParentAssistantProviderState::Unavailable
    } else if busy || scheduler.degraded_state != LocalAiDegradedState::None {
        ParentAssistantProviderState::Degraded
    } else {
        ParentAssistantProviderState::Configured
    };
    let scheduler_job_status = if runtime.unavailable_reason.is_some() {
        LocalAiProviderSchedulerJobStatus::Unavailable
    } else if scheduler.current_job_class.is_some() {
        LocalAiProviderSchedulerJobStatus::Running
    } else if queue_depth > 0 {
        LocalAiProviderSchedulerJobStatus::Queued
    } else {
        LocalAiProviderSchedulerJobStatus::Complete
    };

    ParentAssistantProviderStatus {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        backend_state: ParentAssistantBackendState::RuntimeBacked,
        provider_id: runtime.provider_id,
        model_id: runtime.model_id,
        provider_state,
        scheduler_job_status,
        degraded_state: scheduler.degraded_state,
        unavailable_reason: runtime.unavailable_reason.or(scheduler.unavailable_reason),
        queue_depth,
        busy,
        api_provider_boundary: api_provider_boundary(),
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

fn api_provider_boundary() -> ParentAssistantApiProviderBoundary {
    ParentAssistantApiProviderBoundary {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        provider_id: constants::parent_assistant::API_PROVIDER_ID_NOT_AUTHORIZED.to_string(),
        authorization_state: ParentAssistantApiAuthorizationState::NotAuthorized,
        custody_label: constants::parent_assistant::API_PROVIDER_CUSTODY_LABEL.to_string(),
        retention_policy: constants::parent_assistant::API_PROVIDER_RETENTION_POLICY.to_string(),
        deletion_policy: constants::parent_assistant::API_PROVIDER_DELETION_POLICY.to_string(),
        citations: vec![default_evidence_context()],
        provider_state: ParentAssistantProviderState::Unavailable,
        unavailable_reason: Some(
            constants::parent_assistant::API_PROVIDER_NOT_AUTHORIZED_REASON.to_string(),
        ),
        child_safety_or_enforcement_use_allowed: false,
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
