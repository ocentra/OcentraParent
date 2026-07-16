#![forbid(unsafe_code)]

use ocentra_eventing::error::EventingError;

use crate::policy_source::PolicyScheduleTimeBudget;

mod basics;
mod carryover;
mod effective_until;
mod reset;

pub(crate) fn assert_schedule_time_budget(
    budget: &PolicyScheduleTimeBudget,
) -> Result<(), EventingError> {
    basics::assert_schedule_time_budget_basics(budget)?;
    effective_until::assert_schedule_time_budget_effective_until(budget)?;
    reset::assert_schedule_time_budget_reset(budget)?;
    carryover::assert_schedule_time_budget_carryover(budget)?;
    Ok(())
}
