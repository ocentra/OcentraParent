use super::super::*;

pub(super) fn threshold_points_for_state(
    state: NetworkRiskBudgetState,
    thresholds: &NetworkRiskBudgetThresholds,
) -> u16 {
    match state {
        NetworkRiskBudgetState::WithinBudget => 0,
        NetworkRiskBudgetState::MonitorThreshold => thresholds.monitor_points,
        NetworkRiskBudgetState::AskParentThreshold => thresholds.ask_parent_points,
        NetworkRiskBudgetState::WarnChildThreshold => thresholds.warn_child_points,
        NetworkRiskBudgetState::LimitThreshold => thresholds.limit_points,
        NetworkRiskBudgetState::BlockThreshold => thresholds.block_points,
    }
}
