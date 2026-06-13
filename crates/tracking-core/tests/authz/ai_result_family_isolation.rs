use ocentra_parent_agent_protocol::{
    constants, TrackingChildDeviceId, TrackingConfidenceBasis, TrackingNearbyPlaceClassifiedEvent,
    TrackingParentActionRequirement, TrackingPlaceCategory,
};

#[test]
fn ai_result_from_wrong_child_or_device_is_rejected_before_policy() {
    let report = ocentra_tracking_core::observe_tracking_location(
        ocentra_tracking_core::default_location_observed_event(),
    );
    let request = report
        .ai_analysis_requested
        .expect(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);
    let result = TrackingNearbyPlaceClassifiedEvent {
        child_device_id: TrackingChildDeviceId::parse(
            constants::tracking_runtime::DEFAULT_GEOFENCE_RULE_REF,
        )
        .expect(constants::tracking_runtime::DEFAULT_GEOFENCE_RULE_REF),
        child_profile_id: request.child_profile_id.clone(),
        source_ai_request_id: request.ai_request_id.clone(),
        evidence_refs: request.evidence_refs.clone(),
        place_category: TrackingPlaceCategory::parse(constants::tracking_runtime::PLACE_CATEGORY_HOSPITAL)
            .expect(constants::tracking_runtime::PLACE_CATEGORY_HOSPITAL),
        confidence_basis: TrackingConfidenceBasis::parse(
            constants::tracking_runtime::CONFIDENCE_BASIS_AI_BOUNDARY_CONTRACT,
        )
        .expect(constants::tracking_runtime::CONFIDENCE_BASIS_AI_BOUNDARY_CONTRACT),
        parent_action_requirement: TrackingParentActionRequirement::Required,
    };

    let decision = ocentra_tracking_core::validate_tracking_ai_result_as_evidence(&request, &result);

    assert_eq!(
        decision.decision_state,
        constants::tracking_runtime::AI_RESULT_REJECTED_WRONG_CHILD_OR_DEVICE_REF
    );
}
