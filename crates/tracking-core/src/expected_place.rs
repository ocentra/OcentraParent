use ocentra_parent_agent_protocol::{
    constants, tracking_evaluation_id_from_observation_id, TrackingCapabilityStatus,
    TrackingEvidenceRecordedEvent, TrackingExpectedPlaceExceptionState, TrackingExpectedPlaceState,
    TrackingExpectedPlaceStateEvaluatedEvent, TrackingParentActionRequirement, TrackingReasonCode,
    TrackingScheduleId, TrackingTransitionKind,
};

#[derive(Clone, Debug, PartialEq, Eq)]
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
    if window.start_minute_of_day <= window.end_minute_of_day {
        return minute_of_day >= window.start_minute_of_day
            && minute_of_day <= window.end_minute_of_day;
    }

    minute_of_day >= window.start_minute_of_day || minute_of_day <= window.end_minute_of_day
}

pub fn evaluate_expected_place_state(
    event: &TrackingEvidenceRecordedEvent,
    evaluation: TrackingExpectedPlaceEvaluation,
) -> TrackingExpectedPlaceStateEvaluatedEvent {
    let (expected_place_state, reason_codes) = expected_place_outcome_for(&evaluation);

    TrackingExpectedPlaceStateEvaluatedEvent {
        child_device_id: event.child_device_id.clone(),
        child_profile_id: event.child_profile_id.clone(),
        evaluation_id: tracking_evaluation_id_from_observation_id(&event.source_observation_id),
        schedule_id: evaluation.schedule_id,
        expected_place_ref: event.expected_place_ref.clone(),
        source_observation_id: event.source_observation_id.clone(),
        source_observed_at: event.source_observed_at.clone(),
        expected_place_state: TrackingExpectedPlaceState::parse(expected_place_state)
            .expect(constants::tracking_runtime::EXPECTED_PLACE_STATE_UNKNOWN),
        distance_tolerance_meters: evaluation.distance_tolerance_meters,
        late_grace_seconds: evaluation.late_grace_seconds,
        early_exit_grace_seconds: evaluation.early_exit_grace_seconds,
        exception_state: evaluation
            .active_exception
            .as_ref()
            .map(protocol_exception_state_for_expected_place_exception),
        reason_codes,
        evidence_refs: vec![event.evidence_ref.clone()],
        parent_action_requirement: parent_action_requirement_for_expected_place_state(
            expected_place_state,
        ),
    }
}

fn parent_action_requirement_for_expected_place_state(
    expected_place_state: &'static str,
) -> TrackingParentActionRequirement {
    if expected_place_state == constants::tracking_runtime::EXPECTED_PLACE_STATE_LEFT_EXPECTED_PLACE
        || expected_place_state == constants::tracking_runtime::EXPECTED_PLACE_STATE_LATE_ARRIVAL
    {
        TrackingParentActionRequirement::Required
    } else {
        TrackingParentActionRequirement::NotRequired
    }
}

fn expected_place_outcome_for(
    evaluation: &TrackingExpectedPlaceEvaluation,
) -> (&'static str, Vec<TrackingReasonCode>) {
    if !evaluation.schedule_enabled {
        return (
            constants::tracking_runtime::EXPECTED_PLACE_STATE_MANUAL_REQUIRED,
            vec![reason_code(
                constants::tracking_runtime::REASON_EXPECTED_PLACE_SCHEDULE_DISABLED,
            )],
        );
    }

    if capability_requires_manual_review(&evaluation.capability_status) {
        return (
            constants::tracking_runtime::EXPECTED_PLACE_STATE_MANUAL_REQUIRED,
            vec![reason_code(
                constants::tracking_runtime::REASON_FRESH_LOCATION_REQUIRED,
            )],
        );
    }

    if let Some(active_exception) = &evaluation.active_exception {
        return (
            constants::tracking_runtime::EXPECTED_PLACE_STATE_UNKNOWN,
            vec![reason_code(reason_code_for_expected_place_exception(
                active_exception,
            ))],
        );
    }

    if !evaluation.within_expected_window {
        return (
            constants::tracking_runtime::EXPECTED_PLACE_STATE_UNKNOWN,
            vec![reason_code(
                constants::tracking_runtime::REASON_OUTSIDE_EXPECTED_PLACE_WINDOW,
            )],
        );
    }

    if evaluation.late_grace_active {
        return (
            constants::tracking_runtime::EXPECTED_PLACE_STATE_UNKNOWN,
            vec![reason_code(
                constants::tracking_runtime::REASON_EXPECTED_PLACE_LATE_GRACE_ACTIVE,
            )],
        );
    }

    if evaluation.early_exit_grace_active {
        return (
            constants::tracking_runtime::EXPECTED_PLACE_STATE_UNKNOWN,
            vec![reason_code(
                constants::tracking_runtime::REASON_EXPECTED_PLACE_EARLY_EXIT_GRACE_ACTIVE,
            )],
        );
    }

    if evaluation.transition_kind == constants::tracking_runtime::GEOFENCE_TRANSITION_ENTER
        || evaluation.transition_kind == constants::tracking_runtime::GEOFENCE_TRANSITION_DWELL
    {
        return (
            constants::tracking_runtime::EXPECTED_PLACE_STATE_WHERE_EXPECTED,
            vec![reason_code(
                constants::tracking_runtime::REASON_INSIDE_EXPECTED_PLACE_WINDOW,
            )],
        );
    }

    if evaluation.transition_kind == constants::tracking_runtime::GEOFENCE_TRANSITION_EXIT {
        return (
            constants::tracking_runtime::EXPECTED_PLACE_STATE_LEFT_EXPECTED_PLACE,
            vec![reason_code(
                constants::tracking_runtime::REASON_EXITED_EXPECTED_PLACE_WINDOW,
            )],
        );
    }

    if evaluation.transition_kind == constants::tracking_runtime::GEOFENCE_TRANSITION_MISSED_ARRIVAL
    {
        return (
            constants::tracking_runtime::EXPECTED_PLACE_STATE_LATE_ARRIVAL,
            vec![reason_code(
                constants::tracking_runtime::REASON_MISSED_EXPECTED_PLACE_ARRIVAL,
            )],
        );
    }

    (
        constants::tracking_runtime::EXPECTED_PLACE_STATE_UNKNOWN,
        vec![reason_code(
            constants::tracking_runtime::REASON_EXPECTED_PLACE_AMBIGUOUS,
        )],
    )
}

fn capability_requires_manual_review(capability_status: &TrackingCapabilityStatus) -> bool {
    capability_status.as_str() == constants::tracking_runtime::CAPABILITY_STATUS_STALE
        || capability_status.as_str() == constants::tracking_runtime::CAPABILITY_STATUS_LAST_KNOWN
        || capability_status.as_str()
            == constants::tracking_runtime::CAPABILITY_STATUS_OFFLINE_LAST_KNOWN_ONLY
        || capability_status.as_str()
            == constants::tracking_runtime::CAPABILITY_STATUS_PERMISSION_REQUIRED
        || capability_status.as_str()
            == constants::tracking_runtime::CAPABILITY_STATUS_BACKGROUND_PERMISSION_REQUIRED
        || capability_status.as_str()
            == constants::tracking_runtime::CAPABILITY_STATUS_APPROXIMATE_ONLY
        || capability_status.as_str()
            == constants::tracking_runtime::CAPABILITY_STATUS_MANUAL_REQUIRED
        || capability_status.as_str() == constants::tracking_runtime::CAPABILITY_STATUS_UNAVAILABLE
        || capability_status.as_str()
            == constants::tracking_runtime::CAPABILITY_STATUS_ADAPTER_ERROR
        || capability_status.as_str()
            == constants::tracking_runtime::CAPABILITY_STATUS_DISABLED_BY_PARENT
}

fn reason_code(value: &'static str) -> TrackingReasonCode {
    TrackingReasonCode::parse(value).expect(value)
}

fn reason_code_for_expected_place_exception(
    active_exception: &TrackingExpectedPlaceException,
) -> &'static str {
    match active_exception {
        TrackingExpectedPlaceException::HolidayMode => {
            constants::tracking_runtime::REASON_EXPECTED_PLACE_HOLIDAY_EXCEPTION_ACTIVE
        }
        TrackingExpectedPlaceException::TripException => {
            constants::tracking_runtime::REASON_EXPECTED_PLACE_TRIP_EXCEPTION_ACTIVE
        }
    }
}

fn protocol_exception_state_for_expected_place_exception(
    active_exception: &TrackingExpectedPlaceException,
) -> TrackingExpectedPlaceExceptionState {
    match active_exception {
        TrackingExpectedPlaceException::HolidayMode => {
            TrackingExpectedPlaceExceptionState::HolidayMode
        }
        TrackingExpectedPlaceException::TripException => {
            TrackingExpectedPlaceExceptionState::TripException
        }
    }
}

pub fn default_expected_place_evaluation() -> TrackingExpectedPlaceEvaluation {
    TrackingExpectedPlaceEvaluation {
        schedule_id: TrackingScheduleId::parse(
            constants::tracking_runtime::DEFAULT_EXPECTED_PLACE_SCHEDULE_ID,
        )
        .expect(constants::tracking_runtime::DEFAULT_EXPECTED_PLACE_SCHEDULE_ID),
        schedule_enabled: true,
        within_expected_window: true,
        distance_tolerance_meters: Some(
            constants::tracking_runtime::DEFAULT_EXPECTED_PLACE_DISTANCE_TOLERANCE_METERS,
        ),
        capability_status: TrackingCapabilityStatus::parse(
            constants::tracking_runtime::CAPABILITY_STATUS_LIVE,
        )
        .expect(constants::tracking_runtime::CAPABILITY_STATUS_LIVE),
        transition_kind: TrackingTransitionKind::parse(
            constants::tracking_runtime::GEOFENCE_TRANSITION_AMBIGUOUS,
        )
        .expect(constants::tracking_runtime::GEOFENCE_TRANSITION_AMBIGUOUS),
        late_grace_seconds: constants::tracking_runtime::DEFAULT_EXPECTED_PLACE_LATE_GRACE_SECONDS,
        early_exit_grace_seconds:
            constants::tracking_runtime::DEFAULT_EXPECTED_PLACE_EARLY_EXIT_GRACE_SECONDS,
        late_grace_active: false,
        early_exit_grace_active: false,
        active_exception: None,
    }
}
