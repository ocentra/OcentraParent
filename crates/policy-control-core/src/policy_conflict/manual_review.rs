#![forbid(unsafe_code)]

use ocentra_eventing::error::EventingError;

use super::{schedules, PolicyConflictKind, PolicyScheduleClockSource, PolicyScheduleWindow};

pub(super) fn schedule_manual_review_conflict_kind(
    schedule: &PolicyScheduleWindow,
) -> Result<Option<PolicyConflictKind>, EventingError> {
    if schedules::schedule_has_nonexistent_local_time(schedule)? {
        return Ok(Some(PolicyConflictKind::NonexistentLocalTime));
    }
    if schedules::schedule_has_ambiguous_local_time(schedule)? {
        return Ok(Some(PolicyConflictKind::AmbiguousLocalTime));
    }
    if matches!(
        schedule.time_budget.clock_source,
        PolicyScheduleClockSource::ManualRequired
    ) {
        return Ok(Some(PolicyConflictKind::ClockSkew));
    }
    Ok(None)
}
