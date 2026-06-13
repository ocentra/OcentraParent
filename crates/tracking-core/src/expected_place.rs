use ocentra_parent_agent_protocol::{
    constants, TrackingEvaluationId, TrackingEvidenceRecordedEvent, TrackingExpectedPlaceRef,
    TrackingExpectedPlaceState, TrackingExpectedPlaceStateEvaluatedEvent, TrackingLocationRelation,
    TrackingParentActionRequirement,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackingExpectedPlaceWindow {
    pub start_minute_of_day: u16,
    pub end_minute_of_day: u16,
}

pub fn expected_place_window_contains_minute(
    window: TrackingExpectedPlaceWindow,
    minute_of_day: u16,
) -> bool {
    if window.start_minute_of_day <= window.end_minute_of_day {
        return minute_of_day >= window.start_minute_of_day
            && minute_of_day <= window.end_minute_of_day;
    }

    minute_of_day >= window.start_minute_of_day || minute_of_day <= window.end_minute_of_day
}

pub fn evaluate_expected_place_state(
    event: &TrackingEvidenceRecordedEvent,
) -> TrackingExpectedPlaceStateEvaluatedEvent {
    let expected_place_state = expected_place_state_for_relation(&event.location_relation);

    TrackingExpectedPlaceStateEvaluatedEvent {
        child_device_id: event.child_device_id.clone(),
        child_profile_id: event.child_profile_id.clone(),
        evaluation_id: TrackingEvaluationId::parse(
            constants::tracking_runtime::DEFAULT_EXPECTED_PLACE_EVALUATION_ID,
        )
        .expect(constants::tracking_runtime::DEFAULT_EXPECTED_PLACE_EVALUATION_ID),
        expected_place_ref: TrackingExpectedPlaceRef::parse(
            constants::tracking_runtime::DEFAULT_EXPECTED_PLACE_REF,
        )
        .expect(constants::tracking_runtime::DEFAULT_EXPECTED_PLACE_REF),
        source_observation_id: event.source_observation_id.clone(),
        expected_place_state: TrackingExpectedPlaceState::parse(expected_place_state)
            .expect(constants::tracking_runtime::EXPECTED_PLACE_STATE_UNKNOWN),
        evidence_refs: vec![event.evidence_ref.clone()],
        parent_action_requirement: parent_action_requirement_for_expected_place_state(
            expected_place_state,
        ),
    }
}

fn parent_action_requirement_for_expected_place_state(
    expected_place_state: &'static str,
) -> TrackingParentActionRequirement {
    if expected_place_state
        == constants::tracking_runtime::EXPECTED_PLACE_STATE_AWAY_FROM_EXPECTED_PLACE
    {
        TrackingParentActionRequirement::Required
    } else {
        TrackingParentActionRequirement::NotRequired
    }
}

fn expected_place_state_for_relation(location_relation: &TrackingLocationRelation) -> &'static str {
    if location_relation.as_str()
        == constants::tracking_runtime::LOCATION_RELATION_AT_EXPECTED_PLACE
    {
        constants::tracking_runtime::EXPECTED_PLACE_STATE_AT_EXPECTED_PLACE
    } else if location_relation.as_str()
        == constants::tracking_runtime::LOCATION_RELATION_AWAY_FROM_EXPECTED_PLACE
    {
        constants::tracking_runtime::EXPECTED_PLACE_STATE_AWAY_FROM_EXPECTED_PLACE
    } else {
        constants::tracking_runtime::EXPECTED_PLACE_STATE_UNKNOWN
    }
}
