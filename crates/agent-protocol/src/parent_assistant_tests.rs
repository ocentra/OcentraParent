use super::{
    AgentCommandName, AgentEventName, FamilyReference, LocalAiDegradedState,
    LocalAiProviderSchedulerJobStatus, ParentActionReference, ParentActorReference,
    ParentActorRole, ParentAssistantActionPreview, ParentAssistantActionPreviewKind,
    ParentAssistantAnswer, ParentAssistantAnswerState, ParentAssistantApiAuthorizationState,
    ParentAssistantApiProviderBoundary, ParentAssistantEvidenceContext,
    ParentAssistantGenerateRequest, ParentAssistantProviderState, ParentAssistantScope,
    ParentDeviceReference, ParentEvidenceReference, ParentEvidenceReferenceKind,
};

#[test]
fn parent_assistant_command_and_event_names_match_typescript_contracts() {
    let command = serde_json::to_value(AgentCommandName::AgentParentAssistantAnswerGenerate)
        .expect("command serializes");
    let event = serde_json::to_value(AgentEventName::AgentParentAssistantAnswerReported)
        .expect("event serializes");

    assert_eq!(command, "agent.parent-assistant.answer.generate");
    assert_eq!(event, "agent.parent-assistant.answer.reported");
}

#[test]
fn parent_assistant_request_serializes_cited_evidence_context() {
    let request = sample_request();
    let serialized = serde_json::to_value(&request).expect("request serializes");

    assert_eq!(serialized["schemaVersion"], "v0.6");
    assert_eq!(serialized["question"], "Why did app use increase today?");
    assert_eq!(
        serialized["evidenceContext"][0]["evidence"]["kind"],
        "query-store-summary"
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
        prompt_version: "parent-assistant-local-v1".to_string(),
    };
    let serialized = serde_json::to_value(&answer).expect("answer serializes");

    assert_eq!(serialized["answerState"], "answered");
    assert_eq!(
        serialized["citations"][0]["citationLabel"],
        "Activity summary 1"
    );
    assert_eq!(serialized["actionPreview"]["enforcementApplied"], false);
    assert_eq!(
        serialized["actionPreview"]["childAgentContractRequired"],
        true
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
        prompt_version: "parent-assistant-local-v1".to_string(),
    };
    let serialized = serde_json::to_value(&answer).expect("answer serializes");

    assert_eq!(serialized["providerState"], "unavailable");
    assert_eq!(
        serialized["unavailableReason"],
        "local-ai-provider-unconfigured"
    );
    assert_eq!(serialized["actionPreview"]["actionKind"], "none");
}

#[test]
fn parent_assistant_api_provider_boundary_serializes_parent_authorization_and_custody() {
    let boundary = ParentAssistantApiProviderBoundary {
        schema_version: "v0.6".to_string(),
        provider_id: "api-provider-not-configured".to_string(),
        authorization_state: ParentAssistantApiAuthorizationState::NotAuthorized,
        custody_label: "parent-authorized-api-ai".to_string(),
        retention_policy: "no-retention-without-parent-authorization".to_string(),
        deletion_policy: "delete-provider-cache-on-parent-request".to_string(),
        citations: vec![sample_evidence_context()],
        provider_state: ParentAssistantProviderState::Unavailable,
        unavailable_reason: Some("api-ai-provider-not-authorized".to_string()),
        child_safety_or_enforcement_use_allowed: false,
    };
    let serialized = serde_json::to_value(&boundary).expect("boundary serializes");

    assert_eq!(serialized["authorizationState"], "not-authorized");
    assert_eq!(serialized["custodyLabel"], "parent-authorized-api-ai");
    assert_eq!(
        serialized["citations"][0]["evidence"]["kind"],
        "query-store-summary"
    );
    assert_eq!(serialized["childSafetyOrEnforcementUseAllowed"], false);
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
    }
}
