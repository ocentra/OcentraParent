use ocentra_parent_agent_protocol::{
    constants, AgentCommandEnvelope, AgentEventEnvelope, AgentEventName, LogFieldValue, LogLevel,
    TrackingRetentionSettingsWriteRequest, TrackingRetentionSettingsWriteResult,
    AGENT_PROTOCOL_SCHEMA_VERSION,
};
use std::sync::{Mutex, OnceLock};

use crate::{event_builder::build_event, fields::fields_from_pairs};

#[derive(Debug, Default)]
struct LocalRetentionSettingsState {
    revision: u64,
    retention_window_hours: Option<u16>,
    delete_after_alert_resolved: bool,
    parent_export_prepared: bool,
    remote_sync_enabled: bool,
    remote_ai_enabled: bool,
}

static LOCAL_RETENTION_SETTINGS_STATE: OnceLock<Mutex<LocalRetentionSettingsState>> =
    OnceLock::new();

pub(crate) async fn build_tracking_retention_settings_write_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let (request, accepted) = parse_write_request(&command);
    let local_state_revision = if accepted {
        Some(apply_local_retention_settings_state(&request))
    } else {
        None
    };
    let result = TrackingRetentionSettingsWriteResult {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        command_id: request.command_id,
        settings_kind: request.settings_kind,
        write_state: write_state(accepted),
        accepted_at: constants::tracking_retention_settings_write::ACCEPTED_AT.to_string(),
        source_writer_intent_refs: request.source_writer_intent_refs,
        source_read_model_proof_refs: request.source_read_model_proof_refs,
        source_mutation_proof_refs: vec![
            constants::tracking_retention_settings_write::MUTATION_PROOF_REF.to_string(),
        ],
        applied_retention_window_hours: request.requested_retention_window_hours,
        applied_delete_after_alert_resolved: request.requested_delete_after_alert_resolved,
        parent_export_prepared: request.requested_parent_export,
        remote_sync_enabled: false,
        remote_ai_enabled: false,
        local_service_state_revision: local_state_revision,
        local_service_state_snapshot_ref:
            constants::tracking_retention_settings_write::LOCAL_SERVICE_STATE_SNAPSHOT_REF
                .to_string(),
        durable_settings_persisted: false,
        command_transport_claimed: true,
        service_write_preflight_claimed: true,
        service_mutation_executed: accepted,
        portal_writable_ui_claimed: false,
        platform_runtime_claimed: false,
        child_device_delivery_claimed: false,
        provider_delivery_claimed: false,
        notification_receipt_claimed: false,
        physical_device_claimed: false,
        authority_claimed: false,
        product_claim_ready: false,
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

fn apply_local_retention_settings_state(request: &TrackingRetentionSettingsWriteRequest) -> u64 {
    let state = LOCAL_RETENTION_SETTINGS_STATE
        .get_or_init(|| Mutex::new(LocalRetentionSettingsState::default()));
    let mut guard = state
        .lock()
        .expect(constants::error::AGENT_EVENT_SERIALIZES);
    guard.revision += 1;
    guard.retention_window_hours = request.requested_retention_window_hours;
    guard.delete_after_alert_resolved = request.requested_delete_after_alert_resolved;
    guard.parent_export_prepared = request.requested_parent_export;
    guard.remote_sync_enabled = false;
    guard.remote_ai_enabled = false;
    guard.revision
}

fn parse_write_request(
    command: &AgentCommandEnvelope,
) -> (TrackingRetentionSettingsWriteRequest, bool) {
    match command
        .payload
        .get(constants::field::ACTIVITY_TRACKING_RETENTION_SETTINGS_WRITE_REQUEST)
    {
        Some(LogFieldValue::String(text)) => match serde_json::from_str(text) {
            Ok(request) => (request, true),
            Err(_) => (default_write_request(), false),
        },
        _ => (default_write_request(), false),
    }
}

fn write_state(accepted: bool) -> String {
    if accepted {
        constants::tracking_retention_settings_write::WRITE_STATE_ACCEPTED.to_string()
    } else {
        constants::tracking_retention_settings_write::WRITE_STATE_REJECTED.to_string()
    }
}

fn default_write_request() -> TrackingRetentionSettingsWriteRequest {
    TrackingRetentionSettingsWriteRequest {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        command_id: constants::tracking_retention_settings_write::COMMAND_ID.to_string(),
        settings_kind: constants::tracking_retention_settings_write::SETTINGS_KIND_RETENTION_WINDOW
            .to_string(),
        requested_retention_window_hours: Some(168),
        requested_delete_after_alert_resolved: false,
        requested_parent_export: false,
        requested_remote_sync_enabled: false,
        requested_remote_ai_enabled: false,
        source_writer_intent_refs: vec![
            constants::tracking_retention_settings_write::WRITER_INTENT_REF.to_string(),
        ],
        source_read_model_proof_refs: vec![
            constants::tracking_retention_settings_write::READ_MODEL_PROOF_REF.to_string(),
        ],
    }
}

#[cfg(test)]
mod tests {
    use ocentra_parent_agent_protocol::{
        AgentCommandEnvelope, AgentCommandName, AgentMessageTarget, AgentPeer, AgentPeerRole,
        AgentRoute, LogFields, TrackingRetentionSettingsWriteResult, AGENT_PROTOCOL_SCHEMA_VERSION,
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
            constants::tracking_retention_settings_write::WRITE_STATE_ACCEPTED
        );
        assert!(write_result.command_transport_claimed);
        assert!(write_result.service_write_preflight_claimed);
        assert!(write_result.service_mutation_executed);
        assert_eq!(write_result.applied_retention_window_hours, Some(168));
        assert!(!write_result.remote_sync_enabled);
        assert!(!write_result.remote_ai_enabled);
        assert!(write_result
            .local_service_state_revision
            .is_some_and(|revision| revision > 0));
        assert_eq!(
            write_result.local_service_state_snapshot_ref,
            constants::tracking_retention_settings_write::LOCAL_SERVICE_STATE_SNAPSHOT_REF
        );
        assert!(!write_result.durable_settings_persisted);
        assert!(!write_result.portal_writable_ui_claimed);
        assert!(!write_result.platform_runtime_claimed);
        assert!(!write_result.child_device_delivery_claimed);
        assert!(!write_result.provider_delivery_claimed);
        assert!(!write_result.notification_receipt_claimed);
        assert!(!write_result.physical_device_claimed);
        assert!(!write_result.authority_claimed);
        assert!(!write_result.product_claim_ready);
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
            constants::tracking_retention_settings_write::WRITE_STATE_REJECTED
        );
        assert_eq!(write_result.local_service_state_revision, None);
        assert!(!write_result.durable_settings_persisted);
        assert!(!write_result.service_mutation_executed);
        assert!(!write_result.product_claim_ready);
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
                serde_json::to_string(&default_write_request())
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
