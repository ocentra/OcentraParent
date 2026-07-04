use ocentra_parent_agent_protocol::constants;

use super::runtime_flow_values::{tracking_capability_status, tracking_transition_kind};
use super::{
    default_expected_place_evaluation, TrackingExpectedPlaceEvaluation, TrackingGeofenceEvaluation,
    TrackingGeofenceInsideState, TrackingLocationObservedEvent, TrackingLocationRelation,
    TrackingLocationRelationKind, TrackingParentActionRequirement,
    DEFAULT_EXPECTED_PLACE_LATITUDE_E7, DEFAULT_EXPECTED_PLACE_LONGITUDE_E7,
    EXPECTED_PLACE_MAX_DELTA_E7, PRECISE_EXPECTED_PLACE_ACCURACY_MAX_METERS,
};

pub(super) fn infer_tracking_location_relation(
    event: &TrackingLocationObservedEvent,
) -> TrackingLocationRelationKind {
    let latitude_delta = event
        .latitude_e7
        .abs_diff(DEFAULT_EXPECTED_PLACE_LATITUDE_E7);
    let longitude_delta = event
        .longitude_e7
        .abs_diff(DEFAULT_EXPECTED_PLACE_LONGITUDE_E7);
    let expected_place_delta = latitude_delta.max(longitude_delta);

    if expected_place_delta <= EXPECTED_PLACE_MAX_DELTA_E7 {
        if event.horizontal_accuracy_meters <= PRECISE_EXPECTED_PLACE_ACCURACY_MAX_METERS {
            TrackingLocationRelationKind::At
        } else {
            TrackingLocationRelationKind::UncertainNear
        }
    } else {
        TrackingLocationRelationKind::Away
    }
}

pub(super) fn default_tracking_geofence_evaluation_from_relation(
    relation: &TrackingLocationRelation,
) -> TrackingGeofenceEvaluation {
    if relation.as_str() == constants::tracking_runtime::LOCATION_RELATION_AT_EXPECTED_PLACE {
        TrackingGeofenceEvaluation {
            previous_inside_state: Some(TrackingGeofenceInsideState::Inside),
            current_inside_state: TrackingGeofenceInsideState::Inside,
            capability_status: tracking_capability_status(
                constants::tracking_runtime::CAPABILITY_STATUS_LIVE,
            ),
            distance_meters: Some(0),
            low_accuracy_near_boundary: false,
            grace_period_active: false,
        }
    } else if relation.as_str()
        == constants::tracking_runtime::LOCATION_RELATION_AWAY_FROM_EXPECTED_PLACE
    {
        TrackingGeofenceEvaluation {
            previous_inside_state: Some(TrackingGeofenceInsideState::Inside),
            current_inside_state: TrackingGeofenceInsideState::Outside,
            capability_status: tracking_capability_status(
                constants::tracking_runtime::CAPABILITY_STATUS_LIVE,
            ),
            distance_meters: Some(250),
            low_accuracy_near_boundary: false,
            grace_period_active: false,
        }
    } else {
        TrackingGeofenceEvaluation {
            previous_inside_state: Some(TrackingGeofenceInsideState::Outside),
            current_inside_state: TrackingGeofenceInsideState::Outside,
            capability_status: tracking_capability_status(
                constants::tracking_runtime::CAPABILITY_STATUS_RECENT,
            ),
            distance_meters: None,
            low_accuracy_near_boundary: true,
            grace_period_active: false,
        }
    }
}

pub(super) fn default_tracking_expected_place_evaluation_from_relation(
    relation: &TrackingLocationRelation,
    parent_action_requirement: &TrackingParentActionRequirement,
) -> TrackingExpectedPlaceEvaluation {
    if relation.as_str() == constants::tracking_runtime::LOCATION_RELATION_AT_EXPECTED_PLACE {
        TrackingExpectedPlaceEvaluation {
            transition_kind: tracking_transition_kind(
                constants::tracking_runtime::GEOFENCE_TRANSITION_DWELL,
            ),
            ..default_expected_place_evaluation()
        }
    } else if relation.as_str()
        == constants::tracking_runtime::LOCATION_RELATION_AWAY_FROM_EXPECTED_PLACE
    {
        let transition_kind =
            if *parent_action_requirement == TrackingParentActionRequirement::Required {
                constants::tracking_runtime::GEOFENCE_TRANSITION_EXIT
            } else {
                constants::tracking_runtime::GEOFENCE_TRANSITION_MISSED_ARRIVAL
            };

        TrackingExpectedPlaceEvaluation {
            transition_kind: tracking_transition_kind(transition_kind),
            ..default_expected_place_evaluation()
        }
    } else {
        TrackingExpectedPlaceEvaluation {
            transition_kind: tracking_transition_kind(
                constants::tracking_runtime::GEOFENCE_TRANSITION_AMBIGUOUS,
            ),
            capability_status: tracking_capability_status(
                constants::tracking_runtime::CAPABILITY_STATUS_RECENT,
            ),
            ..default_expected_place_evaluation()
        }
    }
}
