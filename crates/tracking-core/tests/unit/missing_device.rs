use ocentra_parent_agent_protocol::{constants, TrackingMissingDeviceState};
use ocentra_tracking_core::TrackingLowPowerModeState;

#[test]
fn missing_device_mode_exposes_last_known_only_without_live_claim() {
    let decision = ocentra_tracking_core::evaluate_missing_device_mode(
        ocentra_tracking_core::TrackingDeviceStatusInput {
            heartbeat_age_seconds: 1_200,
            battery_percentage: 60,
            low_power_mode_state: TrackingLowPowerModeState::Inactive,
            pending_upload_count: 0,
        },
    );

    assert_eq!(
        decision.missing_device_state,
        TrackingMissingDeviceState::parse(constants::tracking_runtime::MISSING_DEVICE_STATE_LAST_KNOWN_ONLY)
            .expect(constants::tracking_runtime::MISSING_DEVICE_STATE_LAST_KNOWN_ONLY)
    );
    assert_eq!(
        decision.parent_visibility_state,
        ocentra_tracking_core::TrackingLastKnownVisibilityState::LastKnownOnly
    );
}
