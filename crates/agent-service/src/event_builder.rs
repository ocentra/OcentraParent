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
