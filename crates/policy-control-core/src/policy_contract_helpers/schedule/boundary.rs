#![forbid(unsafe_code)]

use super::{
    assert_local_time, assert_utc_timestamp,
    time_budget::validate_policy_schedule_time_budget_status, PolicyContractScheduleBoundary,
    PolicyContractValidationResult,
};

mod optional_sections;
mod state;

pub(super) fn validate_policy_schedule_boundary(
    boundary: &PolicyContractScheduleBoundary,
) -> PolicyContractValidationResult {
    assert_utc_timestamp(&boundary.evaluated_at, "evaluatedAt")?;
    assert_local_time(&boundary.local_time, "localTime")?;
    optional_sections::validate_policy_schedule_boundary_optional_sections(boundary)?;

    if let Some(time_budget) = &boundary.time_budget {
        validate_policy_schedule_time_budget_status(time_budget, &boundary.evaluated_at)?;
    }

    state::validate_policy_schedule_boundary_state(boundary)
}
