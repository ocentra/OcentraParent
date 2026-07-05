#![forbid(unsafe_code)]

use super::super::super::{
    assert_resolution_has_no_review_override_or_replay_artifacts, PolicyContractApprovalOrigin,
    PolicyContractApprovalResolution, PolicyContractValidationResult,
};

pub(crate) fn validate(
    resolution: &PolicyContractApprovalResolution,
) -> PolicyContractValidationResult {
    if resolution.approval.origin != PolicyContractApprovalOrigin::AssistantDraft {
        return Err("preview-only approvals require assistant-draft origin".into());
    }
    assert_resolution_has_no_review_override_or_replay_artifacts(
        resolution,
        "preview-only approvals must remain unconfirmed and override-free",
    )
}
