#![forbid(unsafe_code)]

use super::super::super::{PolicyContractScheduleBoundary, PolicyContractValidationResult};

pub(super) fn validate(
    boundary: &PolicyContractScheduleBoundary,
) -> PolicyContractValidationResult {
    let Some(exception) = &boundary.exception else {
        return Err("exception-active boundaries require exception details".into());
    };
    if boundary.evaluated_at >= exception.starts_at && boundary.evaluated_at < exception.expires_at
    {
        return Ok(());
    }

    Err("exception-active boundaries must be evaluated inside the exception window".into())
}
