use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{AgentLogSnapshot, LogFields, LogLevel};
use ocentra_parent_agent_protocol::transport::{
    AgentEventEnvelope, AgentEventName, AgentPeer, AgentPeerRole,
};
use ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION;

use crate::time::timestamp_now;

pub struct EventIdSuffixText(pub String);

pub struct EventCorrelationIdText(pub String);

pub trait IntoEventIdSuffixText {
    fn into_event_id_suffix_text(self) -> EventIdSuffixText;
}

pub trait IntoEventCorrelationIdText {
    fn into_event_correlation_id_text(self) -> EventCorrelationIdText;
}

impl IntoEventIdSuffixText for EventIdSuffixText {
    fn into_event_id_suffix_text(self) -> EventIdSuffixText {
        self
    }
}

impl IntoEventCorrelationIdText for EventCorrelationIdText {
    fn into_event_correlation_id_text(self) -> EventCorrelationIdText {
        self
    }
}

impl IntoEventIdSuffixText for &str {
    fn into_event_id_suffix_text(self) -> EventIdSuffixText {
        EventIdSuffixText(self.to_string())
    }
}

impl IntoEventIdSuffixText for String {
    fn into_event_id_suffix_text(self) -> EventIdSuffixText {
        EventIdSuffixText(self)
    }
}

impl IntoEventIdSuffixText for &String {
    fn into_event_id_suffix_text(self) -> EventIdSuffixText {
        EventIdSuffixText(self.clone())
    }
}

impl IntoEventCorrelationIdText for &str {
    fn into_event_correlation_id_text(self) -> EventCorrelationIdText {
        EventCorrelationIdText(self.to_string())
    }
}

impl IntoEventCorrelationIdText for String {
    fn into_event_correlation_id_text(self) -> EventCorrelationIdText {
        EventCorrelationIdText(self)
    }
}

impl IntoEventCorrelationIdText for &String {
    fn into_event_correlation_id_text(self) -> EventCorrelationIdText {
        EventCorrelationIdText(self.clone())
    }
}

pub fn build_event<S, C>(
    event_id_suffix: S,
    correlation_id: C,
    target: AgentPeer,
    event: AgentEventName,
    severity: LogLevel,
    payload: LogFields,
    snapshot: Option<AgentLogSnapshot>,
) -> AgentEventEnvelope
where
    S: IntoEventIdSuffixText,
    C: IntoEventCorrelationIdText,
{
    let mut event_id = event_id_suffix.into_event_id_suffix_text().0;
    event_id.push(constants::delimiter::HYPHEN);
    event_id.push_str(&std::process::id().to_string());

    AgentEventEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        event_id,
        correlation_id: correlation_id.into_event_correlation_id_text().0,
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
