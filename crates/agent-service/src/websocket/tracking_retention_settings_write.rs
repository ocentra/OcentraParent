use ocentra_parent_agent_protocol::{
    constants, AgentCommandEnvelope, AgentEventEnvelope, AgentEventName, LogFieldValue, LogLevel,
    TrackingRetentionSettingsWriteResult, AGENT_PROTOCOL_SCHEMA_VERSION,
};

use crate::{event_builder::build_event, fields::fields_from_pairs};

pub(crate) async fn build_tracking_retention_settings_write_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let result = TrackingRetentionSettingsWriteResult {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        command_id: constants::tracking_retention_settings_write::COMMAND_ID.to_string(),
        settings_kind: constants::tracking_retention_settings_write::SETTINGS_KIND_RETENTION_WINDOW
            .to_string(),
        write_state: constants::tracking_retention_settings_write::WRITE_STATE_ACCEPTED.to_string(),
        accepted_at: constants::tracking_retention_settings_write::ACCEPTED_AT.to_string(),
        source_mutation_proof_refs: vec![
            constants::tracking_retention_settings_write::MUTATION_PROOF_REF.to_string(),
        ],
        command_transport_claimed: true,
        service_write_preflight_claimed: true,
        service_mutation_executed: false,
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
        assert!(!write_result.service_mutation_executed);
        assert!(!write_result.portal_writable_ui_claimed);
        assert!(!write_result.platform_runtime_claimed);
        assert!(!write_result.child_device_delivery_claimed);
        assert!(!write_result.provider_delivery_claimed);
        assert!(!write_result.notification_receipt_claimed);
        assert!(!write_result.physical_device_claimed);
        assert!(!write_result.authority_claimed);
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
            payload: LogFields::new(),
        }
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
