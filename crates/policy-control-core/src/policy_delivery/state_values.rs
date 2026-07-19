#![forbid(unsafe_code)]

use super::{
    policy_control, PolicyDeliveryParentVisibleState, PolicyDeliverySequence, PolicyDeliveryState,
    PolicyVersion,
};
use crate::policy_source::PolicyConsumerDomain;

const PARENT_VISIBLE_STATE_BY_DELIVERY_STATE: &[(
    PolicyDeliveryState,
    PolicyDeliveryParentVisibleState,
)] = &[
    (
        PolicyDeliveryState::Queued,
        PolicyDeliveryParentVisibleState::Pending,
    ),
    (
        PolicyDeliveryState::Delivering,
        PolicyDeliveryParentVisibleState::Pending,
    ),
    (
        PolicyDeliveryState::Delivered,
        PolicyDeliveryParentVisibleState::Pending,
    ),
    (
        PolicyDeliveryState::Acknowledged,
        PolicyDeliveryParentVisibleState::Pending,
    ),
    (
        PolicyDeliveryState::Applied,
        PolicyDeliveryParentVisibleState::Applied,
    ),
    (
        PolicyDeliveryState::Rejected,
        PolicyDeliveryParentVisibleState::ManualRequired,
    ),
    (
        PolicyDeliveryState::Superseded,
        PolicyDeliveryParentVisibleState::Superseded,
    ),
    (
        PolicyDeliveryState::RolledBack,
        PolicyDeliveryParentVisibleState::ManualRequired,
    ),
    (
        PolicyDeliveryState::Degraded,
        PolicyDeliveryParentVisibleState::Degraded,
    ),
    (
        PolicyDeliveryState::Offline,
        PolicyDeliveryParentVisibleState::Degraded,
    ),
    (
        PolicyDeliveryState::ExpiredBeforeDelivery,
        PolicyDeliveryParentVisibleState::Degraded,
    ),
    (
        PolicyDeliveryState::RetryScheduled,
        PolicyDeliveryParentVisibleState::Degraded,
    ),
    (
        PolicyDeliveryState::PartialDomainApply,
        PolicyDeliveryParentVisibleState::Degraded,
    ),
    (
        PolicyDeliveryState::BlockedByPermission,
        PolicyDeliveryParentVisibleState::ManualRequired,
    ),
    (
        PolicyDeliveryState::BlockedByCapability,
        PolicyDeliveryParentVisibleState::ManualRequired,
    ),
    (
        PolicyDeliveryState::ManualRequired,
        PolicyDeliveryParentVisibleState::ManualRequired,
    ),
];

const DELIVERY_STATE_NAMES: &[(PolicyDeliveryState, &str)] = &[
    (
        PolicyDeliveryState::Queued,
        policy_control::delivery::STATUS_QUEUED,
    ),
    (PolicyDeliveryState::Delivering, "delivering"),
    (
        PolicyDeliveryState::Delivered,
        policy_control::delivery::STATUS_DELIVERED,
    ),
    (
        PolicyDeliveryState::Acknowledged,
        policy_control::delivery::STATUS_ACKNOWLEDGED,
    ),
    (
        PolicyDeliveryState::Applied,
        policy_control::delivery::STATUS_APPLIED,
    ),
    (
        PolicyDeliveryState::Rejected,
        policy_control::delivery::STATUS_REJECTED,
    ),
    (
        PolicyDeliveryState::Superseded,
        policy_control::delivery::STATUS_SUPERSEDED,
    ),
    (
        PolicyDeliveryState::RolledBack,
        policy_control::delivery::STATUS_ROLLED_BACK,
    ),
    (
        PolicyDeliveryState::Degraded,
        policy_control::delivery::STATUS_DEGRADED,
    ),
    (
        PolicyDeliveryState::Offline,
        policy_control::delivery::STATUS_OFFLINE,
    ),
    (
        PolicyDeliveryState::ExpiredBeforeDelivery,
        "expired-before-delivery",
    ),
    (PolicyDeliveryState::RetryScheduled, "retry-scheduled"),
    (
        PolicyDeliveryState::PartialDomainApply,
        "partial-domain-apply",
    ),
    (
        PolicyDeliveryState::BlockedByPermission,
        "blocked-by-permission",
    ),
    (
        PolicyDeliveryState::BlockedByCapability,
        "blocked-by-capability",
    ),
    (PolicyDeliveryState::ManualRequired, "manual-required"),
];

pub(super) fn policy_delivery_parent_visible_state(
    state: PolicyDeliveryState,
) -> PolicyDeliveryParentVisibleState {
    PARENT_VISIBLE_STATE_BY_DELIVERY_STATE
        .iter()
        .find_map(|(candidate, visible)| (candidate == &state).then_some(*visible))
        .unwrap_or(PolicyDeliveryParentVisibleState::ManualRequired)
}

pub(super) fn policy_delivery_state_name(state: PolicyDeliveryState) -> &'static str {
    DELIVERY_STATE_NAMES
        .iter()
        .find_map(|(candidate, name)| (candidate == &state).then_some(*name))
        .unwrap_or("manual-required")
}

pub(super) fn policy_delivery_domain_name(domain: PolicyConsumerDomain) -> &'static str {
    match domain {
        PolicyConsumerDomain::App => "app",
        PolicyConsumerDomain::Browser => "browser",
        PolicyConsumerDomain::Network => "network",
        PolicyConsumerDomain::Tracking => "tracking",
        PolicyConsumerDomain::Screen => "screen",
        PolicyConsumerDomain::Ai => "ai",
    }
}

pub(super) fn conflicting_replay_value(sequence: PolicyDeliverySequence) -> String {
    let mut value =
        String::from(policy_control::delivery::VALUE_CONFLICTING_REPLAY_FOR_SEQUENCE_PREFIX);
    value.push_str(&sequence.value().to_string());
    value.push_str(" with mismatched transition provenance");
    value
}

pub(super) fn invalid_transition_value(
    current: PolicyDeliveryState,
    next: PolicyDeliveryState,
) -> String {
    let mut value = String::from(policy_control::delivery::VALUE_INVALID_TRANSITION_PREFIX);
    value.push_str(policy_delivery_state_name(current));
    value.push_str(policy_control::delivery::VALUE_INVALID_TRANSITION_SEPARATOR);
    value.push_str(policy_delivery_state_name(next));
    value
}

pub(super) fn unexpected_reason_code_value(state: PolicyDeliveryState) -> String {
    let mut value = String::from(policy_control::delivery::VALUE_UNEXPECTED_REASON_CODE_PREFIX);
    value.push_str("present for ");
    value.push_str(policy_delivery_state_name(state));
    value
}

pub(super) fn unexpected_replacement_policy_version_value(
    policy_version: PolicyVersion,
    state: PolicyDeliveryState,
) -> String {
    let mut value =
        String::from(policy_control::delivery::VALUE_UNEXPECTED_REPLACEMENT_POLICY_VERSION_PREFIX);
    value.push_str(&policy_version.value().to_string());
    value.push_str(policy_control::delivery::VALUE_FOR_STATE_SEPARATOR);
    value.push_str(policy_delivery_state_name(state));
    value
}

pub(super) fn unexpected_rollback_reference_state_value(
    rollback_reference_state: PolicyDeliveryState,
    state: PolicyDeliveryState,
) -> String {
    let mut value =
        String::from(policy_control::delivery::VALUE_UNEXPECTED_ROLLBACK_REFERENCE_STATE_PREFIX);
    value.push_str(policy_delivery_state_name(rollback_reference_state));
    value.push_str(policy_control::delivery::VALUE_FOR_STATE_SEPARATOR);
    value.push_str(policy_delivery_state_name(state));
    value
}

pub(super) fn missing_reason_code_value(state: PolicyDeliveryState) -> String {
    let mut value = String::from(policy_control::delivery::VALUE_MISSING_REASON_CODE_FOR_PREFIX);
    value.push_str(policy_delivery_state_name(state));
    value
}

pub(super) fn missing_replacement_policy_version_value(state: PolicyDeliveryState) -> String {
    let mut value =
        String::from(policy_control::delivery::VALUE_MISSING_REPLACEMENT_POLICY_VERSION_FOR_PREFIX);
    value.push_str(policy_delivery_state_name(state));
    value
}

pub(super) fn replacement_policy_version_must_be_newer_value(
    superseded_by_policy_version: PolicyVersion,
    current_policy_version: PolicyVersion,
) -> String {
    let mut value = String::from(policy_control::delivery::VALUE_REPLACEMENT_POLICY_VERSION_PREFIX);
    value.push_str(&superseded_by_policy_version.value().to_string());
    value.push_str(policy_control::delivery::VALUE_MUST_BE_NEWER_THAN_SEPARATOR);
    value.push_str(&current_policy_version.value().to_string());
    value
}

pub(super) fn missing_rollback_reference_state_value(state: PolicyDeliveryState) -> String {
    let mut value =
        String::from(policy_control::delivery::VALUE_MISSING_ROLLBACK_REFERENCE_STATE_FOR_PREFIX);
    value.push_str(policy_delivery_state_name(state));
    value
}
