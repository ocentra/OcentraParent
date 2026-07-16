#![forbid(unsafe_code)]

use ocentra_parent_agent_protocol::activity::policy_preview::PolicyRequestStatus;
use ocentra_parent_agent_protocol::constants::policy_control;

use super::{PolicyApprovalId, PolicyRequestSubmissionKey};

mod names;

pub(crate) fn policy_request_status_name(status: PolicyRequestStatus) -> &'static str {
    names::policy_request_status_name(status)
}

pub(crate) fn duplicate_submission_key_value(
    submission_key: &PolicyRequestSubmissionKey,
) -> String {
    let mut value = String::from(policy_control::request::VALUE_DUPLICATE_SUBMISSION_KEY_PREFIX);
    value.push_str(submission_key.as_str());
    value.push_str(policy_control::request::VALUE_DUPLICATE_SUBMISSION_KEY_SUFFIX);
    value
}

pub(crate) fn policy_override_id_value(approval_id: &PolicyApprovalId) -> String {
    let mut value = String::from(policy_control::request::OVERRIDE_ID_PREFIX);
    value.push_str(approval_id.as_str());
    value
}
