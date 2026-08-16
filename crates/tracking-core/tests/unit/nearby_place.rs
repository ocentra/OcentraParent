use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::tracking::identifiers::{
    TrackingNearbyPlaceAmbiguityState, TrackingNearbyPlaceProviderKind, TrackingReasonCode,
};
use ocentra_policy_control_core::policy_authority::AiResultAuthorityState;
use ocentra_tracking_core::nearby_place::TrackingNearbyPlaceProviderAvailabilityState;

#[test]
fn nearby_place_provider_request_never_drives_policy_directly() {
    let observed = ocentra_tracking_core::runtime_flow::default_location_observed_event();
    let evidence =
        ocentra_tracking_core::runtime_flow::record_tracking_evidence_from_location(&observed);

    let decision = ocentra_tracking_core::nearby_place::request_nearby_place_provider_analysis(
        &evidence,
        TrackingNearbyPlaceProviderAvailabilityState::Available,
        2,
    );

    assert_eq!(
        decision.provider_kind,
        TrackingNearbyPlaceProviderKind::parse(
            constants::tracking_runtime::NEARBY_PROVIDER_KIND_LOCAL_CACHE,
        )
        .expect_value(constants::tracking_runtime::NEARBY_PROVIDER_KIND_LOCAL_CACHE)
    );
    assert_eq!(
        decision.ambiguity_state,
        TrackingNearbyPlaceAmbiguityState::parse(
            constants::tracking_runtime::NEARBY_PLACE_AMBIGUITY_MULTIPLE_CANDIDATES,
        )
        .expect_value(constants::tracking_runtime::NEARBY_PLACE_AMBIGUITY_MULTIPLE_CANDIDATES)
    );
    assert_eq!(
        decision.provider_ref.as_ref().map(|value| value.as_str()),
        Some(constants::tracking_runtime::DEFAULT_TRACKING_PROVIDER_REF)
    );
    assert_eq!(
        decision.query_radius_meters,
        constants::tracking_runtime::DEFAULT_NEARBY_QUERY_RADIUS_METERS
    );
    assert_eq!(decision.distance_meters, None);
    assert_eq!(
        decision.reason_codes,
        vec![TrackingReasonCode::parse(
            constants::tracking_runtime::REASON_NEARBY_PLACE_AMBIGUITY_PRESERVED
        )
        .expect_value(constants::tracking_runtime::REASON_NEARBY_PLACE_AMBIGUITY_PRESERVED)]
    );
    assert_eq!(
        decision.ai_result_authority_state,
        AiResultAuthorityState::EvidenceOnly
    );
}
#[test]
fn nearby_place_provider_unavailable_degrades_without_policy_authority() {
    let observed = ocentra_tracking_core::runtime_flow::default_location_observed_event();
    let evidence =
        ocentra_tracking_core::runtime_flow::record_tracking_evidence_from_location(&observed);

    let decision = ocentra_tracking_core::nearby_place::request_nearby_place_provider_analysis(
        &evidence,
        TrackingNearbyPlaceProviderAvailabilityState::Unavailable,
        0,
    );

    assert_eq!(
        decision.provider_kind,
        constants::tracking_runtime::NEARBY_PROVIDER_KIND_UNAVAILABLE
    );
    assert!(decision.provider_ref.is_none());
    assert_eq!(
        decision.ambiguity_state,
        constants::tracking_runtime::NEARBY_PLACE_AMBIGUITY_PROVIDER_UNAVAILABLE
    );
    assert_eq!(
        decision.reason_codes,
        vec![TrackingReasonCode::parse(
            constants::tracking_runtime::REASON_NEARBY_PLACE_PROVIDER_UNAVAILABLE
        )
        .expect_value(constants::tracking_runtime::REASON_NEARBY_PLACE_PROVIDER_UNAVAILABLE)]
    );
    assert_eq!(
        decision.ai_result_authority_state,
        AiResultAuthorityState::EvidenceOnly
    );
}
#[test]
fn nearby_place_classification_helper_reuses_canonical_provider_decision_shape() {
    let observed = ocentra_tracking_core::runtime_flow::default_location_observed_event();
    let report = ocentra_tracking_core::runtime_flow::observe_tracking_location(observed);
    let request = report
        .ai_analysis_requested
        .expect_value(constants::tracking_runtime::ERROR_TRACKING_RUNTIME_FLOW_RECORDED);

    let classified =
        ocentra_tracking_core::nearby_place::classify_tracking_nearby_place_request(&request);

    assert_eq!(classified.source_ai_request_id, request.ai_request_id);
    assert_eq!(
        classified.source_location_evidence_ref,
        request.evidence_refs[0]
    );
    assert_eq!(classified.source_observed_at, request.source_observed_at);
    assert_eq!(
        classified.provider_kind,
        constants::tracking_runtime::NEARBY_PROVIDER_KIND_LOCAL_CACHE
    );
    assert_eq!(
        classified.provider_ref.as_ref().map(|value| value.as_str()),
        Some(constants::tracking_runtime::DEFAULT_TRACKING_PROVIDER_REF)
    );
    assert_eq!(
        classified.ambiguity_state,
        constants::tracking_runtime::NEARBY_PLACE_AMBIGUITY_CLEAR
    );
    assert_eq!(
        classified.reason_codes,
        vec![TrackingReasonCode::parse(
            constants::tracking_runtime::REASON_NEARBY_PLACE_SINGLE_CANDIDATE
        )
        .expect_value(constants::tracking_runtime::REASON_NEARBY_PLACE_SINGLE_CANDIDATE)]
    );
}
