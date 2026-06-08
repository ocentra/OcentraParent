use ocentra_parent_agent_protocol::{
    constants, AgentCommandEnvelope, AgentCommandName, AgentEventName, AgentMessageTarget,
    AgentPeer, AgentPeerRole, AgentRoute, AGENT_PROTOCOL_SCHEMA_VERSION,
    APP_GAME_ADAPTER_DISPATCH_RESULT_TEST_COMMAND_ID,
    APP_GAME_ADAPTER_DISPATCH_RESULT_TEST_DEVICE_ID,
    APP_GAME_ADAPTER_DISPATCH_RESULT_TEST_PORTAL_PEER,
    APP_GAME_ADAPTER_DISPATCH_RESULT_TEST_SENT_AT, APP_GAME_PARENT_PLATFORM_WINDOWS,
};

use super::app_game_adapter_dispatch_result_payload::build_activity_app_game_adapter_dispatch_result_report;

#[tokio::test]
async fn app_game_adapter_dispatch_result_command_returns_typed_event() {
    let command = AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: APP_GAME_ADAPTER_DISPATCH_RESULT_TEST_COMMAND_ID.to_string(),
        sent_at: APP_GAME_ADAPTER_DISPATCH_RESULT_TEST_SENT_AT.to_string(),
        source: AgentPeer {
            peer_id: APP_GAME_ADAPTER_DISPATCH_RESULT_TEST_PORTAL_PEER.to_string(),
            role: AgentPeerRole::Portal,
        },
        target: AgentMessageTarget {
            device_id: APP_GAME_ADAPTER_DISPATCH_RESULT_TEST_DEVICE_ID.to_string(),
            platform: APP_GAME_PARENT_PLATFORM_WINDOWS.to_string(),
            route: AgentRoute::Localhost,
        },
        command: AgentCommandName::AgentActivityAppGameAdapterDispatchResultReadModelGet,
        payload: Default::default(),
    };

    let event = build_activity_app_game_adapter_dispatch_result_report(command).await;

    assert_eq!(
        event.event,
        AgentEventName::AgentActivityAppGameAdapterDispatchResultReadModelReported
    );
    assert_eq!(
        event
            .payload
            .get(constants::field::APP_GAME_ADAPTER_DISPATCH_RESULT_READ_MODEL)
            .is_some(),
        true
    );
}
