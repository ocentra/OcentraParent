use ocentra_eventing::ids::{CorrelationId, EventId};
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

pub(super) fn event_custody(
    observation: &NetworkObservation,
) -> Result<ocentra_eventing::ids::EventCustody, ocentra_eventing::error::EventingError> {
    super::identity::event_custody(observation)
}

pub(super) fn network_correlation_id(
    observation: &NetworkObservation,
    observed_at: &str,
) -> String {
    super::identity::network_correlation_id(observation, observed_at)
}

pub(super) fn network_event_id(
    phase: NetworkRuntimePhase,
    source_event_id: &str,
) -> Result<EventId, ocentra_eventing::error::EventingError> {
    super::identity::network_event_id(phase, source_event_id)
}

pub(super) fn network_event_id_string(phase: NetworkRuntimePhase, source_event_id: &str) -> String {
    super::identity::network_event_id_string(phase, source_event_id)
}

pub(super) fn network_source_correlation_id(
    source_event_id: &str,
) -> Result<CorrelationId, ocentra_eventing::error::EventingError> {
    super::identity::network_source_correlation_id(source_event_id)
}

pub(super) fn network_fallback_event_id(
    phase: NetworkRuntimePhase,
    observation: &NetworkObservation,
    observed_at: &str,
) -> Result<EventId, ocentra_eventing::error::EventingError> {
    super::identity::network_fallback_event_id(phase, observation, observed_at)
}

pub(super) fn network_aggregate_key(
    payload: &crate::network_event_runtime::NetworkRuntimeEventPayload,
) -> String {
    super::identity_payload::network_aggregate_key(payload)
}
