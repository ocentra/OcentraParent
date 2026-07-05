#![forbid(unsafe_code)]

use ocentra_eventing::error::EventingError;

use super::{DstTransitionKind, PolicyScheduleWindow};

mod calendar;
mod time_ranges;
mod transitions;

pub(super) fn schedule_has_nonexistent_local_time(
    schedule: &PolicyScheduleWindow,
) -> Result<bool, EventingError> {
    transitions::schedule_has_nonexistent_local_time(schedule)
}

pub(super) fn schedule_has_ambiguous_local_time(
    schedule: &PolicyScheduleWindow,
) -> Result<bool, EventingError> {
    transitions::schedule_has_ambiguous_local_time(schedule)
}

pub(super) fn schedule_has_transition_local_time(
    schedule: &PolicyScheduleWindow,
    transition: DstTransitionKind,
) -> Result<bool, EventingError> {
    transitions::schedule_has_transition_local_time(schedule, transition)
}

pub(super) fn schedule_on_single_transition_day(
    schedule: &PolicyScheduleWindow,
    transition: DstTransitionKind,
) -> Result<bool, EventingError> {
    transitions::schedule_on_single_transition_day(schedule, transition)
}

pub(super) fn schedule_uses_supported_wp07_dst_timezone(schedule: &PolicyScheduleWindow) -> bool {
    transitions::schedule_uses_supported_wp07_dst_timezone(schedule)
}

pub(super) fn normalized_time_ranges(schedule: &PolicyScheduleWindow) -> Vec<(u16, u16)> {
    time_ranges::normalized_time_ranges(schedule)
}

pub(super) fn parse_clock_minutes(value: &str) -> Option<u16> {
    time_ranges::parse_clock_minutes(value)
}

pub(super) fn parse_utc_date(
    field: &'static str,
    value: &str,
) -> Result<super::UtcDate, EventingError> {
    calendar::parse_utc_date(field, value)
}

pub(super) fn day_of_week(year: i32, month: u8, day: u8) -> u8 {
    calendar::day_of_week(year, month, day)
}
