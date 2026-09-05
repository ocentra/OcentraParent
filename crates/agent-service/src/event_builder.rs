#[path = "event_builder/build.rs"]
mod build;
#[path = "event_builder/portal_peer.rs"]
mod portal_peer;

pub fn build_event<S, C>(
    event_id_suffix: S,
    correlation_id: C,
    target: ocentra_parent_agent_protocol::transport::AgentPeer,
    event: ocentra_parent_agent_protocol::transport::AgentEventName,
    severity: ocentra_parent_agent_protocol::logging::LogLevel,
    payload: ocentra_parent_agent_protocol::logging::LogFields,
    snapshot: Option<ocentra_parent_agent_protocol::logging::AgentLogSnapshot>,
) -> ocentra_parent_agent_protocol::transport::AgentEventEnvelope
where
    S: build::IntoEventIdSuffixText,
    C: build::IntoEventCorrelationIdText,
{
    build::build_event(
        event_id_suffix,
        correlation_id,
        target,
        event,
        severity,
        payload,
        snapshot,
    )
}

pub fn portal_peer() -> ocentra_parent_agent_protocol::transport::AgentPeer {
    portal_peer::portal_peer()
}
