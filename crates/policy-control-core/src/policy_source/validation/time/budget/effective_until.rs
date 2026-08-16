#![forbid(unsafe_code)]

use ocentra_eventing::error::EventingError;
use ocentra_parent_agent_protocol::constants::policy_control;

use crate::policy_source::PolicyScheduleTimeBudget;

use super::super::format;

pub(crate) fn assert_schedule_time_budget_effective_until(
    budget: &PolicyScheduleTimeBudget,
) -> Result<(), EventingError> {
    if let Some(effective_until) = &budget.effective_until {
        format::assert_utc_timestamp(
            policy_control::source::FIELD_SCHEDULE_EFFECTIVE_UNTIL,
            effective_until,
        )?;
        if effective_until <= &budget.effective_from {
            return Err(EventingError::InvalidValue {
                field: policy_control::source::FIELD_SCHEDULE_EFFECTIVE_UNTIL,
                value: effective_until.clone(),
            });
        }
    }

    Ok(())
}
