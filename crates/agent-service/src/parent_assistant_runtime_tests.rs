use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use ocentra_parent_agent_protocol::{
    constants, policy_constants as policy, AgentCommandEnvelope, AgentCommandName,
    AgentMessageTarget, AgentPeer, AgentPeerRole, AgentRoute, FamilyReference,
    LocalAiDegradedState, LocalAiProviderSchedulerJobClass, LocalAiProviderSchedulerJobStatus,
    ParentActorReference, ParentActorRole, ParentAssistantAnswerState,
    ParentAssistantEvidenceContext, ParentAssistantGenerateRequest, ParentAssistantProviderState,
    ParentAssistantScope, ParentEvidenceReference, ParentEvidenceReferenceKind,
    AGENT_PROTOCOL_SCHEMA_VERSION,
};

use crate::{
    activity_surface_store::ActivitySurfaceStoreSnapshot,
    fields::fields_from_pairs,
    local_ai_provider_scheduler::LocalAiProviderSchedulerRuntime,
    local_ai_runtime_config::LocalAiRuntimeConfigSnapshot,
    local_ai_runtime_status::local_ai_runtime_status_for_model_from_config,
    parent_assistant_runtime::{
        generate_parent_assistant_answer_with_scheduler, request_from_command,
    },
};

#[tokio::test]
async fn parent_assistant_unconfigured_provider_returns_cited_unavailable_answer() {
    let scheduler = LocalAiProviderSchedulerRuntime::new_for_test();
    let config = LocalAiRuntimeConfigSnapshot::unconfigured();

    let answer = generate_parent_assistant_answer_with_scheduler(
        &command(),
        request(None),
        &config,
        &scheduler,
    )
    .await;

    assert_eq!(
        answer.provider_state,
        ParentAssistantProviderState::Unavailable
    );
    assert_eq!(answer.answer_state, ParentAssistantAnswerState::Unavailable);
    assert_eq!(
        answer.scheduler_job_status,
        LocalAiProviderSchedulerJobStatus::Unavailable
    );
    assert_eq!(answer.citations.len(), 1);
    assert!(answer.action_preview.child_agent_contract_required);
    assert!(!answer.action_preview.enforcement_applied);
    assert_eq!(scheduler.status_snapshot().current_job_class, None);
}

#[tokio::test]
async fn parent_assistant_busy_provider_degrades_without_running_or_enforcing() {
    let scheduler = LocalAiProviderSchedulerRuntime::new_for_test();
    let runtime_binary = write_temp_file(constants::local_ai_runtime::PROVIDER_ID_LOCAL_LLAMA_CLI);
    let model_file = write_temp_file(constants::local_ai_runtime::MODEL_ID_DEFAULT_GEMMA_4);
    let config = LocalAiRuntimeConfigSnapshot::from_parts_with_execution(
        Some(runtime_binary.clone()),
        Some(model_file.clone()),
        None,
        None,
        true,
        constants::local_ai_runtime::DEFAULT_GENERATION_TIMEOUT_MS,
        constants::local_ai_runtime::DEFAULT_GENERATION_MAX_TOKENS,
    );
    let (runtime, _, _) = local_ai_runtime_status_for_model_from_config(
        constants::local_ai_runtime::TEST_CHECKED_AT.to_string(),
        &config,
        Some(config.model_id()),
    );
    scheduler.record_running_job(&runtime, LocalAiProviderSchedulerJobClass::ParentReport);

    let answer = generate_parent_assistant_answer_with_scheduler(
        &command(),
        request(Some(config.model_id().to_string())),
        &config,
        &scheduler,
    )
    .await;

    remove_temp_file(runtime_binary);
    remove_temp_file(model_file);

    assert_eq!(
        answer.provider_state,
        ParentAssistantProviderState::Degraded
    );
    assert_eq!(answer.answer_state, ParentAssistantAnswerState::Degraded);
    assert_eq!(
        answer.scheduler_job_status,
        LocalAiProviderSchedulerJobStatus::Queued
    );
    assert_eq!(answer.degraded_state, LocalAiDegradedState::Overloaded);
    assert_eq!(answer.local_ai_result_id, None);
    assert!(answer.action_preview.child_agent_contract_required);
    assert!(!answer.action_preview.enforcement_applied);
}

#[test]
fn parent_assistant_request_cites_activity_snapshot_when_prompt_has_no_summary() {
    let snapshot = ActivitySurfaceStoreSnapshot {
        device_id: constants::activity_surface::DEFAULT_DEVICE_ID.to_string(),
        recent_returned: 1,
        last_event_id: Some(constants::event_id::ACTIVITY_RECENT_SUMMARY_REPORTED.to_string()),
        last_observed_at: Some(constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string()),
        browser_returned: 0,
        network_returned: 0,
        games_returned: 0,
        screen_returned: 0,
    };

    let request = request_from_command(
        &command(),
        &LocalAiRuntimeConfigSnapshot::unconfigured(),
        Some(snapshot),
    );

    assert_eq!(
        request.evidence_context[0].evidence.evidence_reference_id,
        constants::event_id::ACTIVITY_RECENT_SUMMARY_REPORTED
    );
    assert_eq!(
        request.evidence_context[0].evidence.observed_at,
        constants::activity_store::TEST_SECOND_OBSERVED_AT
    );
    assert_eq!(
        request.evidence_context[0].allowed_summary,
        expected_activity_context_summary()
    );
    assert_eq!(
        request.evidence_context[1].evidence.kind,
        ParentEvidenceReferenceKind::ActivityEvent
    );
}

fn expected_activity_context_summary() -> String {
    let mut summary = constants::parent_assistant::ACTIVITY_CONTEXT_PREFIX.to_string();
    summary.push_str(constants::parent_assistant::ACTIVITY_CONTEXT_RECENT_LABEL);
    summary.push('1');
    summary.push_str(constants::parent_assistant::ACTIVITY_CONTEXT_SCREEN_LABEL);
    summary.push('0');
    summary.push_str(constants::parent_assistant::ACTIVITY_CONTEXT_BROWSER_LABEL);
    summary.push('0');
    summary.push_str(constants::parent_assistant::ACTIVITY_CONTEXT_GAMES_LABEL);
    summary.push('0');
    summary.push_str(constants::parent_assistant::ACTIVITY_CONTEXT_NETWORK_LABEL);
    summary.push('0');
    summary
}

fn request(model_id: Option<String>) -> ParentAssistantGenerateRequest {
    ParentAssistantGenerateRequest {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        request_id: constants::parent_assistant::DEFAULT_REQUEST_ID.to_string(),
        thread_id: constants::parent_assistant::DEFAULT_THREAD_ID.to_string(),
        message_id: constants::parent_assistant::DEFAULT_MESSAGE_ID.to_string(),
        asked_at: constants::local_ai_runtime::TEST_CHECKED_AT.to_string(),
        actor: ParentActorReference {
            actor_id: constants::parent_assistant::DEFAULT_PARENT_ACTOR_ID.to_string(),
            role: ParentActorRole::Parent,
        },
        scope: ParentAssistantScope {
            family: FamilyReference {
                family_id: constants::parent_assistant::DEFAULT_FAMILY_ID.to_string(),
            },
            device: None,
        },
        question: constants::parent_assistant::DEFAULT_QUESTION.to_string(),
        evidence_context: vec![ParentAssistantEvidenceContext {
            evidence: ParentEvidenceReference {
                evidence_reference_id: constants::field::ACTIVITY_DIGEST.to_string(),
                kind: ParentEvidenceReferenceKind::QueryStoreSummary,
                observed_at: constants::local_ai_runtime::TEST_CHECKED_AT.to_string(),
            },
            citation_label: constants::parent_assistant::DEFAULT_CITATION_LABEL.to_string(),
            allowed_summary: constants::parent_assistant::DEFAULT_ALLOWED_SUMMARY.to_string(),
        }],
        model_id,
        max_output_tokens: constants::local_ai_runtime::DEFAULT_GENERATION_MAX_TOKENS,
        timeout_ms: constants::local_ai_runtime::DEFAULT_GENERATION_TIMEOUT_MS,
    }
}

fn command() -> AgentCommandEnvelope {
    AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: constants::parent_assistant::DEFAULT_MESSAGE_ID.to_string(),
        sent_at: constants::local_ai_runtime::TEST_CHECKED_AT.to_string(),
        source: AgentPeer {
            peer_id: constants::peer::PORTAL_DEV.to_string(),
            role: AgentPeerRole::Portal,
        },
        target: AgentMessageTarget {
            device_id: constants::activity_surface::DEFAULT_DEVICE_ID.to_string(),
            platform: policy::TEST_PARENT_DEVICE_PLATFORM_WINDOWS.to_string(),
            route: AgentRoute::Localhost,
        },
        command: AgentCommandName::AgentParentAssistantAnswerGenerate,
        payload: fields_from_pairs(Vec::new()),
    }
}

fn write_temp_file(prefix: &str) -> PathBuf {
    let path = unique_temp_path(prefix);
    fs::write(&path, constants::local_ai_runtime::TEST_CHECKED_AT)
        .expect(constants::error::LOCAL_AI_RUNTIME_SPAWNS);
    path
}

fn unique_temp_path(prefix: &str) -> PathBuf {
    let mut name = prefix.to_string();
    name.push(constants::delimiter::HYPHEN);
    name.push_str(&std::process::id().to_string());
    name.push(constants::delimiter::HYPHEN);
    name.push_str(&nanos_now().to_string());
    let mut path = std::env::temp_dir();
    path.push(name);
    path
}

fn nanos_now() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect(constants::error::AGENT_EVENT_SERIALIZES)
        .as_nanos()
}

fn remove_temp_file(path: PathBuf) {
    let _ = fs::remove_file(path);
}
