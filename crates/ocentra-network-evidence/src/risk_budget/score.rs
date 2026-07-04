mod age_pressure;
mod calculate;
mod helpers;
mod intervention;
mod state;
mod threshold_points;

use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct NetworkRiskBudgetScore {
    pub(super) total_risk_points: u16,
    pub(super) age_profile_points: u16,
    pub(super) active_signal_points: u16,
    pub(super) prior_event_points: u16,
    pub(super) safe_behavior_credit_applied_points: u16,
    pub(super) cited_signal_refs: Vec<String>,
    pub(super) cited_audit_refs: Vec<String>,
    pub(super) cited_evidence_refs: Vec<String>,
    pub(super) cited_prior_event_refs: Vec<String>,
}

pub(super) fn calculate_score(input: &NetworkRiskBudgetThresholdInput) -> NetworkRiskBudgetScore {
    calculate::calculate_score(input)
}

pub(super) fn intervention_state(
    input: &NetworkRiskBudgetThresholdInput,
    risk_budget_state: NetworkRiskBudgetState,
) -> NetworkInterventionState {
    intervention::intervention_state(input, risk_budget_state)
}

pub(super) fn score_state(
    total_risk_points: u16,
    thresholds: &NetworkRiskBudgetThresholds,
) -> NetworkRiskBudgetState {
    state::score_state(total_risk_points, thresholds)
}

pub(super) fn threshold_points_for_state(
    state: NetworkRiskBudgetState,
    thresholds: &NetworkRiskBudgetThresholds,
) -> u16 {
    threshold_points::threshold_points_for_state(state, thresholds)
}

pub(super) fn age_pressure_points(age_band: NetworkRiskBudgetAgeBand) -> u32 {
    age_pressure::age_pressure_points(age_band)
}

pub(super) fn push_unique(values: &mut Vec<String>, value: &str) {
    helpers::push_unique(values, value)
}
