use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};
use ocentra_parent_agent_protocol::social_alert_report_read_model::SocialAlertReportReadModelSnapshot;
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentCommandName, AgentEventName, AgentMessageTarget, AgentPeer,
    AgentPeerRole, AgentRoute,
};
use ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_INTENT_HIGH_RISK;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PROVIDER_STATUS_MANUAL_REQUIRED;

use ocentra_parent_agent_service::test_support::handle_local_command_text_for_test;

#[tokio::test]
async fn social_alert_report_command_reports_service_backed_intent_rows() {
    let body = serde_json::to_string(&command_envelope())
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES);
    let event =
        handle_local_command_text_for_test(crate::test_text::TestText::from_display(body)).await;
    let read_model = social_alert_report_payload(&crate::test_invariants::log_field(
        &event.payload,
        constants::field::BROWSER_SOCIAL_ALERT_REPORT_READ_MODEL,
        constants::error::AGENT_EVENT_SERIALIZES,
    ));

    assert_eq!(
        event.event,
        AgentEventName::AgentBrowserSocialAlertReportReadModelReported
    );
    assert_eq!(read_model.intents.len(), 2);
    assert_eq!(read_model.provider_status_rows.len(), 2);
    assert_eq!(
        read_model.intents[0].intent_kind,
        SOCIAL_ALERT_REPORT_INTENT_HIGH_RISK
    );
    assert_eq!(
        read_model.provider_status_rows[0].provider_status,
        SOCIAL_ALERT_REPORT_PROVIDER_STATUS_MANUAL_REQUIRED
    );
    assert!(read_model.provider_status_rows[0]
        .provider_receipt_refs
        .is_empty());
    assert!(!read_model.provider_status_rows[0].provider_delivery_implemented);
    assert!(!read_model.provider_status_rows[0].provider_delivery_observed);
    assert!(!read_model.provider_status_rows[0].delivered_notification_claimed);
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
            serde_json::from_str(text).expect_value(constants::error::AGENT_EVENT_SERIALIZES)
        }
        _ => std::process::abort(),
    }
}
