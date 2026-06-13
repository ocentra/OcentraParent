use ocentra_child_policy_core::TrackingPolicyViolationState;
use ocentra_parent_agent_protocol::{
    constants, TrackingAiRequestId, TrackingChildDeviceId, TrackingChildProfileId,
    TrackingConfidenceBasis, TrackingEvidenceRef, TrackingNearbyPlaceClassifiedEvent,
    TrackingParentActionRequirement, TrackingPlaceCategory,
};

fn tracking_nearby_place_fixture(
    parent_action_requirement: TrackingParentActionRequirement,
) -> TrackingNearbyPlaceClassifiedEvent {
    TrackingNearbyPlaceClassifiedEvent {
        child_device_id: TrackingChildDeviceId::parse(
            constants::tracking_runtime::DEFAULT_CHILD_DEVICE_ID,
        )
        .expect(constants::tracking_runtime::DEFAULT_CHILD_DEVICE_ID),
        child_profile_id: TrackingChildProfileId::parse(
            constants::tracking_runtime::DEFAULT_CHILD_PROFILE_ID,
        )
        .expect(constants::tracking_runtime::DEFAULT_CHILD_PROFILE_ID),
        source_ai_request_id: TrackingAiRequestId::parse(
            constants::tracking_runtime::DEFAULT_AI_REQUEST_ID,
        )
        .expect(constants::tracking_runtime::DEFAULT_AI_REQUEST_ID),
        evidence_refs: vec![TrackingEvidenceRef::parse(
            constants::tracking_runtime::DEFAULT_EVIDENCE_REF,
        )
        .expect(constants::tracking_runtime::DEFAULT_EVIDENCE_REF)],
        place_category: TrackingPlaceCategory::parse(
            constants::tracking_runtime::PLACE_CATEGORY_HOSPITAL,
        )
        .expect(constants::tracking_runtime::PLACE_CATEGORY_HOSPITAL),
        confidence_basis: TrackingConfidenceBasis::parse(
            constants::tracking_runtime::CONFIDENCE_BASIS_AI_BOUNDARY_CONTRACT,
        )
        .expect(constants::tracking_runtime::CONFIDENCE_BASIS_AI_BOUNDARY_CONTRACT),
        parent_action_requirement,
    }
}

#[test]
fn tracking_policy_emits_review_violation_for_hospital_nearby_place() {
    let classified = tracking_nearby_place_fixture(TrackingParentActionRequirement::Required);

    let decision = ocentra_child_policy_core::evaluate_tracking_nearby_place_policy(&classified);
    let violation = decision
        .policy_violation_detected
        .expect("hospital tracking policy violation is expected");

    assert_eq!(
        decision.violation_state,
        TrackingPolicyViolationState::Detected
    );
    assert_eq!(
        violation.policy_rule_ref,
        constants::tracking_runtime::POLICY_RULE_EXPECTED_PLACE
    );
    assert_eq!(
        violation.severity,
        constants::tracking_runtime::POLICY_SEVERITY_REVIEW
    );
    assert_eq!(violation.evidence_refs, classified.evidence_refs);
}

#[test]
fn tracking_policy_does_not_emit_violation_for_observe_only_mode() {
    let classified = tracking_nearby_place_fixture(TrackingParentActionRequirement::NotRequired);

    let decision = ocentra_child_policy_core::evaluate_tracking_nearby_place_policy(&classified);

    assert_eq!(
        decision.violation_state,
        TrackingPolicyViolationState::NotDetected
    );
    assert!(decision.policy_violation_detected.is_none());
}
