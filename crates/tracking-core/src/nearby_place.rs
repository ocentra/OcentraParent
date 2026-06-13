use ocentra_parent_agent_protocol::{
    constants, TrackingEvidenceRecordedEvent, TrackingEvidenceRef, TrackingNearbyPlaceAmbiguityState,
    TrackingNearbyPlaceProviderState, TrackingNearbyPlaceRequestId,
};
use ocentra_policy_control_core::AiResultAuthorityState;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TrackingNearbyPlaceProviderAvailabilityState {
    Available,
    Unavailable,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackingNearbyPlaceProviderDecision {
    pub request_state: TrackingNearbyPlaceProviderState,
    pub request_id: TrackingNearbyPlaceRequestId,
    pub evidence_refs: Vec<TrackingEvidenceRef>,
    pub ambiguity_state: TrackingNearbyPlaceAmbiguityState,
    pub ai_result_authority_state: AiResultAuthorityState,
}

pub fn request_nearby_place_provider_analysis(
    event: &TrackingEvidenceRecordedEvent,
    provider_availability_state: TrackingNearbyPlaceProviderAvailabilityState,
    candidate_count: u16,
) -> TrackingNearbyPlaceProviderDecision {
    if provider_availability_state == TrackingNearbyPlaceProviderAvailabilityState::Unavailable {
        return TrackingNearbyPlaceProviderDecision {
            request_state: nearby_provider_state(
                constants::tracking_runtime::NEARBY_PLACE_PROVIDER_UNAVAILABLE,
            ),
            request_id: nearby_request_id(),
            evidence_refs: vec![event.evidence_ref.clone()],
            ambiguity_state: nearby_ambiguity_state(
                constants::tracking_runtime::NEARBY_PLACE_AMBIGUITY_HIGH,
            ),
            ai_result_authority_state: AiResultAuthorityState::EvidenceOnly,
        };
    }

    TrackingNearbyPlaceProviderDecision {
        request_state: nearby_provider_state(
            constants::tracking_runtime::NEARBY_PLACE_PROVIDER_REQUESTED,
        ),
        request_id: nearby_request_id(),
        evidence_refs: vec![event.evidence_ref.clone()],
        ambiguity_state: ambiguity_state_for_candidate_count(candidate_count),
        ai_result_authority_state: AiResultAuthorityState::EvidenceOnly,
    }
}

fn ambiguity_state_for_candidate_count(candidate_count: u16) -> TrackingNearbyPlaceAmbiguityState {
    if candidate_count == 1 {
        nearby_ambiguity_state(constants::tracking_runtime::NEARBY_PLACE_AMBIGUITY_LOW)
    } else {
        nearby_ambiguity_state(constants::tracking_runtime::NEARBY_PLACE_AMBIGUITY_HIGH)
    }
}

fn nearby_request_id() -> TrackingNearbyPlaceRequestId {
    TrackingNearbyPlaceRequestId::parse(constants::tracking_runtime::DEFAULT_NEARBY_PLACE_REQUEST_ID)
        .expect(constants::tracking_runtime::DEFAULT_NEARBY_PLACE_REQUEST_ID)
}

fn nearby_provider_state(value: &'static str) -> TrackingNearbyPlaceProviderState {
    TrackingNearbyPlaceProviderState::parse(value)
        .expect(constants::tracking_runtime::NEARBY_PLACE_PROVIDER_REQUESTED)
}

fn nearby_ambiguity_state(value: &'static str) -> TrackingNearbyPlaceAmbiguityState {
    TrackingNearbyPlaceAmbiguityState::parse(value)
        .expect(constants::tracking_runtime::NEARBY_PLACE_AMBIGUITY_HIGH)
}
