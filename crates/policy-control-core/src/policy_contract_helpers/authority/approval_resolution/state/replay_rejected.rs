#![forbid(unsafe_code)]

use super::super::super::{
    assert_resolution_has_no_review_or_override_artifacts, PolicyContractApprovalResolution,
    PolicyContractValidationResult,
};

pub(crate) fn validate(
    resolution: &PolicyContractApprovalResolution,
) -> PolicyContractValidationResult {
    if resolution.replay_of_approval_id.is_none() {
        return Err("replay-rejected state requires replayOfApprovalId".into());
    }
    assert_resolution_has_no_review_or_override_artifacts(
        resolution,
        "replay-rejected state cannot include review or override artifacts",
    )
}
