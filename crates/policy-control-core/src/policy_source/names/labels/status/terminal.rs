#![forbid(unsafe_code)]

use crate::policy_source::PolicySourceStatus;
use ocentra_parent_agent_protocol::constants::policy_control;

const TERMINAL_POLICY_STATUS_NAMES: [&str; 7] = [
    policy_control::source::STATUS_PARTIALLY_ACTIVE,
    policy_control::source::STATUS_REJECTED,
    policy_control::source::STATUS_SUPERSEDED,
    policy_control::source::STATUS_ROLLED_BACK,
    policy_control::source::STATUS_STALE,
    policy_control::source::STATUS_EXPIRED,
    policy_control::source::STATUS_MANUAL_REQUIRED,
];

const TERMINAL_POLICY_STATUS_SLOTS: [(PolicySourceStatus, TerminalPolicyStatus); 7] = [
    (
        PolicySourceStatus::PartiallyActive,
        TerminalPolicyStatus::PartiallyActive,
    ),
    (PolicySourceStatus::Rejected, TerminalPolicyStatus::Rejected),
    (
        PolicySourceStatus::Superseded,
        TerminalPolicyStatus::Superseded,
    ),
    (
        PolicySourceStatus::RolledBack,
        TerminalPolicyStatus::RolledBack,
    ),
    (PolicySourceStatus::Stale, TerminalPolicyStatus::Stale),
    (PolicySourceStatus::Expired, TerminalPolicyStatus::Expired),
    (
        PolicySourceStatus::ManualRequired,
        TerminalPolicyStatus::ManualRequired,
    ),
];

#[repr(usize)]
#[derive(Clone, Copy)]
pub(super) enum TerminalPolicyStatus {
    PartiallyActive,
    Rejected,
    Superseded,
    RolledBack,
    Stale,
    Expired,
    ManualRequired,
}

pub(super) fn status_slot(status: PolicySourceStatus) -> Option<TerminalPolicyStatus> {
    TERMINAL_POLICY_STATUS_SLOTS
        .iter()
        .find_map(|(candidate, slot)| (candidate == &status).then_some(*slot))
}

pub(super) fn policy_status_name(status: TerminalPolicyStatus) -> &'static str {
    TERMINAL_POLICY_STATUS_NAMES[status as usize]
}
