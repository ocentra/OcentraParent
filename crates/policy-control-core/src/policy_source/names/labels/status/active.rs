#![forbid(unsafe_code)]

use crate::policy_source::PolicySourceStatus;
use ocentra_parent_agent_protocol::constants::policy_control;

pub(super) fn policy_status_name(status: PolicySourceStatus) -> &'static str {
    match status {
        PolicySourceStatus::Draft => policy_control::source::STATUS_DRAFT,
        PolicySourceStatus::Preview => policy_control::source::STATUS_PREVIEW,
        PolicySourceStatus::Confirmed => policy_control::source::STATUS_CONFIRMED,
        PolicySourceStatus::Queued => policy_control::source::STATUS_QUEUED,
        PolicySourceStatus::Delivered => policy_control::source::STATUS_DELIVERED,
        PolicySourceStatus::Acknowledged => policy_control::source::STATUS_ACKNOWLEDGED,
        PolicySourceStatus::Active => policy_control::source::STATUS_ACTIVE,
        _ => unreachable!("active policy status helper called with non-active status"),
    }
}
