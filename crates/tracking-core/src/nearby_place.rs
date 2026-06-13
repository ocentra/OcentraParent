use ocentra_parent_agent_protocol::{
    constants, tracking_nearby_place_request_id_from_evidence_ref, TrackingAiAnalysisRequestedEvent,
    TrackingConfidenceBasis, TrackingEvidenceRecordedEvent, TrackingEvidenceRef,
    TrackingNearbyPlaceAmbiguityState, TrackingNearbyPlaceClassifiedEvent,
    TrackingNearbyPlaceProviderKind, TrackingNearbyPlaceRequestId, TrackingPlaceCategory,
    TrackingProviderRef, TrackingReasonCode,
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
    provider_decision_from_evidence_ref(
        &event.evidence_ref,
        provider_availability_state,
        candidate_count,
    )
}

pub fn classify_tracking_nearby_place_request(
    event: &TrackingAiAnalysisRequestedEvent,
) -> TrackingNearbyPlaceClassifiedEvent {
    let source_location_evidence_ref = event.evidence_refs[0].clone();
    let provider_decision = provider_decision_from_evidence_ref(
        &source_location_evidence_ref,
        TrackingNearbyPlaceProviderAvailabilityState::Available,
        1,
    );

    TrackingNearbyPlaceClassifiedEvent {
        child_device_id: event.child_device_id.clone(),
        child_profile_id: event.child_profile_id.clone(),
        source_ai_request_id: event.ai_request_id.clone(),
        source_location_evidence_ref,
        evidence_refs: event.evidence_refs.clone(),
        provider_kind: provider_decision.provider_kind,
        provider_ref: provider_decision.provider_ref,
        query_radius_meters: provider_decision.query_radius_meters,
        distance_meters: provider_decision.distance_meters,
        place_category: tracking_place_category(constants::tracking_runtime::PLACE_CATEGORY_HOSPITAL),
        confidence: constants::tracking_runtime::DEFAULT_NEARBY_PLACE_CONFIDENCE,
        confidence_basis: tracking_confidence_basis(
            constants::tracking_runtime::CONFIDENCE_BASIS_AI_BOUNDARY_CONTRACT,
        ),
        ambiguity_state: provider_decision.ambiguity_state,
        reason_codes: provider_decision.reason_codes,
        parent_action_requirement: event.parent_action_requirement.clone(),
    }
}

fn provider_decision_from_evidence_ref(
    evidence_ref: &TrackingEvidenceRef,
    provider_availability_state: TrackingNearbyPlaceProviderAvailabilityState,
    candidate_count: u16,
) -> TrackingNearbyPlaceProviderDecision {
    if provider_availability_state == TrackingNearbyPlaceProviderAvailabilityState::Unavailable {
        return TrackingNearbyPlaceProviderDecision {
            provider_kind: nearby_provider_kind(
                constants::tracking_runtime::NEARBY_PROVIDER_KIND_UNAVAILABLE,
            ),
            provider_ref: None,
            request_id: nearby_request_id(evidence_ref),
            evidence_refs: vec![evidence_ref.clone()],
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
        request_id: nearby_request_id(evidence_ref),
        evidence_refs: vec![evidence_ref.clone()],
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

fn tracking_place_category(value: &'static str) -> TrackingPlaceCategory {
    TrackingPlaceCategory::parse(value).expect(constants::tracking_runtime::PLACE_CATEGORY_HOSPITAL)
}

fn tracking_confidence_basis(value: &'static str) -> TrackingConfidenceBasis {
    TrackingConfidenceBasis::parse(value)
        .expect(constants::tracking_runtime::CONFIDENCE_BASIS_AI_BOUNDARY_CONTRACT)
}

fn nearby_ambiguity_state(value: &'static str) -> TrackingNearbyPlaceAmbiguityState {
    TrackingNearbyPlaceAmbiguityState::parse(value).expect(
        constants::tracking_runtime::NEARBY_PLACE_AMBIGUITY_MULTIPLE_CANDIDATES,
    )
}

fn tracking_reason_code(value: &'static str) -> TrackingReasonCode {
    TrackingReasonCode::parse(value).expect(value)
}
