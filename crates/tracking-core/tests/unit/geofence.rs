use ocentra_parent_agent_protocol::{constants, TrackingCapabilityStatus};
use ocentra_tracking_core::{TrackingGeofenceEvaluation, TrackingGeofenceInsideState};

#[test]
fn geofence_transition_marks_low_accuracy_boundaries_as_ambiguous() {
    let observed = ocentra_tracking_core::default_location_observed_event();

    let transition = ocentra_tracking_core::detect_geofence_transition(
        &observed,
        TrackingGeofenceEvaluation {
            previous_inside_state: Some(TrackingGeofenceInsideState::Outside),
            current_inside_state: TrackingGeofenceInsideState::Inside,
            capability_status: TrackingCapabilityStatus::parse(
                constants::tracking_runtime::CAPABILITY_STATUS_RECENT,
            )
            .expect(constants::tracking_runtime::CAPABILITY_STATUS_RECENT),
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
    let observed = ocentra_tracking_core::default_location_observed_event();

    let transition = ocentra_tracking_core::detect_geofence_transition(
        &observed,
        TrackingGeofenceEvaluation {
            previous_inside_state: Some(TrackingGeofenceInsideState::Inside),
            current_inside_state: TrackingGeofenceInsideState::Inside,
            capability_status: TrackingCapabilityStatus::parse(
                constants::tracking_runtime::CAPABILITY_STATUS_STALE,
            )
            .expect(constants::tracking_runtime::CAPABILITY_STATUS_STALE),
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
