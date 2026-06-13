use ocentra_child_runtime::{
    parent_tracking_config_updated_event_from_command, publish_parent_tracking_config_updated_event,
};
use ocentra_parent_runtime_core::{
    route_parent_tracking_config_update_event, ChildAcknowledgementState, ChildRuntimePublishState,
};
use ocentra_parent_agent_protocol::{
    constants, default_tracking_retention_settings_write_request, AgentCommandEnvelope,
    AgentEventEnvelope, AgentEventName, LogFieldValue, LogLevel, TrackingConfigAckState,
    TrackingDurableSettingsPersistenceState, TrackingExecutionClaimState, TrackingRemoteAiState,
    TrackingRemoteSyncState, TrackingRetentionWriteState,
    tracking_durable_settings_store_ref, tracking_local_service_state_snapshot_ref,
    tracking_mutation_proof_ref, tracking_retention_accepted_at,
    tracking_retention_write_state_accepted, tracking_retention_write_state_rejected,
    TrackingRetentionSettingsWriteRequest, TrackingRetentionSettingsWriteResult,
    AGENT_PROTOCOL_SCHEMA_VERSION,
};

use crate::{event_builder::build_event, fields::fields_from_pairs};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TrackingWriteRequestParseState {
    Accepted,
    Rejected,
}

pub(crate) async fn build_tracking_retention_settings_write_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let (request, parse_state) = parse_write_request(&command);
    let event_flow_report = if parse_state == TrackingWriteRequestParseState::Accepted {
        let parent_event =
            parent_tracking_config_updated_event_from_command(&command, request.clone());
        let dispatch_decision = route_parent_tracking_config_update_event(
            &parent_event,
            ChildAcknowledgementState::Required,
        );
        if dispatch_decision.child_runtime_publish_state == ChildRuntimePublishState::Publish {
            publish_parent_tracking_config_updated_event(&parent_event)
                .await
                .ok()
        } else {
            None
        }
    } else {
        None
    };
    let applied_report = event_flow_report
        .as_ref()
        .map(|report| &report.applied_report);
    let child_response = event_flow_report
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
        applied_delete_after_alert_resolution_state:
            request.requested_delete_after_alert_resolution_state,
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

#[cfg(test)]
mod tests {
    use ocentra_child_runtime::tracking_retention_settings_durable_store_path;
    use ocentra_parent_agent_protocol::{
        AgentCommandEnvelope, AgentCommandName, AgentMessageTarget, AgentPeer, AgentPeerRole,
        AgentRoute, LogFields, TrackingConfigAckState, TrackingDurableSettingsPersistenceState,
        TrackingExecutionClaimState, TrackingRemoteAiState, TrackingRemoteSyncState,
        TrackingRetentionSettingsWriteResult, TrackingRetentionWriteState,
        AGENT_PROTOCOL_SCHEMA_VERSION,
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
        assert!(write_result.command_transport_claimed);
        assert!(write_result.service_write_preflight_claimed);
        assert!(write_result.service_mutation_executed);
        assert_eq!(write_result.applied_retention_window_hours, Some(168));
        assert_eq!(
            write_result.remote_sync_state,
            TrackingRemoteSyncState::Disabled
        );
        assert_eq!(write_result.remote_ai_state, TrackingRemoteAiState::Disabled);
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
