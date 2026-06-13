use ocentra_parent_agent_protocol::{
    child_domain_notification_requested_event, ChildDomainNotificationRequestedEvent,
    ChildDomainPolicyViolationDetectedEvent,
};

pub fn request_child_domain_parent_notification(
    event: &ChildDomainPolicyViolationDetectedEvent,
) -> ChildDomainNotificationRequestedEvent {
    child_domain_notification_requested_event(event)
}
