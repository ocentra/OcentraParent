use ocentra_parent_agent_protocol::{
    constants, ParentNotificationRequestedEvent, TrackingNotificationChannel,
    TrackingPolicyViolationDetectedEvent, tracking_notification_id_from_violation_id,
};

pub fn request_parent_notification_from_policy_violation(
    event: &TrackingPolicyViolationDetectedEvent,
) -> ParentNotificationRequestedEvent {
    ParentNotificationRequestedEvent {
        child_device_id: event.child_device_id.clone(),
        child_profile_id: event.child_profile_id.clone(),
        notification_id: tracking_notification_id_from_violation_id(&event.violation_id),
        source_policy_violation_id: event.violation_id.clone(),
        channel: TrackingNotificationChannel::parse(
            constants::tracking_runtime::NOTIFICATION_CHANNEL_PARENT_PORTAL,
        )
        .expect(constants::tracking_runtime::NOTIFICATION_CHANNEL_PARENT_PORTAL),
        evidence_refs: event.evidence_refs.clone(),
    }
}
