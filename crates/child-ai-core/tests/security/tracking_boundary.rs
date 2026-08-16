use ocentra_eventing::error::EventingError;
use ocentra_evidence::PrivatePayloadState;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::tracking::identifiers::{
    TrackingAiPurpose, TrackingAiRequestId, TrackingChildDeviceId, TrackingChildProfileId,
    TrackingEvidenceRef, TrackingReasonCode, TrackingTimestamp, TrackingUncertaintyCode,
};
use ocentra_parent_agent_protocol::tracking::runtime_event::{
    TrackingAiAnalysisRequestedEvent, TrackingParentActionRequirement,
};

fn tracking_ai_request_fixture(
) -> Result<TrackingAiAnalysisRequestedEvent, Box<dyn std::error::Error>> {
    Ok(TrackingAiAnalysisRequestedEvent {
        child_device_id: TrackingChildDeviceId::parse(
            constants::tracking_runtime::DEFAULT_CHILD_DEVICE_ID,
        )?,
        child_profile_id: TrackingChildProfileId::parse(
            constants::tracking_runtime::DEFAULT_CHILD_PROFILE_ID,
        )?,
        ai_request_id: TrackingAiRequestId::parse(
            constants::tracking_runtime::DEFAULT_AI_REQUEST_ID,
        )?,
        evidence_refs: vec![TrackingEvidenceRef::parse(
            constants::tracking_runtime::DEFAULT_EVIDENCE_REF,
        )?],
        source_observed_at: TrackingTimestamp::parse(
            constants::tracking_runtime::DEFAULT_OBSERVED_AT,
        )?,
        uncertainty_code: TrackingUncertaintyCode::parse(
            constants::tracking_runtime::UNCERTAINTY_CODE_NEARBY_PLACE_CLASSIFICATION_REQUIRED,
        )?,
        allowed_analysis_purpose: TrackingAiPurpose::parse(
            constants::tracking_runtime::ALLOWED_AI_PURPOSE_NEARBY_PLACE_CLASSIFICATION,
        )?,
        parent_action_requirement: TrackingParentActionRequirement::Required,
        private_payload_state: PrivatePayloadState::Excluded,
    })
}

#[test]
fn tracking_ai_classification_preserves_request_evidence_refs(
) -> Result<(), Box<dyn std::error::Error>> {
    let request = tracking_ai_request_fixture()?;

    let result =
        ocentra_child_ai_core::tracking_boundary::classify_tracking_nearby_place(&request)?;

    assert_eq!(result.child_device_id, request.child_device_id);
    assert_eq!(result.child_profile_id, request.child_profile_id);
    assert_eq!(result.source_ai_request_id, request.ai_request_id);
    assert_eq!(
        result.source_location_evidence_ref,
        request.evidence_refs[0]
    );
    assert_eq!(result.source_observed_at, request.source_observed_at);
    assert_eq!(result.evidence_refs, request.evidence_refs);
    assert_eq!(
        result.provider_kind,
        constants::tracking_runtime::NEARBY_PROVIDER_KIND_LOCAL_CACHE
    );
    assert_eq!(
        result.provider_ref.as_ref().map(|value| value.as_str()),
        Some(constants::tracking_runtime::DEFAULT_TRACKING_PROVIDER_REF)
    );
    assert_eq!(
        result.query_radius_meters,
        constants::tracking_runtime::DEFAULT_NEARBY_QUERY_RADIUS_METERS
    );
    assert_eq!(
        result.distance_meters,
        Some(constants::tracking_runtime::DEFAULT_NEARBY_DISTANCE_METERS)
    );
    assert_eq!(
        result.confidence,
        constants::tracking_runtime::DEFAULT_NEARBY_PLACE_CONFIDENCE
    );
    assert_eq!(
        result.ambiguity_state,
        constants::tracking_runtime::NEARBY_PLACE_AMBIGUITY_CLEAR
    );
    assert_eq!(
        result.reason_codes,
        vec![TrackingReasonCode::parse(
            constants::tracking_runtime::REASON_NEARBY_PLACE_SINGLE_CANDIDATE
        )?]
    );
    assert_eq!(
        result.parent_action_requirement,
        request.parent_action_requirement
    );
    assert_eq!(
        result.confidence_basis,
        constants::tracking_runtime::CONFIDENCE_BASIS_AI_BOUNDARY_CONTRACT
    );
    Ok(())
}

#[test]
fn tracking_ai_classification_rejects_unsupported_analysis_purpose(
) -> Result<(), Box<dyn std::error::Error>> {
    let mut request = tracking_ai_request_fixture()?;
    request.allowed_analysis_purpose = TrackingAiPurpose::parse("unsupported-purpose")?;

    let error =
        match ocentra_child_ai_core::tracking_boundary::classify_tracking_nearby_place(&request) {
            Err(error) => error,
            Ok(_) => return Err(std::io::Error::other("must reject").into()),
        };

    assert_eq!(
        error,
        EventingError::InvalidValue {
            field: "tracking.allowed_analysis_purpose",
            value: String::from("unsupported-purpose"),
        }
    );
    Ok(())
}

#[test]
fn tracking_ai_classification_rejects_unexpected_uncertainty_code(
) -> Result<(), Box<dyn std::error::Error>> {
    let mut request = tracking_ai_request_fixture()?;
    request.uncertainty_code = TrackingUncertaintyCode::parse("unexpected-uncertainty")?;

    let error =
        match ocentra_child_ai_core::tracking_boundary::classify_tracking_nearby_place(&request) {
            Err(error) => error,
            Ok(_) => return Err(std::io::Error::other("must reject").into()),
        };

    assert_eq!(
        error,
        EventingError::InvalidValue {
            field: "tracking.uncertainty_code",
            value: String::from("unexpected-uncertainty"),
        }
    );
    Ok(())
}

#[test]
fn tracking_ai_classification_rejects_private_payload_inclusion(
) -> Result<(), Box<dyn std::error::Error>> {
    let mut request = tracking_ai_request_fixture()?;
    request.private_payload_state = PrivatePayloadState::Included;

    let error =
        match ocentra_child_ai_core::tracking_boundary::classify_tracking_nearby_place(&request) {
            Err(error) => error,
            Ok(_) => return Err(std::io::Error::other("must reject").into()),
        };

    assert_eq!(
        error,
        EventingError::InvalidValue {
            field: "tracking.private_payload_state",
            value: String::from("Included"),
        }
    );
    Ok(())
}

#[test]
fn tracking_ai_classification_rejects_missing_evidence_refs(
) -> Result<(), Box<dyn std::error::Error>> {
    let mut request = tracking_ai_request_fixture()?;
    request.evidence_refs.clear();

    let error =
        match ocentra_child_ai_core::tracking_boundary::classify_tracking_nearby_place(&request) {
            Err(error) => error,
            Ok(_) => return Err(std::io::Error::other("must reject").into()),
        };

    assert_eq!(
        error,
        EventingError::InvalidValue {
            field: "tracking.evidence_refs",
            value: String::from("empty"),
        }
    );
    Ok(())
}
