use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::tracking::identifiers::{
    tracking_evaluation_id_from_observation_id, TrackingCapabilityStatus,
    TrackingExpectedPlaceState, TrackingScheduleId, TrackingTransitionKind,
};
use ocentra_parent_agent_protocol::tracking::runtime_event::{
    TrackingEvidenceRecordedEvent, TrackingExpectedPlaceStateEvaluatedEvent,
};

#[path = "expected_place_evaluation_rules.rs"]
mod expected_place_evaluation_rules;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TrackingExpectedPlaceWindow {
    pub start_minute_of_day: u16,
    pub end_minute_of_day: u16,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackingExpectedPlaceEvaluation {
    pub schedule_id: TrackingScheduleId,
    pub schedule_enabled: bool,
    pub within_expected_window: bool,
    pub distance_tolerance_meters: Option<u32>,
    pub capability_status: TrackingCapabilityStatus,
    pub transition_kind: TrackingTransitionKind,
    pub late_grace_seconds: u32,
    pub early_exit_grace_seconds: u32,
    pub late_grace_active: bool,
    pub early_exit_grace_active: bool,
    pub active_exception: Option<TrackingExpectedPlaceException>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TrackingExpectedPlaceException {
    HolidayMode,
    TripException,
}

pub fn expected_place_window_contains_minute(
    window: TrackingExpectedPlaceWindow,
    minute_of_day: u16,
) -> bool {
    let TrackingExpectedPlaceWindow {
        start_minute_of_day,
        end_minute_of_day,
    } = window;

    if start_minute_of_day <= end_minute_of_day {
        return minute_of_day >= start_minute_of_day && minute_of_day <= end_minute_of_day;
    }

    minute_of_day >= start_minute_of_day || minute_of_day <= end_minute_of_day
}

pub fn evaluate_expected_place_state(
    event: &TrackingEvidenceRecordedEvent,
    evaluation: TrackingExpectedPlaceEvaluation,
) -> TrackingExpectedPlaceStateEvaluatedEvent {
    let (expected_place_state, reason_codes) =
        expected_place_evaluation_rules::expected_place_outcome_for(&evaluation);

    TrackingExpectedPlaceStateEvaluatedEvent {
        child_device_id: event.child_device_id.clone(),
        child_profile_id: event.child_profile_id.clone(),
        evaluation_id: tracking_evaluation_id_from_observation_id(&event.source_observation_id),
        schedule_id: evaluation.schedule_id,
        expected_place_ref: event.expected_place_ref.clone(),
        source_observation_id: event.source_observation_id.clone(),
        source_observed_at: event.source_observed_at.clone(),
        expected_place_state: parse_contract_text(
            expected_place_state,
            TrackingExpectedPlaceState::parse,
        ),
        distance_tolerance_meters: evaluation.distance_tolerance_meters,
        late_grace_seconds: evaluation.late_grace_seconds,
        early_exit_grace_seconds: evaluation.early_exit_grace_seconds,
        exception_state: evaluation.active_exception.as_ref().map(
            expected_place_evaluation_rules::protocol_exception_state_for_expected_place_exception,
        ),
        reason_codes,
        evidence_refs: vec![event.evidence_ref.clone()],
        parent_action_requirement:
            expected_place_evaluation_rules::parent_action_requirement_for_expected_place_state(
                expected_place_state,
            ),
    }
}

pub fn default_expected_place_evaluation() -> TrackingExpectedPlaceEvaluation {
    TrackingExpectedPlaceEvaluation {
        schedule_id: parse_contract_text(
            constants::tracking_runtime::DEFAULT_EXPECTED_PLACE_SCHEDULE_ID,
            TrackingScheduleId::parse,
        ),
        schedule_enabled: true,
        within_expected_window: true,
        distance_tolerance_meters: Some(
            constants::tracking_runtime::DEFAULT_EXPECTED_PLACE_DISTANCE_TOLERANCE_METERS,
        ),
        capability_status: parse_contract_text(
            constants::tracking_runtime::CAPABILITY_STATUS_LIVE,
            TrackingCapabilityStatus::parse,
        ),
        transition_kind: parse_contract_text(
            constants::tracking_runtime::GEOFENCE_TRANSITION_AMBIGUOUS,
            TrackingTransitionKind::parse,
        ),
        late_grace_seconds: constants::tracking_runtime::DEFAULT_EXPECTED_PLACE_LATE_GRACE_SECONDS,
        early_exit_grace_seconds:
            constants::tracking_runtime::DEFAULT_EXPECTED_PLACE_EARLY_EXIT_GRACE_SECONDS,
        late_grace_active: false,
        early_exit_grace_active: false,
        active_exception: None,
    }
}

fn parse_contract_text<T, E>(
    value: &'static str,
    parse: impl FnOnce(&'static str) -> Result<T, E>,
) -> T
where
    E: core::fmt::Debug,
{
    parse(value).expect_value("tracking expected-place contract drift")
}
