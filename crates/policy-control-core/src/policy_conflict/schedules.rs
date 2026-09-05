#![forbid(unsafe_code)]

use ocentra_eventing::error::EventingError;

use super::PolicyScheduleWindow;

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

pub(super) fn schedule_uses_supported_wp07_dst_timezone(schedule: &PolicyScheduleWindow) -> bool {
    transitions::schedule_uses_supported_wp07_dst_timezone(schedule)
}

pub(super) fn normalized_time_ranges(schedule: &PolicyScheduleWindow) -> Vec<(u16, u16)> {
    time_ranges::normalized_time_ranges(schedule)
}
