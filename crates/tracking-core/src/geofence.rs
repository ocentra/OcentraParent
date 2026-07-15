use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::tracking::identifiers::{
    tracking_evidence_ref_from_observation_id, tracking_transition_id_from_observation_id,
    TrackingCapabilityStatus, TrackingChildDeviceId, TrackingChildProfileId, TrackingEvidenceRef,
    TrackingGeofenceRuleRef, TrackingObservationId, TrackingReasonCode, TrackingTimestamp,
    TrackingTransitionKind,
};
use ocentra_parent_agent_protocol::tracking::runtime_event::{
    TrackingGeofenceTransitionDetectedEvent, TrackingLocationObservedEvent,
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
    pub capability_status: TrackingCapabilityStatus,
    pub distance_meters: Option<u32>,
    pub low_accuracy_near_boundary: bool,
    pub grace_period_active: bool,
}

pub fn detect_geofence_transition(
    event: &TrackingLocationObservedEvent,
    evaluation: TrackingGeofenceEvaluation,
) -> TrackingGeofenceTransitionDetectedEvent {
    detect_geofence_transition_with_refs(
        event,
        parse_contract_text(
            constants::tracking_runtime::DEFAULT_GEOFENCE_RULE_REF,
            TrackingGeofenceRuleRef::parse,
        ),
        evaluation,
        vec![tracking_evidence_ref_from_observation_id(
            &event.observation_id,
        )],
    )
}

fn detect_geofence_transition_with_refs(
    event: &TrackingLocationObservedEvent,
    geofence_rule_ref: TrackingGeofenceRuleRef,
    evaluation: TrackingGeofenceEvaluation,
    evidence_refs: Vec<TrackingEvidenceRef>,
) -> TrackingGeofenceTransitionDetectedEvent {
    geofence_transition_from_parts(
        event.child_device_id.clone(),
        event.child_profile_id.clone(),
        event.observation_id.clone(),
        event.observed_at.clone(),
        geofence_rule_ref,
        evaluation,
        evidence_refs,
    )
}

pub(crate) fn geofence_transition_from_parts(
    child_device_id: TrackingChildDeviceId,
    child_profile_id: TrackingChildProfileId,
    source_observation_id: TrackingObservationId,
    source_observed_at: TrackingTimestamp,
    geofence_rule_ref: TrackingGeofenceRuleRef,
    evaluation: TrackingGeofenceEvaluation,
    evidence_refs: Vec<TrackingEvidenceRef>,
) -> TrackingGeofenceTransitionDetectedEvent {
    let (transition_kind, reason_codes) = transition_outcome_for(&evaluation);

    TrackingGeofenceTransitionDetectedEvent {
        child_device_id,
        child_profile_id,
        transition_id: tracking_transition_id_from_observation_id(&source_observation_id),
        geofence_rule_ref,
        source_observation_id,
        source_observed_at,
        transition_kind: parse_contract_text(transition_kind, TrackingTransitionKind::parse),
        capability_status: evaluation.capability_status,
        distance_meters: evaluation.distance_meters,
        reason_codes,
        evidence_refs,
    }
}

fn transition_outcome_for(
    evaluation: &TrackingGeofenceEvaluation,
) -> (&'static str, Vec<TrackingReasonCode>) {
    if evaluation.grace_period_active {
        return (
            constants::tracking_runtime::GEOFENCE_TRANSITION_AMBIGUOUS,
            vec![reason_code(
                constants::tracking_runtime::REASON_GEOFENCE_GRACE_ACTIVE,
            )],
        );
    }

    if evaluation.low_accuracy_near_boundary {
        return (
            constants::tracking_runtime::GEOFENCE_TRANSITION_AMBIGUOUS,
            vec![reason_code(
                constants::tracking_runtime::REASON_LOCATION_ACCURACY_BELOW_RULE_THRESHOLD,
            )],
        );
    }

    if capability_requires_stale_rejection(&evaluation.capability_status) {
        return (
            constants::tracking_runtime::GEOFENCE_TRANSITION_STALE_AT_PLACE,
            vec![reason_code(
                constants::tracking_runtime::REASON_STALE_LOCATION_REJECTED,
            )],
        );
    }

    let transition_kind = match (
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
    };

    let reason = if evaluation.current_inside_state == TrackingGeofenceInsideState::Inside {
        constants::tracking_runtime::REASON_INSIDE_GEOFENCE_WITH_ACCURACY
    } else {
        constants::tracking_runtime::REASON_OUTSIDE_GEOFENCE_WITH_ACCURACY
    };

    (transition_kind, vec![reason_code(reason)])
}

fn capability_requires_stale_rejection(capability_status: &TrackingCapabilityStatus) -> bool {
    capability_status.as_str() == constants::tracking_runtime::CAPABILITY_STATUS_STALE
        || capability_status.as_str() == constants::tracking_runtime::CAPABILITY_STATUS_LAST_KNOWN
        || capability_status.as_str()
            == constants::tracking_runtime::CAPABILITY_STATUS_OFFLINE_LAST_KNOWN_ONLY
}

fn reason_code(value: &'static str) -> TrackingReasonCode {
    parse_contract_text(value, TrackingReasonCode::parse)
}

fn parse_contract_text<T, E>(
    value: &'static str,
    parse: impl FnOnce(&'static str) -> Result<T, E>,
) -> T
where
    E: core::fmt::Debug,
{
    parse(value).expect_value("tracking geofence contract drift")
}
