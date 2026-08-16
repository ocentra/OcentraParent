use std::path::PathBuf as TestPathBuf;
use std::primitive::str as TestStr;
use std::string::String as TestString;
use std::{error::Error, fs::remove_file, io::Error as IoError};

use ocentra_parent_agent_core::activity_store::ActivityStore;
use ocentra_parent_agent_protocol::activity::policy::PolicyTargetType;
use ocentra_parent_agent_protocol::activity::policy_preview::PolicyAssistantConfirmationState;
use ocentra_parent_agent_protocol::activity::policy_preview::PolicyRequestOrigin as ProtocolPolicyRequestOrigin;
use ocentra_parent_agent_protocol::activity::policy_preview::PolicyRequestStatus;
use ocentra_parent_agent_protocol::activity::policy_preview::PolicySourceStatus;
use ocentra_parent_agent_protocol::activity::policy_preview::PolicySourceSurface;
use ocentra_parent_agent_protocol::activity::{
    ActivityEvent, ActivityEventKind, ActivityEvidenceRef, ActivityObserver, ActivitySource,
    ActivitySubject, ActivitySubjectKind,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::logging::LogFields;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;
use ocentra_parent_agent_protocol::transport::AgentCommandName;
use ocentra_parent_agent_protocol::transport::AgentEventName;
use ocentra_parent_agent_protocol::transport::AgentMessageTarget;
use ocentra_parent_agent_protocol::transport::AgentPeer;
use ocentra_parent_agent_protocol::transport::AgentPeerRole;
use ocentra_parent_agent_protocol::transport::AgentRoute;
use ocentra_parent_agent_protocol::transport::PolicyRequestAssistantPreviewConfirmAction;
use ocentra_parent_agent_protocol::transport::PolicyRequestAssistantPreviewConfirmActorRole;
use ocentra_parent_agent_protocol::transport::PolicyRequestAssistantPreviewConfirmActorState;
use ocentra_parent_agent_protocol::transport::PolicyRequestAssistantPreviewConfirmClaimState;
use ocentra_parent_agent_protocol::transport::PolicyRequestAssistantPreviewConfirmRequest;
use ocentra_parent_agent_protocol::transport::PolicyRequestAssistantPreviewConfirmResult;
use ocentra_parent_agent_protocol::transport::PolicyRequestAssistantPreviewConfirmResultState;
use ocentra_parent_agent_protocol::transport::PolicyRequestAssistantPreviewConfirmTargetKind;
use ocentra_parent_agent_protocol::transport::PolicyRequestParentResolutionDecision;
use ocentra_parent_agent_protocol::transport::PolicyRequestParentResolutionDeliveryBinding;
use ocentra_parent_agent_protocol::transport::PolicyRequestParentResolutionRequest;
use ocentra_parent_agent_protocol::transport::PolicyRequestParentResolutionResult;
use ocentra_parent_agent_protocol::transport::PolicyRequestParentResolutionResultState;
use ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION;
use ocentra_policy_control_core::policy_request::ChildPolicyRequest;

use crate::activity_store_path::ActivityDbPath;
use crate::{
    activity_report_env_lock::REPORT_ENV_LOCK, fields::fields_from_pairs,
    policy_request_confirm::default_policy_request_assistant_preview_confirm_request,
};
use ocentra_parent_agent_service::test_support::handle_local_command_text_for_test;

type TestResult = Result<(), Box<dyn Error>>;

#[tokio::test]
async fn policy_request_resolution_persistence_rejects_missing_store_parent() -> TestResult {
    let path = ActivityDbPath(
        std::env::temp_dir()
            .join(format!(
                "ocentra-policy-resolution-missing-parent-{}",
                std::process::id()
            ))
            .join("activity.sqlite"),
    );
    let result =
        crate::policy_request_resolution_persistence::persist_activity_event(path, test_event())
            .await;
    assert_eq!(
        result.err(),
        Some(crate::policy_request_resolution_persistence::ActivityPersistenceError::Unavailable)
    );
    Ok(())
}

fn test_event() -> ActivityEvent {
    ActivityEvent {
        schema_version: ocentra_parent_agent_protocol::ACTIVITY_SCHEMA_VERSION,
        event_id: "audit.policy-request.test".to_string(),
        observed_at: "2026-06-18T00:10:00Z".to_string(),
        source: ActivitySource {
            device_id: "local-dev-agent".to_string(),
            platform: "windows".to_string(),
            observer: ActivityObserver::AgentService,
            source_id: "policy-request-parent-resolution".to_string(),
        },
        kind: ActivityEventKind::EnforcementAuditRecorded,
        subject: ActivitySubject {
            kind: ActivitySubjectKind::Device,
            subject_id: "child-profile-1".to_string(),
            display_name: None,
        },
        fields: LogFields::new(),
        evidence: Vec::<ActivityEvidenceRef>::new(),
    }
}

#[tokio::test]
async fn policy_request_assistant_preview_confirm_accepts_valid_parent_confirmation() -> TestResult
{
    let _guard = REPORT_ENV_LOCK.lock().await;
    let store_path = temp_path("policy-request-confirm");
    cleanup_path(&store_path);
    std::env::set_var(constants::env_var::ACTIVITY_DB_PATH, &store_path);

    let test_result: TestResult = async {
        let body = serde_json::to_string(&command_envelope(
            &default_policy_request_assistant_preview_confirm_request(),
        )?)?;
        let event =
            handle_local_command_text_for_test(crate::test_text::TestText::from_display(body))
                .await;
        let result = result_payload(&crate::test_invariants::log_field(
            &event.payload,
            constants::field::POLICY_REQUEST_ASSISTANT_PREVIEW_CONFIRM_RESULT,
            constants::error::AGENT_EVENT_SERIALIZES,
        ))?;
        let store = ActivityStore::open(&store_path).map_err(|error| {
            IoError::other(format!(
                "{}: {error:?}",
                constants::error::ACTIVITY_STORE_OPENS
            ))
        })?;
        let read_model = store
            .policy_preview_read_model(
                constants::activity_store::DEFAULT_RECENT_LIMIT,
                "2026-06-18T00:10:30Z",
            )
            .map_err(|error| {
                IoError::other(format!(
                    "{}: {error:?}",
                    constants::error::ACTIVITY_STORE_QUERIES
                ))
            })?;

        assert_eq!(
            event.event,
            AgentEventName::AgentPolicyRequestAssistantPreviewConfirmReported
        );
        assert_confirmed_preview_result(&result);
        assert_eq!(read_model.returned, 1);
        assert_eq!(
            read_model.rows[0].target.target_type,
            PolicyTargetType::Site
        );
        assert_eq!(read_model.rows[0].target.target_value, "example.test");
        assert_eq!(
            read_model.rows[0].policy_source_status,
            Some(PolicySourceStatus::Confirmed)
        );
        assert_eq!(
            read_model.rows[0].policy_source_surface,
            Some(PolicySourceSurface::AiPreview)
        );
        assert_eq!(
            read_model.rows[0].policy_request_origin,
            Some(ProtocolPolicyRequestOrigin::AssistantDraft)
        );
        assert_eq!(
            read_model.rows[0].policy_assistant_confirmation_state,
            Some(PolicyAssistantConfirmationState::ParentConfirmed)
        );
        assert_eq!(
            read_model.rows[0].policy_request_status,
            Some(PolicyRequestStatus::PendingParentReview)
        );
        assert_eq!(
            read_model.rows[0].policy_reviewed_by_actor_id.as_deref(),
            Some("parent-1")
        );
        assert_eq!(
            read_model.rows[0].policy_audit_reference_id.as_deref(),
            Some("audit.policy-request.confirmed")
        );
        let fields = store
            .enforcement_audit_fields_by_event_id("audit.policy-request.confirmed")
            .map_err(|error| {
                IoError::other(format!(
                    "{}: {error:?}",
                    constants::error::ACTIVITY_STORE_QUERIES
                ))
            })?
            .ok_or_else(|| IoError::other(constants::error::ACTIVITY_STORE_QUERIES))?;
        let canonical_request_json = fields
            .get(constants::policy_control::request::FIELD_CANONICAL_CONFIRMED_REQUEST_JSON)
            .and_then(|value| match value {
                LogFieldValue::String(value) => Some(value),
                _ => None,
            })
            .ok_or_else(|| IoError::other(constants::error::ACTIVITY_STORE_QUERIES))?;
        let canonical_request: ChildPolicyRequest = serde_json::from_str(canonical_request_json)?;
        assert_eq!(canonical_request.request_id.as_str(), "policy-request-1");
        assert_eq!(
            canonical_request.assistant_confirmation_state,
            PolicyAssistantConfirmationState::ParentConfirmed
        );

        Ok(())
    }
    .await;

    std::env::remove_var(constants::env_var::ACTIVITY_DB_PATH);
    cleanup_path(&store_path);
    test_result
}

#[tokio::test]
async fn policy_request_assistant_preview_confirm_leaves_unsupported_targets_unclaimed(
) -> TestResult {
    let _guard = REPORT_ENV_LOCK.lock().await;
    let store_path = temp_path("policy-request-confirm-unsupported");
    cleanup_path(&store_path);
    std::env::set_var(constants::env_var::ACTIVITY_DB_PATH, &store_path);

    let mut request = default_policy_request_assistant_preview_confirm_request();
    request.target_kind = PolicyRequestAssistantPreviewConfirmTargetKind::ChildProfile;
    request.target_reference_id = "child-profile-1".to_string();

    let test_result: TestResult = async {
        let body = serde_json::to_string(&command_envelope(&request)?)?;
        let event =
            handle_local_command_text_for_test(crate::test_text::TestText::from_display(body))
                .await;
        let result = result_payload(&crate::test_invariants::log_field(
            &event.payload,
            constants::field::POLICY_REQUEST_ASSISTANT_PREVIEW_CONFIRM_RESULT,
            constants::error::AGENT_EVENT_SERIALIZES,
        ))?;
        let store = ActivityStore::open(&store_path).map_err(|error| {
            IoError::other(format!(
                "{}: {error:?}",
                constants::error::ACTIVITY_STORE_OPENS
            ))
        })?;
        let read_model = store
            .policy_preview_read_model(
                constants::activity_store::DEFAULT_RECENT_LIMIT,
                "2026-06-18T00:10:30Z",
            )
            .map_err(|error| {
                IoError::other(format!(
                    "{}: {error:?}",
                    constants::error::ACTIVITY_STORE_QUERIES
                ))
            })?;

        assert_eq!(
            result.result_state,
            PolicyRequestAssistantPreviewConfirmResultState::Confirmed
        );
        assert_eq!(
            result.activity_store_mutation_claim_state,
            PolicyRequestAssistantPreviewConfirmClaimState::Unclaimed
        );
        assert_eq!(
            result.upstream_writer_claim_state,
            PolicyRequestAssistantPreviewConfirmClaimState::Unclaimed
        );
        assert_eq!(
            result.read_model_projection_claim_state,
            PolicyRequestAssistantPreviewConfirmClaimState::Unclaimed
        );
        assert_eq!(read_model.returned, 0);

        Ok(())
    }
    .await;

    std::env::remove_var(constants::env_var::ACTIVITY_DB_PATH);
    cleanup_path(&store_path);
    test_result
}

#[tokio::test]
async fn policy_request_assistant_preview_confirm_rejects_missing_typed_request_payload(
) -> TestResult {
    let body = serde_json::to_string(&command_envelope_without_request()?)?;
    let event =
        handle_local_command_text_for_test(crate::test_text::TestText::from_display(body)).await;
    let result = result_payload(&crate::test_invariants::log_field(
        &event.payload,
        constants::field::POLICY_REQUEST_ASSISTANT_PREVIEW_CONFIRM_RESULT,
        constants::error::AGENT_EVENT_SERIALIZES,
    ))?;

    assert_eq!(
        result.result_state,
        PolicyRequestAssistantPreviewConfirmResultState::Rejected
    );
    assert_eq!(
        result.policy_request_status,
        PolicyRequestStatus::PreviewOnly
    );
    assert_eq!(
        result.policy_assistant_confirmation_state,
        PolicyAssistantConfirmationState::ParentConfirmationRequired
    );
    assert_eq!(result.rejection_reason.as_deref(), Some("invalid-request"));

    Ok(())
}

#[tokio::test]
async fn policy_request_assistant_preview_confirm_rejects_invalid_parent_authority() -> TestResult {
    let mut request = default_policy_request_assistant_preview_confirm_request();
    request.confirmation_actor_role = PolicyRequestAssistantPreviewConfirmActorRole::Observer;
    let body = serde_json::to_string(&command_envelope(&request)?)?;
    let event =
        handle_local_command_text_for_test(crate::test_text::TestText::from_display(body)).await;
    let result = result_payload(&crate::test_invariants::log_field(
        &event.payload,
        constants::field::POLICY_REQUEST_ASSISTANT_PREVIEW_CONFIRM_RESULT,
        constants::error::AGENT_EVENT_SERIALIZES,
    ))?;

    assert_eq!(
        result.result_state,
        PolicyRequestAssistantPreviewConfirmResultState::Rejected
    );
    assert_eq!(
        result.rejection_reason.as_deref(),
        Some("invalid eventing value for policy_request.actor_role: observer")
    );

    Ok(())
}

#[tokio::test]
async fn policy_request_parent_resolution_reconstructs_confirmed_request_and_replays_safely(
) -> TestResult {
    let _guard = REPORT_ENV_LOCK.lock().await;
    let store_path = temp_path("policy-request-resolution");
    cleanup_path(&store_path);
    std::env::set_var(constants::env_var::ACTIVITY_DB_PATH, &store_path);

    let test_result: TestResult = async {
        let confirmation_body = serde_json::to_string(&command_envelope(
            &default_policy_request_assistant_preview_confirm_request(),
        )?)?;
        let _confirmation = handle_local_command_text_for_test(
            crate::test_text::TestText::from_display(confirmation_body),
        )
        .await;

        let resolution_request = default_parent_resolution_request();
        let resolution_body =
            serde_json::to_string(&parent_resolution_command_envelope(&resolution_request)?)?;
        let event = handle_local_command_text_for_test(crate::test_text::TestText::from_display(
            resolution_body,
        ))
        .await;
        let result = parent_resolution_result(&event)?;

        assert_eq!(
            event.event,
            AgentEventName::AgentPolicyRequestParentResolutionResolved
        );
        assert_eq!(
            result.result_state,
            PolicyRequestParentResolutionResultState::Resolved
        );
        assert_eq!(result.policy_request_status, PolicyRequestStatus::Approved);
        assert_eq!(result.request_id.as_deref(), Some("policy-request-1"));
        assert_eq!(
            result.temporary_override_id.as_deref(),
            Some("policy-override:approval-1")
        );
        assert_eq!(
            result.child_device_delivery_claim_state,
            PolicyRequestAssistantPreviewConfirmClaimState::Unclaimed
        );

        let store = ActivityStore::open(&store_path)
            .map_err(|error| IoError::other(format!("activity store opens: {error:?}")))?;
        let resolution_fields = store
            .enforcement_audit_fields_by_event_id("audit.policy-request.resolved")
            .map_err(|error| IoError::other(format!("activity fields query: {error:?}")))?
            .ok_or_else(|| IoError::other(constants::error::ACTIVITY_STORE_QUERIES))?;
        assert!(resolution_fields
            .get(constants::policy_control::request::FIELD_CANONICAL_RESOLVED_REQUEST_JSON)
            .is_some());

        let replay_body =
            serde_json::to_string(&parent_resolution_command_envelope(&resolution_request)?)?;
        let replay_event = handle_local_command_text_for_test(
            crate::test_text::TestText::from_display(replay_body),
        )
        .await;
        let replay_result = parent_resolution_result(&replay_event)?;
        assert_eq!(
            replay_result.result_state,
            PolicyRequestParentResolutionResultState::Resolved
        );
        assert_eq!(
            replay_result.rejection_reason.as_deref(),
            Some("replayed-resolution")
        );

        assert_resolution_replays_with_different_audit_reference(&resolution_request).await?;
        let second_audit_fields = store
            .enforcement_audit_fields_by_event_id("audit.policy-request.resolved-second")
            .map_err(|error| IoError::other(format!("activity fields query: {error:?}")))?;
        assert!(second_audit_fields.is_none());

        let mut mismatched_request = resolution_request.clone();
        crate::test_invariants::require_some(
            mismatched_request.delivery_binding.as_mut(),
            "delivery binding fixture",
        )
        .household_id = "another-household".to_string();
        let mismatched_event =
            handle_local_command_text_for_test(crate::test_text::TestText::from_display(
                serde_json::to_string(&parent_resolution_command_envelope(&mismatched_request)?)?,
            ))
            .await;
        let mismatched_result = parent_resolution_result(&mismatched_event)?;
        assert_eq!(
            mismatched_result.result_state,
            PolicyRequestParentResolutionResultState::Rejected
        );
        assert_eq!(
            mismatched_result.rejection_reason.as_deref(),
            Some("policy_delivery_binding.household_id: request-binding-mismatch")
        );

        Ok(())
    }
    .await;

    std::env::remove_var(constants::env_var::ACTIVITY_DB_PATH);
    cleanup_path(&store_path);
    test_result
}

async fn assert_resolution_replays_with_different_audit_reference(
    resolution_request: &PolicyRequestParentResolutionRequest,
) -> TestResult {
    let mut different_audit_request = resolution_request.clone();
    different_audit_request.approval_audit_reference_id =
        "audit.policy-request.resolved-second".to_string();
    let different_audit_event = handle_local_command_text_for_test(
        crate::test_text::TestText::from_display(serde_json::to_string(
            &parent_resolution_command_envelope(&different_audit_request)?,
        )?),
    )
    .await;
    let different_audit_result = parent_resolution_result(&different_audit_event)?;
    assert_eq!(
        different_audit_result.result_state,
        PolicyRequestParentResolutionResultState::Resolved
    );
    assert_eq!(
        different_audit_result.rejection_reason.as_deref(),
        Some("replayed-resolution")
    );
    Ok(())
}

#[tokio::test]
async fn policy_request_parent_resolution_persists_denial_audit_without_override() -> TestResult {
    let _guard = REPORT_ENV_LOCK.lock().await;
    let store_path = temp_path("policy-request-resolution-denied");
    cleanup_path(&store_path);
    std::env::set_var(constants::env_var::ACTIVITY_DB_PATH, &store_path);

    let test_result: TestResult = async {
        let confirmation_body = serde_json::to_string(&command_envelope(
            &default_policy_request_assistant_preview_confirm_request(),
        )?)?;
        let _confirmation = handle_local_command_text_for_test(
            crate::test_text::TestText::from_display(confirmation_body),
        )
        .await;

        let mut resolution_request = default_parent_resolution_request();
        resolution_request.command_id =
            "policy-request-parent-resolution-denied-command".to_string();
        resolution_request.approval_id = "approval-denied".to_string();
        resolution_request.decision = PolicyRequestParentResolutionDecision::Deny;
        resolution_request.approved_action = None;
        resolution_request.override_expires_at = None;
        resolution_request.approval_audit_reference_id = "audit.policy-request.denied".to_string();
        resolution_request.delivery_binding = None;

        let event = handle_local_command_text_for_test(crate::test_text::TestText::from_display(
            serde_json::to_string(&parent_resolution_command_envelope(&resolution_request)?)?,
        ))
        .await;
        let result = parent_resolution_result(&event)?;

        assert_eq!(
            event.event,
            AgentEventName::AgentPolicyRequestParentResolutionResolved
        );
        assert_eq!(
            result.result_state,
            PolicyRequestParentResolutionResultState::Resolved
        );
        assert_eq!(result.policy_request_status, PolicyRequestStatus::Denied);
        assert_eq!(result.request_id.as_deref(), Some("policy-request-1"));
        assert_eq!(
            result.resolved_approval_id.as_deref(),
            Some("approval-denied")
        );
        assert_eq!(result.temporary_override_id, None);

        let store = ActivityStore::open(&store_path)
            .map_err(|error| IoError::other(format!("activity store opens: {error:?}")))?;
        let resolution_fields = store
            .enforcement_audit_fields_by_event_id("audit.policy-request.denied")
            .map_err(|error| IoError::other(format!("activity fields query: {error:?}")))?
            .ok_or_else(|| IoError::other(constants::error::ACTIVITY_STORE_QUERIES))?;
        assert_eq!(
            resolution_fields.get(constants::field::POLICY_REQUEST_STATUS),
            Some(&LogFieldValue::String(
                constants::policy_control::request::STATUS_DENIED.to_string(),
            ))
        );
        assert!(resolution_fields
            .get(constants::policy_control::request::FIELD_CANONICAL_RESOLVED_REQUEST_JSON)
            .is_some());
        assert!(resolution_fields
            .get(constants::policy_control::request::FIELD_CANONICAL_TEMPORARY_OVERRIDE_JSON)
            .is_none());

        Ok(())
    }
    .await;

    std::env::remove_var(constants::env_var::ACTIVITY_DB_PATH);
    cleanup_path(&store_path);
    test_result
}

#[tokio::test]
async fn policy_request_parent_resolution_rejects_missing_confirmed_audit() -> TestResult {
    let _guard = REPORT_ENV_LOCK.lock().await;
    let store_path = temp_path("policy-request-resolution-missing");
    cleanup_path(&store_path);
    std::env::set_var(constants::env_var::ACTIVITY_DB_PATH, &store_path);
    let request = default_parent_resolution_request();
    let event = handle_local_command_text_for_test(crate::test_text::TestText::from_display(
        serde_json::to_string(&parent_resolution_command_envelope(&request)?)?,
    ))
    .await;
    let result = parent_resolution_result(&event)?;

    assert_eq!(
        result.result_state,
        PolicyRequestParentResolutionResultState::Rejected
    );
    assert_eq!(
        result.rejection_reason.as_deref(),
        Some("confirmed-request-not-found")
    );
    std::env::remove_var(constants::env_var::ACTIVITY_DB_PATH);
    cleanup_path(&store_path);
    Ok(())
}

fn command_envelope(
    request: &PolicyRequestAssistantPreviewConfirmRequest,
) -> Result<AgentCommandEnvelope, serde_json::Error> {
    Ok(AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: "cmd-policy-request-confirm-1".to_string(),
        sent_at: "2026-06-18T00:10:00Z".to_string(),
        source: AgentPeer {
            peer_id: constants::peer::PORTAL_DEV.to_string(),
            role: AgentPeerRole::Portal,
        },
        target: AgentMessageTarget {
            device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            platform: "windows".to_string(),
            route: AgentRoute::Localhost,
        },
        command: AgentCommandName::AgentPolicyRequestAssistantPreviewConfirm,
        payload: fields_from_pairs(vec![(
            constants::field::POLICY_REQUEST_ASSISTANT_PREVIEW_CONFIRM_REQUEST,
            LogFieldValue::String(serde_json::to_string(&request)?),
        )]),
    })
}

fn command_envelope_without_request() -> Result<AgentCommandEnvelope, serde_json::Error> {
    Ok(AgentCommandEnvelope {
        payload: LogFields::new(),
        ..command_envelope(&default_policy_request_assistant_preview_confirm_request())?
    })
}

fn default_parent_resolution_request() -> PolicyRequestParentResolutionRequest {
    PolicyRequestParentResolutionRequest {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        command_id: "policy-request-parent-resolution-command".to_string(),
        confirmed_audit_reference_id: "audit.policy-request.confirmed".to_string(),
        approval_id: "approval-1".to_string(),
        parent_actor_id: "parent-1".to_string(),
        parent_actor_role: PolicyRequestAssistantPreviewConfirmActorRole::Parent,
        parent_actor_state: PolicyRequestAssistantPreviewConfirmActorState::Active,
        decision: PolicyRequestParentResolutionDecision::Grant,
        approved_action: Some(PolicyRequestAssistantPreviewConfirmAction::TimeLimit),
        approved_bonus_minutes: None,
        override_expires_at: Some("2026-06-18T00:30:00Z".to_string()),
        decided_at: "2026-06-18T00:10:00Z".to_string(),
        approval_audit_reference_id: "audit.policy-request.resolved".to_string(),
        delivery_binding: Some(PolicyRequestParentResolutionDeliveryBinding {
            household_id: "family-local".to_string(),
            child_profile_id: "child-profile-1".to_string(),
            device_id: Some(constants::peer::LOCAL_DEV_AGENT.to_string()),
            source_document_id: "policy-document-1".to_string(),
            policy_version: 1,
        }),
    }
}

fn parent_resolution_command_envelope(
    request: &PolicyRequestParentResolutionRequest,
) -> Result<AgentCommandEnvelope, serde_json::Error> {
    Ok(AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: request.command_id.clone(),
        sent_at: "2026-06-18T00:10:00Z".to_string(),
        source: AgentPeer {
            peer_id: constants::peer::PORTAL_DEV.to_string(),
            role: AgentPeerRole::Portal,
        },
        target: AgentMessageTarget {
            device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            platform: "windows".to_string(),
            route: AgentRoute::Localhost,
        },
        command: AgentCommandName::AgentPolicyRequestParentResolutionResolve,
        payload: fields_from_pairs(vec![(
            constants::field::POLICY_REQUEST_PARENT_RESOLUTION_REQUEST,
            LogFieldValue::String(serde_json::to_string(request)?),
        )]),
    })
}

fn parent_resolution_result(
    event: &ocentra_parent_agent_protocol::transport::AgentEventEnvelope,
) -> Result<PolicyRequestParentResolutionResult, Box<dyn Error>> {
    match event
        .payload
        .get(constants::field::POLICY_REQUEST_PARENT_RESOLUTION_RESULT)
    {
        Some(LogFieldValue::String(text)) => Ok(serde_json::from_str(text)?),
        _ => Err(Box::new(IoError::other(
            constants::error::AGENT_EVENT_SERIALIZES,
        ))),
    }
}

fn result_payload(
    value: &LogFieldValue,
) -> Result<PolicyRequestAssistantPreviewConfirmResult, Box<dyn Error>> {
    match value {
        LogFieldValue::String(text) => Ok(serde_json::from_str(text)?),
        _ => Err(Box::new(IoError::other(
            constants::error::AGENT_EVENT_SERIALIZES,
        ))),
    }
}

fn temp_path(suffix: impl AsRef<TestStr>) -> TestPathBuf {
    let suffix = suffix.as_ref();
    let mut name = TestString::from(constants::activity_store::TEST_FILE_PREFIX);
    name.push_str(&std::process::id().to_string());
    name.push(constants::delimiter::HYPHEN);
    name.push_str(suffix);

    let mut path = std::env::temp_dir();
    path.push(name);
    path.set_extension(constants::activity_store::FILE_EXTENSION);
    path
}

fn cleanup_path(path: &TestPathBuf) {
    let _ = remove_file(path);
    let mut wal_path = path.clone();
    wal_path.set_extension(constants::activity_store::WAL_FILE_EXTENSION);
    let _ = remove_file(wal_path);
    let mut shm_path = path.clone();
    shm_path.set_extension(constants::activity_store::SHM_FILE_EXTENSION);
    let _ = remove_file(shm_path);
}

fn assert_confirmed_preview_result(result: &PolicyRequestAssistantPreviewConfirmResult) {
    assert_eq!(
        result.result_state,
        PolicyRequestAssistantPreviewConfirmResultState::Confirmed
    );
    assert_eq!(
        result.policy_request_status,
        PolicyRequestStatus::PendingParentReview
    );
    assert_eq!(
        result.policy_assistant_confirmation_state,
        PolicyAssistantConfirmationState::ParentConfirmed
    );
    assert_eq!(
        result.command_transport_claim_state,
        PolicyRequestAssistantPreviewConfirmClaimState::Claimed
    );
    assert_eq!(
        result.service_validation_claim_state,
        PolicyRequestAssistantPreviewConfirmClaimState::Claimed
    );
    assert_eq!(
        result.activity_store_mutation_claim_state,
        PolicyRequestAssistantPreviewConfirmClaimState::Claimed
    );
    assert_eq!(
        result.upstream_writer_claim_state,
        PolicyRequestAssistantPreviewConfirmClaimState::Claimed
    );
    assert_eq!(
        result.read_model_projection_claim_state,
        PolicyRequestAssistantPreviewConfirmClaimState::Claimed
    );
    assert_eq!(
        result.product_claim_state,
        PolicyRequestAssistantPreviewConfirmClaimState::Unclaimed
    );
    assert!(result.rejection_reason.as_deref().is_none());
}
