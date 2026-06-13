use ocentra_parent_agent_protocol::{
    constants, tracking_nearby_place_request_id_from_evidence_ref, TrackingEvidenceRecordedEvent,
    TrackingEvidenceRef, TrackingNearbyPlaceAmbiguityState, TrackingNearbyPlaceProviderKind,
    TrackingNearbyPlaceRequestId, TrackingProviderRef, TrackingReasonCode,
};
use ocentra_policy_control_core::AiResultAuthorityState;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TrackingNearbyPlaceProviderAvailabilityState {
    Available,
    Unavailable,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackingNearbyPlaceProviderDecision {
    pub provider_kind: TrackingNearbyPlaceProviderKind,
    pub provider_ref: Option<TrackingProviderRef>,
    pub request_id: TrackingNearbyPlaceRequestId,
    pub evidence_refs: Vec<TrackingEvidenceRef>,
    pub query_radius_meters: u32,
    pub distance_meters: Option<u32>,
    pub ambiguity_state: TrackingNearbyPlaceAmbiguityState,
    pub reason_codes: Vec<TrackingReasonCode>,
    pub ai_result_authority_state: AiResultAuthorityState,
}

pub fn request_nearby_place_provider_analysis(
    event: &TrackingEvidenceRecordedEvent,
    provider_availability_state: TrackingNearbyPlaceProviderAvailabilityState,
    candidate_count: u16,
) -> TrackingNearbyPlaceProviderDecision {
    if provider_availability_state == TrackingNearbyPlaceProviderAvailabilityState::Unavailable {
        return TrackingNearbyPlaceProviderDecision {
            provider_kind: nearby_provider_kind(
                constants::tracking_runtime::NEARBY_PROVIDER_KIND_UNAVAILABLE,
            ),
            provider_ref: None,
            request_id: nearby_request_id(&event.evidence_ref),
            evidence_refs: vec![event.evidence_ref.clone()],
            query_radius_meters: constants::tracking_runtime::DEFAULT_NEARBY_QUERY_RADIUS_METERS,
            distance_meters: None,
            ambiguity_state: nearby_ambiguity_state(
                constants::tracking_runtime::NEARBY_PLACE_AMBIGUITY_PROVIDER_UNAVAILABLE,
            ),
            reason_codes: vec![tracking_reason_code(
                constants::tracking_runtime::REASON_NEARBY_PLACE_PROVIDER_UNAVAILABLE,
            )],
            ai_result_authority_state: AiResultAuthorityState::EvidenceOnly,
        };
    }

    TrackingNearbyPlaceProviderDecision {
        provider_kind: nearby_provider_kind(
            constants::tracking_runtime::NEARBY_PROVIDER_KIND_LOCAL_CACHE,
        ),
        provider_ref: Some(tracking_provider_ref(
            constants::tracking_runtime::DEFAULT_TRACKING_PROVIDER_REF,
        )),
        request_id: nearby_request_id(&event.evidence_ref),
        evidence_refs: vec![event.evidence_ref.clone()],
        query_radius_meters: constants::tracking_runtime::DEFAULT_NEARBY_QUERY_RADIUS_METERS,
        distance_meters: nearby_distance_meters(candidate_count),
        ambiguity_state: ambiguity_state_for_candidate_count(candidate_count),
        reason_codes: nearby_reason_codes(candidate_count),
        ai_result_authority_state: AiResultAuthorityState::EvidenceOnly,
    }
}

fn ambiguity_state_for_candidate_count(candidate_count: u16) -> TrackingNearbyPlaceAmbiguityState {
    match candidate_count {
        1 => nearby_ambiguity_state(constants::tracking_runtime::NEARBY_PLACE_AMBIGUITY_CLEAR),
        0 => nearby_ambiguity_state(constants::tracking_runtime::NEARBY_PLACE_AMBIGUITY_UNKNOWN),
        _ => nearby_ambiguity_state(
            constants::tracking_runtime::NEARBY_PLACE_AMBIGUITY_MULTIPLE_CANDIDATES,
        ),
    }
}

fn nearby_distance_meters(candidate_count: u16) -> Option<u32> {
    (candidate_count == 1).then_some(constants::tracking_runtime::DEFAULT_NEARBY_DISTANCE_METERS)
}

fn nearby_reason_codes(candidate_count: u16) -> Vec<TrackingReasonCode> {
    let reason = match candidate_count {
        0 => constants::tracking_runtime::REASON_NEARBY_PLACE_NO_CANDIDATES,
        1 => constants::tracking_runtime::REASON_NEARBY_PLACE_SINGLE_CANDIDATE,
        _ => constants::tracking_runtime::REASON_NEARBY_PLACE_AMBIGUITY_PRESERVED,
    };
    vec![tracking_reason_code(reason)]
}

fn nearby_request_id(evidence_ref: &TrackingEvidenceRef) -> TrackingNearbyPlaceRequestId {
    tracking_nearby_place_request_id_from_evidence_ref(evidence_ref)
}

fn nearby_provider_kind(value: &'static str) -> TrackingNearbyPlaceProviderKind {
    TrackingNearbyPlaceProviderKind::parse(value)
        .expect(constants::tracking_runtime::NEARBY_PROVIDER_KIND_LOCAL_CACHE)
}

fn tracking_provider_ref(value: &'static str) -> TrackingProviderRef {
    TrackingProviderRef::parse(value).expect(constants::tracking_runtime::DEFAULT_TRACKING_PROVIDER_REF)
}

fn nearby_ambiguity_state(value: &'static str) -> TrackingNearbyPlaceAmbiguityState {
    TrackingNearbyPlaceAmbiguityState::parse(value).expect(
        constants::tracking_runtime::NEARBY_PLACE_AMBIGUITY_MULTIPLE_CANDIDATES,
    )
}

fn tracking_reason_code(value: &'static str) -> TrackingReasonCode {
    TrackingReasonCode::parse(value).expect(value)
}
