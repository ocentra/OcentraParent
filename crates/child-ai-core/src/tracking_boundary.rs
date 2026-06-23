use ocentra_eventing::error::EventingError;
use ocentra_evidence::PrivatePayloadState;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::tracking::runtime_event::{
    TrackingAiAnalysisRequestedEvent, TrackingNearbyPlaceClassifiedEvent,
};

pub fn classify_tracking_nearby_place(
    event: &TrackingAiAnalysisRequestedEvent,
) -> Result<TrackingNearbyPlaceClassifiedEvent, EventingError> {
    validate_tracking_nearby_place_classification_request(event)?;

    Ok(ocentra_tracking_core::nearby_place::classify_tracking_nearby_place_request(event))
}

fn validate_tracking_nearby_place_classification_request(
    event: &TrackingAiAnalysisRequestedEvent,
) -> Result<(), EventingError> {
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

    Ok(())
}
