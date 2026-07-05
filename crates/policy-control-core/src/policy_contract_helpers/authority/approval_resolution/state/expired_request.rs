#![forbid(unsafe_code)]

use super::super::super::{
    assert_resolution_has_no_review_override_or_replay_artifacts, PolicyContractApprovalResolution,
    PolicyContractValidationResult,
};

pub(crate) fn validate(
    resolution: &PolicyContractApprovalResolution,
) -> PolicyContractValidationResult {
    if resolution.evaluated_at < resolution.approval.expires_at {
        return Err(
            "expired-request state requires evaluatedAt on or after approval.expiresAt".into(),
        );
    }
    assert_resolution_has_no_review_override_or_replay_artifacts(
        resolution,
        "expired-request state cannot include review or override artifacts",
    )
}
