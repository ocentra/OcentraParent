use ocentra_parent_agent_protocol::{
    constants, TrackingCapabilityStatus, TrackingParentActionRequirement, TrackingTransitionKind,
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
    assert_eq!(expected_place.evidence_refs, vec![evidence.evidence_ref]);
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
}
