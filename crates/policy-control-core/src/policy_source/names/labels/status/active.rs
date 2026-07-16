#![forbid(unsafe_code)]

use crate::policy_source::PolicySourceStatus;
use ocentra_parent_agent_protocol::constants::policy_control;

const ACTIVE_POLICY_STATUS_NAMES: [&str; 7] = [
    policy_control::source::STATUS_DRAFT,
    policy_control::source::STATUS_PREVIEW,
    policy_control::source::STATUS_CONFIRMED,
    policy_control::source::STATUS_QUEUED,
    policy_control::source::STATUS_DELIVERED,
    policy_control::source::STATUS_ACKNOWLEDGED,
    policy_control::source::STATUS_ACTIVE,
];

const ACTIVE_POLICY_STATUS_SLOTS: [(PolicySourceStatus, ActivePolicyStatus); 7] = [
    (PolicySourceStatus::Draft, ActivePolicyStatus::Draft),
    (PolicySourceStatus::Preview, ActivePolicyStatus::Preview),
    (PolicySourceStatus::Confirmed, ActivePolicyStatus::Confirmed),
    (PolicySourceStatus::Queued, ActivePolicyStatus::Queued),
    (PolicySourceStatus::Delivered, ActivePolicyStatus::Delivered),
    (
        PolicySourceStatus::Acknowledged,
        ActivePolicyStatus::Acknowledged,
    ),
    (PolicySourceStatus::Active, ActivePolicyStatus::Active),
];

#[repr(usize)]
#[derive(Clone, Copy)]
pub(super) enum ActivePolicyStatus {
    Draft,
    Preview,
    Confirmed,
    Queued,
    Delivered,
    Acknowledged,
    Active,
}

pub(super) fn status_slot(status: PolicySourceStatus) -> Option<ActivePolicyStatus> {
    ACTIVE_POLICY_STATUS_SLOTS
        .iter()
        .find_map(|(candidate, slot)| (candidate == &status).then_some(*slot))
}

pub(super) fn policy_status_name(status: ActivePolicyStatus) -> &'static str {
    ACTIVE_POLICY_STATUS_NAMES[status as usize]
}
