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

pub(super) fn transition_allowed(current: PolicyDeliveryState, next: PolicyDeliveryState) -> bool {
    current == next
        || match current {
            PolicyDeliveryState::Queued => QUEUED_TRANSITIONS.contains(&next),
            PolicyDeliveryState::Delivering => DELIVERING_TRANSITIONS.contains(&next),
            PolicyDeliveryState::Delivered => DELIVERED_TRANSITIONS.contains(&next),
            PolicyDeliveryState::Acknowledged => ACKNOWLEDGED_TRANSITIONS.contains(&next),
            PolicyDeliveryState::Applied => APPLIED_TRANSITIONS.contains(&next),
            PolicyDeliveryState::Rejected => next == PolicyDeliveryState::Superseded,
            PolicyDeliveryState::Superseded => false,
            PolicyDeliveryState::RolledBack => next == PolicyDeliveryState::Superseded,
            PolicyDeliveryState::Degraded => DEGRADED_TRANSITIONS.contains(&next),
            PolicyDeliveryState::Offline => OFFLINE_TRANSITIONS.contains(&next),
            PolicyDeliveryState::ExpiredBeforeDelivery => next == PolicyDeliveryState::Superseded,
            PolicyDeliveryState::RetryScheduled => RETRY_SCHEDULED_TRANSITIONS.contains(&next),
            PolicyDeliveryState::PartialDomainApply => PARTIAL_DOMAIN_TRANSITIONS.contains(&next),
            PolicyDeliveryState::BlockedByPermission
            | PolicyDeliveryState::BlockedByCapability
            | PolicyDeliveryState::ManualRequired => BLOCKED_TRANSITIONS.contains(&next),
        }
}
