use ocentra_parent_agent_protocol::{
    constants, AgentCommandEnvelope, AgentCommandName, AgentEventName, AgentMessageTarget,
    AgentPeer, AgentPeerRole, AgentRoute, LogFieldValue, LogFields,
    TrackingRetentionSettingsMutationRequest, TrackingRetentionSettingsMutationResult,
    AGENT_PROTOCOL_SCHEMA_VERSION, TRACKING_RETENTION_SETTINGS_MUTATION_REJECTION_INVALID_REQUEST,
    TRACKING_RETENTION_SETTINGS_MUTATION_STATE_ACCEPTED,
    TRACKING_RETENTION_SETTINGS_MUTATION_STATE_REJECTED,
};

use crate::{lan_pairing::LanPairingRuntime, websocket::handle_command_text_for_test};

#[tokio::test]
async fn tracking_retention_settings_mutation_command_reports_service_execution() {
    let body = serde_json::to_string(&command_envelope(payload_for(request())))
        .expect(constants::error::AGENT_EVENT_SERIALIZES);
    let event = handle_command_text_for_test(&body, LanPairingRuntime::empty(), None).await;
    let result = mutation_result(
        &event.payload[constants::field::ACTIVITY_TRACKING_RETENTION_SETTINGS_MUTATION],
    );

    assert_eq!(
        event.event,
        AgentEventName::AgentActivityTrackingRetentionSettingsMutationReported
    );
    assert_eq!(
        result.mutation_state,
        TRACKING_RETENTION_SETTINGS_MUTATION_STATE_ACCEPTED
    );
    assert!(result.service_mutation_executed);
    assert!(!result.durable_persistence_claimed);
    assert!(!result.portal_ui_claimed);
    assert!(!result.child_device_delivery_claimed);
    assert!(!result.physical_device_claimed);
    assert!(!result.product_claim_ready);
    assert_eq!(
        result.evidence_reference_ids,
        vec![constants::activity_store::TEST_TRACKING_EVIDENCE_REFERENCE_ID.to_string()]
    );
}

#[tokio::test]
async fn tracking_retention_settings_mutation_command_rejects_missing_payload() {
    let body = serde_json::to_string(&command_envelope(LogFields::new()))
        .expect(constants::error::AGENT_EVENT_SERIALIZES);
    let event = handle_command_text_for_test(&body, LanPairingRuntime::empty(), None).await;
    let result = mutation_result(
        &event.payload[constants::field::ACTIVITY_TRACKING_RETENTION_SETTINGS_MUTATION],
    );

    assert_eq!(
        result.mutation_state,
        TRACKING_RETENTION_SETTINGS_MUTATION_STATE_REJECTED
    );
    assert_eq!(
        result.rejection_reason.as_deref(),
        Some(TRACKING_RETENTION_SETTINGS_MUTATION_REJECTION_INVALID_REQUEST)
    );
    assert!(!result.service_mutation_executed);
    assert!(!result.product_claim_ready);
}

fn request() -> TrackingRetentionSettingsMutationRequest {
    TrackingRetentionSettingsMutationRequest {
        request_id: constants::activity_store::TEST_TRACKING_RETENTION_DELETE_EVENT_ID.to_string(),
        intent_id: constants::activity_store::TEST_TRACKING_SUBJECT_ID.to_string(),
        settings_kind: constants::activity_subject_kind::RETENTION.to_string(),
        write_action: constants::field::COMMAND.to_string(),
        requested_value: constants::field::ACTIVE_STATE.to_string(),
        evidence_reference_ids: vec![
            constants::activity_store::TEST_TRACKING_EVIDENCE_REFERENCE_ID.to_string(),
        ],
        source_read_model_proof_refs: vec![
            constants::field::ACTIVITY_TRACKING_READ_MODEL.to_string()
        ],
        writer_boundary_proof_refs: vec![constants::field::CLAIM_BOUNDARY.to_string()],
        audit_refs: vec![constants::field::EVENT_REF.to_string()],
    }
}

fn payload_for(request: TrackingRetentionSettingsMutationRequest) -> LogFields {
    let mut payload = LogFields::new();
    payload.insert(
        constants::field::ACTIVITY_TRACKING_RETENTION_SETTINGS_MUTATION.to_string(),
        LogFieldValue::String(
            serde_json::to_string(&request).expect(constants::error::AGENT_EVENT_SERIALIZES),
        ),
    );
    payload
}

fn command_envelope(payload: LogFields) -> AgentCommandEnvelope {
    AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: constants::event_id::ACTIVITY_TRACKING_RETENTION_SETTINGS_MUTATION_REPORTED
            .to_string(),
        sent_at: constants::activity_store::TEST_TRACKING_RETENTION_DELETE_OBSERVED_AT.to_string(),
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
        command: AgentCommandName::AgentActivityTrackingRetentionSettingsMutate,
        payload,
    }
}

fn mutation_result(value: &LogFieldValue) -> TrackingRetentionSettingsMutationResult {
    match value {
        LogFieldValue::String(text) => {
            serde_json::from_str(text).expect(constants::error::AGENT_EVENT_SERIALIZES)
        }
        _ => std::panic::panic_any(constants::error::AGENT_EVENT_SERIALIZES),
    }
}
