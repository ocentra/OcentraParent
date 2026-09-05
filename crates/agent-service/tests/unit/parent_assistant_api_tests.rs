use std::path::PathBuf as TestPathBuf;
use std::string::String as TestString;
use std::{
    fs,
    time::{SystemTime, UNIX_EPOCH},
};

use ocentra_parent_agent_protocol as parent_protocol;
use ocentra_parent_agent_protocol::activity::{
    policy::ParentEvidenceReference, policy::ParentEvidenceReferenceKind,
};
use ocentra_parent_agent_protocol::activity_surface::{
    ActivityReadModelState, ActivityReportCustodyLabel, ActivityReportDocument,
    ActivityReportFrequency, ActivityReportSection, ActivityReportSectionKind,
    ActivityReportSourceLabel, ActivityReportSourceReachabilityState, ActivityReportSourceState,
    ActivitySavedReportMetadata, ActivitySavedReportState, ActivitySurfaceScope,
    ActivitySurfaceScopeKind,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogLevel};
use ocentra_parent_agent_protocol::parent_assistant::{
    provider_route::ParentAssistantProviderRoutingState,
    provider_route::ParentAssistantProviderSelection, ParentAssistantActionConfirmResult,
    ParentAssistantActionConfirmState, ParentAssistantActionPreviewKind,
    ParentAssistantActionPreviewResult, ParentAssistantActionPreviewState,
    ParentAssistantApiAuthorizationState, ParentAssistantApiProviderAccessState,
    ParentAssistantBackendState, ParentAssistantEvidenceContext, ParentAssistantProviderState,
    ParentAssistantProviderStatus, ParentAssistantRunCancelResult, ParentAssistantRunCancelState,
    ParentAssistantRunState, ParentAssistantThreadResponse, ParentAssistantThreadState,
};
use ocentra_parent_agent_protocol::policy_constants;
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentCommandName, AgentEventName, AgentMessageTarget, AgentPeer,
    AgentPeerRole, AgentRoute,
};
use ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION;

use crate::{
    fields::fields_from_pairs,
    parent_assistant_api::{
        api_boundary, build_parent_assistant_scaffold_event,
        thread_store::{
            record_message_for_thread, thread_response_for_command_in_dir, ParentAssistantThreadId,
        },
    },
    test_require_json_decode::require_json_decode,
    test_require_log_string_field::require_log_string_field,
    test_require_ok::require_ok,
    test_require_some::require_some,
    test_serialize_json::serialize_test_json,
};

#[tokio::test]
async fn parent_assistant_thread_message_record_updates_the_durable_thread() {
    let _guard = crate::activity_report_env_lock::REPORT_ENV_LOCK
        .lock()
        .await;
    let root = unique_temp_dir();
    let directory = root.join(constants::parent_assistant::THREAD_STORAGE_DIR);
    let create = command(
        AgentCommandName::AgentParentAssistantThreadCreate,
        fields_from_pairs(vec![(
            constants::parent_assistant::FIELD_THREAD_ID,
            LogFieldValue::String(constants::parent_assistant::DEFAULT_THREAD_ID.to_string()),
        )]),
    );
    let _ = thread_response_for_command_in_dir(&create, &directory);
    std::env::set_var(constants::env_var::DEV_LOG_DIR, &root);
    record_message_for_thread(ParentAssistantThreadId(
        constants::parent_assistant::DEFAULT_THREAD_ID.to_string(),
    ));
    let response = thread_response_for_command_in_dir(
        &command(
            AgentCommandName::AgentParentAssistantThreadList,
            Default::default(),
        ),
        &directory,
    );
    std::env::remove_var(constants::env_var::DEV_LOG_DIR);
    let _ = fs::remove_dir_all(root);

    let thread = require_some(
        response.active_thread.as_ref(),
        constants::error::AGENT_EVENT_SERIALIZES,
    );
    assert_eq!(
        thread.thread_id,
        constants::parent_assistant::DEFAULT_THREAD_ID
    );
    assert_eq!(thread.message_count, 1);
}

#[test]
fn parent_assistant_thread_create_returns_durable_service_state() {
    let event = build_parent_assistant_scaffold_event(command(
        AgentCommandName::AgentParentAssistantThreadCreate,
        fields_from_pairs(vec![(
            constants::parent_assistant::FIELD_THREAD_ID,
            LogFieldValue::String(constants::parent_assistant::DEFAULT_THREAD_ID.to_string()),
        )]),
    ));
    let response = thread_response_payload(&crate::test_log_field::log_field(
        &event.payload,
        constants::parent_assistant::FIELD_THREAD_RESPONSE,
        constants::error::AGENT_EVENT_SERIALIZES,
    ));

    assert_eq!(
        event.event,
        AgentEventName::AgentParentAssistantThreadUpdated
    );
    assert_eq!(event.severity, LogLevel::Info);
    assert_eq!(
        response.backend_state,
        ParentAssistantBackendState::DurableLocal
    );
    assert_eq!(
        require_some(
            response.active_thread.as_ref(),
            constants::error::AGENT_EVENT_SERIALIZES,
        )
        .state,
        ParentAssistantThreadState::Open
    );
}

#[test]
fn parent_assistant_thread_list_reads_durable_local_store_after_create() {
    let directory = unique_temp_dir();
    let create = command(
        AgentCommandName::AgentParentAssistantThreadCreate,
        fields_from_pairs(vec![(
            constants::parent_assistant::FIELD_THREAD_ID,
            LogFieldValue::String(constants::parent_assistant::DEFAULT_THREAD_ID.to_string()),
        )]),
    );
    let list = command(
        AgentCommandName::AgentParentAssistantThreadList,
        Default::default(),
    );

    let created = thread_response_for_command_in_dir(&create, &directory);
    let listed = thread_response_for_command_in_dir(&list, &directory);

    let _ = fs::remove_dir_all(directory);

    assert_eq!(
        created.backend_state,
        ParentAssistantBackendState::DurableLocal
    );
    assert_eq!(
        listed.backend_state,
        ParentAssistantBackendState::DurableLocal
    );
    assert_eq!(listed.threads.len(), 1);
    assert_eq!(
        require_some(
            listed.active_thread.as_ref(),
            constants::error::AGENT_EVENT_SERIALIZES,
        )
        .thread_id,
        constants::parent_assistant::DEFAULT_THREAD_ID
    );
}

#[test]
fn parent_assistant_provider_status_reports_local_runtime_and_api_boundary() {
    let event = build_parent_assistant_scaffold_event(command(
        AgentCommandName::AgentParentAssistantProviderStatusGet,
        Default::default(),
    ));
    let status = provider_status_payload(&crate::test_log_field::log_field(
        &event.payload,
        constants::parent_assistant::FIELD_PROVIDER_STATUS,
        constants::error::AGENT_EVENT_SERIALIZES,
    ));

    assert_eq!(
        event.event,
        AgentEventName::AgentParentAssistantProviderDegraded
    );
    assert_eq!(
        status.backend_state,
        ParentAssistantBackendState::RuntimeBacked
    );
    assert_eq!(
        status.provider_state,
        ParentAssistantProviderState::Unavailable
    );
    assert_eq!(status.run_state, ParentAssistantRunState::Unavailable);
    assert_eq!(
        status.scheduler_status.singleton_scope,
        ocentra_parent_agent_protocol::local_ai_runtime::scheduler::LocalAiProviderSingletonScope::PhysicalDevice
    );
    assert_eq!(status.scheduler_status.queue.total(), 0);
    assert_eq!(status.queue_depth, 0);
    assert!(
        !status
            .api_provider_boundary
            .child_safety_or_enforcement_use_allowed
    );
    assert!(status.api_provider_boundary.parent_authorization_required);
    assert!(status.api_provider_boundary.evidence_citation_required);
    assert_eq!(
        status.provider_route.routing_state,
        ParentAssistantProviderRoutingState::NoProviderAvailable
    );
    assert_eq!(
        status.provider_route.selected_provider,
        ParentAssistantProviderSelection::None
    );
    assert_eq!(
        status.provider_route.api_access_state,
        ParentAssistantApiProviderAccessState::NotAuthorized
    );
    assert!(
        !status
            .provider_route
            .child_safety_or_enforcement_use_allowed
    );
}

#[test]
fn parent_assistant_api_boundary_requires_authorization_without_remote_adapter_claim() {
    assert!(!api_boundary::api_authorization_context_is_complete(
        &command(
            AgentCommandName::AgentParentAssistantProviderStatusGet,
            Default::default(),
        )
    ));

    let boundary = api_boundary::api_provider_boundary_for_access_state(
        &[evidence_context()],
        ParentAssistantApiProviderAccessState::AuthorizedUnavailable,
    );

    assert_eq!(
        boundary.authorization_state,
        ParentAssistantApiAuthorizationState::Authorized
    );
    assert_eq!(
        boundary.access_state,
        ParentAssistantApiProviderAccessState::AuthorizedUnavailable
    );
    assert_eq!(
        boundary.provider_state,
        ParentAssistantProviderState::Unavailable
    );
    assert_eq!(
        boundary.unavailable_reason.as_deref(),
        Some(constants::parent_assistant::API_PROVIDER_AUTHORIZED_UNAVAILABLE_REASON)
    );
    assert!(!boundary.child_safety_or_enforcement_use_allowed);
    assert!(boundary.parent_authorization_required);
    assert!(boundary.evidence_citation_required);
    assert_eq!(boundary.citations.len(), 1);

    let degraded_boundary = api_boundary::api_provider_boundary_for_access_state(
        &[evidence_context()],
        ParentAssistantApiProviderAccessState::AuthorizedDegraded,
    );

    assert_eq!(
        degraded_boundary.authorization_state,
        ParentAssistantApiAuthorizationState::Authorized
    );
    assert_eq!(
        degraded_boundary.access_state,
        ParentAssistantApiProviderAccessState::AuthorizedDegraded
    );
    assert_eq!(
        degraded_boundary.provider_state,
        ParentAssistantProviderState::Degraded
    );
    assert_eq!(
        degraded_boundary.unavailable_reason.as_deref(),
        Some(constants::parent_assistant::API_PROVIDER_AUTHORIZED_DEGRADED_REASON)
    );
    assert!(!degraded_boundary.child_safety_or_enforcement_use_allowed);
    assert_eq!(degraded_boundary.citations.len(), 1);

    assert_authorized_api_routes(&boundary, &degraded_boundary);
}

fn assert_authorized_api_routes(
    boundary: &ocentra_parent_agent_protocol::parent_assistant::ParentAssistantApiProviderBoundary,
    degraded_boundary: &ocentra_parent_agent_protocol::parent_assistant::ParentAssistantApiProviderBoundary,
) {
    let route = api_boundary::provider_route(ParentAssistantProviderState::Unavailable, boundary);
    let degraded_route =
        api_boundary::provider_route(ParentAssistantProviderState::Unavailable, degraded_boundary);
    assert_eq!(
        route.routing_state,
        ParentAssistantProviderRoutingState::ApiProviderAuthorizedUnavailable
    );
    assert_eq!(
        degraded_route.routing_state,
        ParentAssistantProviderRoutingState::ApiProviderAuthorizedDegraded
    );
    assert!(
        !route.child_safety_or_enforcement_use_allowed
            && !degraded_route.child_safety_or_enforcement_use_allowed
    );
}

#[test]
fn parent_assistant_api_boundary_denies_api_without_authorization() {
    let boundary = api_boundary::api_provider_boundary_for_access_state(
        &[evidence_context()],
        ParentAssistantApiProviderAccessState::NotAuthorized,
    );

    assert_eq!(
        boundary.authorization_state,
        ParentAssistantApiAuthorizationState::NotAuthorized
    );
    assert_eq!(
        boundary.access_state,
        ParentAssistantApiProviderAccessState::NotAuthorized
    );
    assert_eq!(
        boundary.provider_id,
        constants::parent_assistant::API_PROVIDER_ID_NOT_AUTHORIZED
    );
    assert_eq!(
        boundary.provider_state,
        ParentAssistantProviderState::Unavailable
    );
    assert_eq!(
        boundary.unavailable_reason.as_deref(),
        Some(constants::parent_assistant::API_PROVIDER_NOT_AUTHORIZED_REASON)
    );
    assert!(!boundary.child_safety_or_enforcement_use_allowed);
    assert!(boundary.parent_authorization_required);
    assert!(boundary.evidence_citation_required);
    assert_eq!(
        boundary.retention_state,
        constants::parent_assistant::API_PROVIDER_RETENTION_NO_AUTHORIZATION
    );
    assert_eq!(boundary.citations.len(), 1);
}

#[test]
fn parent_assistant_run_cancel_reports_not_running_without_process_kill_claim() {
    let event = build_parent_assistant_scaffold_event(command(
        AgentCommandName::AgentParentAssistantRunCancel,
        fields_from_pairs(vec![(
            constants::parent_assistant::FIELD_RUN_ID,
            LogFieldValue::String(constants::parent_assistant::DEFAULT_RUN_ID.to_string()),
        )]),
    ));
    let result = run_cancel_payload(&crate::test_log_field::log_field(
        &event.payload,
        constants::parent_assistant::FIELD_RUN_CANCEL_RESULT,
        constants::error::AGENT_EVENT_SERIALIZES,
    ));

    assert_eq!(
        event.event,
        AgentEventName::AgentParentAssistantErrorReported
    );
    assert_eq!(
        result.cancel_state,
        ParentAssistantRunCancelState::NotRunning
    );
    assert_eq!(result.run_state, ParentAssistantRunState::Completed);
    assert_eq!(
        result.unavailable_reason.as_deref(),
        Some(constants::parent_assistant::RUN_NOT_RUNNING_REASON)
    );
}

#[test]
fn parent_assistant_action_preview_returns_draft_without_policy_write_or_enforcement() {
    let event = build_parent_assistant_scaffold_event(command(
        AgentCommandName::AgentParentAssistantActionPreview,
        fields_from_pairs(vec![
            (
                constants::parent_assistant::FIELD_ACTION_INTENT_ID,
                LogFieldValue::String(
                    constants::parent_assistant::DEFAULT_ACTION_INTENT_ID.to_string(),
                ),
            ),
            (
                constants::field::PARENT_ASSISTANT_QUESTION,
                LogFieldValue::String(
                    constants::parent_assistant::TEST_POLICY_QUESTION.to_string(),
                ),
            ),
            (
                constants::field::ACTIVITY_REPORT_DOCUMENT,
                LogFieldValue::String(activity_report_document_json()),
            ),
        ]),
    ));
    let result = action_preview_payload(&crate::test_log_field::log_field(
        &event.payload,
        constants::field::PARENT_ASSISTANT_ACTION_PREVIEW,
        constants::error::AGENT_EVENT_SERIALIZES,
    ));

    assert_eq!(
        event.event,
        AgentEventName::AgentParentAssistantActionPreviewed
    );
    assert_eq!(
        result.preview_state,
        ParentAssistantActionPreviewState::Draft
    );
    assert_eq!(
        result.preview.action_kind,
        ParentAssistantActionPreviewKind::PolicySuggestion
    );
    assert!(result.requires_controller_lease);
    assert!(result.preview_required);
    assert!(result.preview_satisfied);
    assert!(!result.raw_assistant_prose_accepted);
    assert!(result.parent_confirmation_required);
    assert!(!result.parent_confirmation_recorded);
    assert_eq!(result.evidence_context.len(), 2);
    assert_eq!(result.source_refs.len(), 2);
    let report_context = require_some(
        result.evidence_context.iter().find(|context| {
            context.citation_label == constants::parent_assistant::ACTIVITY_REPORT_CITATION_LABEL
        }),
        constants::error::AGENT_EVENT_SERIALIZES,
    );
    assert_eq!(
        report_context.evidence.evidence_reference_id,
        constants::activity_surface::REPORT_ID_DAILY
    );
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
    assert!(report_context
        .allowed_summary
        .contains(constants::activity_surface::SAVED_STATE_SAVED));
    assert!(!result.enforcement_applied);
    assert!(!result.policy_written);
    assert!(!result.preview.enforcement_applied);
}

macro_rules! assert_rejected_action_confirm {
    ($result:expr, $reason:expr) => {{
        assert_eq!(
            $result.confirm_state,
            ParentAssistantActionConfirmState::Rejected
        );
        assert!(!$result.preview_satisfied);
        assert_eq!($result.reason, $reason);
        assert!(!$result.enforcement_applied);
        assert!(!$result.policy_written);
    }};
}

#[test]
fn parent_assistant_action_confirm_requires_child_contract_without_enforcement() {
    let intent = (
        constants::parent_assistant::FIELD_ACTION_INTENT_ID,
        LogFieldValue::String(constants::parent_assistant::DEFAULT_ACTION_INTENT_ID.to_string()),
    );
    let preview = (
        constants::field::PARENT_ASSISTANT_ACTION_PREVIEW_ID,
        LogFieldValue::String(constants::parent_assistant::DEFAULT_PREVIEW_ID.to_string()),
    );
    let event = build_parent_assistant_scaffold_event(command(
        AgentCommandName::AgentParentAssistantActionConfirm,
        fields_from_pairs(vec![intent.clone(), preview.clone()]),
    ));
    let result = action_confirm_payload(&crate::test_log_field::log_field(
        &event.payload,
        constants::parent_assistant::FIELD_ACTION_CONFIRM_RESULT,
        constants::error::AGENT_EVENT_SERIALIZES,
    ));

    assert_eq!(
        event.event,
        AgentEventName::AgentParentAssistantActionConfirmed
    );
    assert_eq!(
        result.backend_state,
        ParentAssistantBackendState::ContractRequired
    );
    assert_eq!(
        result.confirm_state,
        ParentAssistantActionConfirmState::ContractRequired
    );
    assert_eq!(
        result.preview_id.as_deref(),
        Some(constants::parent_assistant::DEFAULT_PREVIEW_ID)
    );
    assert!(result.preview_required);
    assert!(result.preview_satisfied);
    assert!(!result.raw_assistant_prose_accepted);
    assert!(result.parent_confirmation_required);
    assert!(!result.parent_confirmation_recorded);
    assert_eq!(result.source_refs.len(), 1);
    assert!(result.requires_controller_lease);
    assert!(result.child_agent_contract_required);
    assert!(!result.enforcement_applied);
    assert!(!result.policy_written);

    let missing_preview_event = build_parent_assistant_scaffold_event(command(
        AgentCommandName::AgentParentAssistantActionConfirm,
        fields_from_pairs(vec![intent.clone()]),
    ));
    let missing_preview = action_confirm_payload(&crate::test_log_field::log_field(
        &missing_preview_event.payload,
        constants::parent_assistant::FIELD_ACTION_CONFIRM_RESULT,
        constants::error::AGENT_EVENT_SERIALIZES,
    ));
    assert_rejected_action_confirm!(
        missing_preview,
        constants::parent_assistant::ACTION_CONFIRM_PREVIEW_REQUIRED_REASON
    );
    assert!(missing_preview.preview_required);

    let raw_event = build_parent_assistant_scaffold_event(command(
        AgentCommandName::AgentParentAssistantActionConfirm,
        fields_from_pairs(vec![
            intent,
            preview,
            (
                constants::field::PARENT_ASSISTANT_ACTION_RAW_PROSE,
                LogFieldValue::String(
                    constants::parent_assistant::TEST_POLICY_QUESTION.to_string(),
                ),
            ),
        ]),
    ));
    let raw = action_confirm_payload(&crate::test_log_field::log_field(
        &raw_event.payload,
        constants::parent_assistant::FIELD_ACTION_CONFIRM_RESULT,
        constants::error::AGENT_EVENT_SERIALIZES,
    ));
    assert_rejected_action_confirm!(
        raw,
        constants::parent_assistant::ACTION_CONFIRM_RAW_PROSE_REJECTED_REASON
    );
    assert!(!raw.raw_assistant_prose_accepted);
}

fn command(
    command_name: AgentCommandName,
    payload: ocentra_parent_agent_protocol::logging::LogFields,
) -> AgentCommandEnvelope {
    AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: constants::parent_assistant::DEFAULT_MESSAGE_ID.to_string(),
        sent_at: constants::local_ai_runtime::TEST_CHECKED_AT.to_string(),
        source: AgentPeer {
            peer_id: constants::peer::PORTAL_DEV.to_string(),
            role: AgentPeerRole::Portal,
        },
        target: AgentMessageTarget {
            device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            platform: policy_constants::TEST_PARENT_DEVICE_PLATFORM_WINDOWS.to_string(),
            route: AgentRoute::Localhost,
        },
        command: command_name,
        payload,
    }
}

fn thread_response_payload(value: &LogFieldValue) -> ParentAssistantThreadResponse {
    decode_payload(value)
}

fn provider_status_payload(value: &LogFieldValue) -> ParentAssistantProviderStatus {
    decode_payload(value)
}

fn run_cancel_payload(value: &LogFieldValue) -> ParentAssistantRunCancelResult {
    decode_payload(value)
}

fn action_confirm_payload(value: &LogFieldValue) -> ParentAssistantActionConfirmResult {
    decode_payload(value)
}

fn action_preview_payload(value: &LogFieldValue) -> ParentAssistantActionPreviewResult {
    decode_payload(value)
}

fn decode_payload<T>(value: &LogFieldValue) -> T
where
    T: serde::de::DeserializeOwned,
{
    require_json_decode(
        require_log_string_field(Some(value), constants::error::AGENT_EVENT_SERIALIZES),
        constants::error::AGENT_EVENT_SERIALIZES,
    )
}

fn evidence_context() -> ParentAssistantEvidenceContext {
    ParentAssistantEvidenceContext {
        evidence: ParentEvidenceReference {
            evidence_reference_id: constants::field::ACTIVITY_DIGEST.to_string(),
            kind: ParentEvidenceReferenceKind::QueryStoreSummary,
            observed_at: constants::local_ai_runtime::TEST_CHECKED_AT.to_string(),
        },
        citation_label: constants::parent_assistant::DEFAULT_CITATION_LABEL.to_string(),
        allowed_summary: constants::parent_assistant::DEFAULT_ALLOWED_SUMMARY.to_string(),
        custody_label: constants::parent_assistant::EVIDENCE_CUSTODY_ACTIVITY_SUMMARY.to_string(),
        source_label: constants::parent_assistant::EVIDENCE_SOURCE_ACTIVITY_QUERY_STORE_SUMMARY
            .to_string(),
        raw_child_evidence_included: false,
        direct_enforcement_allowed: false,
    }
}

fn activity_report_document_json() -> TestString {
    serialize_test_json(&ActivityReportDocument {
        schema_version: parent_protocol::ACTIVITY_SURFACE_SCHEMA_VERSION,
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
            ActivityReportSourceState {
                device_id: constants::activity_surface::DEFAULT_DEVICE_ID.to_string(),
                reachability_state: ActivityReportSourceReachabilityState::Reachable,
                state: ActivityReadModelState::Ready,
                reason: Some(constants::activity_surface::SUMMARY_FAMILY_LOCAL_SOURCE.to_string()),
                last_updated_at: Some(
                    constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
                ),
                custody_label: ActivityReportCustodyLabel::ChildDeviceLocalSummary,
                source_label: ActivityReportSourceLabel::ActivityQueryStoreSummary,
                raw_child_evidence_included: false,
            },
            ActivityReportSourceState {
                device_id: constants::activity_surface::FAMILY_SOURCE_OFFLINE_ID.to_string(),
                reachability_state: ActivityReportSourceReachabilityState::Offline,
                state: ActivityReadModelState::Offline,
                reason: Some(
                    constants::activity_surface::SUMMARY_FAMILY_SOURCE_UNREACHABLE.to_string(),
                ),
                last_updated_at: Some(
                    constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
                ),
                custody_label: ActivityReportCustodyLabel::ChildDeviceLocalSummary,
                source_label: ActivityReportSourceLabel::FamilyFanoutSourceState,
                raw_child_evidence_included: false,
            },
        ],
        sections: vec![ActivityReportSection {
            section_kind: ActivityReportSectionKind::Summary,
            title: constants::activity_surface::SECTION_SUMMARY.to_string(),
            state: ActivityReadModelState::Ready,
            summary: constants::activity_surface::SUMMARY_READY.to_string(),
            item_count: 1,
            evidence: Vec::new(),
        }],
    })
}

fn unique_temp_dir() -> TestPathBuf {
    let mut name = constants::parent_assistant::THREAD_STORAGE_DIR.to_string();
    name.push(constants::delimiter::HYPHEN);
    name.push_str(&std::process::id().to_string());
    name.push(constants::delimiter::HYPHEN);
    name.push_str(
        &require_ok(
            SystemTime::now().duration_since(UNIX_EPOCH),
            constants::error::AGENT_EVENT_SERIALIZES,
        )
        .as_nanos()
        .to_string(),
    );
    let mut path = std::env::temp_dir();
    path.push(name);
    path
}
