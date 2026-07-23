use super::{
    NetworkEvidenceGrade, NetworkInterventionState, NetworkPolicyDecisionAction,
    NetworkRiskBudgetState, NetworkRuntimeEvidenceGrade,
};

pub(super) fn expected(
    evidence_grade: NetworkRuntimeEvidenceGrade,
) -> (
    NetworkEvidenceGrade,
    NetworkRiskBudgetState,
    NetworkInterventionState,
    NetworkPolicyDecisionAction,
) {
    match evidence_grade {
        NetworkRuntimeEvidenceGrade::DomainAndProcessMetadata => (
            NetworkEvidenceGrade::B,
            NetworkRiskBudgetState::ObserveOnly,
            NetworkInterventionState::DryRunOnly,
            NetworkPolicyDecisionAction::Observe,
        ),
        NetworkRuntimeEvidenceGrade::IpOrProcessPartialMetadata => (
            NetworkEvidenceGrade::C,
            NetworkRiskBudgetState::ManualReviewRequired,
            NetworkInterventionState::ManualRequired,
            NetworkPolicyDecisionAction::AskParent,
        ),
        NetworkRuntimeEvidenceGrade::AdapterUnavailable => (
            NetworkEvidenceGrade::D,
            NetworkRiskBudgetState::Unavailable,
            NetworkInterventionState::Unavailable,
            NetworkPolicyDecisionAction::Unknown,
        ),
    }
}
