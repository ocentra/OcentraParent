use ocentra_evidence::ManualReviewState;
use ocentra_parent_agent_protocol::constants;
use ocentra_tracking_core::TrackingLowPowerModeState;

#[test]
fn offline_device_state_remains_last_known_only_manual_required() {
    let decision = ocentra_tracking_core::evaluate_tracking_device_status(
        ocentra_tracking_core::TrackingDeviceStatusInput {
            heartbeat_age_seconds: 901,
            battery_percentage: 90,
            low_power_mode_state: TrackingLowPowerModeState::Inactive,
            pending_upload_count: 0,
        },
    );

    assert_eq!(
        decision.device_status,
        constants::tracking_runtime::DEVICE_STATUS_OFFLINE_LAST_KNOWN_ONLY
    );
    assert_eq!(decision.manual_review_state, ManualReviewState::Required);
}
