#![forbid(unsafe_code)]

use super::super::{
    PolicyContractScheduleBoundary, PolicyContractScheduleBoundaryState,
    PolicyContractValidationResult,
};

mod clock_skew;
mod dst_gap;
mod dst_overlap;
mod exception_active;
mod expired;

pub(super) fn validate_policy_schedule_boundary_state(
    boundary: &PolicyContractScheduleBoundary,
) -> PolicyContractValidationResult {
    match boundary.state {
        PolicyContractScheduleBoundaryState::DstGap => dst_gap::validate(boundary),
        PolicyContractScheduleBoundaryState::DstOverlap => dst_overlap::validate(boundary),
        PolicyContractScheduleBoundaryState::ClockSkew => clock_skew::validate(boundary),
        PolicyContractScheduleBoundaryState::ExceptionActive => {
            exception_active::validate(boundary)
        }
        PolicyContractScheduleBoundaryState::Expired => expired::validate(boundary),
        PolicyContractScheduleBoundaryState::WithinWindow
        | PolicyContractScheduleBoundaryState::OutsideWindow => Ok(()),
    }
}
