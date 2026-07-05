#![forbid(unsafe_code)]

use crate::policy_source::PolicySourceStatus;
use ocentra_parent_agent_protocol::constants::policy_control;

pub(super) fn policy_status_name(status: PolicySourceStatus) -> &'static str {
    match status {
        PolicySourceStatus::PartiallyActive => policy_control::source::STATUS_PARTIALLY_ACTIVE,
        PolicySourceStatus::Rejected => policy_control::source::STATUS_REJECTED,
        PolicySourceStatus::Superseded => policy_control::source::STATUS_SUPERSEDED,
        PolicySourceStatus::RolledBack => policy_control::source::STATUS_ROLLED_BACK,
        PolicySourceStatus::Stale => policy_control::source::STATUS_STALE,
        PolicySourceStatus::Expired => policy_control::source::STATUS_EXPIRED,
        PolicySourceStatus::ManualRequired => policy_control::source::STATUS_MANUAL_REQUIRED,
        _ => unreachable!("terminal policy status helper called with active status"),
    }
}
