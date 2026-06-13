use ocentra_parent_agent_protocol::{
    constants, TrackingAcknowledgementState, TrackingParentAcknowledgementRecordedEvent,
    TrackingPolicyViolationDetectedEvent, TrackingTimestamp,
    tracking_acknowledgement_id_from_violation_id,
};

pub fn record_parent_acknowledgement(
    event: &TrackingPolicyViolationDetectedEvent,
) -> TrackingParentAcknowledgementRecordedEvent {
    TrackingParentAcknowledgementRecordedEvent {
        child_device_id: event.child_device_id.clone(),
        child_profile_id: event.child_profile_id.clone(),
        acknowledgement_id: tracking_acknowledgement_id_from_violation_id(&event.violation_id),
        source_policy_violation_id: event.violation_id.clone(),
        acknowledged_at: TrackingTimestamp::parse(constants::tracking_runtime::DEFAULT_OBSERVED_AT)
            .expect(constants::tracking_runtime::DEFAULT_OBSERVED_AT),
        acknowledgement_state: TrackingAcknowledgementState::parse(
            constants::tracking_runtime::ACKNOWLEDGEMENT_STATE_ACKNOWLEDGED,
        )
        .expect(constants::tracking_runtime::ACKNOWLEDGEMENT_STATE_ACKNOWLEDGED),
        evidence_refs: event.evidence_refs.clone(),
    }
}
