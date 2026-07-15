use ocentra_eventing::envelope::DomainEvent;
use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::tracking::identifiers::{
    tracking_evidence_ref_from_observation_id, TrackingCapabilityStatus, TrackingChildDeviceId,
    TrackingChildProfileId, TrackingEvidenceRef, TrackingObservationId, TrackingPolicyRuleRef,
    TrackingPolicySeverity, TrackingPolicyViolationId, TrackingTimestamp, TrackingTransitionKind,
};
use ocentra_parent_agent_protocol::tracking::runtime_event::{
    TrackingParentActionRequirement, TrackingPolicyViolationDetectedEvent, TrackingRuntimeMode,
};
use ocentra_tracking_core::geofence::TrackingGeofenceInsideState;
use ocentra_tracking_core::parent_acknowledgement::record_parent_acknowledgement;

#[test]
fn tracking_observe_only_evidence_carries_no_parent_action_authority() {
    let mut observed = ocentra_tracking_core::runtime_flow::default_location_observed_event();
    observed.config.tracking_mode = TrackingRuntimeMode::ObserveOnly;
    let evidence =
        ocentra_tracking_core::runtime_flow::record_tracking_evidence_from_location(&observed);
    let serialized = serde_json::to_value(&evidence).expect_value("tracking evidence serializes");

    assert_eq!(
        evidence.parent_action_requirement,
        TrackingParentActionRequirement::NotRequired
    );
    assert_eq!(evidence.expected_place_ref, observed.expected_place_ref);
    assert_eq!(
        serialized["expectedPlaceRef"],
        observed.expected_place_ref.as_str()
    );
}
#[test]
fn tracking_geofence_transition_event_uses_protocol_contract() {
    let event = ocentra_tracking_core::geofence::detect_geofence_transition(
        &ocentra_tracking_core::runtime_flow::default_location_observed_event(),
        ocentra_tracking_core::geofence::TrackingGeofenceEvaluation {
            previous_inside_state: Some(TrackingGeofenceInsideState::Outside),
            current_inside_state: TrackingGeofenceInsideState::Inside,
            capability_status: TrackingCapabilityStatus::parse(
                constants::tracking_runtime::CAPABILITY_STATUS_LIVE,
            )
            .expect_value(constants::tracking_runtime::CAPABILITY_STATUS_LIVE),
            distance_meters: Some(0),
            low_accuracy_near_boundary: false,
            grace_period_active: false,
        },
    );

    let contract = event
        .contract()
        .expect_value(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);

    assert_eq!(
        contract.event_type.as_str(),
        constants::tracking_runtime::TRACKING_GEOFENCE_TRANSITION_DETECTED_EVENT_TYPE
    );
    assert_eq!(
        event.capability_status,
        constants::tracking_runtime::CAPABILITY_STATUS_LIVE
    );
    assert_eq!(
        event.reason_codes[0],
        constants::tracking_runtime::REASON_INSIDE_GEOFENCE_WITH_ACCURACY
    );
}
#[test]
fn tracking_geofence_transition_event_serializes_rule_and_evidence_citations() {
    let mut observed = ocentra_tracking_core::runtime_flow::default_location_observed_event();
    observed.observation_id =
        TrackingObservationId::parse("tracking-observation-contract-geofence-citations")
            .expect_value("tracking contract geofence observation id parses");
    let evidence_ref = tracking_evidence_ref_from_observation_id(&observed.observation_id);

    let event = ocentra_tracking_core::geofence::detect_geofence_transition(
        &observed,
        ocentra_tracking_core::geofence::TrackingGeofenceEvaluation {
            previous_inside_state: Some(TrackingGeofenceInsideState::Inside),
            current_inside_state: TrackingGeofenceInsideState::Outside,
            capability_status: TrackingCapabilityStatus::parse(
                constants::tracking_runtime::CAPABILITY_STATUS_LIVE,
            )
            .expect_value(constants::tracking_runtime::CAPABILITY_STATUS_LIVE),
            distance_meters: Some(125),
            low_accuracy_near_boundary: false,
            grace_period_active: false,
        },
    );
    let serialized =
        serde_json::to_value(&event).expect_value("tracking geofence transition serializes");

    assert_eq!(
        serialized["geofenceRuleRef"],
        constants::tracking_runtime::DEFAULT_GEOFENCE_RULE_REF
    );
    assert_eq!(
        serialized["evidenceRefs"],
        serde_json::json!([evidence_ref.as_str()])
    );
}

#[test]
fn tracking_parent_acknowledgement_event_uses_protocol_contract() {
    let violation = TrackingPolicyViolationDetectedEvent {
        child_device_id: TrackingChildDeviceId::parse(
            constants::tracking_runtime::DEFAULT_CHILD_DEVICE_ID,
        )
        .expect_value(constants::tracking_runtime::DEFAULT_CHILD_DEVICE_ID),
        child_profile_id: TrackingChildProfileId::parse(
            constants::tracking_runtime::DEFAULT_CHILD_PROFILE_ID,
        )
        .expect_value(constants::tracking_runtime::DEFAULT_CHILD_PROFILE_ID),
        violation_id: TrackingPolicyViolationId::parse(
            constants::tracking_runtime::DEFAULT_POLICY_VIOLATION_ID,
        )
        .expect_value(constants::tracking_runtime::DEFAULT_POLICY_VIOLATION_ID),
        policy_rule_ref: TrackingPolicyRuleRef::parse(
            constants::tracking_runtime::POLICY_RULE_EXPECTED_PLACE,
        )
        .expect_value(constants::tracking_runtime::POLICY_RULE_EXPECTED_PLACE),
        severity: TrackingPolicySeverity::parse(
            constants::tracking_runtime::POLICY_SEVERITY_REVIEW,
        )
        .expect_value(constants::tracking_runtime::POLICY_SEVERITY_REVIEW),
        detected_at: TrackingTimestamp::parse(constants::tracking_runtime::DEFAULT_OBSERVED_AT)
            .expect_value(constants::tracking_runtime::DEFAULT_OBSERVED_AT),
        evidence_refs: vec![TrackingEvidenceRef::parse(
            constants::tracking_runtime::DEFAULT_EVIDENCE_REF,
        )
        .expect_value(constants::tracking_runtime::DEFAULT_EVIDENCE_REF)],
    };
    let acknowledgement = record_parent_acknowledgement(&violation);

    let contract = acknowledgement
        .contract()
        .expect_value(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);

    assert_eq!(
        contract.event_type.as_str(),
        constants::tracking_runtime::TRACKING_PARENT_ACKNOWLEDGEMENT_RECORDED_EVENT_TYPE
    );
    assert_eq!(acknowledgement.acknowledged_at, violation.detected_at);
}

#[test]
fn tracking_expected_place_event_carries_schedule_evidence_and_parent_action() {
    let observed = ocentra_tracking_core::runtime_flow::default_location_observed_event();
    let evidence =
        ocentra_tracking_core::runtime_flow::record_tracking_evidence_from_location(&observed);
    let expected_place = ocentra_tracking_core::expected_place::evaluate_expected_place_state(
        &evidence,
        ocentra_tracking_core::expected_place::TrackingExpectedPlaceEvaluation {
            transition_kind: TrackingTransitionKind::parse(
                constants::tracking_runtime::GEOFENCE_TRANSITION_MISSED_ARRIVAL,
            )
            .expect_value(constants::tracking_runtime::GEOFENCE_TRANSITION_MISSED_ARRIVAL),
            ..ocentra_tracking_core::expected_place::default_expected_place_evaluation()
        },
    );

    let contract = expected_place
        .contract()
        .expect_value(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);
    let serialized = serde_json::to_value(&expected_place)
        .expect_value("tracking expected-place event serializes");

    assert_eq!(
        contract.event_type.as_str(),
        constants::tracking_runtime::TRACKING_EXPECTED_PLACE_STATE_EVALUATED_EVENT_TYPE
    );
    assert_eq!(
        expected_place.expected_place_state,
        constants::tracking_runtime::EXPECTED_PLACE_STATE_LATE_ARRIVAL
    );
    assert_eq!(
        expected_place.parent_action_requirement,
        TrackingParentActionRequirement::Required
    );
    assert_eq!(
        serialized["expectedPlaceRef"],
        constants::tracking_runtime::DEFAULT_EXPECTED_PLACE_REF
    );
    assert_eq!(
        serialized["scheduleId"],
        constants::tracking_runtime::DEFAULT_EXPECTED_PLACE_SCHEDULE_ID
    );
    assert_eq!(
        serialized["distanceToleranceMeters"],
        constants::tracking_runtime::DEFAULT_EXPECTED_PLACE_DISTANCE_TOLERANCE_METERS
    );
    assert_eq!(
        serialized["lateGraceSeconds"],
        constants::tracking_runtime::DEFAULT_EXPECTED_PLACE_LATE_GRACE_SECONDS
    );
    assert_eq!(
        serialized["earlyExitGraceSeconds"],
        constants::tracking_runtime::DEFAULT_EXPECTED_PLACE_EARLY_EXIT_GRACE_SECONDS
    );
    assert!(serialized["exceptionState"].is_null());
    assert_eq!(
        serialized["evidenceRefs"],
        serde_json::json!([evidence.evidence_ref.as_str()])
    );
}
