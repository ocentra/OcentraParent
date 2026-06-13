use ocentra_parent_agent_protocol::{constants, TrackingCapabilityStatus};
use ocentra_tracking_core::TrackingGeofenceInsideState;

#[test]
fn geofence_transition_detects_enter_exit_dwell_unchanged_ambiguous_and_stale_ordering() {
    let observed = ocentra_tracking_core::default_location_observed_event();

    let enter = ocentra_tracking_core::detect_geofence_transition(
        &observed,
        ocentra_tracking_core::TrackingGeofenceEvaluation {
            previous_inside_state: Some(TrackingGeofenceInsideState::Outside),
            current_inside_state: TrackingGeofenceInsideState::Inside,
            capability_status: TrackingCapabilityStatus::parse(
                constants::tracking_runtime::CAPABILITY_STATUS_LIVE,
            )
            .expect(constants::tracking_runtime::CAPABILITY_STATUS_LIVE),
            distance_meters: Some(0),
            low_accuracy_near_boundary: false,
            grace_period_active: false,
        },
    );
    let exit = ocentra_tracking_core::detect_geofence_transition(
        &observed,
        ocentra_tracking_core::TrackingGeofenceEvaluation {
            previous_inside_state: Some(TrackingGeofenceInsideState::Inside),
            current_inside_state: TrackingGeofenceInsideState::Outside,
            capability_status: TrackingCapabilityStatus::parse(
                constants::tracking_runtime::CAPABILITY_STATUS_LIVE,
            )
            .expect(constants::tracking_runtime::CAPABILITY_STATUS_LIVE),
            distance_meters: Some(120),
            low_accuracy_near_boundary: false,
            grace_period_active: false,
        },
    );
    let dwell = ocentra_tracking_core::detect_geofence_transition(
        &observed,
        ocentra_tracking_core::TrackingGeofenceEvaluation {
            previous_inside_state: Some(TrackingGeofenceInsideState::Inside),
            current_inside_state: TrackingGeofenceInsideState::Inside,
            capability_status: TrackingCapabilityStatus::parse(
                constants::tracking_runtime::CAPABILITY_STATUS_LIVE,
            )
            .expect(constants::tracking_runtime::CAPABILITY_STATUS_LIVE),
            distance_meters: Some(0),
            low_accuracy_near_boundary: false,
            grace_period_active: false,
        },
    );
    let unchanged = ocentra_tracking_core::detect_geofence_transition(
        &observed,
        ocentra_tracking_core::TrackingGeofenceEvaluation {
            previous_inside_state: Some(TrackingGeofenceInsideState::Outside),
            current_inside_state: TrackingGeofenceInsideState::Outside,
            capability_status: TrackingCapabilityStatus::parse(
                constants::tracking_runtime::CAPABILITY_STATUS_LIVE,
            )
            .expect(constants::tracking_runtime::CAPABILITY_STATUS_LIVE),
            distance_meters: Some(420),
            low_accuracy_near_boundary: false,
            grace_period_active: false,
        },
    );
    let ambiguous = ocentra_tracking_core::detect_geofence_transition(
        &observed,
        ocentra_tracking_core::TrackingGeofenceEvaluation {
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
    let stale = ocentra_tracking_core::detect_geofence_transition(
        &observed,
        ocentra_tracking_core::TrackingGeofenceEvaluation {
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
    assert_eq!(
        ambiguous.transition_kind,
        constants::tracking_runtime::GEOFENCE_TRANSITION_AMBIGUOUS
    );
    assert_eq!(
        stale.transition_kind,
        constants::tracking_runtime::GEOFENCE_TRANSITION_STALE_AT_PLACE
    );
}
