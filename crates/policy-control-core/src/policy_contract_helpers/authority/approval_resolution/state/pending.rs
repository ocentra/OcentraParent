#![forbid(unsafe_code)]

use super::super::super::PolicyContractApprovalResolution;
use super::super::super::PolicyContractValidationResult;

pub(crate) fn validate(
    resolution: &PolicyContractApprovalResolution,
) -> PolicyContractValidationResult {
    if resolution.reviewed_by_actor_id.is_some()
        || resolution.reviewed_at.is_some()
        || resolution.audit_reference_id.is_some()
        || resolution.override_grant.is_some()
        || resolution.replay_of_approval_id.is_some()
    {
        return Err(
            "pending approvals cannot include review, replay, or override artifacts".into(),
        );
    }

    Ok(())
}
