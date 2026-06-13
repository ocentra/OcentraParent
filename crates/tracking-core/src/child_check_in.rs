use ocentra_parent_agent_protocol::{
    constants, TrackingCheckInId, TrackingCheckInState, TrackingChildCheckInRecordedEvent,
    TrackingEvidenceRef, TrackingLocationObservedEvent, TrackingTimestamp,
};

pub fn record_child_check_in(event: &TrackingLocationObservedEvent) -> TrackingChildCheckInRecordedEvent {
    TrackingChildCheckInRecordedEvent {
        child_device_id: event.child_device_id.clone(),
        child_profile_id: event.child_profile_id.clone(),
        check_in_id: TrackingCheckInId::parse(constants::tracking_runtime::DEFAULT_CHILD_CHECK_IN_ID)
            .expect(constants::tracking_runtime::DEFAULT_CHILD_CHECK_IN_ID),
        source_observation_id: event.observation_id.clone(),
        checked_in_at: TrackingTimestamp::parse(constants::tracking_runtime::DEFAULT_OBSERVED_AT)
            .expect(constants::tracking_runtime::DEFAULT_OBSERVED_AT),
        check_in_state: TrackingCheckInState::parse(constants::tracking_runtime::CHECK_IN_STATE_RECEIVED)
            .expect(constants::tracking_runtime::CHECK_IN_STATE_RECEIVED),
        evidence_refs: vec![
            TrackingEvidenceRef::parse(constants::tracking_runtime::DEFAULT_EVIDENCE_REF)
                .expect(constants::tracking_runtime::DEFAULT_EVIDENCE_REF),
        ],
    }
}
