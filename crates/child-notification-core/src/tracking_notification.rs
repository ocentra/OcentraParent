use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::tracking::identifiers::{
    tracking_notification_id_from_violation_id, TrackingNotificationChannel,
};
use ocentra_parent_agent_protocol::tracking::runtime_event::{
    ParentNotificationRequestedEvent, TrackingPolicyViolationDetectedEvent,
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
        .expect_value(constants::tracking_runtime::NOTIFICATION_CHANNEL_PARENT_PORTAL),
        requested_at: event.detected_at.clone(),
        evidence_refs: event.evidence_refs.clone(),
    }
}
