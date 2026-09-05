use ocentra_eventing::ids::EventId;
use ocentra_parent_agent_protocol::{
    constants,
    network_flow::{NetworkRuntimeEventPayload, NetworkRuntimePhase},
};

use crate::NetworkObservation;

pub(super) fn network_fallback_event_id(
    phase: NetworkRuntimePhase,
    observation: &NetworkObservation,
    observed_at: &str,
) -> Result<EventId, ocentra_eventing::error::EventingError> {
    let mut identity = Vec::new();
    append_framed(&mut identity, phase.event_type());
    append_framed(&mut identity, observed_at);
    append_framed(&mut identity, observation.status.as_protocol_str());
    append_optional(
        &mut identity,
        observation.protocol.map(|value| value.as_protocol_str()),
    );
    append_optional(&mut identity, observation.local_ip.as_deref());
    append_optional_number(&mut identity, observation.local_port);
    append_optional(&mut identity, observation.destination_ip.as_deref());
    append_optional_number(&mut identity, observation.destination_port);
    append_optional(&mut identity, observation.destination_domain.as_deref());
    append_optional(
        &mut identity,
        observation.tcp_state.map(|value| value.as_protocol_str()),
    );
    append_optional_number(&mut identity, observation.pid);
    append_optional(&mut identity, observation.process_name.as_deref());
    append_framed(&mut identity, &observation.associated_pid_count.to_string());
    EventId::parse(super::identity::event_id_text(&identity))
}

pub(super) fn network_aggregate_key(payload: &NetworkRuntimeEventPayload) -> String {
    let mut value = String::from(constants::network_flow::AGGREGATE_NETWORK_FLOW_PREFIX);
    match (&payload.destination_domain, &payload.destination_ip) {
        (Some(domain), _) => value.push_str(domain),
        (None, Some(ip)) => append_ip_key(&mut value, ip, payload.destination_port),
        (None, None) => value.push_str(payload.capability_status.as_protocol_str()),
    }
    value
}

fn append_ip_key(value: &mut String, ip: &str, port: Option<u16>) {
    value.push_str(ip);
    if let Some(port) = port {
        value.push(constants::delimiter::HYPHEN);
        value.push_str(&port.to_string());
    }
}

fn append_framed(identity: &mut Vec<u8>, value: &str) {
    let bytes = value.as_bytes();
    identity.extend_from_slice(&(bytes.len() as u64).to_be_bytes());
    identity.extend_from_slice(bytes);
}

fn append_optional(identity: &mut Vec<u8>, value: Option<&str>) {
    match value {
        Some(value) => {
            identity.push(1);
            append_framed(identity, value);
        }
        None => identity.push(0),
    }
}

fn append_optional_number<T: ToString>(identity: &mut Vec<u8>, value: Option<T>) {
    match value {
        Some(value) => {
            identity.push(1);
            append_framed(identity, &value.to_string());
        }
        None => identity.push(0),
    }
}
