#![forbid(unsafe_code)]

use ocentra_eventing::error::EventingError;
use ocentra_parent_agent_protocol::constants::policy_control;

use crate::policy_source::{
    PolicyScheduleBudgetCarryoverMode, PolicyScheduleBudgetResetKind, PolicyScheduleTimeBudget,
};

pub(crate) mod format;

pub(crate) fn assert_schedule_time_budget(budget: &PolicyScheduleTimeBudget) -> Result<(), EventingError> {
    assert_schedule_time_budget_basics(budget)?;
    assert_schedule_time_budget_effective_until(budget)?;
    assert_schedule_time_budget_reset(budget)?;
    assert_schedule_time_budget_carryover(budget)?;
    Ok(())
}

fn assert_schedule_time_budget_basics(
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

fn assert_schedule_time_budget_effective_until(
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

fn assert_schedule_time_budget_reset(
    budget: &PolicyScheduleTimeBudget,
) -> Result<(), EventingError> {
    match budget.reset.kind {
        PolicyScheduleBudgetResetKind::Weekly => {
            if budget.reset.day.is_none() {
                return Err(EventingError::InvalidValue {
                    field: policy_control::source::FIELD_SCHEDULE_RESET_DAY,
                    value: "missing-weekly-reset-day".to_string(),
                });
            }
        }
        PolicyScheduleBudgetResetKind::Daily | PolicyScheduleBudgetResetKind::Monthly => {
            if budget.reset.day.is_some() {
                return Err(EventingError::InvalidValue {
                    field: policy_control::source::FIELD_SCHEDULE_RESET_DAY,
                    value: "unexpected-reset-day".to_string(),
                });
            }
        }
    }

    Ok(())
}

fn assert_schedule_time_budget_carryover(
    budget: &PolicyScheduleTimeBudget,
) -> Result<(), EventingError> {
    match budget.carryover.mode {
        PolicyScheduleBudgetCarryoverMode::DiscardUnused => {
            if budget.carryover.max_minutes.is_some() {
                return Err(EventingError::InvalidValue {
                    field: policy_control::source::FIELD_SCHEDULE_CARRYOVER_MAX_MINUTES,
                    value: "discard-unused".to_string(),
                });
            }
        }
        PolicyScheduleBudgetCarryoverMode::CapCarryover => {
            if budget.carryover.max_minutes.unwrap_or(0) == 0 {
                return Err(EventingError::InvalidValue {
                    field: policy_control::source::FIELD_SCHEDULE_CARRYOVER_MAX_MINUTES,
                    value: "cap-carryover".to_string(),
                });
            }
        }
        PolicyScheduleBudgetCarryoverMode::CarryForward => {}
    }

    Ok(())
}
