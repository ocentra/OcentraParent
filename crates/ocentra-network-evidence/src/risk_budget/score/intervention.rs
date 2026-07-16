use super::super::*;

pub(super) fn intervention_state(
    input: &NetworkRiskBudgetThresholdInput,
    risk_budget_state: NetworkRiskBudgetState,
) -> NetworkInterventionState {
    match risk_budget_state {
        NetworkRiskBudgetState::WithinBudget => NetworkInterventionState::Ignore,
        NetworkRiskBudgetState::MonitorThreshold => NetworkInterventionState::Monitor,
        NetworkRiskBudgetState::AskParentThreshold => NetworkInterventionState::AskParent,
        NetworkRiskBudgetState::WarnChildThreshold => {
            if input.household_policy.child_warning_allowed {
                NetworkInterventionState::WarnChild
            } else {
                NetworkInterventionState::AskParent
            }
        }
        NetworkRiskBudgetState::LimitThreshold => {
            if input.household_policy.limit_policy_allowed && adapter_control_ready(input) {
                NetworkInterventionState::Limit
            } else {
                NetworkInterventionState::ManualRequired
            }
        }
        NetworkRiskBudgetState::BlockThreshold => {
            if input.household_policy.block_policy_allowed
                && input.household_policy.strict_block_policy_enabled
                && adapter_control_ready(input)
            {
                NetworkInterventionState::Block
            } else {
                NetworkInterventionState::ManualRequired
            }
        }
    }
}

fn adapter_control_ready(input: &NetworkRiskBudgetThresholdInput) -> bool {
    input.adapter_proof_state == NetworkRiskBudgetAdapterProofState::Ready
        && input.signals.iter().any(|signal| {
            signal.evidence_tier == NetworkRiskBudgetEvidenceTier::AdapterProofReady
                && !signal.signature_only
        })
}
