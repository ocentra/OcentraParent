use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::tracking::identifiers::{
    tracking_evidence_ref_from_observation_id, TrackingCapabilityStatus, TrackingObservationId,
};
use ocentra_tracking_core::geofence::{TrackingGeofenceEvaluation, TrackingGeofenceInsideState};

#[test]
fn geofence_transition_marks_low_accuracy_boundaries_as_ambiguous() {
    let observed = ocentra_tracking_core::runtime_flow::default_location_observed_event();

    let transition = ocentra_tracking_core::geofence::detect_geofence_transition(
        &observed,
        TrackingGeofenceEvaluation {
            previous_inside_state: Some(TrackingGeofenceInsideState::Outside),
            current_inside_state: TrackingGeofenceInsideState::Inside,
            capability_status: TrackingCapabilityStatus::parse(
                constants::tracking_runtime::CAPABILITY_STATUS_RECENT,
            )
            .expect_value(constants::tracking_runtime::CAPABILITY_STATUS_RECENT),
            distance_meters: Some(35),
            low_accuracy_near_boundary: true,
            grace_period_active: false,
        },
    );

    assert_eq!(
        transition.transition_kind,
        constants::tracking_runtime::GEOFENCE_TRANSITION_AMBIGUOUS
    );
    assert_eq!(
        transition.reason_codes[0],
        constants::tracking_runtime::REASON_LOCATION_ACCURACY_BELOW_RULE_THRESHOLD
    );
    assert_eq!(transition.source_observed_at, observed.observed_at);
}

#[test]
fn geofence_transition_rejects_stale_location_as_stale_at_place() {
    let observed = ocentra_tracking_core::runtime_flow::default_location_observed_event();

    let transition = ocentra_tracking_core::geofence::detect_geofence_transition(
        &observed,
        TrackingGeofenceEvaluation {
            previous_inside_state: Some(TrackingGeofenceInsideState::Inside),
            current_inside_state: TrackingGeofenceInsideState::Inside,
            capability_status: TrackingCapabilityStatus::parse(
                constants::tracking_runtime::CAPABILITY_STATUS_STALE,
            )
            .expect_value(constants::tracking_runtime::CAPABILITY_STATUS_STALE),
            distance_meters: Some(0),
            low_accuracy_near_boundary: false,
            grace_period_active: false,
        },
    );

    assert_eq!(
        transition.transition_kind,
        constants::tracking_runtime::GEOFENCE_TRANSITION_STALE_AT_PLACE
    );
    assert_eq!(
        transition.reason_codes[0],
        constants::tracking_runtime::REASON_STALE_LOCATION_REJECTED
    );
    assert_eq!(transition.source_observed_at, observed.observed_at);
}

#[test]
fn geofence_transition_grace_period_suppresses_exit_and_preserves_citations() {
    let mut observed = ocentra_tracking_core::runtime_flow::default_location_observed_event();
    observed.observation_id = TrackingObservationId::parse("tracking-observation-grace-period")
        .expect_value("tracking grace observation id parses");
    let evidence_ref = tracking_evidence_ref_from_observation_id(&observed.observation_id);

    let transition = ocentra_tracking_core::geofence::detect_geofence_transition(
        &observed,
        TrackingGeofenceEvaluation {
            previous_inside_state: Some(TrackingGeofenceInsideState::Inside),
            current_inside_state: TrackingGeofenceInsideState::Outside,
            capability_status: TrackingCapabilityStatus::parse(
                constants::tracking_runtime::CAPABILITY_STATUS_LIVE,
            )
            .expect_value(constants::tracking_runtime::CAPABILITY_STATUS_LIVE),
            distance_meters: Some(18),
            low_accuracy_near_boundary: false,
            grace_period_active: true,
        },
    );

    assert_eq!(
        transition.transition_kind,
        constants::tracking_runtime::GEOFENCE_TRANSITION_AMBIGUOUS
    );
    assert_eq!(
        transition.reason_codes,
        vec![constants::tracking_runtime::REASON_GEOFENCE_GRACE_ACTIVE]
    );
    assert_eq!(
        transition.geofence_rule_ref,
        constants::tracking_runtime::DEFAULT_GEOFENCE_RULE_REF
    );
    assert_eq!(transition.evidence_refs, vec![evidence_ref]);
}
