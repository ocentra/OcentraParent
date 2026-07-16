#[path = "../../src/event_builder.rs"]
mod event_builder;

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogLevel};
use ocentra_parent_agent_protocol::transport::AgentEventName;

#[test]
fn build_event_targets_portal_peer_without_inline_literals() {
    let event = event_builder::build_event(
        constants::event_id::HEALTH_REPORTED,
        constants::event_id::HEALTH_REPORTED,
        event_builder::portal_peer(),
        AgentEventName::AgentHealthReported,
        LogLevel::Info,
        crate::fields::fields_from_pairs(vec![(
            constants::field::ONLINE,
            LogFieldValue::Boolean(true),
        )]),
        None,
    );

    assert_eq!(event.target.peer_id, constants::peer::PORTAL_DEV);
    assert_eq!(
        event.payload.get(constants::field::ONLINE),
        Some(&LogFieldValue::Boolean(true))
    );
}
