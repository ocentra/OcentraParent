#![forbid(unsafe_code)]

use ocentra_eventing::error::EventingError;

use crate::policy_source::PolicyScheduleTimeBudget;

mod budget;
pub(crate) mod format;

pub(crate) fn assert_schedule_time_budget(
    budget: &PolicyScheduleTimeBudget,
) -> Result<(), EventingError> {
    budget::assert_schedule_time_budget(budget)
}
