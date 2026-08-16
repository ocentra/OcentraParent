#![forbid(unsafe_code)]

use ocentra_eventing::error::EventingError;
use ocentra_parent_agent_protocol::constants::policy_control;

use crate::policy_source::PolicyScheduleWindow;

use super::super::DstTransitionKind;
use super::{calendar, time_ranges};

pub(super) fn schedule_has_nonexistent_local_time(
    schedule: &PolicyScheduleWindow,
) -> Result<bool, EventingError> {
    schedule_has_dst_transition_conflict(schedule, DstTransitionKind::SpringForward)
}

pub(super) fn schedule_has_ambiguous_local_time(
    schedule: &PolicyScheduleWindow,
) -> Result<bool, EventingError> {
    schedule_has_dst_transition_conflict(schedule, DstTransitionKind::FallBack)
}

pub(super) fn schedule_has_transition_local_time(
    schedule: &PolicyScheduleWindow,
    transition: DstTransitionKind,
) -> Result<bool, EventingError> {
    let (range_start, range_end) = transition_minute_window(transition);
    let values = [
        schedule.starts_at.as_str(),
        schedule.ends_at.as_str(),
        schedule.time_budget.reset.local_time.as_str(),
    ];

    for value in values {
        let minutes =
            time_ranges::parse_clock_minutes(value).ok_or_else(|| EventingError::InvalidValue {
                field: policy_control::source::FIELD_SCHEDULE_STARTS_AT,
                value: value.to_string(),
            })?;
        if (range_start..range_end).contains(&minutes) {
            return Ok(true);
        }
    }

    Ok(false)
}

pub(super) fn schedule_on_single_transition_day(
    schedule: &PolicyScheduleWindow,
    transition: DstTransitionKind,
) -> Result<bool, EventingError> {
    Ok(calendar::schedule_transition_date(schedule)?
        .map(|date| calendar::is_transition_day(date, transition))
        .unwrap_or(false))
}

pub(super) fn schedule_uses_supported_wp07_dst_timezone(schedule: &PolicyScheduleWindow) -> bool {
    matches!(
        schedule.timezone_name.as_str(),
        "America/Toronto" | "America/Vancouver" | "America/Los_Angeles" | "America/Winnipeg"
    )
}

fn schedule_has_dst_transition_conflict(
    schedule: &PolicyScheduleWindow,
    transition: DstTransitionKind,
) -> Result<bool, EventingError> {
    if !schedule_uses_supported_wp07_dst_timezone(schedule) {
        return Ok(false);
    }
    if !schedule_on_single_transition_day(schedule, transition)? {
        return Ok(false);
    }
    schedule_has_transition_local_time(schedule, transition)
}

fn transition_minute_window(transition: DstTransitionKind) -> (u16, u16) {
    match transition {
        DstTransitionKind::SpringForward => (120, 180),
        DstTransitionKind::FallBack => (60, 120),
    }
}
