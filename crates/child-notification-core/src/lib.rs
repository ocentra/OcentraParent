#![forbid(unsafe_code)]

use ocentra_parent_agent_protocol::{
    child_domain_ref, constants, ChildDomainNotificationRequestedEvent,
    ChildDomainPolicyViolationDetectedEvent, ParentNotificationRequestedEvent,
    TrackingPolicyViolationDetectedEvent,
};

pub const CRATE_NAME: &str = "ocentra-child-notification-core";

pub fn request_parent_notification_from_policy_violation(
    event: &TrackingPolicyViolationDetectedEvent,
) -> ParentNotificationRequestedEvent {
    ParentNotificationRequestedEvent {
        child_device_id: event.child_device_id.clone(),
        child_profile_id: event.child_profile_id.clone(),
        notification_id: constants::tracking_runtime::DEFAULT_NOTIFICATION_ID.to_string(),
        source_policy_violation_id: event.violation_id.clone(),
        channel: constants::tracking_runtime::NOTIFICATION_CHANNEL_PARENT_PORTAL.to_string(),
        evidence_refs: event.evidence_refs.clone(),
    }
}

pub fn request_child_domain_parent_notification(
    event: &ChildDomainPolicyViolationDetectedEvent,
) -> ChildDomainNotificationRequestedEvent {
    ChildDomainNotificationRequestedEvent {
        event_type: constants::child_domain_runtime::NOTIFICATION_REQUESTED_EVENT_TYPE.to_string(),
        domain: event.domain.clone(),
        child_device_id: event.child_device_id.clone(),
        child_profile_id: event.child_profile_id.clone(),
        notification_id: child_domain_ref(
            &event.domain,
            constants::child_domain_runtime::DEFAULT_NOTIFICATION_ID_SUFFIX,
        ),
        source_policy_violation_id: event.violation_id.clone(),
        channel: constants::child_domain_runtime::NOTIFICATION_CHANNEL_PARENT_PORTAL.to_string(),
        evidence_refs: event.evidence_refs.clone(),
    }
}
