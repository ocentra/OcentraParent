use ocentra_parent_agent_protocol::{
    constants, AgentCommandEnvelope, AgentCommandName, AgentEventName, AgentMessageTarget,
    AgentPeer, AgentPeerRole, AgentRoute, LogFieldValue, LogFields,
    SocialAlertReportReadModelSnapshot, AGENT_PROTOCOL_SCHEMA_VERSION,
    SOCIAL_ALERT_REPORT_INTENT_HIGH_RISK,
};

use crate::{lan_pairing::LanPairingRuntime, websocket::handle_command_text_for_test};

#[tokio::test]
async fn social_alert_report_command_reports_service_backed_intent_rows() {
    let body =
        serde_json::to_string(&command_envelope()).expect(constants::error::AGENT_EVENT_SERIALIZES);
    let event = handle_command_text_for_test(&body, LanPairingRuntime::empty(), None).await;
    let read_model = social_alert_report_payload(
        &event.payload[constants::field::BROWSER_SOCIAL_ALERT_REPORT_READ_MODEL],
    );

    assert_eq!(
        event.event,
        AgentEventName::AgentBrowserSocialAlertReportReadModelReported
    );
    assert_eq!(read_model.intents.len(), 2);
    assert_eq!(
        read_model.intents[0].intent_kind,
        SOCIAL_ALERT_REPORT_INTENT_HIGH_RISK
    );
    assert!(!read_model.intents[0].provider_delivery_attempted);
    assert!(!read_model.intents[0].parent_notification_ui_claimed);
    assert!(!read_model.intents[0].final_policy_decision_claimed);
    assert!(!read_model.intents[0].enforcement_claimed);
}

fn command_envelope() -> AgentCommandEnvelope {
    AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: constants::event_id::BROWSER_SOCIAL_ALERT_REPORT_READ_MODEL_REPORTED
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
        command: AgentCommandName::AgentBrowserSocialAlertReportReadModelGet,
        payload: LogFields::new(),
    }
}

fn social_alert_report_payload(value: &LogFieldValue) -> SocialAlertReportReadModelSnapshot {
    match value {
        LogFieldValue::String(text) => {
            serde_json::from_str(text).expect(constants::error::AGENT_EVENT_SERIALIZES)
        }
        _ => std::panic::panic_any(constants::error::AGENT_EVENT_SERIALIZES),
    }
}
