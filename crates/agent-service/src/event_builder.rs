use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{AgentLogSnapshot, LogFields, LogLevel};
use ocentra_parent_agent_protocol::transport::{
    AgentEventEnvelope, AgentEventName, AgentPeer, AgentPeerRole,
};
use ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION;

use crate::time::timestamp_now;

pub fn build_event(
    event_id_suffix: &str,
    correlation_id: &str,
    target: AgentPeer,
    event: AgentEventName,
    severity: LogLevel,
    payload: LogFields,
    snapshot: Option<AgentLogSnapshot>,
) -> AgentEventEnvelope {
    let mut event_id = String::from(event_id_suffix);
    event_id.push(constants::delimiter::HYPHEN);
    event_id.push_str(&std::process::id().to_string());

    AgentEventEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        event_id,
        correlation_id: correlation_id.to_string(),
        sent_at: timestamp_now(),
        source: AgentPeer {
            peer_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            role: AgentPeerRole::AgentService,
        },
        target,
        event,
        severity,
        payload,
        snapshot,
    }
}

pub fn portal_peer() -> AgentPeer {
    AgentPeer {
        peer_id: constants::peer::PORTAL_DEV.to_string(),
        role: AgentPeerRole::Portal,
    }
}

#[cfg(test)]
mod tests {
    use ocentra_parent_agent_protocol::constants;
    use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogLevel};
    use ocentra_parent_agent_protocol::transport::AgentEventName;

    use crate::{
        event_builder::{build_event, portal_peer},
        fields::fields_from_pairs,
    };

    #[test]
    fn build_event_targets_portal_peer_without_inline_literals() {
        let event = build_event(
            constants::event_id::HEALTH_REPORTED,
            constants::event_id::HEALTH_REPORTED,
            portal_peer(),
            AgentEventName::AgentHealthReported,
            LogLevel::Info,
            fields_from_pairs(vec![(
                constants::field::ONLINE,
                LogFieldValue::Boolean(true),
            )]),
            None,
        );

        assert_eq!(event.target.peer_id, constants::peer::PORTAL_DEV);
        assert!(event.payload.contains_key(constants::field::ONLINE));
    }
}
