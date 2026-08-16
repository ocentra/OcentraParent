#![forbid(unsafe_code)]

use super::super::{
    assert_utc_timestamp, PolicyContractScheduleTimeBudgetStatus, PolicyContractValidationResult,
};

mod bonus_time;
mod offline_recovery;

pub(super) fn validate_policy_schedule_time_budget_status(
    time_budget: &PolicyContractScheduleTimeBudgetStatus,
    evaluated_at: &str,
) -> PolicyContractValidationResult {
    if time_budget.budget_window_minutes == 0 {
        return Err("timeBudget.budgetWindowMinutes must be a positive number".into());
    }
    assert_utc_timestamp(&time_budget.reset_at, "timeBudget.resetAt")?;
    if time_budget.reset_at.as_str() <= evaluated_at {
        return Err("timeBudget.resetAt must be after evaluatedAt".into());
    }

    offline_recovery::validate(time_budget)?;
    bonus_time::validate(time_budget, evaluated_at)
}
