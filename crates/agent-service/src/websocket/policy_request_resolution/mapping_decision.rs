use ocentra_parent_agent_protocol::transport::PolicyRequestParentResolutionDecision;
use ocentra_policy_control_core::policy_request::PolicyApprovalDecision;

pub(super) fn map(value: PolicyRequestParentResolutionDecision) -> PolicyApprovalDecision {
    match value {
        PolicyRequestParentResolutionDecision::Grant => PolicyApprovalDecision::Grant,
        PolicyRequestParentResolutionDecision::Deny => PolicyApprovalDecision::Deny,
        PolicyRequestParentResolutionDecision::Modify => PolicyApprovalDecision::Modify,
        PolicyRequestParentResolutionDecision::Expire => PolicyApprovalDecision::Expire,
    }
}
