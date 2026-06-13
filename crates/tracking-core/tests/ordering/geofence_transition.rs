use ocentra_parent_agent_protocol::constants;
use ocentra_tracking_core::TrackingGeofenceInsideState;

#[test]
fn geofence_transition_detects_enter_exit_dwell_and_unchanged_ordering() {
    let observed = ocentra_tracking_core::default_location_observed_event();

    let enter = ocentra_tracking_core::detect_geofence_transition(
        &observed,
        ocentra_tracking_core::TrackingGeofenceEvaluation {
            previous_inside_state: Some(TrackingGeofenceInsideState::Outside),
            current_inside_state: TrackingGeofenceInsideState::Inside,
        },
    );
    let exit = ocentra_tracking_core::detect_geofence_transition(
        &observed,
        ocentra_tracking_core::TrackingGeofenceEvaluation {
            previous_inside_state: Some(TrackingGeofenceInsideState::Inside),
            current_inside_state: TrackingGeofenceInsideState::Outside,
        },
    );
    let dwell = ocentra_tracking_core::detect_geofence_transition(
        &observed,
        ocentra_tracking_core::TrackingGeofenceEvaluation {
            previous_inside_state: Some(TrackingGeofenceInsideState::Inside),
            current_inside_state: TrackingGeofenceInsideState::Inside,
        },
    );
    let unchanged = ocentra_tracking_core::detect_geofence_transition(
        &observed,
        ocentra_tracking_core::TrackingGeofenceEvaluation {
            previous_inside_state: Some(TrackingGeofenceInsideState::Outside),
            current_inside_state: TrackingGeofenceInsideState::Outside,
        },
    );

    assert_eq!(
        enter.transition_kind,
        constants::tracking_runtime::GEOFENCE_TRANSITION_ENTER
    );
    assert_eq!(
        exit.transition_kind,
        constants::tracking_runtime::GEOFENCE_TRANSITION_EXIT
    );
    assert_eq!(
        dwell.transition_kind,
        constants::tracking_runtime::GEOFENCE_TRANSITION_DWELL
    );
    assert_eq!(
        unchanged.transition_kind,
        constants::tracking_runtime::GEOFENCE_TRANSITION_UNCHANGED
    );
}
