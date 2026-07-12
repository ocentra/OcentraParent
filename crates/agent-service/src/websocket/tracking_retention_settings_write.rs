use ocentra_child_runtime::tracking_config_update_flow::TrackingConfigUpdateEventFlowReport;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::logging::LogLevel;
use ocentra_parent_agent_protocol::parent_controller_events::ParentActionReceivedEvent;
use ocentra_parent_agent_protocol::tracking::config_update_event::parent_tracking_config_updated_event_from_command;
use ocentra_parent_agent_protocol::tracking::config_update_event::TrackingConfigUpdateRequest;
use ocentra_parent_agent_protocol::tracking::identifiers::TrackingRetentionWriteState;
use ocentra_parent_agent_protocol::tracking::retention_settings_write_command::default_tracking_retention_settings_write_request;
use ocentra_parent_agent_protocol::tracking::retention_settings_write_command::tracking_durable_settings_store_ref;
use ocentra_parent_agent_protocol::tracking::retention_settings_write_command::tracking_local_service_state_snapshot_ref;
use ocentra_parent_agent_protocol::tracking::retention_settings_write_command::tracking_mutation_proof_ref;
use ocentra_parent_agent_protocol::tracking::retention_settings_write_command::tracking_retention_accepted_at;
use ocentra_parent_agent_protocol::tracking::retention_settings_write_command::tracking_retention_write_state_accepted;
use ocentra_parent_agent_protocol::tracking::retention_settings_write_command::tracking_retention_write_state_rejected;
use ocentra_parent_agent_protocol::tracking::retention_settings_write_command::TrackingConfigAckState;
use ocentra_parent_agent_protocol::tracking::retention_settings_write_command::TrackingDurableSettingsPersistenceState;
use ocentra_parent_agent_protocol::tracking::retention_settings_write_command::TrackingExecutionClaimState;
use ocentra_parent_agent_protocol::tracking::retention_settings_write_command::TrackingRemoteAiState;
use ocentra_parent_agent_protocol::tracking::retention_settings_write_command::TrackingRemoteSyncState;
use ocentra_parent_agent_protocol::tracking::retention_settings_write_command::TrackingRetentionSettingsWriteRequest;
use ocentra_parent_agent_protocol::tracking::retention_settings_write_command::TrackingRetentionSettingsWriteResult;
use ocentra_parent_agent_protocol::tracking::runtime_event::default_tracking_runtime_config;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventName;
use ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION;
use ocentra_parent_runtime_core::tracking_config_update_flow::publish_parent_tracking_config_updated_event_flow;
use ocentra_parent_runtime_core::tracking_dispatch::{
    ChildAcknowledgementState, ParentRuntimeOriginState,
};

use crate::{event_builder::build_event, fields::fields_from_pairs};

use ocentra_parent_agent_protocol::child_agent::child_agent_events::ChildCommandReceivedEvent;
use ocentra_parent_agent_protocol::parent_controller_events::{
    ParentChildCommandForwardRequestedEvent, ParentCommandRejectedEvent,
    ParentCommandValidatedEvent,
};
use ocentra_parent_agent_protocol::tracking::config_update_event::{
    TrackingConfigAuditEntryCommittedEvent, TrackingConfigChangeApprovedEvent,
    TrackingConfigChangeRejectedEvent, TrackingConfigChangeRequestedEvent,
    TrackingConfigPolicyDecisionCompletedEvent, TrackingConfigPolicyEvaluationRequestedEvent,
    TrackingConfigPortalReadModelUpdatedEvent,
};

#[path = "tracking_retention_settings_write_events.rs"]
mod tracking_events;

use self::tracking_events::{
    tracking_child_command_received_event, tracking_parent_action_received_event,
    tracking_parent_child_command_forward_requested_event, tracking_parent_command_rejected_event,
    tracking_parent_command_validated_event,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum TrackingWriteRequestParseState {
    Accepted,
    Rejected,
}

#[derive(Clone, Debug)]
pub(crate) struct TrackingRetentionSettingsWriteFlowReport {
    pub(crate) parent_action_received: ParentActionReceivedEvent,
    pub(crate) parent_command_validated: Option<ParentCommandValidatedEvent>,
    pub(crate) parent_command_rejected: Option<ParentCommandRejectedEvent>,
    pub(crate) change_requested: Option<TrackingConfigChangeRequestedEvent>,
    pub(crate) policy_evaluation_requested: Option<TrackingConfigPolicyEvaluationRequestedEvent>,
    pub(crate) policy_decision_completed: Option<TrackingConfigPolicyDecisionCompletedEvent>,
    pub(crate) change_approved: Option<TrackingConfigChangeApprovedEvent>,
    pub(crate) change_rejected: Option<TrackingConfigChangeRejectedEvent>,
    pub(crate) child_command_forward_requested: Option<ParentChildCommandForwardRequestedEvent>,
    pub(crate) child_command_received: Option<ChildCommandReceivedEvent>,
    pub(crate) child_runtime_flow: Option<TrackingConfigUpdateEventFlowReport>,
    pub(crate) audit_entry_committed: Option<TrackingConfigAuditEntryCommittedEvent>,
    pub(crate) portal_read_model_updated: Option<TrackingConfigPortalReadModelUpdatedEvent>,
}

pub(crate) async fn build_tracking_retention_settings_write_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let (request, parse_state) = parse_write_request(&command);
    let flow_report =
        execute_tracking_retention_settings_write_flow(&command, &request, parse_state).await;
    let result = build_tracking_retention_settings_write_result(request, parse_state, &flow_report);
    let result_text = serde_json::to_string(&result).unwrap_or_default();

    let flow_observability_text =
        tracking_retention_settings_write_flow_observability(&flow_report).to_string();

    build_event(
        constants::tracking_retention_settings_write::EVENT_ID,
        &command.message_id,
        command.source,
        AgentEventName::AgentActivityTrackingRetentionSettingsWriteReported,
        LogLevel::Info,
        fields_from_pairs(vec![
            (
                constants::field::ACTIVITY_TRACKING_RETENTION_SETTINGS_WRITE_RESULT,
                LogFieldValue::String(result_text),
            ),
            (
                constants::tracking_retention_settings_write::FLOW_OBSERVABILITY_FIELD,
                LogFieldValue::String(flow_observability_text),
            ),
        ]),
        None,
    )
}

fn build_tracking_retention_settings_write_result(
    request: TrackingRetentionSettingsWriteRequest,
    parse_state: TrackingWriteRequestParseState,
    flow_report: &TrackingRetentionSettingsWriteFlowReport,
) -> TrackingRetentionSettingsWriteResult {
    let applied_report = flow_report
        .child_runtime_flow
        .as_ref()
        .map(|report| &report.applied_report);
    let child_response = flow_report
        .child_runtime_flow
        .as_ref()
        .map(|report| &report.parent_request_report.response);

    TrackingRetentionSettingsWriteResult {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        command_id: request.command_id,
        settings_kind: request.settings_kind,
        write_state: write_state(parse_state),
        accepted_at: tracking_retention_accepted_at(),
        source_writer_intent_refs: request.source_writer_intent_refs,
        source_read_model_proof_refs: request.source_read_model_proof_refs,
        source_mutation_proof_refs: vec![tracking_mutation_proof_ref()],
        applied_retention_window_hours: request.requested_retention_window_hours,
        applied_delete_after_alert_resolution_state: request
            .requested_delete_after_alert_resolution_state,
        parent_export_state: request.requested_parent_export_state,
        remote_sync_state: TrackingRemoteSyncState::Disabled,
        remote_ai_state: TrackingRemoteAiState::Disabled,
        local_service_state_revision: applied_report
            .as_ref()
            .map(|report| report.applied_state.local_service_state_revision),
        local_service_state_snapshot_ref: tracking_local_service_state_snapshot_ref(),
        durable_settings_store_ref: tracking_durable_settings_store_ref(),
        durable_settings_persistence_state: applied_report
            .as_ref()
            .map(|report| report.applied_state.durable_settings_persistence_state)
            .unwrap_or(TrackingDurableSettingsPersistenceState::NotPersisted),
        child_config_response_state: child_response.map(|response| response.response_state.clone()),
        effective_tracking_state: child_response
            .map(|response| response.effective_tracking_state.clone()),
        child_config_ack_state: if child_response.is_some() {
            TrackingConfigAckState::Received
        } else {
            TrackingConfigAckState::Missing
        },
        command_transport_claim_state: TrackingExecutionClaimState::Claimed,
        service_write_preflight_claim_state: TrackingExecutionClaimState::Claimed,
        service_mutation_execution_state: if applied_report.is_some() {
            TrackingExecutionClaimState::Claimed
        } else {
            TrackingExecutionClaimState::Unclaimed
        },
        portal_writable_ui_claim_state: TrackingExecutionClaimState::Unclaimed,
        platform_runtime_claim_state: TrackingExecutionClaimState::Unclaimed,
        child_device_delivery_claim_state: TrackingExecutionClaimState::Unclaimed,
        provider_delivery_claim_state: TrackingExecutionClaimState::Unclaimed,
        notification_receipt_claim_state: TrackingExecutionClaimState::Unclaimed,
        physical_device_claim_state: TrackingExecutionClaimState::Unclaimed,
        authority_claim_state: TrackingExecutionClaimState::Unclaimed,
        product_claim_state: TrackingExecutionClaimState::Unclaimed,
    }
}

fn tracking_retention_settings_write_flow_observability(
    flow_report: &TrackingRetentionSettingsWriteFlowReport,
) -> serde_json::Value {
    let mut observability = serde_json::Map::new();
    observability.insert(
        constants::tracking_retention_settings_write::FLOW_PARENT_ACTION_EVENT_REF.to_string(),
        serde_json::Value::String(
            flow_report
                .parent_action_received
                .parent_action_event_ref
                .clone(),
        ),
    );
    observability.insert(
        constants::tracking_retention_settings_write::FLOW_PARENT_COMMAND_VALIDATED.to_string(),
        serde_json::Value::Bool(flow_report.parent_command_validated.is_some()),
    );
    observability.insert(
        constants::tracking_retention_settings_write::FLOW_PARENT_COMMAND_REJECTED.to_string(),
        serde_json::Value::Bool(flow_report.parent_command_rejected.is_some()),
    );
    observability.insert(
        constants::tracking_retention_settings_write::FLOW_CHANGE_REQUESTED.to_string(),
        serde_json::Value::Bool(flow_report.change_requested.is_some()),
    );
    observability.insert(
        constants::tracking_retention_settings_write::FLOW_POLICY_EVALUATION_REQUESTED.to_string(),
        serde_json::Value::Bool(flow_report.policy_evaluation_requested.is_some()),
    );
    observability.insert(
        constants::tracking_retention_settings_write::FLOW_POLICY_DECISION_COMPLETED.to_string(),
        serde_json::Value::Bool(flow_report.policy_decision_completed.is_some()),
    );
    observability.insert(
        constants::tracking_retention_settings_write::FLOW_CHANGE_APPROVED.to_string(),
        serde_json::Value::Bool(flow_report.change_approved.is_some()),
    );
    observability.insert(
        constants::tracking_retention_settings_write::FLOW_CHANGE_REJECTED.to_string(),
        serde_json::Value::Bool(flow_report.change_rejected.is_some()),
    );
    observability.insert(
        constants::tracking_retention_settings_write::FLOW_CHILD_COMMAND_FORWARD_REQUESTED
            .to_string(),
        serde_json::Value::Bool(flow_report.child_command_forward_requested.is_some()),
    );
    observability.insert(
        constants::tracking_retention_settings_write::FLOW_CHILD_COMMAND_RECEIVED.to_string(),
        serde_json::Value::Bool(flow_report.child_command_received.is_some()),
    );
    observability.insert(
        constants::tracking_retention_settings_write::FLOW_CHILD_RUNTIME_FLOW.to_string(),
        serde_json::Value::Bool(flow_report.child_runtime_flow.is_some()),
    );
    observability.insert(
        constants::tracking_retention_settings_write::FLOW_AUDIT_ENTRY_COMMITTED.to_string(),
        serde_json::Value::Bool(flow_report.audit_entry_committed.is_some()),
    );
    observability.insert(
        constants::tracking_retention_settings_write::FLOW_PORTAL_READ_MODEL_UPDATED.to_string(),
        serde_json::Value::Bool(flow_report.portal_read_model_updated.is_some()),
    );
    serde_json::Value::Object(observability)
}

fn parse_write_request(
    command: &AgentCommandEnvelope,
) -> (
    TrackingRetentionSettingsWriteRequest,
    TrackingWriteRequestParseState,
) {
    match command
        .payload
        .get(constants::field::ACTIVITY_TRACKING_RETENTION_SETTINGS_WRITE_REQUEST)
    {
        Some(LogFieldValue::String(text)) => match serde_json::from_str(text) {
            Ok(request) => (request, TrackingWriteRequestParseState::Accepted),
            Err(_) => (
                default_tracking_retention_settings_write_request(),
                TrackingWriteRequestParseState::Rejected,
            ),
        },
        _ => (
            default_tracking_retention_settings_write_request(),
            TrackingWriteRequestParseState::Rejected,
        ),
    }
}

fn write_state(parse_state: TrackingWriteRequestParseState) -> TrackingRetentionWriteState {
    match parse_state {
        TrackingWriteRequestParseState::Accepted => tracking_retention_write_state_accepted(),
        TrackingWriteRequestParseState::Rejected => tracking_retention_write_state_rejected(),
    }
}

pub(crate) async fn execute_tracking_retention_settings_write_flow(
    command: &AgentCommandEnvelope,
    request: &TrackingRetentionSettingsWriteRequest,
    parse_state: TrackingWriteRequestParseState,
) -> TrackingRetentionSettingsWriteFlowReport {
    let parent_action_received = tracking_parent_action_received_event(command, request);
    if parse_state == TrackingWriteRequestParseState::Rejected {
        return rejected_tracking_retention_settings_write_flow_report(
            request,
            parent_action_received,
        );
    }

    accepted_tracking_retention_settings_write_flow_report(command, request, parent_action_received)
        .await
}

fn rejected_tracking_retention_settings_write_flow_report(
    _request: &TrackingRetentionSettingsWriteRequest,
    _parent_action_received: ParentActionReceivedEvent,
) -> TrackingRetentionSettingsWriteFlowReport {
    let parent_command_rejected = tracking_parent_command_rejected_event(
        &_request.command_id,
        &_parent_action_received,
        constants::tracking_config_update::REJECTION_REASON_INVALID_REQUEST,
    );

    TrackingRetentionSettingsWriteFlowReport {
        parent_action_received: _parent_action_received,
        parent_command_validated: None,
        parent_command_rejected: Some(parent_command_rejected),
        change_requested: None,
        policy_evaluation_requested: None,
        policy_decision_completed: None,
        change_approved: None,
        change_rejected: None,
        child_command_forward_requested: None,
        child_command_received: None,
        child_runtime_flow: None,
        audit_entry_committed: None,
        portal_read_model_updated: None,
    }
}

async fn accepted_tracking_retention_settings_write_flow_report(
    command: &AgentCommandEnvelope,
    request: &TrackingRetentionSettingsWriteRequest,
    parent_action_received: ParentActionReceivedEvent,
) -> TrackingRetentionSettingsWriteFlowReport {
    let parent_event = parent_tracking_config_updated_event_from_command(
        command,
        TrackingConfigUpdateRequest {
            command_id: request.command_id.clone(),
            runtime_config: default_tracking_runtime_config(),
            retention_settings: request.clone(),
        },
    );
    let parent_command_validated =
        tracking_parent_command_validated_event(&request.command_id, &parent_action_received);
    let parent_runtime_flow = publish_parent_tracking_config_updated_event_flow(
        parent_action_received.parent_action_event_ref.clone(),
        &parent_event,
        ChildAcknowledgementState::Required,
        ParentRuntimeOriginState::TrustedLocalUi,
    )
    .await
    .ok();
    let child_command_forward_requested = parent_runtime_flow.as_ref().and_then(|report| {
        report.change_approved_event.as_ref().map(|_| {
            tracking_parent_child_command_forward_requested_event(
                &request.command_id,
                &parent_command_validated,
                &parent_event,
            )
        })
    });
    let child_command_received =
        child_command_forward_requested
            .as_ref()
            .map(|forward_requested| {
                tracking_child_command_received_event(&request.command_id, forward_requested)
            });

    TrackingRetentionSettingsWriteFlowReport {
        parent_action_received,
        parent_command_validated: Some(parent_command_validated),
        parent_command_rejected: None,
        change_requested: parent_runtime_flow
            .as_ref()
            .map(|report| report.change_requested_event.clone()),
        policy_evaluation_requested: parent_runtime_flow
            .as_ref()
            .map(|report| report.policy_evaluation_event.clone()),
        policy_decision_completed: parent_runtime_flow
            .as_ref()
            .map(|report| report.policy_decision_event.clone()),
        change_approved: parent_runtime_flow
            .as_ref()
            .and_then(|report| report.change_approved_event.clone()),
        change_rejected: parent_runtime_flow
            .as_ref()
            .and_then(|report| report.change_rejected_event.clone()),
        child_command_forward_requested,
        child_command_received,
        child_runtime_flow: parent_runtime_flow
            .as_ref()
            .and_then(|report| report.child_runtime_flow.clone()),
        audit_entry_committed: parent_runtime_flow
            .as_ref()
            .map(|report| report.audit_event.clone()),
        portal_read_model_updated: parent_runtime_flow
            .as_ref()
            .map(|report| report.portal_event.clone()),
    }
}
