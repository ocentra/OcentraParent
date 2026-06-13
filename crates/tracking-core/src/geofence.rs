use ocentra_parent_agent_protocol::{
    constants, TrackingEvidenceRef, TrackingGeofenceRuleRef, TrackingGeofenceTransitionDetectedEvent,
    TrackingLocationObservedEvent, TrackingTransitionId, TrackingTransitionKind,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TrackingGeofenceInsideState {
    Inside,
    Outside,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackingGeofenceEvaluation {
    pub previous_inside_state: Option<TrackingGeofenceInsideState>,
    pub current_inside_state: TrackingGeofenceInsideState,
}

pub fn detect_geofence_transition(
    event: &TrackingLocationObservedEvent,
    evaluation: TrackingGeofenceEvaluation,
) -> TrackingGeofenceTransitionDetectedEvent {
    TrackingGeofenceTransitionDetectedEvent {
        child_device_id: event.child_device_id.clone(),
        child_profile_id: event.child_profile_id.clone(),
        transition_id: TrackingTransitionId::parse(
            constants::tracking_runtime::DEFAULT_GEOFENCE_TRANSITION_ID,
        )
        .expect(constants::tracking_runtime::DEFAULT_GEOFENCE_TRANSITION_ID),
        geofence_rule_ref: TrackingGeofenceRuleRef::parse(
            constants::tracking_runtime::DEFAULT_GEOFENCE_RULE_REF,
        )
        .expect(constants::tracking_runtime::DEFAULT_GEOFENCE_RULE_REF),
        source_observation_id: event.observation_id.clone(),
        transition_kind: TrackingTransitionKind::parse(transition_kind_for(evaluation))
            .expect(constants::tracking_runtime::GEOFENCE_TRANSITION_ENTER),
        evidence_refs: vec![
            TrackingEvidenceRef::parse(constants::tracking_runtime::DEFAULT_EVIDENCE_REF)
                .expect(constants::tracking_runtime::DEFAULT_EVIDENCE_REF),
        ],
    }
}

fn transition_kind_for(evaluation: TrackingGeofenceEvaluation) -> &'static str {
    match (
        evaluation.previous_inside_state,
        evaluation.current_inside_state,
    ) {
        (Some(TrackingGeofenceInsideState::Outside), TrackingGeofenceInsideState::Inside) => {
            constants::tracking_runtime::GEOFENCE_TRANSITION_ENTER
        }
        (Some(TrackingGeofenceInsideState::Inside), TrackingGeofenceInsideState::Outside) => {
            constants::tracking_runtime::GEOFENCE_TRANSITION_EXIT
        }
        (Some(TrackingGeofenceInsideState::Inside), TrackingGeofenceInsideState::Inside) => {
            constants::tracking_runtime::GEOFENCE_TRANSITION_DWELL
        }
        (Some(TrackingGeofenceInsideState::Outside), TrackingGeofenceInsideState::Outside) => {
            constants::tracking_runtime::GEOFENCE_TRANSITION_UNCHANGED
        }
        (None, TrackingGeofenceInsideState::Inside) => {
            constants::tracking_runtime::GEOFENCE_TRANSITION_ENTER
        }
        (None, TrackingGeofenceInsideState::Outside) => {
            constants::tracking_runtime::GEOFENCE_TRANSITION_UNCHANGED
        }
    }
}
