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

#[cfg(test)]
use ocentra_parent_agent_protocol::child_agent::child_agent_events::{
    ChildCommandKind, ChildCommandReceivedEvent,
};
#[cfg(test)]
use ocentra_parent_agent_protocol::parent_controller_events::{
    ParentChildCommandForwardRequestedEvent, ParentChildCommandTransportBoundary,
    ParentCommandRejectedEvent, ParentCommandValidatedEvent, ParentCommandValidationState,
    ParentControllerActionKind,
};
#[cfg(test)]
use ocentra_parent_agent_protocol::tracking::config_update_event::{
    TrackingConfigAuditEntryCommittedEvent, TrackingConfigChangeApprovedEvent,
    TrackingConfigChangeRejectedEvent, TrackingConfigChangeRequestedEvent,
    TrackingConfigPolicyDecisionCompletedEvent, TrackingConfigPolicyEvaluationRequestedEvent,
    TrackingConfigPortalReadModelUpdatedEvent,
};

#[path = "tracking_retention_settings_write_events.rs"]
mod tracking_events;

use self::tracking_events::tracking_parent_action_received_event;
#[cfg(test)]
use self::tracking_events::{
    tracking_child_command_received_event, tracking_parent_child_command_forward_requested_event,
    tracking_parent_command_rejected_event, tracking_parent_command_validated_event,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TrackingWriteRequestParseState {
    Accepted,
    Rejected,
}

#[derive(Clone, Debug)]
struct TrackingRetentionSettingsWriteFlowReport {
    #[cfg(test)]
    parent_action_received: ParentActionReceivedEvent,
    #[cfg(test)]
    parent_command_validated: Option<ParentCommandValidatedEvent>,
    #[cfg(test)]
    parent_command_rejected: Option<ParentCommandRejectedEvent>,
    #[cfg(test)]
    change_requested: Option<TrackingConfigChangeRequestedEvent>,
    #[cfg(test)]
    policy_evaluation_requested: Option<TrackingConfigPolicyEvaluationRequestedEvent>,
    #[cfg(test)]
    policy_decision_completed: Option<TrackingConfigPolicyDecisionCompletedEvent>,
    #[cfg(test)]
    change_approved: Option<TrackingConfigChangeApprovedEvent>,
    #[cfg(test)]
    change_rejected: Option<TrackingConfigChangeRejectedEvent>,
    #[cfg(test)]
    child_command_forward_requested: Option<ParentChildCommandForwardRequestedEvent>,
    #[cfg(test)]
    child_command_received: Option<ChildCommandReceivedEvent>,
    child_runtime_flow: Option<TrackingConfigUpdateEventFlowReport>,
    #[cfg(test)]
    audit_entry_committed: Option<TrackingConfigAuditEntryCommittedEvent>,
    #[cfg(test)]
    portal_read_model_updated: Option<TrackingConfigPortalReadModelUpdatedEvent>,
}

pub(crate) async fn build_tracking_retention_settings_write_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let (request, parse_state) = parse_write_request(&command);
    let flow_report =
        execute_tracking_retention_settings_write_flow(&command, &request, parse_state).await;
    let applied_report = flow_report
        .child_runtime_flow
        .as_ref()
        .map(|report| &report.applied_report);
    let child_response = flow_report
        .child_runtime_flow
        .as_ref()
        .map(|report| &report.parent_request_report.response);
    let result = TrackingRetentionSettingsWriteResult {
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
    };
    let result_text = serde_json::to_string(&result).unwrap_or_default();

    build_event(
        constants::tracking_retention_settings_write::EVENT_ID,
        &command.message_id,
        command.source,
        AgentEventName::AgentActivityTrackingRetentionSettingsWriteReported,
        LogLevel::Info,
        fields_from_pairs(vec![(
            constants::field::ACTIVITY_TRACKING_RETENTION_SETTINGS_WRITE_RESULT,
            LogFieldValue::String(result_text),
        )]),
        None,
    )
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

async fn execute_tracking_retention_settings_write_flow(
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
    #[cfg(test)]
    let parent_command_rejected = tracking_parent_command_rejected_event(
        &_request.command_id,
        &_parent_action_received,
        constants::tracking_config_update::REJECTION_REASON_INVALID_REQUEST,
    );

    TrackingRetentionSettingsWriteFlowReport {
        #[cfg(test)]
        parent_action_received: _parent_action_received,
        #[cfg(test)]
        parent_command_validated: None,
        #[cfg(test)]
        parent_command_rejected: Some(parent_command_rejected),
        #[cfg(test)]
        change_requested: None,
        #[cfg(test)]
        policy_evaluation_requested: None,
        #[cfg(test)]
        policy_decision_completed: None,
        #[cfg(test)]
        change_approved: None,
        #[cfg(test)]
        change_rejected: None,
        #[cfg(test)]
        child_command_forward_requested: None,
        #[cfg(test)]
        child_command_received: None,
        child_runtime_flow: None,
        #[cfg(test)]
        audit_entry_committed: None,
        #[cfg(test)]
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
    #[cfg(test)]
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
    #[cfg(test)]
    let child_command_forward_requested = parent_runtime_flow.as_ref().and_then(|report| {
        report.change_approved_event.as_ref().map(|_| {
            tracking_parent_child_command_forward_requested_event(
                &request.command_id,
                &parent_command_validated,
                &parent_event,
            )
        })
    });
    #[cfg(test)]
    let child_command_received =
        child_command_forward_requested
            .as_ref()
            .map(|forward_requested| {
                tracking_child_command_received_event(&request.command_id, forward_requested)
            });

    TrackingRetentionSettingsWriteFlowReport {
        #[cfg(test)]
        parent_action_received,
        #[cfg(test)]
        parent_command_validated: Some(parent_command_validated),
        #[cfg(test)]
        parent_command_rejected: None,
        #[cfg(test)]
        change_requested: parent_runtime_flow
            .as_ref()
            .map(|report| report.change_requested_event.clone()),
        #[cfg(test)]
        policy_evaluation_requested: parent_runtime_flow
            .as_ref()
            .map(|report| report.policy_evaluation_event.clone()),
        #[cfg(test)]
        policy_decision_completed: parent_runtime_flow
            .as_ref()
            .map(|report| report.policy_decision_event.clone()),
        #[cfg(test)]
        change_approved: parent_runtime_flow
            .as_ref()
            .and_then(|report| report.change_approved_event.clone()),
        #[cfg(test)]
        change_rejected: parent_runtime_flow
            .as_ref()
            .and_then(|report| report.change_rejected_event.clone()),
        #[cfg(test)]
        child_command_forward_requested,
        #[cfg(test)]
        child_command_received,
        child_runtime_flow: parent_runtime_flow
            .as_ref()
            .and_then(|report| report.child_runtime_flow.clone()),
        #[cfg(test)]
        audit_entry_committed: parent_runtime_flow
            .as_ref()
            .map(|report| report.audit_event.clone()),
        #[cfg(test)]
        portal_read_model_updated: parent_runtime_flow
            .as_ref()
            .map(|report| report.portal_event.clone()),
    }
}

#[cfg(test)]
mod tests {
    use std::{error::Error, io::Error as IoError};

    use ocentra_child_runtime::tracking_config_update_flow::tracking_retention_settings_durable_store_path;
    use ocentra_parent_agent_protocol::logging::LogFields;
    use ocentra_parent_agent_protocol::tracking::config_update_event::TrackingConfigAuditOutcome;
    use ocentra_parent_agent_protocol::tracking::config_update_event::TrackingConfigPolicyDecisionState;
    use ocentra_parent_agent_protocol::tracking::config_update_event::TrackingConfigPortalUpdateKind;
    use ocentra_parent_agent_protocol::tracking::retention_settings_write_command::TrackingConfigAckState;
    use ocentra_parent_agent_protocol::tracking::retention_settings_write_command::TrackingDurableSettingsPersistenceState;
    use ocentra_parent_agent_protocol::tracking::retention_settings_write_command::TrackingExecutionClaimState;
    use ocentra_parent_agent_protocol::tracking::retention_settings_write_command::TrackingRemoteAiState;
    use ocentra_parent_agent_protocol::tracking::retention_settings_write_command::TrackingRemoteSyncState;
    use ocentra_parent_agent_protocol::tracking::retention_settings_write_command::TrackingRetentionSettingsWriteResult;
    use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;
    use ocentra_parent_agent_protocol::transport::AgentCommandName;
    use ocentra_parent_agent_protocol::transport::AgentMessageTarget;
    use ocentra_parent_agent_protocol::transport::AgentPeer;
    use ocentra_parent_agent_protocol::transport::AgentPeerRole;
    use ocentra_parent_agent_protocol::transport::AgentRoute;
    use ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION;

    use super::*;
    use crate::{lan_pairing::LanPairingRuntime, websocket::handle_command_text_for_test};

    type TestResult = Result<(), Box<dyn Error>>;

    #[tokio::test]
    async fn retention_settings_write_command_reports_service_backed_transport_boundary(
    ) -> TestResult {
        let body = serde_json::to_string(&command_envelope()?)?;
        let event = handle_command_text_for_test(&body, LanPairingRuntime::empty(), None).await;
        let write_result = write_result_payload(
            &event.payload[constants::field::ACTIVITY_TRACKING_RETENTION_SETTINGS_WRITE_RESULT],
        )?;

        assert_eq!(
            event.event,
            AgentEventName::AgentActivityTrackingRetentionSettingsWriteReported
        );
        assert_service_backed_write_result(&write_result);
        assert!(tracking_retention_settings_durable_store_path().exists());
        assert_unclaimed_tracking_claim_states(&write_result);

        Ok(())
    }

    #[tokio::test]
    async fn retention_settings_write_flow_builds_parent_child_policy_and_audit_chain() -> TestResult
    {
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
            ParentControllerActionKind::UpdateTrackingConfig
        );
        assert_eq!(
            flow_report
                .parent_command_validated
                .as_ref()
                .ok_or_else(|| IoError::other(constants::error::AGENT_EVENT_SERIALIZES))?
                .validation_state,
            ParentCommandValidationState::Validated
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
            ParentChildCommandTransportBoundary::TypedLocalServiceTransport
        );
        assert_eq!(
            flow_report
                .child_command_received
                .as_ref()
                .ok_or_else(|| IoError::other(constants::error::AGENT_EVENT_SERIALIZES))?
                .command_kind,
            ChildCommandKind::ApplyTrackingConfig
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
        assert!(flow_report.child_runtime_flow.is_some());

        Ok(())
    }

    #[tokio::test]
    async fn retention_settings_write_command_rejects_missing_typed_request_payload() -> TestResult
    {
        let body = serde_json::to_string(&command_envelope_without_request()?)?;
        let event = handle_command_text_for_test(&body, LanPairingRuntime::empty(), None).await;
        let write_result = write_result_payload(
            &event.payload[constants::field::ACTIVITY_TRACKING_RETENTION_SETTINGS_WRITE_RESULT],
        )?;

        assert_eq!(
            event.event,
            AgentEventName::AgentActivityTrackingRetentionSettingsWriteReported
        );
        assert_eq!(
            write_result.write_state,
            tracking_retention_write_state_rejected()
        );
        assert_eq!(write_result.local_service_state_revision, None);
        assert_eq!(
            write_result.durable_settings_store_ref,
            tracking_durable_settings_store_ref()
        );
        assert_eq!(
            write_result.durable_settings_persistence_state,
            TrackingDurableSettingsPersistenceState::NotPersisted
        );
        assert_eq!(
            write_result.service_mutation_execution_state,
            TrackingExecutionClaimState::Unclaimed
        );
        assert_eq!(
            write_result.child_config_ack_state,
            TrackingConfigAckState::Missing
        );
        assert_eq!(
            write_result.product_claim_state,
            TrackingExecutionClaimState::Unclaimed
        );

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

    fn command_envelope_without_request() -> Result<AgentCommandEnvelope, serde_json::Error> {
        Ok(AgentCommandEnvelope {
            payload: LogFields::new(),
            ..command_envelope()?
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

    fn assert_unclaimed_tracking_claim_states(write_result: &TrackingRetentionSettingsWriteResult) {
        assert_eq!(
            write_result.portal_writable_ui_claim_state,
            TrackingExecutionClaimState::Unclaimed
        );
        assert_eq!(
            write_result.platform_runtime_claim_state,
            TrackingExecutionClaimState::Unclaimed
        );
        assert_eq!(
            write_result.child_device_delivery_claim_state,
            TrackingExecutionClaimState::Unclaimed
        );
        assert_eq!(
            write_result.provider_delivery_claim_state,
            TrackingExecutionClaimState::Unclaimed
        );
        assert_eq!(
            write_result.notification_receipt_claim_state,
            TrackingExecutionClaimState::Unclaimed
        );
        assert_eq!(
            write_result.physical_device_claim_state,
            TrackingExecutionClaimState::Unclaimed
        );
        assert_eq!(
            write_result.authority_claim_state,
            TrackingExecutionClaimState::Unclaimed
        );
        assert_eq!(
            write_result.product_claim_state,
            TrackingExecutionClaimState::Unclaimed
        );
    }
}
