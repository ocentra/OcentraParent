use ocentra_parent_agent_protocol::network_flow::{
    NetworkAiAdvisoryState, NetworkInterventionState, NetworkPolicyDecisionAction,
    NetworkRuntimeEventPayload,
};

pub(crate) fn ai_advisory_state(payload: &NetworkRuntimeEventPayload) -> NetworkAiAdvisoryState {
    match payload.intervention_state {
        NetworkInterventionState::DryRunOnly => NetworkAiAdvisoryState::Completed,
        NetworkInterventionState::ManualRequired => NetworkAiAdvisoryState::ManualReviewRequired,
        NetworkInterventionState::Unavailable => NetworkAiAdvisoryState::ProviderUnavailable,
    }
}

pub(crate) fn policy_decision_action(
    payload: &NetworkRuntimeEventPayload,
) -> NetworkPolicyDecisionAction {
    match payload.intervention_state {
        NetworkInterventionState::DryRunOnly => NetworkPolicyDecisionAction::Observe,
        NetworkInterventionState::ManualRequired => NetworkPolicyDecisionAction::ManualReview,
        NetworkInterventionState::Unavailable => NetworkPolicyDecisionAction::Unknown,
    }
}
