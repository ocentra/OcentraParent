use std::{error::Error, io::Error as IoError};

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};
use ocentra_parent_agent_protocol::tracking::config_update_event::{
    TrackingConfigAuditOutcome, TrackingConfigPolicyDecisionState, TrackingConfigPortalUpdateKind,
    TrackingConfigUpdateResponseState,
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

fn required_flow_value<T>(value: Option<&T>) -> Result<&T, IoError> {
    value.ok_or_else(|| IoError::other(constants::error::AGENT_EVENT_SERIALIZES))
}

#[tokio::test]
async fn retention_settings_write_report_builder_emits_completed_service_result() -> TestResult {
    let event = build_tracking_retention_settings_write_report(command_envelope()?).await;
    let write_result = tracking_write_result_payload(&event.payload)?;
    let observability = tracking_flow_observability(&event.payload)?;

    assert_eq!(
        event.event,
        AgentEventName::AgentActivityTrackingRetentionSettingsWriteReported
    );
    assert_service_backed_write_result(&write_result);
    assert_completed_flow_observability(&observability);

    Ok(())
}

fn assert_completed_flow_observability(observability: &serde_json::Value) {
    assert_eq!(
        observability
            .get(constants::tracking_retention_settings_write::FLOW_PARENT_COMMAND_VALIDATED),
        Some(&serde_json::Value::Bool(true))
    );
    assert_eq!(
        observability.get(constants::tracking_retention_settings_write::FLOW_CHANGE_REQUESTED),
        Some(&serde_json::Value::Bool(true))
    );
    assert_eq!(
        observability
            .get(constants::tracking_retention_settings_write::FLOW_POLICY_EVALUATION_REQUESTED),
        Some(&serde_json::Value::Bool(true))
    );
    assert_eq!(
        observability
            .get(constants::tracking_retention_settings_write::FLOW_POLICY_DECISION_COMPLETED),
        Some(&serde_json::Value::Bool(true))
    );
    assert_eq!(
        observability.get(constants::tracking_retention_settings_write::FLOW_CHANGE_APPROVED),
        Some(&serde_json::Value::Bool(true))
    );
    assert_eq!(
        observability.get(constants::tracking_retention_settings_write::FLOW_CHANGE_REJECTED),
        Some(&serde_json::Value::Bool(false))
    );
    assert_eq!(
        observability.get(
            constants::tracking_retention_settings_write::FLOW_CHILD_COMMAND_FORWARD_REQUESTED
        ),
        Some(&serde_json::Value::Bool(true))
    );
    assert_eq!(
        observability
            .get(constants::tracking_retention_settings_write::FLOW_CHILD_COMMAND_RECEIVED),
        Some(&serde_json::Value::Bool(true))
    );
    assert_eq!(
        observability.get(constants::tracking_retention_settings_write::FLOW_CHILD_RUNTIME_FLOW),
        Some(&serde_json::Value::Bool(true))
    );
    assert_eq!(
        observability.get(constants::tracking_retention_settings_write::FLOW_AUDIT_ENTRY_COMMITTED),
        Some(&serde_json::Value::Bool(true))
    );
    assert_eq!(
        observability
            .get(constants::tracking_retention_settings_write::FLOW_PORTAL_READ_MODEL_UPDATED),
        Some(&serde_json::Value::Bool(true))
    );
    assert_eq!(
        observability.get(constants::tracking_retention_settings_write::FLOW_PARENT_RUNTIME_STATE),
        Some(&serde_json::Value::String(
            constants::tracking_retention_settings_write::FLOW_PARENT_RUNTIME_COMPLETED.to_string()
        ))
    );
    assert_eq!(
        observability.get(constants::tracking_retention_settings_write::FLOW_PARENT_RUNTIME_ERROR),
        Some(&serde_json::Value::Null)
    );
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
        required_flow_value(flow_report.parent_command_validated.as_ref())?.validation_state,
        ocentra_parent_agent_protocol::parent_controller_events::ParentCommandValidationState::Validated
    );
    assert_eq!(
        required_flow_value(flow_report.change_requested.as_ref())?
            .config
            .command_id,
        request.command_id.as_str()
    );
    assert_eq!(
        required_flow_value(flow_report.policy_evaluation_requested.as_ref())?
            .parent_rule_refs
            .len(),
        3
    );
    assert_eq!(
        required_flow_value(flow_report.policy_decision_completed.as_ref())?.decision_state,
        TrackingConfigPolicyDecisionState::Approved
    );
    assert_eq!(
        required_flow_value(flow_report.change_approved.as_ref())?.source_command_id,
        request.command_id
    );
    assert!(flow_report.change_rejected.is_none());
    assert_eq!(
        required_flow_value(flow_report.child_command_forward_requested.as_ref())?
            .transport_boundary,
        ocentra_parent_agent_protocol::parent_controller_events::ParentChildCommandTransportBoundary::TypedLocalServiceTransport
    );
    assert_eq!(
        required_flow_value(flow_report.child_command_received.as_ref())?.command_kind,
        ocentra_parent_agent_protocol::child_agent::child_agent_events::ChildCommandKind::ApplyTrackingConfig
    );
    assert_eq!(
        required_flow_value(flow_report.audit_entry_committed.as_ref())?.audit_outcome,
        TrackingConfigAuditOutcome::Committed
    );
    assert_eq!(
        required_flow_value(flow_report.portal_read_model_updated.as_ref())?.update_kind,
        TrackingConfigPortalUpdateKind::TrackingConfigState
    );
    let child_runtime_flow = required_flow_value(flow_report.child_runtime_flow.as_ref())?;
    assert_eq!(
        child_runtime_flow
            .parent_request_report
            .response
            .response_state,
        TrackingConfigUpdateResponseState::Applied
    );
    assert_eq!(
        child_runtime_flow
            .applied_report
            .applied_state
            .durable_settings_persistence_state,
        TrackingDurableSettingsPersistenceState::Persisted
    );
    assert_eq!(flow_report.parent_runtime_flow_error, None);

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

fn tracking_flow_observability(fields: &LogFields) -> Result<serde_json::Value, Box<dyn Error>> {
    match fields.get(constants::tracking_retention_settings_write::FLOW_OBSERVABILITY_FIELD) {
        Some(LogFieldValue::String(text)) => Ok(serde_json::from_str(text)?),
        _ => Err(Box::new(IoError::other(
            constants::error::AGENT_EVENT_SERIALIZES,
        ))),
    }
}

fn tracking_write_result_payload(
    fields: &LogFields,
) -> Result<TrackingRetentionSettingsWriteResult, Box<dyn Error>> {
    match fields.get(constants::field::ACTIVITY_TRACKING_RETENTION_SETTINGS_WRITE_RESULT) {
        Some(LogFieldValue::String(text)) => Ok(serde_json::from_str(text)?),
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
    assert_eq!(
        write_result
            .local_service_state_revision
            .map(|revision| revision > 0),
        Some(true)
    );
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
