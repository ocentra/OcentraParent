#![forbid(unsafe_code)]

use super::{
    assert_local_time, time_budget::validate_policy_schedule_time_budget, PolicyContractSchedule,
    PolicyContractValidationResult,
};

pub(super) fn validate_policy_schedule(
    schedule: &PolicyContractSchedule,
) -> PolicyContractValidationResult {
    if schedule.windows.is_empty() {
        return Err("schedules must define at least one local window".into());
    }

    for window in &schedule.windows {
        if window.day_count == 0 {
            return Err("schedules must define at least one day for every window".into());
        }
        assert_local_time(&window.start_local_time, "windows.startLocalTime")?;
        assert_local_time(&window.end_local_time, "windows.endLocalTime")?;
    }

    validate_policy_schedule_time_budget(&schedule.time_budget)
}
