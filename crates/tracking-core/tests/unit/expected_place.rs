use ocentra_parent_agent_protocol::{
    constants, TrackingCapabilityStatus, TrackingParentActionRequirement, TrackingScheduleId,
    TrackingTransitionKind,
};

#[test]
fn expected_place_evaluation_marks_uncertain_location_without_parent_action() {
    let observed = ocentra_tracking_core::default_location_observed_event();
    let evidence = ocentra_tracking_core::record_tracking_evidence_from_location(&observed);

    let evaluation = ocentra_tracking_core::default_expected_place_evaluation();
    let evaluation = ocentra_tracking_core::TrackingExpectedPlaceEvaluation {
        transition_kind: TrackingTransitionKind::parse(
            constants::tracking_runtime::GEOFENCE_TRANSITION_AMBIGUOUS,
        )
        .expect(constants::tracking_runtime::GEOFENCE_TRANSITION_AMBIGUOUS),
        ..evaluation
    };

    let expected_place =
        ocentra_tracking_core::evaluate_expected_place_state(&evidence, evaluation);

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
    let observed = ocentra_tracking_core::default_location_observed_event();
    let evidence = ocentra_tracking_core::record_tracking_evidence_from_location(&observed);

    let evaluation = ocentra_tracking_core::TrackingExpectedPlaceEvaluation {
        transition_kind: TrackingTransitionKind::parse(
            constants::tracking_runtime::GEOFENCE_TRANSITION_EXIT,
        )
        .expect(constants::tracking_runtime::GEOFENCE_TRANSITION_EXIT),
        ..ocentra_tracking_core::default_expected_place_evaluation()
    };

    let expected_place =
        ocentra_tracking_core::evaluate_expected_place_state(&evidence, evaluation);

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
    let observed = ocentra_tracking_core::default_location_observed_event();
    let evidence = ocentra_tracking_core::record_tracking_evidence_from_location(&observed);

    let evaluation = ocentra_tracking_core::TrackingExpectedPlaceEvaluation {
        capability_status: TrackingCapabilityStatus::parse(
            constants::tracking_runtime::CAPABILITY_STATUS_STALE,
        )
        .expect(constants::tracking_runtime::CAPABILITY_STATUS_STALE),
        transition_kind: TrackingTransitionKind::parse(
            constants::tracking_runtime::GEOFENCE_TRANSITION_DWELL,
        )
        .expect(constants::tracking_runtime::GEOFENCE_TRANSITION_DWELL),
        ..ocentra_tracking_core::default_expected_place_evaluation()
    };

    let expected_place =
        ocentra_tracking_core::evaluate_expected_place_state(&evidence, evaluation);

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
    let observed = ocentra_tracking_core::default_location_observed_event();
    let evidence = ocentra_tracking_core::record_tracking_evidence_from_location(&observed);

    let evaluation = ocentra_tracking_core::TrackingExpectedPlaceEvaluation {
        transition_kind: TrackingTransitionKind::parse(
            constants::tracking_runtime::GEOFENCE_TRANSITION_MISSED_ARRIVAL,
        )
        .expect(constants::tracking_runtime::GEOFENCE_TRANSITION_MISSED_ARRIVAL),
        ..ocentra_tracking_core::default_expected_place_evaluation()
    };

    let expected_place =
        ocentra_tracking_core::evaluate_expected_place_state(&evidence, evaluation);

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
    let observed = ocentra_tracking_core::default_location_observed_event();
    let evidence = ocentra_tracking_core::record_tracking_evidence_from_location(&observed);

    let evaluation = ocentra_tracking_core::TrackingExpectedPlaceEvaluation {
        transition_kind: TrackingTransitionKind::parse(
            constants::tracking_runtime::GEOFENCE_TRANSITION_MISSED_ARRIVAL,
        )
        .expect(constants::tracking_runtime::GEOFENCE_TRANSITION_MISSED_ARRIVAL),
        late_grace_active: true,
        ..ocentra_tracking_core::default_expected_place_evaluation()
    };

    let expected_place =
        ocentra_tracking_core::evaluate_expected_place_state(&evidence, evaluation);

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
    let observed = ocentra_tracking_core::default_location_observed_event();
    let evidence = ocentra_tracking_core::record_tracking_evidence_from_location(&observed);

    let evaluation = ocentra_tracking_core::TrackingExpectedPlaceEvaluation {
        transition_kind: TrackingTransitionKind::parse(
            constants::tracking_runtime::GEOFENCE_TRANSITION_EXIT,
        )
        .expect(constants::tracking_runtime::GEOFENCE_TRANSITION_EXIT),
        early_exit_grace_active: true,
        ..ocentra_tracking_core::default_expected_place_evaluation()
    };

    let expected_place =
        ocentra_tracking_core::evaluate_expected_place_state(&evidence, evaluation);

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
    let observed = ocentra_tracking_core::default_location_observed_event();
    let evidence = ocentra_tracking_core::record_tracking_evidence_from_location(&observed);

    let evaluation = ocentra_tracking_core::TrackingExpectedPlaceEvaluation {
        schedule_enabled: false,
        transition_kind: TrackingTransitionKind::parse(
            constants::tracking_runtime::GEOFENCE_TRANSITION_DWELL,
        )
        .expect(constants::tracking_runtime::GEOFENCE_TRANSITION_DWELL),
        ..ocentra_tracking_core::default_expected_place_evaluation()
    };

    let expected_place =
        ocentra_tracking_core::evaluate_expected_place_state(&evidence, evaluation);

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
    let observed = ocentra_tracking_core::default_location_observed_event();
    let evidence = ocentra_tracking_core::record_tracking_evidence_from_location(&observed);

    let evaluation = ocentra_tracking_core::TrackingExpectedPlaceEvaluation {
        transition_kind: TrackingTransitionKind::parse(
            constants::tracking_runtime::GEOFENCE_TRANSITION_DWELL,
        )
        .expect(constants::tracking_runtime::GEOFENCE_TRANSITION_DWELL),
        active_exception: Some(ocentra_tracking_core::TrackingExpectedPlaceException::HolidayMode),
        ..ocentra_tracking_core::default_expected_place_evaluation()
    };

    let expected_place =
        ocentra_tracking_core::evaluate_expected_place_state(&evidence, evaluation);

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
        Some(ocentra_parent_agent_protocol::TrackingExpectedPlaceExceptionState::HolidayMode)
    );
}

#[test]
fn expected_place_trip_exception_suppresses_missed_arrival() {
    let observed = ocentra_tracking_core::default_location_observed_event();
    let evidence = ocentra_tracking_core::record_tracking_evidence_from_location(&observed);

    let evaluation = ocentra_tracking_core::TrackingExpectedPlaceEvaluation {
        transition_kind: TrackingTransitionKind::parse(
            constants::tracking_runtime::GEOFENCE_TRANSITION_MISSED_ARRIVAL,
        )
        .expect(constants::tracking_runtime::GEOFENCE_TRANSITION_MISSED_ARRIVAL),
        active_exception: Some(
            ocentra_tracking_core::TrackingExpectedPlaceException::TripException,
        ),
        ..ocentra_tracking_core::default_expected_place_evaluation()
    };

    let expected_place =
        ocentra_tracking_core::evaluate_expected_place_state(&evidence, evaluation);

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
        Some(ocentra_parent_agent_protocol::TrackingExpectedPlaceExceptionState::TripException)
    );
}

#[test]
fn expected_place_keeps_distinct_school_activity_and_calendar_schedule_cases() {
    let observed = ocentra_tracking_core::default_location_observed_event();
    let evidence = ocentra_tracking_core::record_tracking_evidence_from_location(&observed);

    let cases = [
        (
            "school-weekday-schedule",
            constants::tracking_runtime::GEOFENCE_TRANSITION_ENTER,
            75_u32,
            900_u32,
            60_u32,
            constants::tracking_runtime::EXPECTED_PLACE_STATE_WHERE_EXPECTED,
            constants::tracking_runtime::REASON_INSIDE_EXPECTED_PLACE_WINDOW,
            TrackingParentActionRequirement::NotRequired,
        ),
        (
            "after-school-activity-schedule",
            constants::tracking_runtime::GEOFENCE_TRANSITION_EXIT,
            120_u32,
            300_u32,
            180_u32,
            constants::tracking_runtime::EXPECTED_PLACE_STATE_LEFT_EXPECTED_PLACE,
            constants::tracking_runtime::REASON_EXITED_EXPECTED_PLACE_WINDOW,
            TrackingParentActionRequirement::Required,
        ),
        (
            "calendar-appointment-schedule",
            constants::tracking_runtime::GEOFENCE_TRANSITION_MISSED_ARRIVAL,
            30_u32,
            120_u32,
            0_u32,
            constants::tracking_runtime::EXPECTED_PLACE_STATE_LATE_ARRIVAL,
            constants::tracking_runtime::REASON_MISSED_EXPECTED_PLACE_ARRIVAL,
            TrackingParentActionRequirement::Required,
        ),
    ];

    for (
        schedule_id,
        transition_kind,
        distance_tolerance_meters,
        late_grace_seconds,
        early_exit_grace_seconds,
        expected_state,
        expected_reason,
        expected_parent_action_requirement,
    ) in cases
    {
        let evaluation = ocentra_tracking_core::TrackingExpectedPlaceEvaluation {
            schedule_id: TrackingScheduleId::parse(schedule_id).expect(schedule_id),
            transition_kind: TrackingTransitionKind::parse(transition_kind).expect(transition_kind),
            distance_tolerance_meters: Some(distance_tolerance_meters),
            late_grace_seconds,
            early_exit_grace_seconds,
            ..ocentra_tracking_core::default_expected_place_evaluation()
        };

        let expected_place =
            ocentra_tracking_core::evaluate_expected_place_state(&evidence, evaluation);

        assert_eq!(
            expected_place.schedule_id,
            TrackingScheduleId::parse(schedule_id).expect(schedule_id)
        );
        assert_eq!(expected_place.expected_place_state, expected_state);
        assert_eq!(expected_place.reason_codes[0], expected_reason);
        assert_eq!(
            expected_place.parent_action_requirement,
            expected_parent_action_requirement
        );
        assert_eq!(
            expected_place.distance_tolerance_meters,
            Some(distance_tolerance_meters)
        );
        assert_eq!(expected_place.late_grace_seconds, late_grace_seconds);
        assert_eq!(
            expected_place.early_exit_grace_seconds,
            early_exit_grace_seconds
        );
    }
}
