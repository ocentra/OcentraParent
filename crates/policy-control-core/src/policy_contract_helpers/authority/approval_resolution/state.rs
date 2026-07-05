#![forbid(unsafe_code)]

use super::super::{PolicyContractApprovalResolution, PolicyContractValidationResult};

mod denied;
mod expired_request;
mod pending;
mod preview_only;
mod replay_rejected;
mod reviewed;

pub(crate) fn validate_policy_approval_resolution_state_rules(
    resolution: &PolicyContractApprovalResolution,
) -> PolicyContractValidationResult {
    match resolution.state {
        super::super::PolicyContractApprovalState::Pending => pending::validate(resolution),
        super::super::PolicyContractApprovalState::PreviewOnly => {
            preview_only::validate(resolution)
        }
        super::super::PolicyContractApprovalState::ExpiredRequest => {
            expired_request::validate(resolution)
        }
        super::super::PolicyContractApprovalState::ReplayRejected => {
            replay_rejected::validate(resolution)
        }
        super::super::PolicyContractApprovalState::Denied => denied::validate(resolution),
        super::super::PolicyContractApprovalState::Approved
        | super::super::PolicyContractApprovalState::Modified => reviewed::validate(resolution),
    }
}
