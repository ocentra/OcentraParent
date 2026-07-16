use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::tracking::identifiers::TrackingCapabilityStatus;
use ocentra_parent_agent_protocol::tracking::runtime_event::TrackingGeofenceTransitionDetectedEvent;
use ocentra_tracking_core::geofence::TrackingGeofenceInsideState;

#[test]
fn geofence_transition_detects_enter_exit_dwell_unchanged_ambiguous_and_stale_ordering() {
    let enter = detect_transition(
        TrackingGeofenceInsideState::Outside,
        TrackingGeofenceInsideState::Inside,
        constants::tracking_runtime::CAPABILITY_STATUS_LIVE,
        Some(0),
        false,
    );
    let exit = detect_transition(
        TrackingGeofenceInsideState::Inside,
        TrackingGeofenceInsideState::Outside,
        constants::tracking_runtime::CAPABILITY_STATUS_LIVE,
        Some(120),
        false,
    );
    let dwell = detect_transition(
        TrackingGeofenceInsideState::Inside,
        TrackingGeofenceInsideState::Inside,
        constants::tracking_runtime::CAPABILITY_STATUS_LIVE,
        Some(0),
        false,
    );
    let unchanged = detect_transition(
        TrackingGeofenceInsideState::Outside,
        TrackingGeofenceInsideState::Outside,
        constants::tracking_runtime::CAPABILITY_STATUS_LIVE,
        Some(420),
        false,
    );
    let ambiguous = detect_transition(
        TrackingGeofenceInsideState::Outside,
        TrackingGeofenceInsideState::Inside,
        constants::tracking_runtime::CAPABILITY_STATUS_RECENT,
        Some(35),
        true,
    );
    let stale = detect_transition(
        TrackingGeofenceInsideState::Inside,
        TrackingGeofenceInsideState::Inside,
        constants::tracking_runtime::CAPABILITY_STATUS_STALE,
        Some(0),
        false,
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

fn detect_transition(
    previous_inside_state: TrackingGeofenceInsideState,
    current_inside_state: TrackingGeofenceInsideState,
    capability_status: impl core::fmt::Display,
    distance_meters: Option<u32>,
    low_accuracy_near_boundary: bool,
) -> TrackingGeofenceTransitionDetectedEvent {
    let capability_status = capability_status.to_string();
    let observed = ocentra_tracking_core::runtime_flow::default_location_observed_event();
    ocentra_tracking_core::geofence::detect_geofence_transition(
        &observed,
        ocentra_tracking_core::geofence::TrackingGeofenceEvaluation {
            previous_inside_state: Some(previous_inside_state),
            current_inside_state,
            capability_status: TrackingCapabilityStatus::parse(&capability_status)
                .expect_value(&capability_status),
            distance_meters,
            low_accuracy_near_boundary,
            grace_period_active: false,
        },
    )
}
