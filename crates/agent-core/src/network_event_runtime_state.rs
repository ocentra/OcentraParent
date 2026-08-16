use ocentra_network_core::network_runtime::NetworkAiHandoffState;
use ocentra_parent_agent_protocol::network_flow::{
    NetworkAiAuditState, NetworkEvidenceGrade, NetworkEvidenceScope, NetworkPolicyDecisionAction,
    NetworkRiskBudgetState, NetworkRuntimeEvidenceGrade, NetworkRuntimePhase,
};

use crate::network_capture::NetworkObservation;

#[path = "network_event_runtime_state/helpers.rs"]
mod helpers;

pub(crate) fn evidence_scope(observation: &NetworkObservation) -> NetworkEvidenceScope {
    helpers::evidence_scope(observation)
}

pub(crate) fn evidence_grade(observation: &NetworkObservation) -> NetworkRuntimeEvidenceGrade {
    helpers::evidence_grade(observation)
}

pub(crate) fn ai_audit_state(
    phase: NetworkRuntimePhase,
    handoff_state: NetworkAiHandoffState,
) -> NetworkAiAuditState {
    helpers::ai_audit_state(phase, handoff_state)
}

pub(crate) fn risk_budget_state(observation: &NetworkObservation) -> NetworkRiskBudgetState {
    helpers::risk_budget_state(observation)
}

pub(crate) fn evidence_grade_contract(observation: &NetworkObservation) -> NetworkEvidenceGrade {
    match evidence_grade(observation) {
        NetworkRuntimeEvidenceGrade::DomainAndProcessMetadata => NetworkEvidenceGrade::B,
        NetworkRuntimeEvidenceGrade::IpOrProcessPartialMetadata => NetworkEvidenceGrade::C,
        NetworkRuntimeEvidenceGrade::AdapterUnavailable => NetworkEvidenceGrade::D,
    }
}

pub(crate) fn policy_action(observation: &NetworkObservation) -> NetworkPolicyDecisionAction {
    // This is the runtime boundary for the evidence policy semantics: monitor,
    // parent review, and no-action/unavailable respectively.
    match risk_budget_state(observation) {
        NetworkRiskBudgetState::ObserveOnly => NetworkPolicyDecisionAction::Observe,
        NetworkRiskBudgetState::ManualReviewRequired => NetworkPolicyDecisionAction::AskParent,
        NetworkRiskBudgetState::Unavailable => NetworkPolicyDecisionAction::Unknown,
    }
}
