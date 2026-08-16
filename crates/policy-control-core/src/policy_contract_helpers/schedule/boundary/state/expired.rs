#![forbid(unsafe_code)]

use super::super::super::{PolicyContractScheduleBoundary, PolicyContractValidationResult};

pub(super) fn validate(
    boundary: &PolicyContractScheduleBoundary,
) -> PolicyContractValidationResult {
    let Some(expiry) = &boundary.expiry else {
        return Err("expired schedule boundaries require expiry details".into());
    };
    if boundary.evaluated_at < expiry.expires_at {
        return Err("expired schedule boundaries must be evaluated on or after expiry".into());
    }
    Ok(())
}
