use ocentra_parent_agent_protocol::activity_capture::ActivityCaptureCapabilityStatus;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::network_flow::{
    NetworkInterventionState, NetworkRiskBudgetState, NetworkRuntimePhase,
};
use ocentra_eventing::ids::EventId;
use sha2::{Digest, Sha256};

use crate::NetworkObservation;

pub(super) fn intervention_state_from_budget(
    risk_budget_state: &NetworkRiskBudgetState,
) -> NetworkInterventionState {
    match risk_budget_state {
        NetworkRiskBudgetState::ObserveOnly => NetworkInterventionState::DryRunOnly,
        NetworkRiskBudgetState::ManualReviewRequired => NetworkInterventionState::ManualRequired,
        NetworkRiskBudgetState::Unavailable => NetworkInterventionState::Unavailable,
    }
}

pub(super) fn should_publish_phase(
    phase: NetworkRuntimePhase,
    _observation: &NetworkObservation,
) -> bool {
    !matches!(
        phase,
        NetworkRuntimePhase::EnforcementCommandIssued
            | NetworkRuntimePhase::EnforcementResultObserved
    )
}

pub(super) fn event_custody(
    observation: &NetworkObservation,
) -> ocentra_eventing::ids::EventCustody {
    let value = if observation.status == ActivityCaptureCapabilityStatus::Available {
        constants::eventing_source::CUSTODY_LOCAL_QUERY_STORE
    } else {
        constants::eventing_source::CUSTODY_UNAVAILABLE
    };
    ocentra_eventing::ids::EventCustody::parse(value).unwrap_or_else(|_| std::process::abort())
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
    observation: &NetworkObservation,
    observed_at: &str,
) -> Result<EventId, ocentra_eventing::error::EventingError> {
    let mut identity = String::new();
    identity.push_str(phase.event_type());
    identity.push('|');
    identity.push_str(observed_at);
    identity.push('|');
    identity.push_str(observation.status.as_protocol_str());
    for value in [
        observation.protocol.map(|value| value.as_protocol_str().to_owned()),
        observation.local_ip.clone(),
        observation.local_port.map(|value| value.to_string()),
        observation.destination_ip.clone(),
        observation.destination_port.map(|value| value.to_string()),
        observation.destination_domain.clone(),
        observation.tcp_state.map(|value| value.as_protocol_str().to_owned()),
        observation.pid.map(|value| value.to_string()),
        observation.process_name.clone(),
        Some(observation.associated_pid_count.to_string()),
    ] {
        identity.push('|');
        if let Some(value) = value {
            identity.push_str(&value);
        }
    }
    let digest = Sha256::digest(identity.as_bytes());
    EventId::parse(format!(
        "{}{:x}",
        constants::network_flow::NETWORK_RUNTIME_EVENT_ID_PREFIX,
        digest
    ))
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

pub(super) fn network_aggregate_key(
    payload: &crate::network_event_runtime::NetworkRuntimeEventPayload,
) -> String {
    let mut value = String::from(constants::network_flow::AGGREGATE_NETWORK_FLOW_PREFIX);
    if let Some(domain) = &payload.destination_domain {
        value.push_str(domain);
        return value;
    }
    if let Some(ip) = &payload.destination_ip {
        value.push_str(ip);
        if let Some(port) = payload.destination_port {
            value.push(constants::delimiter::HYPHEN);
            value.push_str(&port.to_string());
        }
        return value;
    }
    value.push_str(payload.capability_status.as_protocol_str());
    value
}
