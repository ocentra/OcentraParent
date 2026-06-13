#![forbid(unsafe_code)]

use ocentra_parent_agent_protocol::{
    child_domain_notification_requested_event, constants, ChildDomainNotificationRequestedEvent,
    ChildDomainPolicyViolationDetectedEvent, ParentNotificationRequestedEvent,
    TrackingNotificationChannel, TrackingNotificationId, TrackingPolicyViolationDetectedEvent,
};

pub const CRATE_NAME: &str = "ocentra-child-notification-core";

pub fn request_parent_notification_from_policy_violation(
    event: &TrackingPolicyViolationDetectedEvent,
) -> ParentNotificationRequestedEvent {
    ParentNotificationRequestedEvent {
        child_device_id: event.child_device_id.clone(),
        child_profile_id: event.child_profile_id.clone(),
        notification_id: TrackingNotificationId::parse(
            constants::tracking_runtime::DEFAULT_NOTIFICATION_ID,
        )
        .expect(constants::tracking_runtime::DEFAULT_NOTIFICATION_ID),
        source_policy_violation_id: event.violation_id.clone(),
        channel: TrackingNotificationChannel::parse(
            constants::tracking_runtime::NOTIFICATION_CHANNEL_PARENT_PORTAL,
        )
        .expect(constants::tracking_runtime::NOTIFICATION_CHANNEL_PARENT_PORTAL),
        evidence_refs: event.evidence_refs.clone(),
    }
}

pub fn request_child_domain_parent_notification(
    event: &ChildDomainPolicyViolationDetectedEvent,
) -> ChildDomainNotificationRequestedEvent {
    child_domain_notification_requested_event(event)
}
