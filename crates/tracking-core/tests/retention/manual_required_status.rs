use ocentra_eventing::expect_value::ExpectValue;
use ocentra_evidence::ManualReviewState;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::tracking::identifiers::TrackingChildDeviceId;
use ocentra_tracking_core::status::{
    TrackingChargingState, TrackingConnectivityState, TrackingLowPowerModeState,
    TrackingRadioState, TrackingRuntimeServiceState,
};

fn child_device_id() -> TrackingChildDeviceId {
    TrackingChildDeviceId::parse(constants::tracking_runtime::DEFAULT_CHILD_DEVICE_ID)
        .expect_value(constants::tracking_runtime::DEFAULT_CHILD_DEVICE_ID)
}

#[test]
fn offline_device_state_remains_last_known_only_manual_required() {
    let decision = ocentra_tracking_core::status::evaluate_tracking_device_status(
        ocentra_tracking_core::status::TrackingDeviceStatusInput {
            child_device_id: child_device_id(),
            last_heartbeat_age_seconds: 901,
            last_location_sample_age_seconds: 60,
            last_parent_sync_age_seconds: 60,
            battery_percentage: Some(90),
            charging_state: TrackingChargingState::Discharging,
            low_power_mode_state: TrackingLowPowerModeState::Inactive,
            connectivity_state: TrackingConnectivityState::Online,
            radio_state: TrackingRadioState::Enabled,
            pending_upload_count: 0,
            service_state: TrackingRuntimeServiceState::Running,
        },
    );

    assert_eq!(
        decision.device_status,
        constants::tracking_runtime::DEVICE_STATUS_OFFLINE_LAST_KNOWN_ONLY
    );
    assert_eq!(decision.manual_review_state, ManualReviewState::Required);
    assert!(decision
        .degraded_reasons
        .contains(&constants::tracking_runtime::REASON_TRACKING_HEARTBEAT_STALE));
}
