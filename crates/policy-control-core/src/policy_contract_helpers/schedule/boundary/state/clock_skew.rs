#![forbid(unsafe_code)]

use super::super::super::{PolicyContractScheduleBoundary, PolicyContractValidationResult};

pub(super) fn validate(
    boundary: &PolicyContractScheduleBoundary,
) -> PolicyContractValidationResult {
    let Some(clock_skew) = &boundary.clock_skew else {
        return Err("clock-skew boundaries require clockSkew details".into());
    };
    let observed_skew_minutes = i64::from(clock_skew.observed_skew_minutes).abs();
    let allowed_skew_minutes = i64::from(clock_skew.allowed_skew_minutes);
    if observed_skew_minutes <= allowed_skew_minutes {
        return Err("clock-skew boundaries require skew beyond the allowed tolerance".into());
    }
    Ok(())
}
