#![forbid(unsafe_code)]

use ocentra_eventing::error::EventingError;
use ocentra_parent_agent_protocol::constants::policy_control;

use crate::policy_source::{PolicyScheduleBudgetCarryoverMode, PolicyScheduleTimeBudget};

pub(crate) fn assert_schedule_time_budget_carryover(
    budget: &PolicyScheduleTimeBudget,
) -> Result<(), EventingError> {
    assert_schedule_time_budget_carryover_mode(budget.carryover.mode, budget.carryover.max_minutes)
}

fn assert_schedule_time_budget_carryover_mode(
    mode: PolicyScheduleBudgetCarryoverMode,
    max_minutes: Option<u16>,
) -> Result<(), EventingError> {
    match mode {
        PolicyScheduleBudgetCarryoverMode::DiscardUnused => {
            assert_discard_unused_carryover(max_minutes)
        }
        PolicyScheduleBudgetCarryoverMode::CapCarryover => assert_cap_carryover(max_minutes),
        PolicyScheduleBudgetCarryoverMode::CarryForward => Ok(()),
    }
}

fn assert_discard_unused_carryover(max_minutes: Option<u16>) -> Result<(), EventingError> {
    if max_minutes.is_none() {
        return Ok(());
    }

    Err(EventingError::InvalidValue {
        field: policy_control::source::FIELD_SCHEDULE_CARRYOVER_MAX_MINUTES,
        value: "discard-unused".to_string(),
    })
}

fn assert_cap_carryover(max_minutes: Option<u16>) -> Result<(), EventingError> {
    if max_minutes.unwrap_or(0) > 0 {
        return Ok(());
    }

    Err(EventingError::InvalidValue {
        field: policy_control::source::FIELD_SCHEDULE_CARRYOVER_MAX_MINUTES,
        value: "cap-carryover".to_string(),
    })
}
