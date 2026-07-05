#![forbid(unsafe_code)]

use super::super::super::override_grant::validate_policy_override_grant;
use super::super::super::{PolicyContractApprovalResolution, PolicyContractValidationResult};

pub(crate) fn validate(
    resolution: &PolicyContractApprovalResolution,
) -> PolicyContractValidationResult {
    if resolution.reviewed_by_actor_id.is_none()
        || resolution.reviewed_at.is_none()
        || resolution.audit_reference_id.is_none()
        || resolution.override_grant.is_none()
    {
        return Err(
            "approved and modified approvals require review, audit, and override artifacts".into(),
        );
    }
    if resolution.replay_of_approval_id.is_some() {
        return Err("approved and modified approvals cannot point at replayOfApprovalId".into());
    }
    if resolution.reviewed_by_actor_id.as_deref()
        == Some(resolution.approval.child_profile_id.as_str())
    {
        return Err("child requests cannot self-approve or self-modify".into());
    }
    let Some(override_grant) = resolution.override_grant.as_ref() else {
        return Err(
            "approved and modified approvals require review, audit, and override artifacts".into(),
        );
    };
    validate_policy_override_grant(
        override_grant,
        &resolution.approval,
        &resolution.evaluated_at,
    )
}
