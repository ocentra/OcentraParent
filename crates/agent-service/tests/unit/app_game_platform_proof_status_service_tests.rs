use ocentra_parent_agent_protocol::app_game_platform_proof_status::APP_GAME_PLATFORM_PROOF_STATUS_READ_MODEL_ID;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentCommandName, AgentEventName, AgentMessageTarget, AgentPeer,
    AgentPeerRole, AgentRoute,
};
use ocentra_parent_agent_protocol::AppGamePlatformProofStatusReadModel;
use ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION;

use crate::test_invariants::{require_json_decode, require_log_string_field, serialize_test_json};
use ocentra_parent_agent_service::test_support::handle_local_command_text_for_test;

const APP_GAME_TEST_TIMESTAMP: &str = "2026-06-03T22:15:00Z";

#[tokio::test]
async fn platform_proof_status_command_reports_live_read_model() {
    let body = serialize_test_json(&command_envelope());
    let event =
        handle_local_command_text_for_test(crate::test_text::TestText::from_display(body)).await;

    assert_eq!(
        event.event,
        AgentEventName::AgentActivityAppGamePlatformProofStatusReadModelReported
    );
    let read_model = platform_proof_status_payload(&crate::test_invariants::log_field(
        &event.payload,
        constants::field::APP_GAME_PLATFORM_PROOF_STATUS_READ_MODEL,
        constants::error::AGENT_EVENT_SERIALIZES,
    ));

    assert_eq!(
        read_model.read_model_id,
        APP_GAME_PLATFORM_PROOF_STATUS_READ_MODEL_ID
    );
    assert_eq!(read_model.returned, 5);
    assert_eq!(read_model.enforcement_ready_count, 1);
}

fn command_envelope() -> AgentCommandEnvelope {
    AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id:
            constants::event_id::ACTIVITY_APP_GAME_PLATFORM_PROOF_STATUS_READ_MODEL_REPORTED
                .to_string(),
        sent_at: APP_GAME_TEST_TIMESTAMP.to_string(),
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
        command: AgentCommandName::AgentActivityAppGamePlatformProofStatusReadModelGet,
        payload: LogFields::new(),
    }
}

fn platform_proof_status_payload(value: &LogFieldValue) -> AppGamePlatformProofStatusReadModel {
    let text = require_log_string_field(Some(value), constants::error::AGENT_EVENT_SERIALIZES);
    require_json_decode(text, constants::error::AGENT_EVENT_SERIALIZES)
}
