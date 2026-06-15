use ocentra_child_runtime::TrackingConfigUpdateEventFlowReport;
use ocentra_parent_agent_protocol::{
    constants, default_tracking_retention_settings_write_request, default_tracking_runtime_config,
    parent_tracking_config_updated_event_from_command, tracking_durable_settings_store_ref,
    tracking_local_service_state_snapshot_ref, tracking_mutation_proof_ref,
    tracking_retention_accepted_at, tracking_retention_write_state_accepted,
    tracking_retention_write_state_rejected, AgentCommandEnvelope, AgentEventEnvelope,
    AgentEventName, ChildCommandKind, ChildCommandReceivedEvent, LogFieldValue, LogLevel,
    ParentActionReceivedEvent, ParentChildCommandForwardRequestedEvent,
    ParentChildCommandTransportBoundary, ParentCommandRejectedEvent, ParentCommandValidatedEvent,
    ParentCommandValidationState, ParentControllerActionKind, ParentControllerSource,
    ParentTrackingConfigUpdatedEvent, TrackingConfigAckState,
    TrackingConfigAuditEntryCommittedEvent, TrackingConfigChangeApprovedEvent,
    TrackingConfigChangeRejectedEvent, TrackingConfigChangeRequestedEvent,
    TrackingConfigPolicyDecisionCompletedEvent, TrackingConfigPolicyEvaluationRequestedEvent,
    TrackingConfigPortalReadModelUpdatedEvent, TrackingConfigUpdateRequest,
    TrackingDurableSettingsPersistenceState, TrackingExecutionClaimState, TrackingRemoteAiState,
    TrackingRemoteSyncState, TrackingRetentionSettingsWriteRequest,
    TrackingRetentionSettingsWriteResult, TrackingRetentionWriteState,
    AGENT_PROTOCOL_SCHEMA_VERSION,
};
use ocentra_parent_runtime_core::tracking_config_update_flow::publish_parent_tracking_config_updated_event_flow;
use ocentra_parent_runtime_core::tracking_dispatch::{
    ChildAcknowledgementState, ParentRuntimeOriginState,
};

use crate::{event_builder::build_event, fields::fields_from_pairs};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TrackingWriteRequestParseState {
    Accepted,
    Rejected,
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
struct TrackingRetentionSettingsWriteFlowReport {
    parent_action_received: ParentActionReceivedEvent,
    parent_command_validated: Option<ParentCommandValidatedEvent>,
    parent_command_rejected: Option<ParentCommandRejectedEvent>,
    change_requested: Option<TrackingConfigChangeRequestedEvent>,
    policy_evaluation_requested: Option<TrackingConfigPolicyEvaluationRequestedEvent>,
    policy_decision_completed: Option<TrackingConfigPolicyDecisionCompletedEvent>,
    change_approved: Option<TrackingConfigChangeApprovedEvent>,
    change_rejected: Option<TrackingConfigChangeRejectedEvent>,
    child_command_forward_requested: Option<ParentChildCommandForwardRequestedEvent>,
    child_command_received: Option<ChildCommandReceivedEvent>,
    child_runtime_flow: Option<TrackingConfigUpdateEventFlowReport>,
    audit_entry_committed: Option<TrackingConfigAuditEntryCommittedEvent>,
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
    let result_text =
        serde_json::to_string(&result).expect(constants::error::AGENT_EVENT_SERIALIZES);

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
        let parent_command_rejected = tracking_parent_command_rejected_event(
            &request.command_id,
            &parent_action_received,
            constants::tracking_config_update::REJECTION_REASON_INVALID_REQUEST,
        );
        return TrackingRetentionSettingsWriteFlowReport {
            parent_action_received,
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
        };
    }

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

fn tracking_parent_action_received_event(
    command: &AgentCommandEnvelope,
    request: &TrackingRetentionSettingsWriteRequest,
) -> ParentActionReceivedEvent {
    ParentActionReceivedEvent {
        schema_version: constants::parent_controller::EVENT_SCHEMA_VERSION,
        parent_action_event_ref: tracking_service_event_ref(&request.command_id, "parent-action"),
        received_at: tracking_retention_accepted_at().to_string(),
        parent_intent_ref: tracking_service_ref(&request.command_id, "parent-intent"),
        parent_profile_ref: tracking_service_ref(&request.command_id, "parent-profile"),
        device_ref: command.target.device_id.clone(),
        action_kind: ParentControllerActionKind::UpdateTrackingConfig,
        source: ParentControllerSource::PortalTypedIntent,
        custody: constants::parent_controller::CUSTODY_LOCAL_SERVICE_VALIDATION.to_string(),
        idempotency_key: tracking_service_idempotency_key(&request.command_id, "parent-action"),
    }
}

fn tracking_parent_command_validated_event(
    command_id: &ocentra_parent_agent_protocol::TrackingRetentionCommandId,
    parent_action_received: &ParentActionReceivedEvent,
) -> ParentCommandValidatedEvent {
    ParentCommandValidatedEvent {
        schema_version: constants::parent_controller::EVENT_SCHEMA_VERSION,
        command_validated_event_ref: tracking_service_event_ref(command_id, "command-validated"),
        parent_action_event_ref: parent_action_received.parent_action_event_ref.clone(),
        parent_command_ref: tracking_parent_command_ref(command_id),
        child_command_ref: Some(tracking_child_command_ref(command_id)),
        validated_at: tracking_retention_accepted_at().to_string(),
        validation_state: ParentCommandValidationState::Validated,
        causation_event_ref: parent_action_received.parent_action_event_ref.clone(),
        idempotency_key: tracking_service_idempotency_key(command_id, "command-validated"),
    }
}

fn tracking_parent_command_rejected_event(
    command_id: &ocentra_parent_agent_protocol::TrackingRetentionCommandId,
    parent_action_received: &ParentActionReceivedEvent,
    rejection_reason_code: impl Into<String>,
) -> ParentCommandRejectedEvent {
    ParentCommandRejectedEvent {
        schema_version: constants::parent_controller::EVENT_SCHEMA_VERSION,
        command_rejected_event_ref: tracking_service_event_ref(command_id, "command-rejected"),
        parent_action_event_ref: parent_action_received.parent_action_event_ref.clone(),
        rejected_at: tracking_retention_accepted_at().to_string(),
        rejection_reason_code: rejection_reason_code.into(),
        causation_event_ref: parent_action_received.parent_action_event_ref.clone(),
        idempotency_key: tracking_service_idempotency_key(command_id, "command-rejected"),
    }
}

fn tracking_parent_child_command_forward_requested_event(
    command_id: &ocentra_parent_agent_protocol::TrackingRetentionCommandId,
    parent_command_validated: &ParentCommandValidatedEvent,
    parent_event: &ParentTrackingConfigUpdatedEvent,
) -> ParentChildCommandForwardRequestedEvent {
    ParentChildCommandForwardRequestedEvent {
        schema_version: constants::parent_controller::EVENT_SCHEMA_VERSION,
        forward_requested_event_ref: tracking_service_event_ref(command_id, "forward-requested"),
        parent_command_ref: parent_command_validated.parent_command_ref.clone(),
        child_command_ref: tracking_child_command_ref(command_id),
        device_ref: parent_event.target.device_id.as_str().to_string(),
        requested_at: tracking_retention_accepted_at().to_string(),
        transport_boundary: ParentChildCommandTransportBoundary::TypedLocalServiceTransport,
        causation_event_ref: parent_command_validated.command_validated_event_ref.clone(),
        idempotency_key: tracking_service_idempotency_key(command_id, "forward-requested"),
    }
}

fn tracking_child_command_received_event(
    command_id: &ocentra_parent_agent_protocol::TrackingRetentionCommandId,
    forward_requested_event: &ParentChildCommandForwardRequestedEvent,
) -> ChildCommandReceivedEvent {
    ChildCommandReceivedEvent {
        schema_version: constants::child_agent::EVENT_SCHEMA_VERSION,
        command_received_event_ref: tracking_service_event_ref(
            command_id,
            "child-command-received",
        ),
        child_command_ref: forward_requested_event.child_command_ref.clone(),
        received_at: tracking_retention_accepted_at().to_string(),
        device_ref: forward_requested_event.device_ref.clone(),
        parent_controller_event_ref: forward_requested_event.forward_requested_event_ref.clone(),
        transport_message_ref: tracking_transport_message_ref(command_id),
        command_kind: ChildCommandKind::ApplyTrackingConfig,
    }
}

fn tracking_parent_command_ref(
    command_id: &ocentra_parent_agent_protocol::TrackingRetentionCommandId,
) -> String {
    tracking_service_ref(
        command_id,
        constants::parent_controller::REF_PARENT_COMMAND_SUFFIX,
    )
}

fn tracking_child_command_ref(
    command_id: &ocentra_parent_agent_protocol::TrackingRetentionCommandId,
) -> String {
    tracking_service_ref(
        command_id,
        constants::parent_controller::REF_CHILD_COMMAND_SUFFIX,
    )
}

fn tracking_transport_message_ref(
    command_id: &ocentra_parent_agent_protocol::TrackingRetentionCommandId,
) -> String {
    tracking_service_ref(
        command_id,
        constants::parent_controller::REF_TRANSPORT_MESSAGE_SUFFIX,
    )
}

fn tracking_service_ref(
    command_id: &ocentra_parent_agent_protocol::TrackingRetentionCommandId,
    suffix: &str,
) -> String {
    format!(
        "{}{}.{}",
        constants::parent_controller::CORRELATION_PARENT_CHILD_RUNTIME_PREFIX,
        command_id,
        suffix
    )
}

fn tracking_service_event_ref(
    command_id: &ocentra_parent_agent_protocol::TrackingRetentionCommandId,
    suffix: &str,
) -> String {
    format!(
        "event.tracking-retention-settings-write.{}.{}",
        command_id, suffix
    )
}

fn tracking_service_idempotency_key(
    command_id: &ocentra_parent_agent_protocol::TrackingRetentionCommandId,
    suffix: &str,
) -> String {
    format!(
        "{}{}.{}",
        constants::parent_controller::IDEMPOTENCY_PARENT_CHILD_RUNTIME_PREFIX,
        command_id,
        suffix
    )
}

#[cfg(test)]
mod tests {
    use ocentra_child_runtime::tracking_retention_settings_durable_store_path;
    use ocentra_parent_agent_protocol::{
        AgentCommandEnvelope, AgentCommandName, AgentMessageTarget, AgentPeer, AgentPeerRole,
        AgentRoute, LogFields, TrackingConfigAckState, TrackingConfigAuditOutcome,
        TrackingConfigPolicyDecisionState, TrackingConfigPortalUpdateKind,
        TrackingDurableSettingsPersistenceState, TrackingExecutionClaimState,
        TrackingRemoteAiState, TrackingRemoteSyncState, TrackingRetentionSettingsWriteResult,
        TrackingRetentionWriteState, AGENT_PROTOCOL_SCHEMA_VERSION,
    };

    use super::*;
    use crate::{lan_pairing::LanPairingRuntime, websocket::handle_command_text_for_test};

    #[tokio::test]
    async fn retention_settings_write_command_reports_service_backed_transport_boundary() {
        let body = serde_json::to_string(&command_envelope())
            .expect(constants::error::AGENT_EVENT_SERIALIZES);
        let event = handle_command_text_for_test(&body, LanPairingRuntime::empty(), None).await;
        let write_result = write_result_payload(
            &event.payload[constants::field::ACTIVITY_TRACKING_RETENTION_SETTINGS_WRITE_RESULT],
        );

        assert_eq!(
            event.event,
            AgentEventName::AgentActivityTrackingRetentionSettingsWriteReported
        );
        assert_eq!(
            write_result.settings_kind,
            constants::tracking_retention_settings_write::SETTINGS_KIND_RETENTION_WINDOW
        );
        assert_eq!(
            write_result.write_state,
            TrackingRetentionWriteState::parse(
                constants::tracking_retention_settings_write::WRITE_STATE_ACCEPTED
            )
            .expect(constants::tracking_retention_settings_write::WRITE_STATE_ACCEPTED)
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
        assert!(tracking_retention_settings_durable_store_path().exists());
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

    #[tokio::test]
    async fn retention_settings_write_flow_builds_parent_child_policy_and_audit_chain() {
        let command = command_envelope();
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
                .expect(constants::error::AGENT_EVENT_SERIALIZES)
                .validation_state,
            ParentCommandValidationState::Validated
        );
        assert_eq!(
            flow_report
                .policy_evaluation_requested
                .as_ref()
                .expect(constants::error::AGENT_EVENT_SERIALIZES)
                .parent_rule_refs
                .len(),
            3
        );
        assert_eq!(
            flow_report
                .policy_decision_completed
                .as_ref()
                .expect(constants::error::AGENT_EVENT_SERIALIZES)
                .decision_state,
            TrackingConfigPolicyDecisionState::Approved
        );
        assert_eq!(
            flow_report
                .child_command_forward_requested
                .as_ref()
                .expect(constants::error::AGENT_EVENT_SERIALIZES)
                .transport_boundary,
            ParentChildCommandTransportBoundary::TypedLocalServiceTransport
        );
        assert_eq!(
            flow_report
                .child_command_received
                .as_ref()
                .expect(constants::error::AGENT_EVENT_SERIALIZES)
                .command_kind,
            ChildCommandKind::ApplyTrackingConfig
        );
        assert_eq!(
            flow_report
                .audit_entry_committed
                .as_ref()
                .expect(constants::error::AGENT_EVENT_SERIALIZES)
                .audit_outcome,
            TrackingConfigAuditOutcome::Committed
        );
        assert_eq!(
            flow_report
                .portal_read_model_updated
                .as_ref()
                .expect(constants::error::AGENT_EVENT_SERIALIZES)
                .update_kind,
            TrackingConfigPortalUpdateKind::TrackingConfigState
        );
        assert!(flow_report.child_runtime_flow.is_some());
    }

    #[tokio::test]
    async fn retention_settings_write_command_rejects_missing_typed_request_payload() {
        let body = serde_json::to_string(&command_envelope_without_request())
            .expect(constants::error::AGENT_EVENT_SERIALIZES);
        let event = handle_command_text_for_test(&body, LanPairingRuntime::empty(), None).await;
        let write_result = write_result_payload(
            &event.payload[constants::field::ACTIVITY_TRACKING_RETENTION_SETTINGS_WRITE_RESULT],
        );

        assert_eq!(
            event.event,
            AgentEventName::AgentActivityTrackingRetentionSettingsWriteReported
        );
        assert_eq!(
            write_result.write_state,
            TrackingRetentionWriteState::parse(
                constants::tracking_retention_settings_write::WRITE_STATE_REJECTED
            )
            .expect(constants::tracking_retention_settings_write::WRITE_STATE_REJECTED)
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
    }

    #[tokio::test]
    async fn retention_settings_write_flow_rejects_before_child_runtime_when_request_is_missing() {
        let command = command_envelope();
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
                .expect(constants::error::AGENT_EVENT_SERIALIZES)
                .rejection_reason_code,
            constants::tracking_config_update::REJECTION_REASON_INVALID_REQUEST
        );
        assert!(flow_report.change_requested.is_none());
        assert!(flow_report.child_runtime_flow.is_none());
        assert!(flow_report.audit_entry_committed.is_none());
    }

    fn command_envelope() -> AgentCommandEnvelope {
        AgentCommandEnvelope {
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
            payload: write_request_payload(),
        }
    }

    fn command_envelope_without_request() -> AgentCommandEnvelope {
        AgentCommandEnvelope {
            payload: LogFields::new(),
            ..command_envelope()
        }
    }

    fn write_request_payload() -> LogFields {
        fields_from_pairs(vec![(
            constants::field::ACTIVITY_TRACKING_RETENTION_SETTINGS_WRITE_REQUEST,
            LogFieldValue::String(
                serde_json::to_string(&default_tracking_retention_settings_write_request())
                    .expect(constants::error::AGENT_EVENT_SERIALIZES),
            ),
        )])
    }

    fn write_result_payload(value: &LogFieldValue) -> TrackingRetentionSettingsWriteResult {
        match value {
            LogFieldValue::String(text) => {
                serde_json::from_str(text).expect(constants::error::AGENT_EVENT_SERIALIZES)
            }
            _ => std::panic::panic_any(constants::error::AGENT_EVENT_SERIALIZES),
        }
    }
}
