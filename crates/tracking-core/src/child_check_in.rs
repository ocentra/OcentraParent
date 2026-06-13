use ocentra_parent_agent_protocol::{
    constants, TrackingCheckInState, TrackingChildCheckInRecordedEvent,
    TrackingLocationObservedEvent, TrackingTimestamp,
    tracking_check_in_id_from_observation_id, tracking_evidence_ref_from_observation_id,
};

pub fn record_child_check_in(
    event: &TrackingLocationObservedEvent,
) -> TrackingChildCheckInRecordedEvent {
    TrackingChildCheckInRecordedEvent {
        child_device_id: event.child_device_id.clone(),
        child_profile_id: event.child_profile_id.clone(),
        check_in_id: tracking_check_in_id_from_observation_id(&event.observation_id),
        source_observation_id: event.observation_id.clone(),
        checked_in_at: TrackingTimestamp::parse(constants::tracking_runtime::DEFAULT_OBSERVED_AT)
            .expect(constants::tracking_runtime::DEFAULT_OBSERVED_AT),
        check_in_state: TrackingCheckInState::parse(
            constants::tracking_runtime::CHECK_IN_STATE_RECEIVED,
        )
        .expect(constants::tracking_runtime::CHECK_IN_STATE_RECEIVED),
        evidence_refs: vec![tracking_evidence_ref_from_observation_id(&event.observation_id)],
    }
}
