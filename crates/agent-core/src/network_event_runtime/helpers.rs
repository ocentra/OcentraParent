use ocentra_parent_agent_protocol::activity_capture::ActivityCaptureCapabilityStatus;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::network_flow::{
    NetworkInterventionState, NetworkRiskBudgetState, NetworkRuntimePhase,
};

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
    value
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
