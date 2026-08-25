use ocentra_parent_agent_protocol::{
    constants,
    logging::{LogFieldValue, LogLevel},
    transport::{
        AgentCommandEnvelope, AgentCommandName, AgentEventEnvelope, AgentEventName, AgentRoute,
    },
};

use crate::{fields::fields_from_pairs, websocket::WebsocketPeerProvenance};

pub(super) fn transport_route_rejection(
    command: &AgentCommandEnvelope,
    provenance: WebsocketPeerProvenance,
) -> Option<AgentEventEnvelope> {
    // This admission check is scoped to the one command whose handler can
    // expose the server-owned platform probe snapshot. Other WebSocket
    // commands retain their existing route/pairing owners.
    if command.command != AgentCommandName::AgentActivityAppGamePlatformProofStatusReadModelGet {
        return None;
    }
    let admissible = match provenance {
        // Loopback is the local portal route; it cannot mint a LAN route.
        WebsocketPeerProvenance::Loopback => command.target.route == AgentRoute::Localhost,
        WebsocketPeerProvenance::LocalNetwork => command.target.route == AgentRoute::LocalNetwork,
        WebsocketPeerProvenance::Unknown => false,
    };
    if admissible {
        None
    } else {
        Some(build_transport_route_rejection(command))
    }
}

fn build_transport_route_rejection(command: &AgentCommandEnvelope) -> AgentEventEnvelope {
    crate::event_builder::build_event(
        constants::event_id::COMMAND_REJECTED,
        &command.message_id,
        // CLONE-JUSTIFICATION: the rejection event owns a source snapshot
        // while admission retains the original envelope for nonce binding.
        command.source.clone(),
        AgentEventName::AgentCommandRejected,
        LogLevel::Warn,
        fields_from_pairs(vec![
            // ALLOC-JUSTIFICATION: fixed protocol rejection fields are owned
            // by the event payload boundary.
            (
                constants::field::LAN_CONTROL_STATE,
                LogFieldValue::String(constants::value::LAN_CONTROL_REJECTED.to_string()),
            ),
            (
                constants::field::LAN_AUDIT_EVENT_TYPE,
                // ALLOC-JUSTIFICATION: fixed protocol rejection fields are
                // owned by the event payload boundary.
                LogFieldValue::String(constants::value::LAN_AUDIT_CONTROL_REJECTED.to_string()),
            ),
            (
                constants::field::LAN_REJECTION_REASON,
                // ALLOC-JUSTIFICATION: fixed protocol rejection fields are
                // owned by the event payload boundary.
                LogFieldValue::String(constants::value::LAN_REASON_UNSUPPORTED_ROUTE.to_string()),
            ),
            (
                constants::field::LAN_AUTHENTICATION_STATE,
                // ALLOC-JUSTIFICATION: fixed protocol rejection fields are
                // owned by the event payload boundary.
                LogFieldValue::String(constants::value::LAN_AUTH_UNAUTHENTICATED.to_string()),
            ),
            (
                constants::field::REASON,
                // ALLOC-JUSTIFICATION: fixed protocol rejection fields are
                // owned by the event payload boundary.
                LogFieldValue::String(constants::value::LAN_REASON_UNSUPPORTED_ROUTE.to_string()),
            ),
        ]),
        None,
    )
}
