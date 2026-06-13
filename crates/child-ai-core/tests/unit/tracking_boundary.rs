use ocentra_evidence::PrivatePayloadState;
use ocentra_parent_agent_protocol::{
    constants, TrackingAiAnalysisRequestedEvent, TrackingAiPurpose, TrackingAiRequestId,
    TrackingChildDeviceId, TrackingChildProfileId, TrackingEvidenceRef,
    TrackingParentActionRequirement, TrackingUncertaintyCode,
};

fn tracking_ai_request_fixture() -> TrackingAiAnalysisRequestedEvent {
    TrackingAiAnalysisRequestedEvent {
        child_device_id: TrackingChildDeviceId::parse(
            constants::tracking_runtime::DEFAULT_CHILD_DEVICE_ID,
        )
        .expect(constants::tracking_runtime::DEFAULT_CHILD_DEVICE_ID),
        child_profile_id: TrackingChildProfileId::parse(
            constants::tracking_runtime::DEFAULT_CHILD_PROFILE_ID,
        )
        .expect(constants::tracking_runtime::DEFAULT_CHILD_PROFILE_ID),
        ai_request_id: TrackingAiRequestId::parse(
            constants::tracking_runtime::DEFAULT_AI_REQUEST_ID,
        )
        .expect(constants::tracking_runtime::DEFAULT_AI_REQUEST_ID),
        evidence_refs: vec![TrackingEvidenceRef::parse(
            constants::tracking_runtime::DEFAULT_EVIDENCE_REF,
        )
        .expect(constants::tracking_runtime::DEFAULT_EVIDENCE_REF)],
        uncertainty_code: TrackingUncertaintyCode::parse(
            constants::tracking_runtime::UNCERTAINTY_CODE_NEARBY_PLACE_CLASSIFICATION_REQUIRED,
        )
        .expect(
            constants::tracking_runtime::UNCERTAINTY_CODE_NEARBY_PLACE_CLASSIFICATION_REQUIRED,
        ),
        allowed_analysis_purpose: TrackingAiPurpose::parse(
            constants::tracking_runtime::ALLOWED_AI_PURPOSE_NEARBY_PLACE_CLASSIFICATION,
        )
        .expect(constants::tracking_runtime::ALLOWED_AI_PURPOSE_NEARBY_PLACE_CLASSIFICATION),
        parent_action_requirement: TrackingParentActionRequirement::Required,
        private_payload_state: PrivatePayloadState::Redacted,
    }
}

#[test]
fn tracking_ai_classification_preserves_request_evidence_refs() {
    let request = tracking_ai_request_fixture();

    let result = ocentra_child_ai_core::classify_tracking_nearby_place(&request);

    assert_eq!(result.child_device_id, request.child_device_id);
    assert_eq!(result.child_profile_id, request.child_profile_id);
    assert_eq!(result.source_ai_request_id, request.ai_request_id);
    assert_eq!(result.evidence_refs, request.evidence_refs);
    assert_eq!(
        result.parent_action_requirement,
        request.parent_action_requirement
    );
    assert_eq!(
        result.confidence_basis,
        constants::tracking_runtime::CONFIDENCE_BASIS_AI_BOUNDARY_CONTRACT
    );
}
