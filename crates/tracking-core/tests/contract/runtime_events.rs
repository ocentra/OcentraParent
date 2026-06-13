use ocentra_eventing::DomainEvent;
use ocentra_parent_agent_protocol::{
    constants, TrackingCapabilityStatus, TrackingChildDeviceId, TrackingChildProfileId,
    TrackingEvidenceRef, TrackingParentActionRequirement, TrackingPolicyRuleRef,
    TrackingPolicySeverity, TrackingPolicyViolationDetectedEvent, TrackingPolicyViolationId,
    TrackingRuntimeMode, TrackingTimestamp,
};
use ocentra_tracking_core::TrackingGeofenceInsideState;

#[test]
fn tracking_observe_only_evidence_carries_no_parent_action_authority() {
    let mut observed = ocentra_tracking_core::default_location_observed_event();
    observed.config.tracking_mode = TrackingRuntimeMode::ObserveOnly;
    let evidence = ocentra_tracking_core::record_tracking_evidence_from_location(&observed);
    let serialized = serde_json::to_value(&evidence).expect("tracking evidence serializes");

    assert_eq!(
        evidence.parent_action_requirement,
        TrackingParentActionRequirement::NotRequired
    );
    assert_eq!(evidence.expected_place_ref, observed.expected_place_ref);
    assert_eq!(serialized["expectedPlaceRef"], observed.expected_place_ref.as_str());
}

#[test]
fn tracking_geofence_transition_event_uses_protocol_contract() {
    let event = ocentra_tracking_core::detect_geofence_transition(
        &ocentra_tracking_core::default_location_observed_event(),
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

    let contract = event
        .contract()
        .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);

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
fn tracking_parent_acknowledgement_event_uses_protocol_contract() {
    let violation = TrackingPolicyViolationDetectedEvent {
        child_device_id: TrackingChildDeviceId::parse(
            constants::tracking_runtime::DEFAULT_CHILD_DEVICE_ID,
        )
        .expect(constants::tracking_runtime::DEFAULT_CHILD_DEVICE_ID),
        child_profile_id: TrackingChildProfileId::parse(
            constants::tracking_runtime::DEFAULT_CHILD_PROFILE_ID,
        )
        .expect(constants::tracking_runtime::DEFAULT_CHILD_PROFILE_ID),
        violation_id: TrackingPolicyViolationId::parse(
            constants::tracking_runtime::DEFAULT_POLICY_VIOLATION_ID,
        )
        .expect(constants::tracking_runtime::DEFAULT_POLICY_VIOLATION_ID),
        policy_rule_ref: TrackingPolicyRuleRef::parse(
            constants::tracking_runtime::POLICY_RULE_EXPECTED_PLACE,
        )
        .expect(constants::tracking_runtime::POLICY_RULE_EXPECTED_PLACE),
        severity: TrackingPolicySeverity::parse(
            constants::tracking_runtime::POLICY_SEVERITY_REVIEW,
        )
        .expect(constants::tracking_runtime::POLICY_SEVERITY_REVIEW),
        detected_at: TrackingTimestamp::parse(constants::tracking_runtime::DEFAULT_OBSERVED_AT)
            .expect(constants::tracking_runtime::DEFAULT_OBSERVED_AT),
        evidence_refs: vec![TrackingEvidenceRef::parse(
            constants::tracking_runtime::DEFAULT_EVIDENCE_REF,
        )
        .expect(constants::tracking_runtime::DEFAULT_EVIDENCE_REF)],
    };
    let acknowledgement = ocentra_tracking_core::record_parent_acknowledgement(&violation);

    let contract = acknowledgement
        .contract()
        .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);

    assert_eq!(
        contract.event_type.as_str(),
        constants::tracking_runtime::TRACKING_PARENT_ACKNOWLEDGEMENT_RECORDED_EVENT_TYPE
    );
    assert_eq!(acknowledgement.acknowledged_at, violation.detected_at);
}
