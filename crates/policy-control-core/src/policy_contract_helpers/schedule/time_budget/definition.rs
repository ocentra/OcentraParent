#![forbid(unsafe_code)]

use super::super::{
    assert_local_time, assert_utc_timestamp, PolicyContractScheduleBudgetResetKind,
    PolicyContractScheduleTimeBudget, PolicyContractValidationResult,
};

mod carryover;

pub(super) fn validate_policy_schedule_time_budget(
    time_budget: &PolicyContractScheduleTimeBudget,
) -> PolicyContractValidationResult {
    assert_local_time(&time_budget.reset.local_time, "timeBudget.reset.localTime")?;
    assert_utc_timestamp(&time_budget.effective_from, "timeBudget.effectiveFrom")?;

    if let Some(effective_until) = &time_budget.effective_until {
        assert_utc_timestamp(effective_until, "timeBudget.effectiveUntil")?;
        if effective_until <= &time_budget.effective_from {
            return Err("timeBudget.effectiveUntil must be after timeBudget.effectiveFrom".into());
        }
    }

    match time_budget.reset.kind {
        PolicyContractScheduleBudgetResetKind::Weekly => {
            if time_budget.reset.day.is_none() {
                return Err("weekly reset rules require timeBudget.reset.day".into());
            }
        }
        PolicyContractScheduleBudgetResetKind::Daily
        | PolicyContractScheduleBudgetResetKind::Monthly => {
            if time_budget.reset.day.is_some() {
                return Err("non-weekly reset rules cannot set timeBudget.reset.day".into());
            }
        }
    }

    carryover::validate_policy_schedule_budget_carryover(time_budget)?;

    Ok(())
}
