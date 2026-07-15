mod claims;
mod input;
mod policy;
mod signal;

use super::*;

pub(super) fn validate_input(
    input: &NetworkRiskBudgetThresholdInput,
) -> Result<(), NetworkRiskBudgetThresholdError> {
    input::validate_input(input)
}

pub(super) fn validate_claims(
    input: &NetworkRiskBudgetThresholdInput,
) -> Result<(), NetworkRiskBudgetThresholdError> {
    claims::validate_claims(input)
}

pub(super) fn validate_thresholds(
    thresholds: &NetworkRiskBudgetThresholds,
) -> Result<(), NetworkRiskBudgetThresholdError> {
    policy::validate_thresholds(thresholds)
}

pub(super) fn validate_policy(
    policy: &NetworkRiskBudgetHouseholdPolicy,
) -> Result<(), NetworkRiskBudgetThresholdError> {
    policy::validate_policy(policy)
}

pub(super) fn validate_safe_behavior_credit(
    policy: &NetworkRiskBudgetHouseholdPolicy,
) -> Result<(), NetworkRiskBudgetThresholdError> {
    policy::validate_safe_behavior_credit(policy)
}

pub(super) fn validate_signal(
    signal: &NetworkRiskBudgetSignal,
) -> Result<(), NetworkRiskBudgetThresholdError> {
    signal::validate_signal(signal)
}
