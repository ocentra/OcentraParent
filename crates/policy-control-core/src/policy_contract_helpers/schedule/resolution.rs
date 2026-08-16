#![forbid(unsafe_code)]

use super::super::authority::PolicyContractApprovalResolution;
use super::PolicyContractValidationResult;

pub(super) fn assert_resolution_has_no_review_or_override_artifacts(
    resolution: &PolicyContractApprovalResolution,
    message: &'static str,
) -> PolicyContractValidationResult {
    if resolution.reviewed_by_actor_id.is_some()
        || resolution.reviewed_at.is_some()
        || resolution.audit_reference_id.is_some()
        || resolution.override_grant.is_some()
    {
        return Err(message.into());
    }
    Ok(())
}

pub(super) fn assert_resolution_has_no_review_override_or_replay_artifacts(
    resolution: &PolicyContractApprovalResolution,
    message: &'static str,
) -> PolicyContractValidationResult {
    assert_resolution_has_no_review_or_override_artifacts(resolution, message)?;
    if resolution.replay_of_approval_id.is_some() {
        return Err(message.into());
    }
    Ok(())
}
