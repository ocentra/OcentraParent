use ocentra_parent_agent_protocol::constants;
use ocentra_tracking_core::location_validation::TrackingLocationValidationResultState;

#[test]
fn location_validation_rejects_malformed_latitude_without_recording_evidence() {
    let mut observed = ocentra_tracking_core::runtime_flow::default_location_observed_event();
    observed.latitude_e7 = 900_000_001;

    let decision =
        ocentra_tracking_core::location_validation::validate_tracking_location_observation(
            &observed,
        );

    assert_eq!(
        decision.result_state,
        TrackingLocationValidationResultState::Rejected
    );
    assert_eq!(
        decision.validation_state,
        constants::tracking_runtime::LOCATION_VALIDATION_REJECTED_LATITUDE
    );
}

#[test]
fn location_validation_rejects_zero_accuracy_without_recording_evidence() {
    let mut observed = ocentra_tracking_core::runtime_flow::default_location_observed_event();
    observed.horizontal_accuracy_meters = 0;

    let decision =
        ocentra_tracking_core::location_validation::validate_tracking_location_observation(
            &observed,
        );

    assert_eq!(
        decision.result_state,
        TrackingLocationValidationResultState::Rejected
    );
    assert_eq!(
        decision.validation_state,
        constants::tracking_runtime::LOCATION_VALIDATION_REJECTED_ACCURACY
    );
}
