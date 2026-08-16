#![forbid(unsafe_code)]

use ocentra_eventing::error::EventingError;
use ocentra_parent_agent_protocol::constants::policy_control;

use crate::policy_source::{PolicyScheduleBudgetResetKind, PolicyScheduleTimeBudget};

pub(crate) fn assert_schedule_time_budget_reset(
    budget: &PolicyScheduleTimeBudget,
) -> Result<(), EventingError> {
    assert_schedule_time_budget_reset_day(budget.reset.kind, budget.reset.day.is_some())
}

fn assert_schedule_time_budget_reset_day(
    kind: PolicyScheduleBudgetResetKind,
    has_day: bool,
) -> Result<(), EventingError> {
    match kind {
        PolicyScheduleBudgetResetKind::Weekly => assert_weekly_reset_day(has_day),
        PolicyScheduleBudgetResetKind::Daily | PolicyScheduleBudgetResetKind::Monthly => {
            assert_non_weekly_reset_day(has_day)
        }
    }
}

fn assert_weekly_reset_day(has_day: bool) -> Result<(), EventingError> {
    if has_day {
        return Ok(());
    }

    Err(EventingError::InvalidValue {
        field: policy_control::source::FIELD_SCHEDULE_RESET_DAY,
        value: "missing-weekly-reset-day".to_string(),
    })
}

fn assert_non_weekly_reset_day(has_day: bool) -> Result<(), EventingError> {
    if !has_day {
        return Ok(());
    }

    Err(EventingError::InvalidValue {
        field: policy_control::source::FIELD_SCHEDULE_RESET_DAY,
        value: "unexpected-reset-day".to_string(),
    })
}
