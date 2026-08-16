#![forbid(unsafe_code)]

use ocentra_parent_agent_protocol::activity::policy_preview::PolicyRequestStatus;
use ocentra_parent_agent_protocol::constants::policy_control;

pub(super) fn policy_request_status_name(status: PolicyRequestStatus) -> &'static str {
    match status {
        PolicyRequestStatus::PreviewOnly => policy_control::request::STATUS_PREVIEW_ONLY,
        PolicyRequestStatus::PendingParentReview => {
            policy_control::request::STATUS_PENDING_PARENT_REVIEW
        }
        PolicyRequestStatus::Approved => policy_control::request::STATUS_APPROVED,
        PolicyRequestStatus::Denied => policy_control::request::STATUS_DENIED,
        PolicyRequestStatus::Modified => policy_control::request::STATUS_MODIFIED,
        PolicyRequestStatus::Expired => policy_control::request::STATUS_EXPIRED,
        PolicyRequestStatus::ReplayRejected => policy_control::request::STATUS_REPLAY_REJECTED,
    }
}
