use super::{
    AgentCommandName, AgentEventName, FamilyReference, LocalAiDegradedState,
    LocalAiProviderSchedulerJobStatus, LocalAiProviderSchedulerLifecycle,
    LocalAiProviderSchedulerQueue, LocalAiProviderSchedulerStatus, LocalAiProviderSingletonScope,
    LocalAiResourceClass, ParentActionReference, ParentActorReference, ParentActorRole,
    ParentAssistantActionConfirmResult, ParentAssistantActionConfirmState,
    ParentAssistantActionPreview, ParentAssistantActionPreviewKind,
    ParentAssistantActionPreviewResult, ParentAssistantActionPreviewState, ParentAssistantAnswer,
    ParentAssistantAnswerState, ParentAssistantApiAuthorizationState,
    ParentAssistantApiProviderAccessState, ParentAssistantApiProviderBoundary,
    ParentAssistantBackendState, ParentAssistantChildAgentValidationState,
    ParentAssistantEvidenceContext, ParentAssistantGenerateRequest, ParentAssistantProviderRoute,
    ParentAssistantProviderRoutingState, ParentAssistantProviderSelection,
    ParentAssistantProviderState, ParentAssistantProviderStatus, ParentAssistantRunCancelResult,
    ParentAssistantRunCancelState, ParentAssistantRunState, ParentAssistantScope,
    ParentAssistantThreadRecord, ParentAssistantThreadResponse, ParentAssistantThreadState,
    ParentDeviceReference, ParentEvidenceReference, ParentEvidenceReferenceKind,
};
use ocentra_eventing::expect_value::ExpectValue;

#[test]
fn parent_assistant_command_and_event_names_match_typescript_contracts() {
    let command = serde_json::to_value(AgentCommandName::AgentParentAssistantAnswerGenerate)
        .expect_value("command serializes");
    let event = serde_json::to_value(AgentEventName::AgentParentAssistantAnswerReported)
        .expect_value("event serializes");

    assert_eq!(command, "agent.parent-assistant.answer.generate");
    assert_eq!(event, "agent.parent-assistant.answer.reported");
}

#[test]
fn parent_assistant_request_serializes_cited_evidence_context() {
    let request = sample_request();
    let serialized = serde_json::to_value(&request).expect_value("request serializes");

    assert_eq!(serialized["schemaVersion"], "v0.6");
    assert_eq!(serialized["question"], "Why did app use increase today?");
    assert_eq!(
        serialized["evidenceContext"][0]["evidence"]["kind"],
        "query-store-summary"
    );
    assert_eq!(
        serialized["evidenceContext"][0]["rawChildEvidenceIncluded"],
        false
    );
    assert_eq!(
        serialized["evidenceContext"][0]["directEnforcementAllowed"],
        false
    );
}

#[test]
fn parent_assistant_answer_serializes_citations_and_action_preview_without_enforcement() {
    let answer = ParentAssistantAnswer {
        schema_version: "v0.6".to_string(),
        request_id: "parent-assistant-request-1".to_string(),
        thread_id: "parent-assistant-thread-1".to_string(),
        message_id: "parent-assistant-message-1".to_string(),
        answered_at: "2026-05-27T06:31:02Z".to_string(),
        provider_id: "local-provider-llama-cli".to_string(),
        model_id: "local-gguf-chat-model".to_string(),
        provider_state: ParentAssistantProviderState::Configured,
        answer_state: ParentAssistantAnswerState::Answered,
        run_state: ParentAssistantRunState::Completed,
        scheduler_job_status: LocalAiProviderSchedulerJobStatus::Complete,
        degraded_state: LocalAiDegradedState::None,
        unavailable_reason: None,
        local_ai_result_id: Some("local-ai-result-parent-assistant-request-1".to_string()),
        answer_text: Some(
            "App use increased because the recent activity window shows a longer game session."
                .to_string(),
        ),
        citations: vec![sample_evidence_context()],
        action_preview: sample_action_preview(false),
        api_provider_boundary: sample_api_provider_boundary(),
        provider_route: sample_provider_route(ParentAssistantProviderState::Configured),
        prompt_version: "parent-assistant-local-v1".to_string(),
    };
    let serialized = serde_json::to_value(&answer).expect_value("answer serializes");

    assert_eq!(serialized["answerState"], "answered");
    assert_eq!(serialized["runState"], "completed");
    assert_eq!(
        serialized["citations"][0]["citationLabel"],
        "Activity summary 1"
    );
    assert_eq!(serialized["actionPreview"]["enforcementApplied"], false);
    assert_eq!(
        serialized["apiProviderBoundary"]["authorizationState"],
        "not-authorized"
    );
    assert_eq!(
        serialized["providerRoute"]["routingState"],
        "local-provider-ready"
    );
    assert_eq!(serialized["providerRoute"]["selectedProvider"], "local");
    assert_eq!(
        serialized["actionPreview"]["childAgentContractRequired"],
        true
    );
}

#[test]
fn parent_assistant_action_preview_result_serializes_draft_without_enforcement() {
    let result = ParentAssistantActionPreviewResult {
        schema_version: "v0.6".to_string(),
        backend_state: ParentAssistantBackendState::RuntimeBacked,
        action_intent_id: "parent-assistant-action-intent-1".to_string(),
        preview_state: ParentAssistantActionPreviewState::Draft,
        preview: sample_action_preview(false),
        evidence_context: vec![sample_evidence_context()],
        preview_required: true,
        preview_satisfied: true,
        raw_assistant_prose_accepted: false,
        parent_confirmation_required: true,
        parent_confirmation_recorded: false,
        child_agent_validation_state:
            ParentAssistantChildAgentValidationState::ChildAgentContractRequired,
        source_refs: vec![sample_evidence_context().evidence],
        audit_reason: "Preview generated from cited parent-owned Activity evidence.".to_string(),
        requires_controller_lease: true,
        child_agent_contract_required: true,
        enforcement_applied: false,
        policy_written: false,
        reason: "action preview draft requires confirmation".to_string(),
    };
    let serialized = serde_json::to_value(&result).expect_value("preview result serializes");

    assert_eq!(serialized["previewState"], "draft");
    assert_eq!(serialized["preview"]["enforcementApplied"], false);
    assert_eq!(
        serialized["evidenceContext"][0]["evidence"]["kind"],
        "query-store-summary"
    );
    assert_eq!(serialized["policyWritten"], false);
    assert_eq!(serialized["childAgentContractRequired"], true);
    assert_eq!(serialized["previewRequired"], true);
    assert_eq!(serialized["previewSatisfied"], true);
    assert_eq!(serialized["rawAssistantProseAccepted"], false);
    assert_eq!(
        serialized["childAgentValidationState"],
        "child-agent-contract-required"
    );
    assert_eq!(
        serialized["sourceRefs"][0]["evidenceReferenceId"],
        "activity-summary-1"
    );
}

#[test]
fn parent_assistant_unavailable_answer_serializes_typed_provider_state() {
    let answer = ParentAssistantAnswer {
        schema_version: "v0.6".to_string(),
        request_id: "parent-assistant-request-2".to_string(),
        thread_id: "parent-assistant-thread-1".to_string(),
        message_id: "parent-assistant-message-2".to_string(),
        answered_at: "2026-05-27T06:32:02Z".to_string(),
        provider_id: "local-provider-unconfigured".to_string(),
        model_id: "safety-model-unconfigured".to_string(),
        provider_state: ParentAssistantProviderState::Unavailable,
        answer_state: ParentAssistantAnswerState::Unavailable,
        run_state: ParentAssistantRunState::Unavailable,
        scheduler_job_status: LocalAiProviderSchedulerJobStatus::Unavailable,
        degraded_state: LocalAiDegradedState::ProviderUnavailable,
        unavailable_reason: Some("local-ai-provider-unconfigured".to_string()),
        local_ai_result_id: None,
        answer_text: None,
        citations: vec![],
        action_preview: ParentAssistantActionPreview {
            preview_id: None,
            action_kind: ParentAssistantActionPreviewKind::None,
            summary: None,
            action_reference: None,
            requires_controller_lease: false,
            child_agent_contract_required: true,
            enforcement_applied: false,
        },
        api_provider_boundary: sample_api_provider_boundary(),
        provider_route: sample_provider_route(ParentAssistantProviderState::Unavailable),
        prompt_version: "parent-assistant-local-v1".to_string(),
    };
    let serialized = serde_json::to_value(&answer).expect_value("answer serializes");

    assert_eq!(serialized["providerState"], "unavailable");
    assert_eq!(
        serialized["unavailableReason"],
        "local-ai-provider-unconfigured"
    );
    assert_eq!(serialized["actionPreview"]["actionKind"], "none");
}

#[test]
fn parent_assistant_api_provider_boundary_serializes_parent_authorization_and_custody() {
    let boundary = sample_api_provider_boundary();
    let serialized = serde_json::to_value(&boundary).expect_value("boundary serializes");

    assert_eq!(serialized["authorizationState"], "not-authorized");
    assert_eq!(serialized["accessState"], "not-authorized");
    assert_eq!(serialized["parentAuthorizationRequired"], true);
    assert_eq!(serialized["evidenceCitationRequired"], true);
    assert_eq!(serialized["custodyLabel"], "parent-authorized-api-ai");
    assert_eq!(serialized["custodyState"], "parent-owned-citations-only");
    assert_eq!(
        serialized["retentionState"],
        "no-retention-without-parent-authorization"
    );
    assert_eq!(
        serialized["deletionState"],
        "delete-provider-cache-on-parent-request"
    );
    assert_eq!(
        serialized["citations"][0]["evidence"]["kind"],
        "query-store-summary"
    );
    assert_eq!(serialized["childSafetyOrEnforcementUseAllowed"], false);
}

#[test]
fn parent_assistant_thread_response_serializes_durable_local_state() {
    let response = ParentAssistantThreadResponse {
        schema_version: "v0.6".to_string(),
        backend_state: ParentAssistantBackendState::DurableLocal,
        active_thread: Some(sample_thread(ParentAssistantThreadState::Open)),
        threads: vec![sample_thread(ParentAssistantThreadState::Open)],
        reason: Some("durable local thread state".to_string()),
    };
    let serialized = serde_json::to_value(&response).expect_value("thread response serializes");

    assert_eq!(serialized["backendState"], "durable-local");
    assert_eq!(serialized["activeThread"]["state"], "open");
}

#[test]
fn parent_assistant_provider_status_serializes_scheduler_and_api_boundaries() {
    let status = ParentAssistantProviderStatus {
        schema_version: "v0.6".to_string(),
        backend_state: ParentAssistantBackendState::RuntimeBacked,
        provider_id: "local-provider-llama-cli".to_string(),
        model_id: "local-gguf-chat-model".to_string(),
        provider_state: ParentAssistantProviderState::Unavailable,
        run_state: ParentAssistantRunState::Unavailable,
        scheduler_job_status: LocalAiProviderSchedulerJobStatus::Unavailable,
        scheduler_status: sample_scheduler_status(LocalAiProviderSchedulerLifecycle::Unavailable),
        degraded_state: LocalAiDegradedState::ProviderUnavailable,
        unavailable_reason: Some("local-ai-provider-unconfigured".to_string()),
        queue_depth: 0,
        busy: false,
        api_provider_boundary: sample_api_provider_boundary(),
        provider_route: sample_provider_route(ParentAssistantProviderState::Unavailable),
    };
    let serialized = serde_json::to_value(&status).expect_value("provider status serializes");

    assert_eq!(serialized["backendState"], "runtime-backed");
    assert_eq!(serialized["schedulerJobStatus"], "unavailable");
    assert_eq!(
        serialized["apiProviderBoundary"]["childSafetyOrEnforcementUseAllowed"],
        false
    );
    assert_eq!(
        serialized["providerRoute"]["routingState"],
        "no-provider-available"
    );
    assert_eq!(
        serialized["providerRoute"]["childSafetyOrEnforcementUseAllowed"],
        false
    );
}

#[test]
fn parent_assistant_cancel_and_confirm_results_do_not_claim_enforcement() {
    let cancel = ParentAssistantRunCancelResult {
        schema_version: "v0.6".to_string(),
        backend_state: ParentAssistantBackendState::RuntimeBacked,
        thread_id: "parent-assistant-thread-1".to_string(),
        run_id: "parent-assistant-run-1".to_string(),
        cancel_state: ParentAssistantRunCancelState::NotRunning,
        run_state: ParentAssistantRunState::Completed,
        provider_state: ParentAssistantProviderState::Unavailable,
        unavailable_reason: Some("parent-assistant-run-not-running".to_string()),
    };
    let confirm = ParentAssistantActionConfirmResult {
        schema_version: "v0.6".to_string(),
        backend_state: ParentAssistantBackendState::ContractRequired,
        action_intent_id: "parent-assistant-action-intent-1".to_string(),
        preview_id: Some("parent-assistant-preview-1".to_string()),
        action_kind: ParentAssistantActionPreviewKind::PolicySuggestion,
        confirm_state: ParentAssistantActionConfirmState::ContractRequired,
        preview_required: true,
        preview_satisfied: true,
        raw_assistant_prose_accepted: false,
        parent_confirmation_required: true,
        parent_confirmation_recorded: false,
        child_agent_validation_state:
            ParentAssistantChildAgentValidationState::ChildAgentContractRequired,
        source_refs: vec![sample_evidence_context().evidence],
        audit_reason:
            "Parent confirmation cannot write policy until child-agent validation is wired."
                .to_string(),
        requires_controller_lease: true,
        child_agent_contract_required: true,
        enforcement_applied: false,
        policy_written: false,
        reason: "controller lease and child-agent policy contract are required".to_string(),
    };
    let cancel_json = serde_json::to_value(&cancel).expect_value("cancel result serializes");
    let confirm_json = serde_json::to_value(&confirm).expect_value("confirm result serializes");

    assert_eq!(cancel_json["cancelState"], "not-running");
    assert_eq!(confirm_json["confirmState"], "contract-required");
    assert_eq!(confirm_json["previewSatisfied"], true);
    assert_eq!(confirm_json["rawAssistantProseAccepted"], false);
    assert_eq!(confirm_json["enforcementApplied"], false);
    assert_eq!(confirm_json["policyWritten"], false);
}

fn sample_api_provider_boundary() -> ParentAssistantApiProviderBoundary {
    ParentAssistantApiProviderBoundary {
        schema_version: "v0.6".to_string(),
        provider_id: "api-provider-not-configured".to_string(),
        authorization_state: ParentAssistantApiAuthorizationState::NotAuthorized,
        access_state: ParentAssistantApiProviderAccessState::NotAuthorized,
        parent_authorization_required: true,
        evidence_citation_required: true,
        custody_label: "parent-authorized-api-ai".to_string(),
        custody_state: "parent-owned-citations-only".to_string(),
        retention_policy: "no-retention-without-parent-authorization".to_string(),
        retention_state: "no-retention-without-parent-authorization".to_string(),
        deletion_policy: "delete-provider-cache-on-parent-request".to_string(),
        deletion_state: "delete-provider-cache-on-parent-request".to_string(),
        citations: vec![sample_evidence_context()],
        provider_state: ParentAssistantProviderState::Unavailable,
        unavailable_reason: Some("api-ai-provider-not-authorized".to_string()),
        child_safety_or_enforcement_use_allowed: false,
    }
}

fn sample_provider_route(
    local_provider_state: ParentAssistantProviderState,
) -> ParentAssistantProviderRoute {
    let (routing_state, selected_provider) = match local_provider_state {
        ParentAssistantProviderState::Configured => (
            ParentAssistantProviderRoutingState::LocalProviderReady,
            ParentAssistantProviderSelection::Local,
        ),
        ParentAssistantProviderState::Degraded => (
            ParentAssistantProviderRoutingState::LocalProviderDegraded,
            ParentAssistantProviderSelection::Local,
        ),
        ParentAssistantProviderState::Unavailable => (
            ParentAssistantProviderRoutingState::NoProviderAvailable,
            ParentAssistantProviderSelection::None,
        ),
    };

    ParentAssistantProviderRoute {
        routing_state,
        selected_provider,
        local_provider_state,
        api_provider_state: ParentAssistantProviderState::Unavailable,
        api_access_state: ParentAssistantApiProviderAccessState::NotAuthorized,
        evidence_citation_required: true,
        remote_ai_optional: true,
        child_safety_or_enforcement_use_allowed: false,
        reason: "Local/API provider route preserves citations and no enforcement use.".to_string(),
    }
}

fn sample_request() -> ParentAssistantGenerateRequest {
    ParentAssistantGenerateRequest {
        schema_version: "v0.6".to_string(),
        request_id: "parent-assistant-request-1".to_string(),
        thread_id: "parent-assistant-thread-1".to_string(),
        message_id: "parent-assistant-message-1".to_string(),
        asked_at: "2026-05-27T06:31:00Z".to_string(),
        actor: sample_actor(),
        scope: ParentAssistantScope {
            family: FamilyReference {
                family_id: "family-local-1".to_string(),
            },
            device: Some(ParentDeviceReference {
                device_id: "child-device-1".to_string(),
                child_profile_id: Some("child-profile-1".to_string()),
                label: "Kitchen laptop".to_string(),
                platform: "windows".to_string(),
            }),
        },
        question: "Why did app use increase today?".to_string(),
        evidence_context: vec![sample_evidence_context()],
        model_id: Some("local-gguf-chat-model".to_string()),
        max_output_tokens: 320,
        timeout_ms: 15_000,
    }
}

fn sample_thread(state: ParentAssistantThreadState) -> ParentAssistantThreadRecord {
    ParentAssistantThreadRecord {
        schema_version: "v0.6".to_string(),
        thread_id: "parent-assistant-thread-1".to_string(),
        title: "Recent activity questions".to_string(),
        state,
        backend_state: ParentAssistantBackendState::DurableLocal,
        created_at: "2026-05-28T17:20:00Z".to_string(),
        updated_at: "2026-05-28T17:20:01Z".to_string(),
        message_count: 0,
    }
}

fn sample_scheduler_status(
    lifecycle_state: LocalAiProviderSchedulerLifecycle,
) -> LocalAiProviderSchedulerStatus {
    LocalAiProviderSchedulerStatus {
        physical_device_id: "physical-device-local".to_string(),
        singleton_scope: LocalAiProviderSingletonScope::PhysicalDevice,
        provider_id: "local-provider-llama-cli".to_string(),
        runtime_reference_id: "local-runtime-llama-cli".to_string(),
        model_id: "local-gguf-chat-model".to_string(),
        model_reference: "local-model-reference".to_string(),
        resource_class: LocalAiResourceClass::Cpu,
        lifecycle_state,
        current_job_class: None,
        queue: LocalAiProviderSchedulerQueue::default(),
        duplicate_runtime_blocked: true,
        degraded_state: LocalAiDegradedState::ProviderUnavailable,
        unavailable_reason: Some("local-ai-provider-unconfigured".to_string()),
        last_checked_at: "2026-05-28T17:20:01Z".to_string(),
    }
}

fn sample_action_preview(enforcement_applied: bool) -> ParentAssistantActionPreview {
    ParentAssistantActionPreview {
        preview_id: Some("parent-assistant-preview-1".to_string()),
        action_kind: ParentAssistantActionPreviewKind::TimeLimitChange,
        summary: Some("Preview a shorter evening game window.".to_string()),
        action_reference: Some(ParentActionReference {
            action_reference_id: "action-preview-1".to_string(),
            actor: sample_actor(),
            policy_version: "policy-v1".to_string(),
            created_at: "2026-05-27T06:31:02Z".to_string(),
        }),
        requires_controller_lease: true,
        child_agent_contract_required: true,
        enforcement_applied,
    }
}

fn sample_actor() -> ParentActorReference {
    ParentActorReference {
        actor_id: "parent-actor-1".to_string(),
        role: ParentActorRole::Parent,
    }
}

fn sample_evidence_context() -> ParentAssistantEvidenceContext {
    ParentAssistantEvidenceContext {
        evidence: ParentEvidenceReference {
            evidence_reference_id: "activity-summary-1".to_string(),
            kind: ParentEvidenceReferenceKind::QueryStoreSummary,
            observed_at: "2026-05-27T06:30:00Z".to_string(),
        },
        citation_label: "Activity summary 1".to_string(),
        allowed_summary: "App use was higher than the daily baseline.".to_string(),
        custody_label: "parent-owned-activity-summary".to_string(),
        source_label: "activity-query-store-summary".to_string(),
        raw_child_evidence_included: false,
        direct_enforcement_allowed: false,
    }
}
