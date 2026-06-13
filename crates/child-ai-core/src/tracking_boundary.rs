use ocentra_eventing::EventingError;
use ocentra_evidence::PrivatePayloadState;
use ocentra_parent_agent_protocol::{
    constants, TrackingAiAnalysisRequestedEvent, TrackingConfidenceBasis,
    TrackingNearbyPlaceAmbiguityState, TrackingNearbyPlaceClassifiedEvent,
    TrackingNearbyPlaceProviderKind, TrackingPlaceCategory, TrackingProviderRef,
    TrackingReasonCode,
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

    let source_location_evidence_ref = event.evidence_refs[0].clone();

    Ok(TrackingNearbyPlaceClassifiedEvent {
        child_device_id: event.child_device_id.clone(),
        child_profile_id: event.child_profile_id.clone(),
        source_ai_request_id: event.ai_request_id.clone(),
        source_location_evidence_ref: source_location_evidence_ref.clone(),
        evidence_refs: event.evidence_refs.clone(),
        provider_kind: tracking_nearby_place_provider_kind(
            constants::tracking_runtime::NEARBY_PROVIDER_KIND_PARENT_DEFINED,
        ),
        provider_ref: Some(tracking_provider_ref(
            constants::tracking_runtime::DEFAULT_TRACKING_PROVIDER_REF,
        )),
        query_radius_meters: constants::tracking_runtime::DEFAULT_NEARBY_QUERY_RADIUS_METERS,
        distance_meters: Some(constants::tracking_runtime::DEFAULT_NEARBY_DISTANCE_METERS),
        place_category: TrackingPlaceCategory::parse(
            constants::tracking_runtime::PLACE_CATEGORY_HOSPITAL,
        )
        .expect(constants::tracking_runtime::PLACE_CATEGORY_HOSPITAL),
        confidence: constants::tracking_runtime::DEFAULT_NEARBY_PLACE_CONFIDENCE,
        confidence_basis: TrackingConfidenceBasis::parse(
            constants::tracking_runtime::CONFIDENCE_BASIS_AI_BOUNDARY_CONTRACT,
        )
        .expect(constants::tracking_runtime::CONFIDENCE_BASIS_AI_BOUNDARY_CONTRACT),
        ambiguity_state: tracking_nearby_place_ambiguity_state(
            constants::tracking_runtime::NEARBY_PLACE_AMBIGUITY_CLEAR,
        ),
        reason_codes: vec![tracking_reason_code(
            constants::tracking_runtime::REASON_PARENT_DEFINED_PLACE_MATCH,
        )],
        parent_action_requirement: event.parent_action_requirement.clone(),
    })
}

fn tracking_nearby_place_provider_kind(value: &'static str) -> TrackingNearbyPlaceProviderKind {
    TrackingNearbyPlaceProviderKind::parse(value)
        .expect(constants::tracking_runtime::NEARBY_PROVIDER_KIND_PARENT_DEFINED)
}

fn tracking_provider_ref(value: &'static str) -> TrackingProviderRef {
    TrackingProviderRef::parse(value).expect(constants::tracking_runtime::DEFAULT_TRACKING_PROVIDER_REF)
}

fn tracking_nearby_place_ambiguity_state(
    value: &'static str,
) -> TrackingNearbyPlaceAmbiguityState {
    TrackingNearbyPlaceAmbiguityState::parse(value)
        .expect(constants::tracking_runtime::NEARBY_PLACE_AMBIGUITY_CLEAR)
}

fn tracking_reason_code(value: &'static str) -> TrackingReasonCode {
    TrackingReasonCode::parse(value).expect(value)
}
