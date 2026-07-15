use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::tracking::identifiers::{
    TrackingCapabilityStatus, TrackingScheduleId, TrackingTransitionKind,
};
use ocentra_parent_agent_protocol::tracking::runtime_event::{
    TrackingExpectedPlaceExceptionState, TrackingParentActionRequirement,
};

struct ExpectedPlaceScheduleCase {
    schedule_id: &'static str,
    transition_kind: &'static str,
    distance_tolerance_meters: u32,
    late_grace_seconds: u32,
    early_exit_grace_seconds: u32,
    expected_state: &'static str,
    expected_reason: &'static str,
    expected_parent_action_requirement: TrackingParentActionRequirement,
}
#[test]
fn expected_place_evaluation_marks_uncertain_location_without_parent_action() {
    let observed = ocentra_tracking_core::runtime_flow::default_location_observed_event();
    let evidence =
        ocentra_tracking_core::runtime_flow::record_tracking_evidence_from_location(&observed);

    let evaluation = ocentra_tracking_core::expected_place::default_expected_place_evaluation();
    let evaluation = ocentra_tracking_core::expected_place::TrackingExpectedPlaceEvaluation {
        transition_kind: TrackingTransitionKind::parse(
            constants::tracking_runtime::GEOFENCE_TRANSITION_AMBIGUOUS,
        )
        .expect_value(constants::tracking_runtime::GEOFENCE_TRANSITION_AMBIGUOUS),
        ..evaluation
    };

    let expected_place =
        ocentra_tracking_core::expected_place::evaluate_expected_place_state(&evidence, evaluation);

    assert_eq!(
        expected_place.expected_place_state,
        constants::tracking_runtime::EXPECTED_PLACE_STATE_UNKNOWN
    );
    assert_eq!(
        expected_place.parent_action_requirement,
        TrackingParentActionRequirement::NotRequired
    );
    assert_eq!(
        expected_place.reason_codes[0],
        constants::tracking_runtime::REASON_EXPECTED_PLACE_AMBIGUOUS
    );
    assert_eq!(
        expected_place.expected_place_ref,
        observed.expected_place_ref
    );
    assert_eq!(expected_place.source_observed_at, observed.observed_at);
    assert_eq!(expected_place.evidence_refs, vec![evidence.evidence_ref]);
    assert_eq!(
        expected_place.distance_tolerance_meters,
        Some(constants::tracking_runtime::DEFAULT_EXPECTED_PLACE_DISTANCE_TOLERANCE_METERS)
    );
    assert_eq!(
        expected_place.late_grace_seconds,
        constants::tracking_runtime::DEFAULT_EXPECTED_PLACE_LATE_GRACE_SECONDS
    );
    assert_eq!(
        expected_place.early_exit_grace_seconds,
        constants::tracking_runtime::DEFAULT_EXPECTED_PLACE_EARLY_EXIT_GRACE_SECONDS
    );
    assert_eq!(expected_place.exception_state, None);
}
#[test]
fn expected_place_exit_requires_parent_action() {
    let observed = ocentra_tracking_core::runtime_flow::default_location_observed_event();
    let evidence =
        ocentra_tracking_core::runtime_flow::record_tracking_evidence_from_location(&observed);

    let evaluation = ocentra_tracking_core::expected_place::TrackingExpectedPlaceEvaluation {
        transition_kind: TrackingTransitionKind::parse(
            constants::tracking_runtime::GEOFENCE_TRANSITION_EXIT,
        )
        .expect_value(constants::tracking_runtime::GEOFENCE_TRANSITION_EXIT),
        ..ocentra_tracking_core::expected_place::default_expected_place_evaluation()
    };

    let expected_place =
        ocentra_tracking_core::expected_place::evaluate_expected_place_state(&evidence, evaluation);

    assert_eq!(
        expected_place.expected_place_state,
        constants::tracking_runtime::EXPECTED_PLACE_STATE_LEFT_EXPECTED_PLACE
    );
    assert_eq!(
        expected_place.parent_action_requirement,
        TrackingParentActionRequirement::Required
    );
    assert_eq!(
        expected_place.reason_codes[0],
        constants::tracking_runtime::REASON_EXITED_EXPECTED_PLACE_WINDOW
    );
    assert_eq!(expected_place.source_observed_at, observed.observed_at);
}

#[test]
fn expected_place_marks_stale_capability_as_manual_required() {
    let observed = ocentra_tracking_core::runtime_flow::default_location_observed_event();
    let evidence =
        ocentra_tracking_core::runtime_flow::record_tracking_evidence_from_location(&observed);

    let evaluation = ocentra_tracking_core::expected_place::TrackingExpectedPlaceEvaluation {
        capability_status: TrackingCapabilityStatus::parse(
            constants::tracking_runtime::CAPABILITY_STATUS_STALE,
        )
        .expect_value(constants::tracking_runtime::CAPABILITY_STATUS_STALE),
        transition_kind: TrackingTransitionKind::parse(
            constants::tracking_runtime::GEOFENCE_TRANSITION_DWELL,
        )
        .expect_value(constants::tracking_runtime::GEOFENCE_TRANSITION_DWELL),
        ..ocentra_tracking_core::expected_place::default_expected_place_evaluation()
    };

    let expected_place =
        ocentra_tracking_core::expected_place::evaluate_expected_place_state(&evidence, evaluation);

    assert_eq!(
        expected_place.expected_place_state,
        constants::tracking_runtime::EXPECTED_PLACE_STATE_MANUAL_REQUIRED
    );
    assert_eq!(
        expected_place.reason_codes[0],
        constants::tracking_runtime::REASON_FRESH_LOCATION_REQUIRED
    );
    assert_eq!(expected_place.source_observed_at, observed.observed_at);
}

#[test]
fn expected_place_missed_arrival_outside_grace_requires_parent_action() {
    let observed = ocentra_tracking_core::runtime_flow::default_location_observed_event();
    let evidence =
        ocentra_tracking_core::runtime_flow::record_tracking_evidence_from_location(&observed);

    let evaluation = ocentra_tracking_core::expected_place::TrackingExpectedPlaceEvaluation {
        transition_kind: TrackingTransitionKind::parse(
            constants::tracking_runtime::GEOFENCE_TRANSITION_MISSED_ARRIVAL,
        )
        .expect_value(constants::tracking_runtime::GEOFENCE_TRANSITION_MISSED_ARRIVAL),
        ..ocentra_tracking_core::expected_place::default_expected_place_evaluation()
    };

    let expected_place =
        ocentra_tracking_core::expected_place::evaluate_expected_place_state(&evidence, evaluation);

    assert_eq!(
        expected_place.expected_place_state,
        constants::tracking_runtime::EXPECTED_PLACE_STATE_LATE_ARRIVAL
    );
    assert_eq!(
        expected_place.parent_action_requirement,
        TrackingParentActionRequirement::Required
    );
    assert_eq!(
        expected_place.reason_codes[0],
        constants::tracking_runtime::REASON_MISSED_EXPECTED_PLACE_ARRIVAL
    );
}

#[test]
fn expected_place_late_grace_suppresses_missed_arrival() {
    let observed = ocentra_tracking_core::runtime_flow::default_location_observed_event();
    let evidence =
        ocentra_tracking_core::runtime_flow::record_tracking_evidence_from_location(&observed);

    let evaluation = ocentra_tracking_core::expected_place::TrackingExpectedPlaceEvaluation {
        transition_kind: TrackingTransitionKind::parse(
            constants::tracking_runtime::GEOFENCE_TRANSITION_MISSED_ARRIVAL,
        )
        .expect_value(constants::tracking_runtime::GEOFENCE_TRANSITION_MISSED_ARRIVAL),
        late_grace_active: true,
        ..ocentra_tracking_core::expected_place::default_expected_place_evaluation()
    };

    let expected_place =
        ocentra_tracking_core::expected_place::evaluate_expected_place_state(&evidence, evaluation);

    assert_eq!(
        expected_place.expected_place_state,
        constants::tracking_runtime::EXPECTED_PLACE_STATE_UNKNOWN
    );
    assert_eq!(
        expected_place.parent_action_requirement,
        TrackingParentActionRequirement::NotRequired
    );
    assert_eq!(
        expected_place.reason_codes[0],
        constants::tracking_runtime::REASON_EXPECTED_PLACE_LATE_GRACE_ACTIVE
    );
}

#[test]
fn expected_place_early_exit_grace_suppresses_exit() {
    let observed = ocentra_tracking_core::runtime_flow::default_location_observed_event();
    let evidence =
        ocentra_tracking_core::runtime_flow::record_tracking_evidence_from_location(&observed);

    let evaluation = ocentra_tracking_core::expected_place::TrackingExpectedPlaceEvaluation {
        transition_kind: TrackingTransitionKind::parse(
            constants::tracking_runtime::GEOFENCE_TRANSITION_EXIT,
        )
        .expect_value(constants::tracking_runtime::GEOFENCE_TRANSITION_EXIT),
        early_exit_grace_active: true,
        ..ocentra_tracking_core::expected_place::default_expected_place_evaluation()
    };

    let expected_place =
        ocentra_tracking_core::expected_place::evaluate_expected_place_state(&evidence, evaluation);

    assert_eq!(
        expected_place.expected_place_state,
        constants::tracking_runtime::EXPECTED_PLACE_STATE_UNKNOWN
    );
    assert_eq!(
        expected_place.parent_action_requirement,
        TrackingParentActionRequirement::NotRequired
    );
    assert_eq!(
        expected_place.reason_codes[0],
        constants::tracking_runtime::REASON_EXPECTED_PLACE_EARLY_EXIT_GRACE_ACTIVE
    );
}

#[test]
fn expected_place_schedule_disabled_stays_manual_required() {
    let observed = ocentra_tracking_core::runtime_flow::default_location_observed_event();
    let evidence =
        ocentra_tracking_core::runtime_flow::record_tracking_evidence_from_location(&observed);

    let evaluation = ocentra_tracking_core::expected_place::TrackingExpectedPlaceEvaluation {
        schedule_enabled: false,
        transition_kind: TrackingTransitionKind::parse(
            constants::tracking_runtime::GEOFENCE_TRANSITION_DWELL,
        )
        .expect_value(constants::tracking_runtime::GEOFENCE_TRANSITION_DWELL),
        ..ocentra_tracking_core::expected_place::default_expected_place_evaluation()
    };

    let expected_place =
        ocentra_tracking_core::expected_place::evaluate_expected_place_state(&evidence, evaluation);

    assert_eq!(
        expected_place.expected_place_state,
        constants::tracking_runtime::EXPECTED_PLACE_STATE_MANUAL_REQUIRED
    );
    assert_eq!(
        expected_place.parent_action_requirement,
        TrackingParentActionRequirement::NotRequired
    );
    assert_eq!(
        expected_place.reason_codes[0],
        constants::tracking_runtime::REASON_EXPECTED_PLACE_SCHEDULE_DISABLED
    );
}

#[test]
fn expected_place_holiday_exception_suppresses_expected_arrival() {
    let observed = ocentra_tracking_core::runtime_flow::default_location_observed_event();
    let evidence =
        ocentra_tracking_core::runtime_flow::record_tracking_evidence_from_location(&observed);

    let evaluation = ocentra_tracking_core::expected_place::TrackingExpectedPlaceEvaluation {
        transition_kind: TrackingTransitionKind::parse(
            constants::tracking_runtime::GEOFENCE_TRANSITION_DWELL,
        )
        .expect_value(constants::tracking_runtime::GEOFENCE_TRANSITION_DWELL),
        active_exception: Some(
            ocentra_tracking_core::expected_place::TrackingExpectedPlaceException::HolidayMode,
        ),
        ..ocentra_tracking_core::expected_place::default_expected_place_evaluation()
    };

    let expected_place =
        ocentra_tracking_core::expected_place::evaluate_expected_place_state(&evidence, evaluation);

    assert_eq!(
        expected_place.expected_place_state,
        constants::tracking_runtime::EXPECTED_PLACE_STATE_UNKNOWN
    );
    assert_eq!(
        expected_place.parent_action_requirement,
        TrackingParentActionRequirement::NotRequired
    );
    assert_eq!(
        expected_place.reason_codes[0],
        constants::tracking_runtime::REASON_EXPECTED_PLACE_HOLIDAY_EXCEPTION_ACTIVE
    );
    assert_eq!(
        expected_place.exception_state,
        Some(TrackingExpectedPlaceExceptionState::HolidayMode)
    );
}

#[test]
fn expected_place_trip_exception_suppresses_missed_arrival() {
    let observed = ocentra_tracking_core::runtime_flow::default_location_observed_event();
    let evidence =
        ocentra_tracking_core::runtime_flow::record_tracking_evidence_from_location(&observed);

    let evaluation = ocentra_tracking_core::expected_place::TrackingExpectedPlaceEvaluation {
        transition_kind: TrackingTransitionKind::parse(
            constants::tracking_runtime::GEOFENCE_TRANSITION_MISSED_ARRIVAL,
        )
        .expect_value(constants::tracking_runtime::GEOFENCE_TRANSITION_MISSED_ARRIVAL),
        active_exception: Some(
            ocentra_tracking_core::expected_place::TrackingExpectedPlaceException::TripException,
        ),
        ..ocentra_tracking_core::expected_place::default_expected_place_evaluation()
    };

    let expected_place =
        ocentra_tracking_core::expected_place::evaluate_expected_place_state(&evidence, evaluation);

    assert_eq!(
        expected_place.expected_place_state,
        constants::tracking_runtime::EXPECTED_PLACE_STATE_UNKNOWN
    );
    assert_eq!(
        expected_place.parent_action_requirement,
        TrackingParentActionRequirement::NotRequired
    );
    assert_eq!(
        expected_place.reason_codes[0],
        constants::tracking_runtime::REASON_EXPECTED_PLACE_TRIP_EXCEPTION_ACTIVE
    );
    assert_eq!(
        expected_place.exception_state,
        Some(TrackingExpectedPlaceExceptionState::TripException)
    );
}

#[test]
fn expected_place_keeps_distinct_school_activity_and_calendar_schedule_cases() {
    let cases = [
        ExpectedPlaceScheduleCase {
            schedule_id: "school-weekday-schedule",
            transition_kind: constants::tracking_runtime::GEOFENCE_TRANSITION_ENTER,
            distance_tolerance_meters: 75,
            late_grace_seconds: 900,
            early_exit_grace_seconds: 60,
            expected_state: constants::tracking_runtime::EXPECTED_PLACE_STATE_WHERE_EXPECTED,
            expected_reason: constants::tracking_runtime::REASON_INSIDE_EXPECTED_PLACE_WINDOW,
            expected_parent_action_requirement: TrackingParentActionRequirement::NotRequired,
        },
        ExpectedPlaceScheduleCase {
            schedule_id: "after-school-activity-schedule",
            transition_kind: constants::tracking_runtime::GEOFENCE_TRANSITION_EXIT,
            distance_tolerance_meters: 120,
            late_grace_seconds: 300,
            early_exit_grace_seconds: 180,
            expected_state: constants::tracking_runtime::EXPECTED_PLACE_STATE_LEFT_EXPECTED_PLACE,
            expected_reason: constants::tracking_runtime::REASON_EXITED_EXPECTED_PLACE_WINDOW,
            expected_parent_action_requirement: TrackingParentActionRequirement::Required,
        },
        ExpectedPlaceScheduleCase {
            schedule_id: "calendar-appointment-schedule",
            transition_kind: constants::tracking_runtime::GEOFENCE_TRANSITION_MISSED_ARRIVAL,
            distance_tolerance_meters: 30,
            late_grace_seconds: 120,
            early_exit_grace_seconds: 0,
            expected_state: constants::tracking_runtime::EXPECTED_PLACE_STATE_LATE_ARRIVAL,
            expected_reason: constants::tracking_runtime::REASON_MISSED_EXPECTED_PLACE_ARRIVAL,
            expected_parent_action_requirement: TrackingParentActionRequirement::Required,
        },
    ];

    for case in cases {
        assert_expected_place_schedule_case(&case);
    }
}

fn assert_expected_place_schedule_case(case: &ExpectedPlaceScheduleCase) {
    let observed = ocentra_tracking_core::runtime_flow::default_location_observed_event();
    let evidence =
        ocentra_tracking_core::runtime_flow::record_tracking_evidence_from_location(&observed);
    let schedule_id = TrackingScheduleId::parse(case.schedule_id).expect_value(case.schedule_id);
    let evaluation = ocentra_tracking_core::expected_place::TrackingExpectedPlaceEvaluation {
        schedule_id: schedule_id.clone(),
        transition_kind: TrackingTransitionKind::parse(case.transition_kind)
            .expect_value(case.transition_kind),
        distance_tolerance_meters: Some(case.distance_tolerance_meters),
        late_grace_seconds: case.late_grace_seconds,
        early_exit_grace_seconds: case.early_exit_grace_seconds,
        ..ocentra_tracking_core::expected_place::default_expected_place_evaluation()
    };
    let expected_place =
        ocentra_tracking_core::expected_place::evaluate_expected_place_state(&evidence, evaluation);

    assert_eq!(expected_place.schedule_id, schedule_id);
    assert_eq!(expected_place.expected_place_state, case.expected_state);
    assert_eq!(expected_place.reason_codes[0], case.expected_reason);
    assert_eq!(
        expected_place.parent_action_requirement,
        case.expected_parent_action_requirement
    );
    assert_eq!(
        expected_place.distance_tolerance_meters,
        Some(case.distance_tolerance_meters)
    );
    assert_eq!(expected_place.late_grace_seconds, case.late_grace_seconds);
    assert_eq!(
        expected_place.early_exit_grace_seconds,
        case.early_exit_grace_seconds
    );
}
