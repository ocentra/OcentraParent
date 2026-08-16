use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::tracking::identifiers::{
    tracking_acknowledgement_id_from_violation_id, TrackingAcknowledgementState,
};
use ocentra_parent_agent_protocol::tracking::runtime_event::{
    TrackingParentAcknowledgementRecordedEvent, TrackingPolicyViolationDetectedEvent,
};

pub fn record_parent_acknowledgement(
    event: &TrackingPolicyViolationDetectedEvent,
) -> TrackingParentAcknowledgementRecordedEvent {
    let acknowledgement_state = TrackingAcknowledgementState::parse(
        constants::tracking_runtime::ACKNOWLEDGEMENT_STATE_ACKNOWLEDGED,
    )
    .expect_value("tracking acknowledgement contract drift");

    TrackingParentAcknowledgementRecordedEvent {
        child_device_id: event.child_device_id.clone(),
        child_profile_id: event.child_profile_id.clone(),
        acknowledgement_id: tracking_acknowledgement_id_from_violation_id(&event.violation_id),
        source_policy_violation_id: event.violation_id.clone(),
        acknowledged_at: event.detected_at.clone(),
        acknowledgement_state,
        evidence_refs: event.evidence_refs.clone(),
    }
}
