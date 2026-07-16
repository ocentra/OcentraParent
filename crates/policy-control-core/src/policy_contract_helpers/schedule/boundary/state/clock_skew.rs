#![forbid(unsafe_code)]

use super::super::super::{PolicyContractScheduleBoundary, PolicyContractValidationResult};

pub(super) fn validate(
    boundary: &PolicyContractScheduleBoundary,
) -> PolicyContractValidationResult {
    let Some(clock_skew) = &boundary.clock_skew else {
        return Err("clock-skew boundaries require clockSkew details".into());
    };
    if clock_skew.observed_skew_minutes.abs() <= clock_skew.allowed_skew_minutes {
        return Err("clock-skew boundaries require skew beyond the allowed tolerance".into());
    }
    Ok(())
}
