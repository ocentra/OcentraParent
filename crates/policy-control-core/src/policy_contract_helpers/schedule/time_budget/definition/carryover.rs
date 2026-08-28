#![forbid(unsafe_code)]

use super::super::super::{
    PolicyContractScheduleBudgetCarryoverMode, PolicyContractScheduleTimeBudget,
    PolicyContractValidationResult,
};

pub(super) fn validate_policy_schedule_budget_carryover(
    time_budget: &PolicyContractScheduleTimeBudget,
) -> PolicyContractValidationResult {
    match time_budget.carryover.mode {
        PolicyContractScheduleBudgetCarryoverMode::DiscardUnused => {
            if time_budget.carryover.max_minutes.is_some() {
                return Err(
                    "discard-unused carryover cannot set timeBudget.carryover.maxMinutes".into(),
                );
            }
        }
        PolicyContractScheduleBudgetCarryoverMode::CarryForward => {
            if time_budget.carryover.max_minutes == Some(0) {
                return Err(
                    "carry-forward carryover cannot set a zero timeBudget.carryover.maxMinutes"
                        .into(),
                );
            }
        }
        PolicyContractScheduleBudgetCarryoverMode::CapCarryover => {
            if time_budget.carryover.max_minutes.unwrap_or(0) == 0 {
                return Err("cap-carryover requires timeBudget.carryover.maxMinutes".into());
            }
        }
    }

    Ok(())
}
