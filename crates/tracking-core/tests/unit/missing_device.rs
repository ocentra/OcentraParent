use ocentra_parent_agent_protocol::{constants, TrackingMissingDeviceState};
use ocentra_tracking_core::{
    TrackingChargingState, TrackingConnectivityState, TrackingLowPowerModeState,
    TrackingRadioState, TrackingRuntimeServiceState,
};

#[test]
fn missing_device_mode_exposes_last_known_only_without_live_claim() {
    let decision = ocentra_tracking_core::evaluate_missing_device_mode(
        ocentra_tracking_core::TrackingDeviceStatusInput {
            last_heartbeat_age_seconds: 1_200,
            last_location_sample_age_seconds: 45,
            last_parent_sync_age_seconds: 45,
            battery_percentage: Some(60),
            charging_state: TrackingChargingState::Discharging,
            low_power_mode_state: TrackingLowPowerModeState::Inactive,
            connectivity_state: TrackingConnectivityState::Online,
            radio_state: TrackingRadioState::Enabled,
            pending_upload_count: 0,
            service_state: TrackingRuntimeServiceState::Running,
        },
    );

    assert_eq!(
        decision.missing_device_state,
        TrackingMissingDeviceState::parse(
            constants::tracking_runtime::MISSING_DEVICE_STATE_LAST_KNOWN_ONLY
        )
        .expect(constants::tracking_runtime::MISSING_DEVICE_STATE_LAST_KNOWN_ONLY)
    );
    assert_eq!(
        decision.parent_visibility_state,
        ocentra_tracking_core::TrackingLastKnownVisibilityState::LastKnownOnly
    );
}
