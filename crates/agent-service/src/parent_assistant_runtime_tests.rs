use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use ocentra_parent_agent_protocol::{
    constants, policy_constants as policy, ActivityReadModelState, ActivityReportCustodyLabel,
    ActivityReportDocument, ActivityReportFrequency, ActivityReportSection,
    ActivityReportSectionKind, ActivityReportSourceLabel, ActivityReportSourceReachabilityState,
    ActivityReportSourceState, ActivitySavedReportMetadata, ActivitySavedReportState,
    ActivitySurfaceScope, ActivitySurfaceScopeKind, AgentCommandEnvelope, AgentCommandName,
    AgentMessageTarget, AgentPeer, AgentPeerRole, AgentRoute, FamilyReference,
    LocalAiDegradedState, LocalAiProviderSchedulerJobClass, LocalAiProviderSchedulerJobStatus,
    ParentActorReference, ParentActorRole, ParentAssistantActionPreviewKind,
    ParentAssistantAnswerState, ParentAssistantApiAuthorizationState,
    ParentAssistantApiProviderAccessState, ParentAssistantEvidenceContext,
    ParentAssistantGenerateRequest, ParentAssistantProviderState, ParentAssistantRunState,
    ParentAssistantScope, ParentEvidenceReference, ParentEvidenceReferenceKind,
    ACTIVITY_SURFACE_SCHEMA_VERSION, AGENT_PROTOCOL_SCHEMA_VERSION,
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

mod activity_history_context_tests;

#[tokio::test]
async fn parent_assistant_unconfigured_provider_returns_cited_unavailable_answer() {
    let scheduler = LocalAiProviderSchedulerRuntime::new_for_test();
    let config = LocalAiRuntimeConfigSnapshot::unconfigured();

    let answer = generate_parent_assistant_answer_with_scheduler(
        &command_with_payload(Default::default()),
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
    assert_eq!(answer.run_state, ParentAssistantRunState::Unavailable);
    assert_eq!(
        answer.scheduler_job_status,
        LocalAiProviderSchedulerJobStatus::Unavailable
    );
    assert_eq!(answer.citations.len(), 1);
    assert!(answer.action_preview.child_agent_contract_required);
    assert!(!answer.action_preview.enforcement_applied);
    assert_eq!(
        answer.api_provider_boundary.authorization_state,
        ParentAssistantApiAuthorizationState::NotAuthorized
    );
    assert_eq!(
        answer.api_provider_boundary.access_state,
        ParentAssistantApiProviderAccessState::NotAuthorized
    );
    assert!(answer.api_provider_boundary.parent_authorization_required);
    assert!(answer.api_provider_boundary.evidence_citation_required);
    assert!(
        !answer
            .api_provider_boundary
            .child_safety_or_enforcement_use_allowed
    );
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
        &command_with_payload(Default::default()),
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
    assert_eq!(answer.answer_state, ParentAssistantAnswerState::Queued);
    assert_eq!(answer.run_state, ParentAssistantRunState::Queued);
    assert_eq!(
        answer.scheduler_job_status,
        LocalAiProviderSchedulerJobStatus::Queued
    );
    assert_eq!(answer.degraded_state, LocalAiDegradedState::Overloaded);
    assert_eq!(answer.local_ai_result_id, None);
    assert!(answer.action_preview.child_agent_contract_required);
    assert!(!answer.action_preview.enforcement_applied);
}

#[tokio::test]
async fn parent_assistant_request_prepares_policy_preview_without_enforcement_or_api_ai() {
    let request = request_from_command(
        &policy_question_command(),
        &LocalAiRuntimeConfigSnapshot::unconfigured(),
        None,
        None,
    );
    let answer = generate_parent_assistant_answer_with_scheduler(
        &policy_question_command(),
        request,
        &LocalAiRuntimeConfigSnapshot::unconfigured(),
        &LocalAiProviderSchedulerRuntime::new_for_test(),
    )
    .await;

    assert_eq!(
        answer.action_preview.action_kind,
        ParentAssistantActionPreviewKind::PolicySuggestion
    );
    assert!(answer.action_preview.requires_controller_lease);
    assert!(answer.action_preview.child_agent_contract_required);
    assert!(!answer.action_preview.enforcement_applied);
    assert_eq!(
        answer.api_provider_boundary.provider_state,
        ParentAssistantProviderState::Unavailable
    );
    assert_eq!(
        answer.api_provider_boundary.access_state,
        ParentAssistantApiProviderAccessState::NotAuthorized
    );
    assert!(answer.api_provider_boundary.parent_authorization_required);
    assert!(answer.api_provider_boundary.evidence_citation_required);
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
        &command_with_payload(Default::default()),
        &LocalAiRuntimeConfigSnapshot::unconfigured(),
        Some(snapshot),
        None,
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
    assert!(!request.evidence_context[0].raw_child_evidence_included);
    assert!(!request.evidence_context[0].direct_enforcement_allowed);
}

#[test]
fn parent_assistant_request_cites_activity_report_document_when_supplied() {
    let request = request_from_command(
        &report_context_command(),
        &LocalAiRuntimeConfigSnapshot::unconfigured(),
        None,
        None,
    );

    let report_context = request
        .evidence_context
        .iter()
        .find(|context| {
            context.citation_label == constants::parent_assistant::ACTIVITY_REPORT_CITATION_LABEL
        })
        .expect(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        report_context.evidence.evidence_reference_id,
        constants::activity_surface::REPORT_ID_DAILY
    );
    assert_eq!(
        report_context.evidence.kind,
        ParentEvidenceReferenceKind::QueryStoreSummary
    );
    let label = |prefix: &str, value: &str| {
        let mut fragment = prefix.to_string();
        fragment.push_str(value);
        fragment
    };
    let raw_child_evidence_flag = false.to_string();
    let expected_fragments = [
        constants::activity_surface::SAVED_STATE_SAVED.to_string(),
        label(
            constants::parent_assistant::ACTIVITY_REPORT_SUMMARY_FILE_LABEL,
            constants::activity_surface::REPORT_FILE_DAILY,
        ),
        label(
            constants::parent_assistant::ACTIVITY_REPORT_SUMMARY_SAVED_AT_LABEL,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        ),
        label(
            constants::parent_assistant::ACTIVITY_REPORT_SUMMARY_STORAGE_REASON_LABEL,
            constants::activity_surface::SUMMARY_STORAGE_SAVED,
        ),
        label(
            constants::parent_assistant::ACTIVITY_REPORT_SUMMARY_OFFLINE_SOURCE_IDS_LABEL,
            constants::activity_surface::FAMILY_SOURCE_OFFLINE_ID,
        ),
        label(
            constants::parent_assistant::ACTIVITY_REPORT_SUMMARY_UNAVAILABLE_SOURCE_IDS_LABEL,
            constants::activity_surface::FAMILY_SOURCE_ERROR_ID,
        ),
        label(
            constants::parent_assistant::ACTIVITY_REPORT_SUMMARY_CUSTODY_LABEL,
            constants::activity_surface::CUSTODY_PARENT_DEVICE_LOCAL_REPORT_JSON,
        ),
        label(
            constants::parent_assistant::ACTIVITY_REPORT_SUMMARY_SOURCE_DATA_LABEL,
            constants::activity_surface::SOURCE_SAVED_REPORT_JSON,
        ),
        label(
            constants::parent_assistant::ACTIVITY_REPORT_SUMMARY_RAW_CHILD_EVIDENCE_LABEL,
            &raw_child_evidence_flag,
        ),
    ];

    for fragment in expected_fragments {
        assert!(report_context.allowed_summary.contains(&fragment));
    }
    assert_eq!(
        report_context.custody_label,
        constants::parent_assistant::EVIDENCE_CUSTODY_ACTIVITY_REPORT
    );
    assert_eq!(
        report_context.source_label,
        constants::parent_assistant::EVIDENCE_SOURCE_SAVED_ACTIVITY_REPORT_HISTORY
    );
    assert!(!report_context.raw_child_evidence_included);
    assert!(!report_context.direct_enforcement_allowed);
}

#[test]
fn parent_assistant_request_preserves_thread_and_message_ids_from_command() {
    let request = request_from_command(
        &thread_message_command(),
        &LocalAiRuntimeConfigSnapshot::unconfigured(),
        None,
        None,
    );

    assert_eq!(
        request.thread_id,
        constants::parent_assistant::TEST_THREAD_ID
    );
    assert_eq!(
        request.message_id,
        constants::parent_assistant::TEST_MESSAGE_ID
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
            custody_label: constants::parent_assistant::EVIDENCE_CUSTODY_ACTIVITY_SUMMARY
                .to_string(),
            source_label: constants::parent_assistant::EVIDENCE_SOURCE_ACTIVITY_QUERY_STORE_SUMMARY
                .to_string(),
            raw_child_evidence_included: false,
            direct_enforcement_allowed: false,
        }],
        model_id,
        max_output_tokens: constants::local_ai_runtime::DEFAULT_GENERATION_MAX_TOKENS,
        timeout_ms: constants::local_ai_runtime::DEFAULT_GENERATION_TIMEOUT_MS,
    }
}

fn policy_question_command() -> AgentCommandEnvelope {
    command_with_payload(fields_from_pairs(vec![(
        constants::field::PARENT_ASSISTANT_QUESTION,
        ocentra_parent_agent_protocol::LogFieldValue::String(
            constants::parent_assistant::TEST_POLICY_QUESTION.to_string(),
        ),
    )]))
}

fn report_context_command() -> AgentCommandEnvelope {
    command_with_payload(fields_from_pairs(vec![(
        constants::field::ACTIVITY_REPORT_DOCUMENT,
        ocentra_parent_agent_protocol::LogFieldValue::String(
            serde_json::to_string(&saved_report_document())
                .expect(constants::error::AGENT_EVENT_SERIALIZES),
        ),
    )]))
}

fn thread_message_command() -> AgentCommandEnvelope {
    command_with_payload(fields_from_pairs(vec![
        (
            constants::parent_assistant::FIELD_THREAD_ID,
            ocentra_parent_agent_protocol::LogFieldValue::String(
                constants::parent_assistant::TEST_THREAD_ID.to_string(),
            ),
        ),
        (
            constants::parent_assistant::FIELD_MESSAGE_ID,
            ocentra_parent_agent_protocol::LogFieldValue::String(
                constants::parent_assistant::TEST_MESSAGE_ID.to_string(),
            ),
        ),
    ]))
}

fn saved_report_document() -> ActivityReportDocument {
    ActivityReportDocument {
        schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
        report_id: constants::activity_surface::REPORT_ID_DAILY.to_string(),
        frequency: ActivityReportFrequency::Daily,
        scope: ActivitySurfaceScope {
            scope_kind: ActivitySurfaceScopeKind::Family,
            family_id: Some(constants::activity_surface::DEFAULT_FAMILY_ID.to_string()),
            device_id: None,
        },
        requested_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        range_start: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        range_end: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        generated_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        saved_metadata: Some(ActivitySavedReportMetadata {
            report_id: constants::activity_surface::REPORT_ID_DAILY.to_string(),
            file_name: constants::activity_surface::REPORT_FILE_DAILY.to_string(),
            saved_state: ActivitySavedReportState::Saved,
            saved_at: Some(constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string()),
            storage_reason: Some(constants::activity_surface::SUMMARY_STORAGE_SAVED.to_string()),
            custody_label: ActivityReportCustodyLabel::ParentDeviceLocalReportJson,
            source_label: ActivityReportSourceLabel::SavedReportJson,
            raw_child_evidence_included: false,
        }),
        source_states: vec![
            report_source_state(
                constants::activity_surface::DEFAULT_DEVICE_ID,
                ActivityReportSourceReachabilityState::Reachable,
                ActivityReadModelState::Ready,
                constants::activity_surface::SUMMARY_FAMILY_LOCAL_SOURCE,
                Some(constants::activity_store::TEST_SECOND_OBSERVED_AT),
                ActivityReportSourceLabel::ActivityQueryStoreSummary,
            ),
            report_source_state(
                constants::activity_surface::FAMILY_SOURCE_OFFLINE_ID,
                ActivityReportSourceReachabilityState::Offline,
                ActivityReadModelState::Offline,
                constants::activity_surface::SUMMARY_FAMILY_SOURCE_UNREACHABLE,
                None,
                ActivityReportSourceLabel::FamilyFanoutSourceState,
            ),
            report_source_state(
                constants::activity_surface::FAMILY_SOURCE_STALE_ID,
                ActivityReportSourceReachabilityState::Unreachable,
                ActivityReadModelState::Stale,
                constants::activity_surface::SUMMARY_FAMILY_SOURCE_STALE,
                Some(constants::activity_store::TEST_FIRST_OBSERVED_AT),
                ActivityReportSourceLabel::FamilyFanoutSourceState,
            ),
            report_source_state(
                constants::activity_surface::FAMILY_SOURCE_ERROR_ID,
                ActivityReportSourceReachabilityState::Error,
                ActivityReadModelState::Unavailable,
                constants::activity_surface::SUMMARY_FAMILY_SOURCE_ERROR,
                None,
                ActivityReportSourceLabel::FamilyFanoutSourceState,
            ),
        ],
        sections: vec![ActivityReportSection {
            section_kind: ActivityReportSectionKind::Summary,
            title: constants::activity_surface::SECTION_SUMMARY.to_string(),
            state: ActivityReadModelState::Ready,
            summary: constants::activity_surface::SUMMARY_READY.to_string(),
            item_count: 1,
            evidence: Vec::new(),
        }],
    }
}

fn report_source_state(
    device_id: &str,
    reachability_state: ActivityReportSourceReachabilityState,
    state: ActivityReadModelState,
    reason: &str,
    last_updated_at: Option<&str>,
    source_label: ActivityReportSourceLabel,
) -> ActivityReportSourceState {
    ActivityReportSourceState {
        device_id: device_id.to_string(),
        reachability_state,
        state,
        reason: Some(reason.to_string()),
        last_updated_at: last_updated_at.map(str::to_string),
        custody_label: ActivityReportCustodyLabel::ChildDeviceLocalSummary,
        source_label,
        raw_child_evidence_included: false,
    }
}

fn command_with_payload(payload: ocentra_parent_agent_protocol::LogFields) -> AgentCommandEnvelope {
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
        payload,
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
