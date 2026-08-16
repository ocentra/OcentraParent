use ocentra_eventing::{
    error::EventingError,
    ids::{CorrelationId, EventId},
};
use ocentra_parent_agent_protocol::activity_capture::ActivityCaptureCapabilityStatus;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::network_flow::NetworkRuntimePhase;
use sha2::{Digest, Sha256};

use crate::NetworkObservation;

pub(super) fn event_custody(
    observation: &NetworkObservation,
) -> Result<ocentra_eventing::ids::EventCustody, EventingError> {
    let value = match observation.status {
        ActivityCaptureCapabilityStatus::Available => {
            constants::eventing_source::CUSTODY_LOCAL_QUERY_STORE
        }
        _ => constants::eventing_source::CUSTODY_UNAVAILABLE,
    };
    ocentra_eventing::ids::EventCustody::parse(value)
}

pub(super) fn network_correlation_id(
    observation: &NetworkObservation,
    observed_at: &str,
) -> String {
    let mut value = String::from(constants::network_flow::CORRELATION_NETWORK_RUNTIME_PREFIX);
    value.push_str(observation.status.as_protocol_str());
    value.push(constants::delimiter::HYPHEN);
    value.push_str(observed_at);
    if observation.destination_ip.is_none() && observation.destination_domain.is_none() {
        append_destination_less_identity(&mut value, observation);
    }
    value
}

pub(super) fn network_event_id(
    phase: NetworkRuntimePhase,
    source_event_id: &str,
) -> Result<EventId, ocentra_eventing::error::EventingError> {
    EventId::parse(source_event_id.to_owned())?;
    EventId::parse(network_event_id_string(phase, source_event_id))
}

pub(super) fn network_event_id_string(phase: NetworkRuntimePhase, source_event_id: &str) -> String {
    let mut identity = Vec::new();
    append_framed(&mut identity, phase.event_type());
    append_framed(&mut identity, source_event_id);
    event_id_text(&identity)
}

pub(super) fn network_source_correlation_id(
    source_event_id: &str,
) -> Result<CorrelationId, ocentra_eventing::error::EventingError> {
    let source_event_id = EventId::parse(source_event_id.to_owned())?;
    let mut identity = Vec::new();
    append_framed(&mut identity, "network-runtime-source-correlation-v1");
    append_framed(&mut identity, source_event_id.as_str());
    CorrelationId::parse(format!(
        "{}{:x}",
        constants::network_flow::CORRELATION_NETWORK_RUNTIME_PREFIX,
        Sha256::digest(identity)
    ))
}

pub(super) fn network_fallback_event_id(
    phase: NetworkRuntimePhase,
    observation: &NetworkObservation,
    observed_at: &str,
) -> Result<EventId, ocentra_eventing::error::EventingError> {
    super::identity_payload::network_fallback_event_id(phase, observation, observed_at)
}

pub(super) fn event_id_text(identity: &[u8]) -> String {
    let digest = Sha256::digest(identity);
    format!(
        "{}{:x}",
        constants::network_flow::NETWORK_RUNTIME_EVENT_ID_PREFIX,
        digest
    )
}

fn append_framed(identity: &mut Vec<u8>, value: &str) {
    let bytes = value.as_bytes();
    identity.extend_from_slice(&(bytes.len() as u64).to_be_bytes());
    identity.extend_from_slice(bytes);
}

fn append_destination_less_identity(value: &mut String, observation: &NetworkObservation) {
    let local_port = observation.local_port.map(|port| port.to_string());
    let process_id = observation.pid.map(|pid| pid.to_string());
    for (field, identity) in [
        (
            constants::field::NETWORK_PROTOCOL,
            observation
                .protocol
                .map(|protocol| protocol.as_protocol_str()),
        ),
        (constants::field::LOCAL_IP, observation.local_ip.as_deref()),
        (constants::field::LOCAL_PORT, local_port.as_deref()),
        (
            constants::field::TCP_STATE,
            observation.tcp_state.map(|state| state.as_protocol_str()),
        ),
        (constants::field::PROCESS_ID, process_id.as_deref()),
    ] {
        value.push(constants::delimiter::HYPHEN);
        value.push_str(field);
        value.push(constants::delimiter::COLON);
        if let Some(identity) = identity {
            value.push_str(identity);
        }
    }
}
