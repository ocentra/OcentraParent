use ocentra_parent_agent_protocol::{
    constants, policy_constants, AgentCommandEnvelope, AgentCommandName, AgentMessageTarget,
    AgentPeer, AgentPeerRole, AgentRoute, LogFieldValue, LogLevel, AGENT_PROTOCOL_SCHEMA_VERSION,
};

use crate::parent_assistant_api::build_parent_assistant_scaffold_event;

#[test]
fn parent_assistant_message_send_returns_typed_scaffold_event() {
    let command = AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: constants::parent_assistant::DEFAULT_MESSAGE_ID.to_string(),
        sent_at: constants::local_ai_runtime::TEST_CHECKED_AT.to_string(),
        source: AgentPeer {
            peer_id: constants::peer::PORTAL_DEV.to_string(),
            role: AgentPeerRole::Portal,
        },
        target: AgentMessageTarget {
            device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            platform: policy_constants::TEST_PARENT_DEVICE_PLATFORM_WINDOWS.to_string(),
            route: AgentRoute::Localhost,
        },
        command: AgentCommandName::AgentParentAssistantMessageSend,
        payload: Default::default(),
    };

    let event = build_parent_assistant_scaffold_event(command);

    assert_eq!(
        event.event,
        ocentra_parent_agent_protocol::AgentEventName::AgentParentAssistantMessageAccepted
    );
    assert_eq!(event.severity, LogLevel::Warn);
    assert_eq!(
        event.payload[constants::field::PARENT_ASSISTANT_BACKEND_STATE],
        LogFieldValue::String(constants::parent_assistant::BACKEND_STATE_SCAFFOLD_ONLY.to_string())
    );
    assert_eq!(
        event.payload[constants::field::REASON],
        LogFieldValue::String(constants::parent_assistant::BACKEND_NOT_CONNECTED.to_string())
    );
}
