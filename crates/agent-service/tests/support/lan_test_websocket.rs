use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogLevel};
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentEventEnvelope, AgentEventName,
};

use crate::{
    event_builder::{build_event, portal_peer},
    fields::fields_from_pairs,
    lan_pairing::LanPairingRuntime,
    lan_test_websocket_dispatch::dispatch_lan_test_command,
    test_text::TestText,
};

pub(crate) async fn handle_command_text_for_test(
    text: TestText,
    lan_pairing: LanPairingRuntime,
    origin: Option<TestText>,
) -> AgentEventEnvelope {
    let text: &str = text.as_ref();
    if text.len() > constants::lan_pairing::LAN_WEBSOCKET_COMMAND_MAX_BYTES {
        return oversized_command_text_rejected();
    }

    match serde_json::from_str::<AgentCommandEnvelope>(text) {
        Ok(command) => dispatch_lan_test_command(lan_pairing, origin, command).await,
        Err(error) => build_event(
            constants::event_id::COMMAND_REJECTED,
            constants::event_id::UNKNOWN_COMMAND,
            portal_peer(),
            AgentEventName::AgentCommandRejected,
            LogLevel::Warn,
            fields_from_pairs(vec![(
                constants::field::REASON,
                LogFieldValue::String(error.to_string()),
            )]),
            None,
        ),
    }
}

fn oversized_command_text_rejected() -> AgentEventEnvelope {
    build_event(
        constants::event_id::COMMAND_REJECTED,
        constants::event_id::UNKNOWN_COMMAND,
        portal_peer(),
        AgentEventName::AgentCommandRejected,
        LogLevel::Warn,
        fields_from_pairs(vec![
            (
                constants::field::LAN_CONTROL_STATE,
                LogFieldValue::String(constants::value::LAN_CONTROL_REJECTED.to_string()),
            ),
            (
                constants::field::LAN_AUDIT_EVENT_TYPE,
                LogFieldValue::String(constants::value::LAN_AUDIT_CONTROL_REJECTED.to_string()),
            ),
            (
                constants::field::LAN_REJECTION_REASON,
                LogFieldValue::String(constants::value::LAN_REASON_PAYLOAD_TOO_LARGE.to_string()),
            ),
            (
                constants::field::LAN_AUTHENTICATION_STATE,
                LogFieldValue::String(constants::value::LAN_AUTH_UNAUTHENTICATED.to_string()),
            ),
            (
                constants::field::REASON,
                LogFieldValue::String(constants::value::LAN_REASON_PAYLOAD_TOO_LARGE.to_string()),
            ),
        ]),
        None,
    )
}
