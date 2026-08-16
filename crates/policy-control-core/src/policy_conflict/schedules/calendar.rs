#![forbid(unsafe_code)]

use ocentra_eventing::error::EventingError;
use ocentra_parent_agent_protocol::constants::policy_control;

use crate::policy_source::PolicyScheduleWindow;

use super::super::{DstTransitionKind, UtcDate};

pub(super) fn schedule_transition_date(
    schedule: &PolicyScheduleWindow,
) -> Result<Option<UtcDate>, EventingError> {
    let Some(effective_until) = &schedule.time_budget.effective_until else {
        return Ok(None);
    };

    let effective_from = parse_utc_date(
        policy_control::source::FIELD_SCHEDULE_EFFECTIVE_FROM,
        &schedule.time_budget.effective_from,
    )?;
    let effective_until = parse_utc_date(
        policy_control::source::FIELD_SCHEDULE_EFFECTIVE_UNTIL,
        effective_until,
    )?;

    if effective_from != effective_until {
        return Ok(None);
    }

    Ok(Some(effective_from))
}

pub(super) fn is_transition_day(date: UtcDate, transition: DstTransitionKind) -> bool {
    let (month, day_start, day_end) = transition_day_window(transition);
    date.month == month
        && (day_start..=day_end).contains(&date.day)
        && day_of_week(date.year, date.month, date.day) == 0
}

fn transition_day_window(transition: DstTransitionKind) -> (u8, u8, u8) {
    match transition {
        DstTransitionKind::SpringForward => (3, 8, 14),
        DstTransitionKind::FallBack => (11, 1, 7),
    }
}

pub(super) fn parse_utc_date(field: &'static str, value: &str) -> Result<UtcDate, EventingError> {
    Ok(UtcDate {
        year: parse_date_component::<i32>(field, value, 0..4)?,
        month: parse_date_component::<u8>(field, value, 5..7)?,
        day: parse_date_component::<u8>(field, value, 8..10)?,
    })
}

fn parse_date_component<T>(
    field: &'static str,
    value: &str,
    range: std::ops::Range<usize>,
) -> Result<T, EventingError>
where
    T: std::str::FromStr,
{
    value
        .get(range)
        .ok_or_else(|| invalid_date_value(field, value))?
        .parse::<T>()
        .map_err(|_error| invalid_date_value(field, value))
}

fn invalid_date_value(field: &'static str, value: &str) -> EventingError {
    EventingError::InvalidValue {
        field,
        value: value.to_string(),
    }
}

pub(super) fn day_of_week(year: i32, month: u8, day: u8) -> u8 {
    let offsets = [0_i32, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let adjusted_year = if month < 3 { year - 1 } else { year };
    ((adjusted_year + (adjusted_year / 4) - (adjusted_year / 100)
        + (adjusted_year / 400)
        + offsets[(month - 1) as usize]
        + i32::from(day))
        % 7) as u8
}
