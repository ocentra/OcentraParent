#![forbid(unsafe_code)]

use super::super::super::PolicyContractApprovalResolution;
use super::super::super::PolicyContractValidationResult;

pub(crate) fn validate(
    resolution: &PolicyContractApprovalResolution,
) -> PolicyContractValidationResult {
    if resolution.reviewed_by_actor_id.is_none()
        || resolution.reviewed_at.is_none()
        || resolution.audit_reference_id.is_none()
    {
        return Err("denied approvals require review and audit artifacts".into());
    }
    if resolution.override_grant.is_some() || resolution.replay_of_approval_id.is_some() {
        return Err("denied approvals cannot include overrides or replay pointers".into());
    }

    Ok(())
}
