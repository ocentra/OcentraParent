use ocentra_parent_agent_protocol::{
    constants, TrackingAcknowledgementId, TrackingAcknowledgementState,
    TrackingParentAcknowledgementRecordedEvent, TrackingPolicyViolationDetectedEvent,
    TrackingTimestamp,
};

pub fn record_parent_acknowledgement(
    event: &TrackingPolicyViolationDetectedEvent,
) -> TrackingParentAcknowledgementRecordedEvent {
    TrackingParentAcknowledgementRecordedEvent {
        child_device_id: event.child_device_id.clone(),
        child_profile_id: event.child_profile_id.clone(),
        acknowledgement_id: TrackingAcknowledgementId::parse(
            constants::tracking_runtime::DEFAULT_PARENT_ACKNOWLEDGEMENT_ID,
        )
        .expect(constants::tracking_runtime::DEFAULT_PARENT_ACKNOWLEDGEMENT_ID),
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
