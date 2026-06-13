use ocentra_eventing::EventingError;
use ocentra_evidence::PrivatePayloadState;
use ocentra_parent_agent_protocol::{
    constants, TrackingAiAnalysisRequestedEvent, TrackingConfidenceBasis,
    TrackingNearbyPlaceClassifiedEvent, TrackingPlaceCategory,
};

pub fn classify_tracking_nearby_place(
    event: &TrackingAiAnalysisRequestedEvent,
) -> Result<TrackingNearbyPlaceClassifiedEvent, EventingError> {
    if event.allowed_analysis_purpose.as_str()
        != constants::tracking_runtime::ALLOWED_AI_PURPOSE_NEARBY_PLACE_CLASSIFICATION
    {
        return Err(EventingError::InvalidValue {
            field: "tracking.allowed_analysis_purpose",
            value: event.allowed_analysis_purpose.as_str().to_string(),
        });
    }

    if event.uncertainty_code.as_str()
        != constants::tracking_runtime::UNCERTAINTY_CODE_NEARBY_PLACE_CLASSIFICATION_REQUIRED
    {
        return Err(EventingError::InvalidValue {
            field: "tracking.uncertainty_code",
            value: event.uncertainty_code.as_str().to_string(),
        });
    }

    if event.private_payload_state != PrivatePayloadState::Excluded {
        return Err(EventingError::InvalidValue {
            field: "tracking.private_payload_state",
            value: format!("{:?}", event.private_payload_state),
        });
    }

    if event.evidence_refs.is_empty() {
        return Err(EventingError::InvalidValue {
            field: "tracking.evidence_refs",
            value: String::from("empty"),
        });
    }

    Ok(TrackingNearbyPlaceClassifiedEvent {
        child_device_id: event.child_device_id.clone(),
        child_profile_id: event.child_profile_id.clone(),
        source_ai_request_id: event.ai_request_id.clone(),
        evidence_refs: event.evidence_refs.clone(),
        place_category: TrackingPlaceCategory::parse(
            constants::tracking_runtime::PLACE_CATEGORY_HOSPITAL,
        )
        .expect(constants::tracking_runtime::PLACE_CATEGORY_HOSPITAL),
        confidence_basis: TrackingConfidenceBasis::parse(
            constants::tracking_runtime::CONFIDENCE_BASIS_AI_BOUNDARY_CONTRACT,
        )
        .expect(constants::tracking_runtime::CONFIDENCE_BASIS_AI_BOUNDARY_CONTRACT),
        parent_action_requirement: event.parent_action_requirement.clone(),
    })
}
