use super::super::*;

pub(super) fn score_state(
    total_risk_points: u16,
    thresholds: &NetworkRiskBudgetThresholds,
) -> NetworkRiskBudgetState {
    if total_risk_points >= thresholds.block_points {
        NetworkRiskBudgetState::BlockThreshold
    } else if total_risk_points >= thresholds.limit_points {
        NetworkRiskBudgetState::LimitThreshold
    } else if total_risk_points >= thresholds.warn_child_points {
        NetworkRiskBudgetState::WarnChildThreshold
    } else if total_risk_points >= thresholds.ask_parent_points {
        NetworkRiskBudgetState::AskParentThreshold
    } else if total_risk_points >= thresholds.monitor_points {
        NetworkRiskBudgetState::MonitorThreshold
    } else {
        NetworkRiskBudgetState::WithinBudget
    }
}
