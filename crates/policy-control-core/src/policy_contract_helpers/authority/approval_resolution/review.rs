#![forbid(unsafe_code)]

use super::super::{
    assert_utc_timestamp, PolicyContractApprovalResolution, PolicyContractValidationResult,
};

pub(crate) fn validate_policy_approval_resolution_review_timestamp(
    resolution: &PolicyContractApprovalResolution,
) -> PolicyContractValidationResult {
    if let Some(reviewed_at) = &resolution.reviewed_at {
        assert_utc_timestamp(reviewed_at, "reviewedAt")?;
        if reviewed_at > &resolution.evaluated_at {
            return Err("reviewedAt cannot be after evaluatedAt".into());
        }
    }

    Ok(())
}
