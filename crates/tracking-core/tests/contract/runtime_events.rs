use ocentra_eventing::DomainEvent;
use ocentra_parent_agent_protocol::{
    constants, TrackingChildDeviceId, TrackingChildProfileId, TrackingParentActionRequirement,
    TrackingEvidenceRef, TrackingPolicyRuleRef, TrackingPolicySeverity,
    TrackingPolicyViolationDetectedEvent, TrackingPolicyViolationId, TrackingRuntimeMode,
};
use ocentra_tracking_core::TrackingGeofenceInsideState;

#[test]
fn tracking_observe_only_evidence_carries_no_parent_action_authority() {
    let mut observed = ocentra_tracking_core::default_location_observed_event();
    observed.config.tracking_mode = TrackingRuntimeMode::ObserveOnly;
    let evidence = ocentra_tracking_core::record_tracking_evidence_from_location(&observed);

    assert_eq!(
        evidence.parent_action_requirement,
        TrackingParentActionRequirement::NotRequired
    );
}

#[test]
fn tracking_geofence_transition_event_uses_protocol_contract() {
    let event = ocentra_tracking_core::detect_geofence_transition(
        &ocentra_tracking_core::default_location_observed_event(),
        ocentra_tracking_core::TrackingGeofenceEvaluation {
            previous_inside_state: Some(TrackingGeofenceInsideState::Outside),
            current_inside_state: TrackingGeofenceInsideState::Inside,
        },
    );

    let contract = event
        .contract()
        .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);

    assert_eq!(
        contract.event_type.as_str(),
        constants::tracking_runtime::TRACKING_GEOFENCE_TRANSITION_DETECTED_EVENT_TYPE
    );
}

#[test]
fn tracking_parent_acknowledgement_event_uses_protocol_contract() {
    let violation = TrackingPolicyViolationDetectedEvent {
        child_device_id: TrackingChildDeviceId::parse(constants::tracking_runtime::DEFAULT_CHILD_DEVICE_ID)
            .expect(constants::tracking_runtime::DEFAULT_CHILD_DEVICE_ID),
        child_profile_id: TrackingChildProfileId::parse(constants::tracking_runtime::DEFAULT_CHILD_PROFILE_ID)
            .expect(constants::tracking_runtime::DEFAULT_CHILD_PROFILE_ID),
        violation_id: TrackingPolicyViolationId::parse(
            constants::tracking_runtime::DEFAULT_POLICY_VIOLATION_ID,
        )
        .expect(constants::tracking_runtime::DEFAULT_POLICY_VIOLATION_ID),
        policy_rule_ref: TrackingPolicyRuleRef::parse(
            constants::tracking_runtime::POLICY_RULE_EXPECTED_PLACE,
        )
        .expect(constants::tracking_runtime::POLICY_RULE_EXPECTED_PLACE),
        severity: TrackingPolicySeverity::parse(constants::tracking_runtime::POLICY_SEVERITY_REVIEW)
            .expect(constants::tracking_runtime::POLICY_SEVERITY_REVIEW),
        evidence_refs: vec![
            TrackingEvidenceRef::parse(constants::tracking_runtime::DEFAULT_EVIDENCE_REF)
                .expect(constants::tracking_runtime::DEFAULT_EVIDENCE_REF),
        ],
    };
    let acknowledgement = ocentra_tracking_core::record_parent_acknowledgement(&violation);

    let contract = acknowledgement
        .contract()
        .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);

    assert_eq!(
        contract.event_type.as_str(),
        constants::tracking_runtime::TRACKING_PARENT_ACKNOWLEDGEMENT_RECORDED_EVENT_TYPE
    );
}
