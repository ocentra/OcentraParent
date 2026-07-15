use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};
use ocentra_parent_agent_protocol::social_parent_notification_delivery_read_model::SocialParentNotificationDeliveryReadinessSnapshot;
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentCommandName, AgentEventName, AgentMessageTarget, AgentPeer,
    AgentPeerRole, AgentRoute,
};
use ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION;
use ocentra_parent_agent_protocol::SOCIAL_PARENT_NOTIFICATION_DELIVERY_STATE_REPORT_READY;
use ocentra_parent_agent_protocol::SOCIAL_REPORT_WRITER_DELIVERY_REPORT_RECEIPT_REF;

use ocentra_parent_agent_service::test_support::handle_local_command_text_for_test;

#[tokio::test]
async fn social_parent_notification_delivery_command_reports_service_backed_readiness_rows() {
    let body = serde_json::to_string(&command_envelope())
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES);
    let event =
        handle_local_command_text_for_test(crate::test_text::TestText::from_display(body)).await;
    let read_model = read_model_payload(&crate::test_invariants::log_field(
        &event.payload,
        constants::field::BROWSER_SOCIAL_PARENT_NOTIFICATION_DELIVERY_READ_MODEL,
        constants::error::AGENT_EVENT_SERIALIZES,
    ));

    assert_eq!(
        event.event,
        AgentEventName::AgentBrowserSocialParentNotificationDeliveryReadModelReported
    );
    assert_eq!(read_model.rows.len(), 3);
    assert_eq!(read_model.parent_report_status_ready_count, 1);
    assert_eq!(
        read_model.rows[0].notification_delivery_readiness_state,
        SOCIAL_PARENT_NOTIFICATION_DELIVERY_STATE_REPORT_READY
    );
    assert_eq!(
        read_model.rows[0].report_receipt_ref.as_deref(),
        Some(SOCIAL_REPORT_WRITER_DELIVERY_REPORT_RECEIPT_REF)
    );
    assert!(!read_model.rows[0].parent_notification_ui_delivered);
    assert!(!read_model.rows[0].provider_delivery_attempted);
    assert!(!read_model.rows[0].provider_receipt_ingested);
    assert!(!read_model.rows[0].final_policy_decision_claimed);
    assert!(!read_model.rows[0].enforcement_claimed);
    assert!(!read_model.parent_notification_ui_delivery_claimed);
    assert!(!read_model.external_runtime_report_delivery_claimed);
    assert!(!read_model.final_policy_execution_claimed);
    assert!(!read_model.enforcement_claimed);
}

fn command_envelope() -> AgentCommandEnvelope {
    AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id:
            constants::event_id::BROWSER_SOCIAL_PARENT_NOTIFICATION_DELIVERY_READ_MODEL_REPORTED
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
        command: AgentCommandName::AgentBrowserSocialParentNotificationDeliveryReadModelGet,
        payload: LogFields::new(),
    }
}

fn read_model_payload(value: &LogFieldValue) -> SocialParentNotificationDeliveryReadinessSnapshot {
    match value {
        LogFieldValue::String(text) => {
            serde_json::from_str(text).expect_value(constants::error::AGENT_EVENT_SERIALIZES)
        }
        _ => std::process::abort(),
    }
}
