use ocentra_network_core::network_runtime::NetworkRuntimeDecision;
use ocentra_parent_agent_protocol::network_flow::NetworkRuntimePhase;

use crate::NetworkObservation;

use ocentra_eventing::{error::EventingError, ids::EventId};

use super::helpers;

pub(super) fn should_publish_phase_for_runtime_decision(
    phase: NetworkRuntimePhase,
    observation: &NetworkObservation,
    decision: &NetworkRuntimeDecision,
) -> bool {
    let _ = decision;
    match phase {
        NetworkRuntimePhase::FlowObserved | NetworkRuntimePhase::ActivityClassified => true,
        NetworkRuntimePhase::DomainObserved => observation.destination_domain.is_some(),
        _ => false,
    }
}

pub(super) fn network_runtime_decision_from_observation(
    observation: &NetworkObservation,
) -> NetworkRuntimeDecision {
    super::decision_input::network_runtime_decision_from_observation(observation)
}

pub(super) fn network_runtime_event_ids_for_source_event(
    source_event_id: &EventId,
    observation: &NetworkObservation,
) -> Result<Vec<EventId>, EventingError> {
    let decision = network_runtime_decision_from_observation(observation);
    NetworkRuntimePhase::ordered_chain()
        .iter()
        .copied()
        .filter(|phase| should_publish_phase_for_runtime_decision(*phase, observation, &decision))
        .map(|phase| helpers::network_event_id(phase, source_event_id.as_str()))
        .collect()
}
