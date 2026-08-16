use ocentra_parent_agent_protocol::network_flow::{
    NetworkAiAdvisoryState, NetworkInterventionState, NetworkRuntimeEventPayload,
};

pub(crate) fn ai_advisory_state(payload: &NetworkRuntimeEventPayload) -> NetworkAiAdvisoryState {
    match payload.intervention_state {
        NetworkInterventionState::DryRunOnly => NetworkAiAdvisoryState::Completed,
        NetworkInterventionState::ManualRequired => NetworkAiAdvisoryState::ManualReviewRequired,
        NetworkInterventionState::Unavailable => NetworkAiAdvisoryState::ProviderUnavailable,
    }
}
