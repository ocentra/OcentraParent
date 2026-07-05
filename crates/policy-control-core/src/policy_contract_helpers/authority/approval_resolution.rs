#![forbid(unsafe_code)]

use super::{
    assert_utc_timestamp, PolicyContractApprovalResolution, PolicyContractValidationResult,
};

mod request;
mod review;
mod state;

pub(crate) fn validate_policy_approval_resolution(
    resolution: &PolicyContractApprovalResolution,
) -> PolicyContractValidationResult {
    request::validate_policy_approval_request(&resolution.approval)?;
    assert_utc_timestamp(&resolution.evaluated_at, "evaluatedAt")?;
    review::validate_policy_approval_resolution_review_timestamp(resolution)?;
    state::validate_policy_approval_resolution_state_rules(resolution)?;
    Ok(())
}
