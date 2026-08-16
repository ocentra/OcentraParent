use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::tracking::identifiers::{
    TrackingAiRequestId, TrackingChildDeviceId, TrackingConfidenceBasis, TrackingEvidenceRef,
    TrackingNearbyPlaceAmbiguityState, TrackingNearbyPlaceProviderKind, TrackingPlaceCategory,
    TrackingProviderRef, TrackingReasonCode, TrackingTimestamp,
};
use ocentra_parent_agent_protocol::tracking::runtime_event::{
    TrackingAiAnalysisRequestedEvent, TrackingNearbyPlaceClassifiedEvent,
    TrackingParentActionRequirement,
};
use ocentra_tracking_core::ai_boundary::validate_tracking_ai_result_as_evidence;

#[test]
fn tracking_accepts_ai_result_only_as_evidence_when_refs_match_request() {
    let observed = ocentra_tracking_core::runtime_flow::default_location_observed_event();
    let report = ocentra_tracking_core::runtime_flow::observe_tracking_location(observed);
    let request = report
        .ai_analysis_requested
        .expect_value(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);
    let result = ai_result_for_request(&request);

    let decision = validate_tracking_ai_result_as_evidence(&request, &result);

    assert_eq!(
        decision.decision_state,
        constants::tracking_runtime::AI_RESULT_ACCEPTED_AS_EVIDENCE
    );
    assert_eq!(decision.accepted_evidence_refs, request.evidence_refs);
}
#[test]
fn tracking_rejects_ai_result_with_hallucinated_evidence_ref() {
    let observed = ocentra_tracking_core::runtime_flow::default_location_observed_event();
    let report = ocentra_tracking_core::runtime_flow::observe_tracking_location(observed);
    let request = report
        .ai_analysis_requested
        .expect_value(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);
    let mut result = ai_result_for_request(&request);
    result.evidence_refs =
        vec![
            TrackingEvidenceRef::parse(constants::tracking_runtime::DEFAULT_GEOFENCE_RULE_REF)
                .expect_value(constants::tracking_runtime::DEFAULT_GEOFENCE_RULE_REF),
        ];

    let decision = validate_tracking_ai_result_as_evidence(&request, &result);

    assert_eq!(
        decision.decision_state,
        constants::tracking_runtime::AI_RESULT_REJECTED_HALLUCINATED_EVIDENCE_REF
    );
    assert!(decision.accepted_evidence_refs.is_empty());
}
#[test]
fn tracking_rejects_ai_result_without_evidence_refs() {
    let observed = ocentra_tracking_core::runtime_flow::default_location_observed_event();
    let report = ocentra_tracking_core::runtime_flow::observe_tracking_location(observed);
    let request = report
        .ai_analysis_requested
        .expect_value(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);
    let mut result = ai_result_for_request(&request);
    result.evidence_refs = Vec::new();

    let decision = validate_tracking_ai_result_as_evidence(&request, &result);

    assert_eq!(
        decision.decision_state,
        constants::tracking_runtime::AI_RESULT_REJECTED_MISSING_EVIDENCE_REF
    );
    assert!(decision.accepted_evidence_refs.is_empty());
}

#[test]
fn tracking_rejects_ai_result_with_stale_correlation() {
    let observed = ocentra_tracking_core::runtime_flow::default_location_observed_event();
    let report = ocentra_tracking_core::runtime_flow::observe_tracking_location(observed);
    let request = report
        .ai_analysis_requested
        .expect_value(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);
    let mut result = ai_result_for_request(&request);
    result.source_ai_request_id =
        TrackingAiRequestId::parse(constants::tracking_runtime::DEFAULT_NEARBY_PLACE_REQUEST_ID)
            .expect_value(constants::tracking_runtime::DEFAULT_NEARBY_PLACE_REQUEST_ID);

    let decision = validate_tracking_ai_result_as_evidence(&request, &result);

    assert_eq!(
        decision.decision_state,
        constants::tracking_runtime::AI_RESULT_REJECTED_STALE_CORRELATION
    );
    assert!(decision.accepted_evidence_refs.is_empty());
}

#[test]
fn tracking_rejects_ai_result_with_mismatched_source_observed_at() {
    let observed = ocentra_tracking_core::runtime_flow::default_location_observed_event();
    let report = ocentra_tracking_core::runtime_flow::observe_tracking_location(observed);
    let request = report
        .ai_analysis_requested
        .expect_value(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);
    let mut result = ai_result_for_request(&request);
    result.source_observed_at =
        TrackingTimestamp::parse("2026-06-12T12:05:00Z").expect_value("valid timestamp");

    let decision = validate_tracking_ai_result_as_evidence(&request, &result);

    assert_eq!(
        decision.decision_state,
        constants::tracking_runtime::AI_RESULT_REJECTED_STALE_CORRELATION
    );
    assert!(decision.accepted_evidence_refs.is_empty());
}

#[test]
fn tracking_rejects_ai_result_with_wrong_child_or_device_ref() {
    let observed = ocentra_tracking_core::runtime_flow::default_location_observed_event();
    let report = ocentra_tracking_core::runtime_flow::observe_tracking_location(observed);
    let request = report
        .ai_analysis_requested
        .expect_value(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);
    let mut result = ai_result_for_request(&request);
    result.child_device_id =
        TrackingChildDeviceId::parse(constants::tracking_runtime::DEFAULT_PARENT_DEFINED_PLACE_ID)
            .expect_value(constants::tracking_runtime::DEFAULT_PARENT_DEFINED_PLACE_ID);

    let decision = validate_tracking_ai_result_as_evidence(&request, &result);

    assert_eq!(
        decision.decision_state,
        constants::tracking_runtime::AI_RESULT_REJECTED_WRONG_CHILD_OR_DEVICE_REF
    );
    assert!(decision.accepted_evidence_refs.is_empty());
}

fn ai_result_for_request(
    request: &TrackingAiAnalysisRequestedEvent,
) -> TrackingNearbyPlaceClassifiedEvent {
    TrackingNearbyPlaceClassifiedEvent {
        child_device_id: request.child_device_id.clone(),
        child_profile_id: request.child_profile_id.clone(),
        source_ai_request_id: request.ai_request_id.clone(),
        source_location_evidence_ref: request.evidence_refs[0].clone(),
        source_observed_at: request.source_observed_at.clone(),
        evidence_refs: request.evidence_refs.clone(),
        provider_kind: TrackingNearbyPlaceProviderKind::parse(
            constants::tracking_runtime::NEARBY_PROVIDER_KIND_LOCAL_CACHE,
        )
        .expect_value(constants::tracking_runtime::NEARBY_PROVIDER_KIND_LOCAL_CACHE),
        provider_ref: Some(
            TrackingProviderRef::parse(constants::tracking_runtime::DEFAULT_TRACKING_PROVIDER_REF)
                .expect_value(constants::tracking_runtime::DEFAULT_TRACKING_PROVIDER_REF),
        ),
        query_radius_meters: constants::tracking_runtime::DEFAULT_NEARBY_QUERY_RADIUS_METERS,
        distance_meters: Some(constants::tracking_runtime::DEFAULT_NEARBY_DISTANCE_METERS),
        place_category: TrackingPlaceCategory::parse(
            constants::tracking_runtime::PLACE_CATEGORY_HOSPITAL,
        )
        .expect_value(constants::tracking_runtime::PLACE_CATEGORY_HOSPITAL),
        confidence: constants::tracking_runtime::DEFAULT_NEARBY_PLACE_CONFIDENCE,
        confidence_basis: TrackingConfidenceBasis::parse(
            constants::tracking_runtime::CONFIDENCE_BASIS_AI_BOUNDARY_CONTRACT,
        )
        .expect_value(constants::tracking_runtime::CONFIDENCE_BASIS_AI_BOUNDARY_CONTRACT),
        ambiguity_state: TrackingNearbyPlaceAmbiguityState::parse(
            constants::tracking_runtime::NEARBY_PLACE_AMBIGUITY_CLEAR,
        )
        .expect_value(constants::tracking_runtime::NEARBY_PLACE_AMBIGUITY_CLEAR),
        reason_codes: vec![TrackingReasonCode::parse(
            constants::tracking_runtime::REASON_NEARBY_PLACE_SINGLE_CANDIDATE,
        )
        .expect_value(constants::tracking_runtime::REASON_NEARBY_PLACE_SINGLE_CANDIDATE)],
        parent_action_requirement: TrackingParentActionRequirement::Required,
    }
}
