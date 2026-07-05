#![forbid(unsafe_code)]

use super::{
    PolicyContractScheduleTimeBudget, PolicyContractScheduleTimeBudgetStatus,
    PolicyContractValidationResult,
};

mod definition;
mod status;

pub(super) fn validate_policy_schedule_time_budget(
    time_budget: &PolicyContractScheduleTimeBudget,
) -> PolicyContractValidationResult {
    definition::validate_policy_schedule_time_budget(time_budget)
}

pub(super) fn validate_policy_schedule_time_budget_status(
    time_budget: &PolicyContractScheduleTimeBudgetStatus,
    evaluated_at: &str,
) -> PolicyContractValidationResult {
    status::validate_policy_schedule_time_budget_status(time_budget, evaluated_at)
}
