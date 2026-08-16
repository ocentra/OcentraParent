#![forbid(unsafe_code)]

use super::PolicyDeliveryState;

const QUEUED_TRANSITIONS: &[PolicyDeliveryState] = &[
    PolicyDeliveryState::Delivering,
    PolicyDeliveryState::Delivered,
    PolicyDeliveryState::Acknowledged,
    PolicyDeliveryState::Applied,
    PolicyDeliveryState::Rejected,
    PolicyDeliveryState::Superseded,
    PolicyDeliveryState::RolledBack,
    PolicyDeliveryState::Degraded,
    PolicyDeliveryState::Offline,
    PolicyDeliveryState::ExpiredBeforeDelivery,
    PolicyDeliveryState::RetryScheduled,
    PolicyDeliveryState::PartialDomainApply,
    PolicyDeliveryState::BlockedByPermission,
    PolicyDeliveryState::BlockedByCapability,
    PolicyDeliveryState::ManualRequired,
];

const DELIVERING_TRANSITIONS: &[PolicyDeliveryState] = &[
    PolicyDeliveryState::Delivered,
    PolicyDeliveryState::Acknowledged,
    PolicyDeliveryState::Applied,
    PolicyDeliveryState::Rejected,
    PolicyDeliveryState::Superseded,
    PolicyDeliveryState::RolledBack,
    PolicyDeliveryState::Degraded,
    PolicyDeliveryState::Offline,
    PolicyDeliveryState::ExpiredBeforeDelivery,
    PolicyDeliveryState::RetryScheduled,
    PolicyDeliveryState::PartialDomainApply,
    PolicyDeliveryState::BlockedByPermission,
    PolicyDeliveryState::BlockedByCapability,
    PolicyDeliveryState::ManualRequired,
];

const DELIVERED_TRANSITIONS: &[PolicyDeliveryState] = &[
    PolicyDeliveryState::Acknowledged,
    PolicyDeliveryState::Applied,
    PolicyDeliveryState::Rejected,
    PolicyDeliveryState::Superseded,
    PolicyDeliveryState::RolledBack,
    PolicyDeliveryState::Degraded,
    PolicyDeliveryState::Offline,
    PolicyDeliveryState::RetryScheduled,
    PolicyDeliveryState::PartialDomainApply,
    PolicyDeliveryState::BlockedByPermission,
    PolicyDeliveryState::BlockedByCapability,
    PolicyDeliveryState::ManualRequired,
];

const ACKNOWLEDGED_TRANSITIONS: &[PolicyDeliveryState] = &[
    PolicyDeliveryState::Applied,
    PolicyDeliveryState::Rejected,
    PolicyDeliveryState::Superseded,
    PolicyDeliveryState::RolledBack,
    PolicyDeliveryState::Degraded,
    PolicyDeliveryState::Offline,
    PolicyDeliveryState::RetryScheduled,
    PolicyDeliveryState::PartialDomainApply,
    PolicyDeliveryState::BlockedByPermission,
    PolicyDeliveryState::BlockedByCapability,
    PolicyDeliveryState::ManualRequired,
];

const APPLIED_TRANSITIONS: &[PolicyDeliveryState] = &[
    PolicyDeliveryState::Superseded,
    PolicyDeliveryState::RolledBack,
    PolicyDeliveryState::Degraded,
    PolicyDeliveryState::RetryScheduled,
    PolicyDeliveryState::BlockedByPermission,
    PolicyDeliveryState::BlockedByCapability,
    PolicyDeliveryState::ManualRequired,
];

const DEGRADED_TRANSITIONS: &[PolicyDeliveryState] = &[
    PolicyDeliveryState::Delivering,
    PolicyDeliveryState::Delivered,
    PolicyDeliveryState::Acknowledged,
    PolicyDeliveryState::Applied,
    PolicyDeliveryState::Rejected,
    PolicyDeliveryState::Superseded,
    PolicyDeliveryState::RolledBack,
    PolicyDeliveryState::Offline,
    PolicyDeliveryState::ExpiredBeforeDelivery,
    PolicyDeliveryState::RetryScheduled,
    PolicyDeliveryState::PartialDomainApply,
    PolicyDeliveryState::BlockedByPermission,
    PolicyDeliveryState::BlockedByCapability,
    PolicyDeliveryState::ManualRequired,
];

const OFFLINE_TRANSITIONS: &[PolicyDeliveryState] = &[
    PolicyDeliveryState::Delivering,
    PolicyDeliveryState::Delivered,
    PolicyDeliveryState::Acknowledged,
    PolicyDeliveryState::Applied,
    PolicyDeliveryState::Rejected,
    PolicyDeliveryState::Superseded,
    PolicyDeliveryState::RolledBack,
    PolicyDeliveryState::Degraded,
    PolicyDeliveryState::ExpiredBeforeDelivery,
    PolicyDeliveryState::RetryScheduled,
    PolicyDeliveryState::PartialDomainApply,
    PolicyDeliveryState::BlockedByPermission,
    PolicyDeliveryState::BlockedByCapability,
    PolicyDeliveryState::ManualRequired,
];

const RETRY_SCHEDULED_TRANSITIONS: &[PolicyDeliveryState] = &[
    PolicyDeliveryState::Delivering,
    PolicyDeliveryState::Delivered,
    PolicyDeliveryState::Acknowledged,
    PolicyDeliveryState::Applied,
    PolicyDeliveryState::Rejected,
    PolicyDeliveryState::Superseded,
    PolicyDeliveryState::RolledBack,
    PolicyDeliveryState::Degraded,
    PolicyDeliveryState::Offline,
    PolicyDeliveryState::ExpiredBeforeDelivery,
    PolicyDeliveryState::PartialDomainApply,
    PolicyDeliveryState::BlockedByPermission,
    PolicyDeliveryState::BlockedByCapability,
    PolicyDeliveryState::ManualRequired,
];

const PARTIAL_DOMAIN_TRANSITIONS: &[PolicyDeliveryState] = &[
    PolicyDeliveryState::Applied,
    PolicyDeliveryState::Rejected,
    PolicyDeliveryState::Superseded,
    PolicyDeliveryState::RolledBack,
    PolicyDeliveryState::Degraded,
    PolicyDeliveryState::Offline,
    PolicyDeliveryState::RetryScheduled,
    PolicyDeliveryState::BlockedByPermission,
    PolicyDeliveryState::BlockedByCapability,
    PolicyDeliveryState::ManualRequired,
];

const BLOCKED_TRANSITIONS: &[PolicyDeliveryState] = &[
    PolicyDeliveryState::Delivering,
    PolicyDeliveryState::Delivered,
    PolicyDeliveryState::Acknowledged,
    PolicyDeliveryState::Applied,
    PolicyDeliveryState::Rejected,
    PolicyDeliveryState::Superseded,
    PolicyDeliveryState::RolledBack,
    PolicyDeliveryState::Degraded,
    PolicyDeliveryState::Offline,
    PolicyDeliveryState::ExpiredBeforeDelivery,
    PolicyDeliveryState::RetryScheduled,
    PolicyDeliveryState::PartialDomainApply,
    PolicyDeliveryState::BlockedByPermission,
    PolicyDeliveryState::BlockedByCapability,
    PolicyDeliveryState::ManualRequired,
];

const REJECTED_TRANSITIONS: &[PolicyDeliveryState] = &[PolicyDeliveryState::Superseded];
const SUPERSEDED_TRANSITIONS: &[PolicyDeliveryState] = &[];
const ROLLED_BACK_TRANSITIONS: &[PolicyDeliveryState] = &[PolicyDeliveryState::Superseded];
const EXPIRED_TRANSITIONS: &[PolicyDeliveryState] = &[PolicyDeliveryState::Superseded];

const TRANSITIONS_BY_CURRENT_STATE: &[(PolicyDeliveryState, &[PolicyDeliveryState])] = &[
    (PolicyDeliveryState::Queued, QUEUED_TRANSITIONS),
    (PolicyDeliveryState::Delivering, DELIVERING_TRANSITIONS),
    (PolicyDeliveryState::Delivered, DELIVERED_TRANSITIONS),
    (PolicyDeliveryState::Acknowledged, ACKNOWLEDGED_TRANSITIONS),
    (PolicyDeliveryState::Applied, APPLIED_TRANSITIONS),
    (PolicyDeliveryState::Rejected, REJECTED_TRANSITIONS),
    (PolicyDeliveryState::Superseded, SUPERSEDED_TRANSITIONS),
    (PolicyDeliveryState::RolledBack, ROLLED_BACK_TRANSITIONS),
    (PolicyDeliveryState::Degraded, DEGRADED_TRANSITIONS),
    (PolicyDeliveryState::Offline, OFFLINE_TRANSITIONS),
    (
        PolicyDeliveryState::ExpiredBeforeDelivery,
        EXPIRED_TRANSITIONS,
    ),
    (
        PolicyDeliveryState::RetryScheduled,
        RETRY_SCHEDULED_TRANSITIONS,
    ),
    (
        PolicyDeliveryState::PartialDomainApply,
        PARTIAL_DOMAIN_TRANSITIONS,
    ),
    (
        PolicyDeliveryState::BlockedByPermission,
        BLOCKED_TRANSITIONS,
    ),
    (
        PolicyDeliveryState::BlockedByCapability,
        BLOCKED_TRANSITIONS,
    ),
    (PolicyDeliveryState::ManualRequired, BLOCKED_TRANSITIONS),
];

pub(super) fn transition_allowed(current: PolicyDeliveryState, next: PolicyDeliveryState) -> bool {
    current == next || allowed_transitions(current).contains(&next)
}

fn allowed_transitions(current: PolicyDeliveryState) -> &'static [PolicyDeliveryState] {
    TRANSITIONS_BY_CURRENT_STATE
        .iter()
        .find_map(|(candidate, transitions)| (candidate == &current).then_some(*transitions))
        .unwrap_or(BLOCKED_TRANSITIONS)
}
