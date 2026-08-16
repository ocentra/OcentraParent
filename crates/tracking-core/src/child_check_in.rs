use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::tracking::identifiers::{
    tracking_check_in_id_from_observation_id, tracking_evidence_ref_from_observation_id,
    TrackingCheckInState,
};
use ocentra_parent_agent_protocol::tracking::runtime_event::{
    TrackingChildCheckInRecordedEvent, TrackingLocationObservedEvent,
};

pub fn record_child_check_in(
    event: &TrackingLocationObservedEvent,
) -> TrackingChildCheckInRecordedEvent {
    let check_in_state =
        TrackingCheckInState::parse(constants::tracking_runtime::CHECK_IN_STATE_RECEIVED)
            .expect_value("tracking check-in state contract drift");

    TrackingChildCheckInRecordedEvent {
        child_device_id: event.child_device_id.clone(),
        child_profile_id: event.child_profile_id.clone(),
        check_in_id: tracking_check_in_id_from_observation_id(&event.observation_id),
        source_observation_id: event.observation_id.clone(),
        checked_in_at: event.observed_at.clone(),
        check_in_state,
        evidence_refs: vec![tracking_evidence_ref_from_observation_id(
            &event.observation_id,
        )],
    }
}
