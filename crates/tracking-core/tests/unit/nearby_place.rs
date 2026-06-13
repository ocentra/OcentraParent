use ocentra_parent_agent_protocol::{
    constants, TrackingNearbyPlaceAmbiguityState, TrackingNearbyPlaceProviderState,
};
use ocentra_policy_control_core::AiResultAuthorityState;
use ocentra_tracking_core::TrackingNearbyPlaceProviderAvailabilityState;

#[test]
fn nearby_place_provider_request_never_drives_policy_directly() {
    let observed = ocentra_tracking_core::default_location_observed_event();
    let evidence = ocentra_tracking_core::record_tracking_evidence_from_location(&observed);

    let decision =
        ocentra_tracking_core::request_nearby_place_provider_analysis(
            &evidence,
            TrackingNearbyPlaceProviderAvailabilityState::Available,
            2,
        );

    assert_eq!(
        decision.request_state,
        TrackingNearbyPlaceProviderState::parse(
            constants::tracking_runtime::NEARBY_PLACE_PROVIDER_REQUESTED,
        )
        .expect(constants::tracking_runtime::NEARBY_PLACE_PROVIDER_REQUESTED)
    );
    assert_eq!(
        decision.ambiguity_state,
        TrackingNearbyPlaceAmbiguityState::parse(
            constants::tracking_runtime::NEARBY_PLACE_AMBIGUITY_HIGH,
        )
        .expect(constants::tracking_runtime::NEARBY_PLACE_AMBIGUITY_HIGH)
    );
    assert_eq!(
        decision.ai_result_authority_state,
        AiResultAuthorityState::EvidenceOnly
    );
}
