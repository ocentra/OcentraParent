#![forbid(unsafe_code)]

use ocentra_eventing::error::EventingError;
use ocentra_parent_agent_protocol::constants::policy_control;

use crate::policy_source::PolicyScheduleTimeBudget;

use super::super::format;

pub(crate) fn assert_schedule_time_budget_basics(
    budget: &PolicyScheduleTimeBudget,
) -> Result<(), EventingError> {
    if budget.budget_window_minutes == 0 {
        return Err(EventingError::InvalidValue {
            field: policy_control::source::FIELD_SCHEDULE_BUDGET_WINDOW_MINUTES,
            value: budget.budget_window_minutes.to_string(),
        });
    }

    if budget.bonus_expiry_minutes == 0 {
        return Err(EventingError::InvalidValue {
            field: policy_control::source::FIELD_SCHEDULE_BONUS_EXPIRY_MINUTES,
            value: budget.bonus_expiry_minutes.to_string(),
        });
    }

    format::assert_local_time(
        policy_control::source::FIELD_SCHEDULE_RESET_LOCAL_TIME,
        &budget.reset.local_time,
    )?;
    format::assert_utc_timestamp(
        policy_control::source::FIELD_SCHEDULE_EFFECTIVE_FROM,
        &budget.effective_from,
    )?;
    Ok(())
}
