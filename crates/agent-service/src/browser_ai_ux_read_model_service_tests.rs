use ocentra_parent_agent_protocol::{
    constants, AgentCommandEnvelope, AgentCommandName, AgentEventName, AgentMessageTarget,
    AgentPeer, AgentPeerRole, AgentRoute, BrowserAiUxReadModel, LogFields,
    AGENT_PROTOCOL_SCHEMA_VERSION,
};

use crate::{lan_pairing::LanPairingRuntime, websocket::handle_command_text_for_test};

#[tokio::test]
async fn browser_ai_ux_read_model_command_reports_service_backed_manual_required_rows() {
    let body =
        serde_json::to_string(&command_envelope()).expect(constants::error::AGENT_EVENT_SERIALIZES);
    let event = handle_command_text_for_test(&body, LanPairingRuntime::empty(), None).await;
    let read_model = browser_ai_ux_read_model_payload(
        &event.payload[constants::field::BROWSER_AI_UX_READ_MODEL],
    );

    assert_eq!(event.event, AgentEventName::AgentBrowserAiUxReadModelReported);
    assert_eq!(read_model.returned, 2);
    assert_eq!(
        read_model.rows[0].child_state,
        constants::browser_ai_ux_read_model::CHILD_STATE_CHECKING
    );
    assert_eq!(
        read_model.rows[1].parent_explanation_state,
        constants::browser_ai_ux_read_model::PARENT_STATE_MANUAL_REQUIRED
    );
    assert_eq!(read_model.rows[0].runtime_delivery_claimed, false);
    assert_eq!(read_model.rows[0].rendered_ui_claimed, false);
    assert_eq!(read_model.rows[0].direct_enforcement_claimed, false);
}

fn command_envelope() -> AgentCommandEnvelope {
    AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: constants::browser_ai_ux_read_model::COMMAND_MESSAGE_ID.to_string(),
        sent_at: constants::browser_ai_ux_read_model::TEST_SENT_AT.to_string(),
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
        command: AgentCommandName::AgentBrowserAiUxReadModelGet,
        payload: LogFields::new(),
    }
}

fn browser_ai_ux_read_model_payload(
    value: &ocentra_parent_agent_protocol::LogFieldValue,
) -> BrowserAiUxReadModel {
    match value {
        ocentra_parent_agent_protocol::LogFieldValue::String(text) => {
            serde_json::from_str(text).expect(constants::error::AGENT_EVENT_SERIALIZES)
        }
        _ => std::panic::panic_any(constants::error::AGENT_EVENT_SERIALIZES),
    }
}
