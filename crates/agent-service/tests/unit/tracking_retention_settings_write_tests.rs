use std::{error::Error, io::Error as IoError};

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};
use ocentra_parent_agent_protocol::tracking::config_update_event::{
    TrackingConfigAuditOutcome, TrackingConfigPolicyDecisionState, TrackingConfigPortalUpdateKind,
};
use ocentra_parent_agent_protocol::tracking::retention_settings_write_command::{
    default_tracking_retention_settings_write_request, tracking_durable_settings_store_ref,
    tracking_local_service_state_snapshot_ref, tracking_retention_write_state_accepted,
    TrackingDurableSettingsPersistenceState, TrackingExecutionClaimState, TrackingRemoteAiState,
    TrackingRemoteSyncState, TrackingRetentionSettingsWriteResult,
};
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentCommandName, AgentEventName, AgentMessageTarget, AgentPeer,
    AgentPeerRole, AgentRoute,
};
use ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION;

use super::tracking_retention_settings_write::{
    build_tracking_retention_settings_write_report, execute_tracking_retention_settings_write_flow,
    TrackingWriteRequestParseState,
};
use crate::fields::fields_from_pairs;

type TestResult = Result<(), Box<dyn Error>>;

#[tokio::test]
async fn retention_settings_write_report_builder_emits_observability_payload() -> TestResult {
    let event = build_tracking_retention_settings_write_report(command_envelope()?).await;
    let write_result = write_result_payload(&crate::test_invariants::log_field(
        &event.payload,
        constants::field::ACTIVITY_TRACKING_RETENTION_SETTINGS_WRITE_RESULT,
        constants::error::AGENT_EVENT_SERIALIZES,
    ))?;

    assert_eq!(
        event.event,
        AgentEventName::AgentActivityTrackingRetentionSettingsWriteReported
    );
    assert_service_backed_write_result(&write_result);
    assert!(matches!(
        event
            .payload
            .get("trackingRetentionSettingsWriteFlowObservability"),
        Some(LogFieldValue::String(text)) if text.contains("parentCommandValidated")
    ));

    Ok(())
}

#[tokio::test]
async fn retention_settings_write_flow_builds_parent_child_policy_and_audit_chain() -> TestResult {
    let command = command_envelope()?;
    let request = default_tracking_retention_settings_write_request();
    let flow_report = execute_tracking_retention_settings_write_flow(
        &command,
        &request,
        TrackingWriteRequestParseState::Accepted,
    )
    .await;

    assert_eq!(
        flow_report.parent_action_received.action_kind,
        ocentra_parent_agent_protocol::parent_controller_events::ParentControllerActionKind::UpdateTrackingConfig
    );
    assert_eq!(
        flow_report
            .parent_command_validated
            .as_ref()
            .ok_or_else(|| IoError::other(constants::error::AGENT_EVENT_SERIALIZES))?
            .validation_state,
        ocentra_parent_agent_protocol::parent_controller_events::ParentCommandValidationState::Validated
    );
    assert_eq!(
        flow_report
            .change_requested
            .as_ref()
            .ok_or_else(|| IoError::other(constants::error::AGENT_EVENT_SERIALIZES))?
            .config
            .command_id,
        request.command_id.as_str()
    );
    assert_eq!(
        flow_report
            .policy_evaluation_requested
            .as_ref()
            .ok_or_else(|| IoError::other(constants::error::AGENT_EVENT_SERIALIZES))?
            .parent_rule_refs
            .len(),
        3
    );
    assert_eq!(
        flow_report
            .change_approved
            .as_ref()
            .ok_or_else(|| IoError::other(constants::error::AGENT_EVENT_SERIALIZES))?
            .source_command_id,
        request.command_id
    );
    assert!(flow_report.change_rejected.is_none());
    assert_eq!(
        flow_report
            .policy_decision_completed
            .as_ref()
            .ok_or_else(|| IoError::other(constants::error::AGENT_EVENT_SERIALIZES))?
            .decision_state,
        TrackingConfigPolicyDecisionState::Approved
    );
    assert_eq!(
        flow_report
            .child_command_forward_requested
            .as_ref()
            .ok_or_else(|| IoError::other(constants::error::AGENT_EVENT_SERIALIZES))?
            .transport_boundary,
        ocentra_parent_agent_protocol::parent_controller_events::ParentChildCommandTransportBoundary::TypedLocalServiceTransport
    );
    assert_eq!(
        flow_report
            .child_command_received
            .as_ref()
            .ok_or_else(|| IoError::other(constants::error::AGENT_EVENT_SERIALIZES))?
            .command_kind,
        ocentra_parent_agent_protocol::child_agent::child_agent_events::ChildCommandKind::ApplyTrackingConfig
    );
    assert_eq!(
        flow_report
            .audit_entry_committed
            .as_ref()
            .ok_or_else(|| IoError::other(constants::error::AGENT_EVENT_SERIALIZES))?
            .audit_outcome,
        TrackingConfigAuditOutcome::Committed
    );
    assert_eq!(
        flow_report
            .portal_read_model_updated
            .as_ref()
            .ok_or_else(|| IoError::other(constants::error::AGENT_EVENT_SERIALIZES))?
            .update_kind,
        TrackingConfigPortalUpdateKind::TrackingConfigState
    );
    let child_runtime_flow = flow_report
        .child_runtime_flow
        .clone()
        .ok_or_else(|| IoError::other(constants::error::AGENT_EVENT_SERIALIZES))?;
    assert_eq!(flow_report.child_runtime_flow, Some(child_runtime_flow));

    Ok(())
}

#[tokio::test]
async fn retention_settings_write_flow_rejects_before_child_runtime_when_request_is_missing(
) -> TestResult {
    let command = command_envelope()?;
    let request = default_tracking_retention_settings_write_request();
    let flow_report = execute_tracking_retention_settings_write_flow(
        &command,
        &request,
        TrackingWriteRequestParseState::Rejected,
    )
    .await;

    assert!(flow_report.parent_command_validated.is_none());
    assert_eq!(
        flow_report
            .parent_command_rejected
            .as_ref()
            .ok_or_else(|| IoError::other(constants::error::AGENT_EVENT_SERIALIZES))?
            .rejection_reason_code,
        constants::tracking_config_update::REJECTION_REASON_INVALID_REQUEST
    );
    assert!(flow_report.change_requested.is_none());
    assert!(flow_report.change_approved.is_none());
    assert!(flow_report.change_rejected.is_none());
    assert!(flow_report.child_runtime_flow.is_none());
    assert!(flow_report.audit_entry_committed.is_none());

    Ok(())
}

fn command_envelope() -> Result<AgentCommandEnvelope, serde_json::Error> {
    Ok(AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: constants::tracking_retention_settings_write::COMMAND_ID.to_string(),
        sent_at: constants::tracking_retention_settings_write::ACCEPTED_AT.to_string(),
        source: AgentPeer {
            peer_id: constants::peer::PORTAL_DEV.to_string(),
            role: AgentPeerRole::Portal,
        },
        target: AgentMessageTarget {
            device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            platform:
                ocentra_parent_agent_protocol::policy_constants::TEST_PARENT_DEVICE_PLATFORM_WINDOWS
                    .to_string(),
            route: AgentRoute::Localhost,
        },
        command: AgentCommandName::AgentActivityTrackingRetentionSettingsWrite,
        payload: write_request_payload()?,
    })
}

fn write_request_payload() -> Result<LogFields, serde_json::Error> {
    Ok(fields_from_pairs(vec![(
        constants::field::ACTIVITY_TRACKING_RETENTION_SETTINGS_WRITE_REQUEST,
        LogFieldValue::String(serde_json::to_string(
            &default_tracking_retention_settings_write_request(),
        )?),
    )]))
}

fn write_result_payload(
    value: &LogFieldValue,
) -> Result<TrackingRetentionSettingsWriteResult, Box<dyn Error>> {
    match value {
        LogFieldValue::String(text) => Ok(serde_json::from_str(text)?),
        _ => Err(Box::new(IoError::other(
            constants::error::AGENT_EVENT_SERIALIZES,
        ))),
    }
}

fn assert_service_backed_write_result(write_result: &TrackingRetentionSettingsWriteResult) {
    assert_eq!(
        write_result.settings_kind,
        constants::tracking_retention_settings_write::SETTINGS_KIND_RETENTION_WINDOW
    );
    assert_eq!(
        write_result.write_state,
        tracking_retention_write_state_accepted()
    );
    assert_eq!(
        write_result.command_transport_claim_state,
        TrackingExecutionClaimState::Claimed
    );
    assert_eq!(
        write_result.service_write_preflight_claim_state,
        TrackingExecutionClaimState::Claimed
    );
    assert_eq!(
        write_result.service_mutation_execution_state,
        TrackingExecutionClaimState::Claimed
    );
    assert_eq!(write_result.applied_retention_window_hours, Some(168));
    assert_eq!(
        write_result.remote_sync_state,
        TrackingRemoteSyncState::Disabled
    );
    assert_eq!(
        write_result.remote_ai_state,
        TrackingRemoteAiState::Disabled
    );
    assert!(write_result
        .local_service_state_revision
        .is_some_and(|revision| revision > 0));
    assert_eq!(
        write_result.local_service_state_snapshot_ref,
        tracking_local_service_state_snapshot_ref()
    );
    assert_eq!(
        write_result.durable_settings_store_ref,
        tracking_durable_settings_store_ref()
    );
    assert_eq!(
        write_result.durable_settings_persistence_state,
        TrackingDurableSettingsPersistenceState::Persisted
    );
}
