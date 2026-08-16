#![forbid(unsafe_code)]

use ocentra_parent_agent_protocol::activity::policy_preview::PolicyRequestStatus;

use super::PolicyApprovalDecision;

pub(crate) fn policy_request_status_for_approval(
    decision: PolicyApprovalDecision,
) -> PolicyRequestStatus {
    match decision {
        PolicyApprovalDecision::Grant => PolicyRequestStatus::Approved,
        PolicyApprovalDecision::Deny => PolicyRequestStatus::Denied,
        PolicyApprovalDecision::Modify => PolicyRequestStatus::Modified,
        PolicyApprovalDecision::Expire => PolicyRequestStatus::Expired,
    }
}
