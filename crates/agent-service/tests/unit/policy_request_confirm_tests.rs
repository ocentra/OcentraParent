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
use ocentra_parent_agent_protocol::transport::PolicyRequestAssistantPreviewConfirmActorRole;
use ocentra_parent_agent_protocol::transport::PolicyRequestAssistantPreviewConfirmClaimState;
use ocentra_parent_agent_protocol::transport::PolicyRequestAssistantPreviewConfirmRequest;
use ocentra_parent_agent_protocol::transport::PolicyRequestAssistantPreviewConfirmResult;
use ocentra_parent_agent_protocol::transport::PolicyRequestAssistantPreviewConfirmResultState;
use ocentra_parent_agent_protocol::transport::PolicyRequestAssistantPreviewConfirmTargetKind;
use ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION;
use ocentra_policy_control_core::policy_request::ChildPolicyRequest;

use crate::{
    activity_report_env_lock::REPORT_ENV_LOCK, fields::fields_from_pairs,
    policy_request_confirm::default_policy_request_assistant_preview_confirm_request,
};
use ocentra_parent_agent_service::test_support::handle_local_command_text_for_test;

type TestResult = Result<(), Box<dyn Error>>;

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
