use ocentra_eventing::expect_value::ExpectValue;
use ocentra_evidence::ManualReviewState;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::tracking::identifiers::TrackingChildDeviceId;
use ocentra_tracking_core::status::{
    TrackingBackgroundCapabilityState, TrackingChargingState, TrackingConnectivityState,
    TrackingLowPowerModeState, TrackingPermissionState, TrackingPlatformState, TrackingRadioState,
    TrackingRuntimeServiceState,
};

fn child_device_id() -> TrackingChildDeviceId {
    TrackingChildDeviceId::parse(constants::tracking_runtime::DEFAULT_CHILD_DEVICE_ID)
        .expect_value(constants::tracking_runtime::DEFAULT_CHILD_DEVICE_ID)
}

#[test]
fn device_status_reports_live_when_runtime_inputs_are_healthy() {
    let decision = ocentra_tracking_core::status::evaluate_tracking_device_status(
        ocentra_tracking_core::status::TrackingDeviceStatusInput {
            child_device_id: child_device_id(),
            last_heartbeat_age_seconds: 45,
            last_location_sample_age_seconds: 30,
            last_parent_sync_age_seconds: 120,
            battery_percentage: Some(82),
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
        constants::tracking_runtime::DEVICE_STATUS_LIVE
    );
    assert_eq!(decision.manual_review_state, ManualReviewState::NotRequired);
    assert!(decision.degraded_reasons.is_empty());
}

#[test]
fn device_status_reports_stale_without_claiming_live_location() {
    let decision = ocentra_tracking_core::status::evaluate_tracking_device_status(
        ocentra_tracking_core::status::TrackingDeviceStatusInput {
            child_device_id: child_device_id(),
            last_heartbeat_age_seconds: 360,
            last_location_sample_age_seconds: 320,
            last_parent_sync_age_seconds: 1_200,
            battery_percentage: Some(70),
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
        constants::tracking_runtime::DEVICE_STATUS_STALE
    );
    assert!(decision
        .degraded_reasons
        .contains(&constants::tracking_runtime::REASON_TRACKING_HEARTBEAT_STALE));
    assert!(decision
        .degraded_reasons
        .contains(&constants::tracking_runtime::REASON_LAST_LOCATION_SAMPLE_STALE));
    assert!(decision
        .degraded_reasons
        .contains(&constants::tracking_runtime::REASON_PARENT_SYNC_LATE));
}

#[test]
fn device_status_reports_pending_upload_backlog_explicitly() {
    let decision = ocentra_tracking_core::status::evaluate_tracking_device_status(
        ocentra_tracking_core::status::TrackingDeviceStatusInput {
            child_device_id: child_device_id(),
            last_heartbeat_age_seconds: 45,
            last_location_sample_age_seconds: 45,
            last_parent_sync_age_seconds: 1_400,
            battery_percentage: Some(72),
            charging_state: TrackingChargingState::Discharging,
            low_power_mode_state: TrackingLowPowerModeState::Inactive,
            connectivity_state: TrackingConnectivityState::Online,
            radio_state: TrackingRadioState::Enabled,
            pending_upload_count: 4,
            service_state: TrackingRuntimeServiceState::Running,
        },
    );

    assert_eq!(
        decision.device_status,
        constants::tracking_runtime::DEVICE_STATUS_PENDING_UPLOAD
    );
    assert!(decision
        .degraded_reasons
        .contains(&constants::tracking_runtime::REASON_PENDING_UPLOAD_BACKLOG));
}

#[test]
fn device_status_reports_service_disabled_explicitly() {
    let decision = ocentra_tracking_core::status::evaluate_tracking_device_status(
        ocentra_tracking_core::status::TrackingDeviceStatusInput {
            child_device_id: child_device_id(),
            last_heartbeat_age_seconds: 10,
            last_location_sample_age_seconds: 10,
            last_parent_sync_age_seconds: 10,
            battery_percentage: Some(90),
            charging_state: TrackingChargingState::Charging,
            low_power_mode_state: TrackingLowPowerModeState::Inactive,
            connectivity_state: TrackingConnectivityState::Online,
            radio_state: TrackingRadioState::Enabled,
            pending_upload_count: 0,
            service_state: TrackingRuntimeServiceState::ServiceDisabled,
        },
    );

    assert_eq!(
        decision.device_status,
        constants::tracking_runtime::DEVICE_STATUS_SERVICE_DISABLED
    );
    assert_eq!(decision.manual_review_state, ManualReviewState::Required);
}

#[test]
fn capability_status_distinguishes_foreground_only_background_ready_and_approximate_only() {
    let foreground_only = ocentra_tracking_core::status::evaluate_tracking_capability_status(
        ocentra_tracking_core::status::TrackingCapabilityStatusInput {
            permission_state: TrackingPermissionState::GrantedForeground,
            platform_state: TrackingPlatformState::Android,
            background_capability_state: TrackingBackgroundCapabilityState::Ready,
            strict_background_required: false,
            service_state: TrackingRuntimeServiceState::Running,
            device_status: constants::tracking_runtime::DEVICE_STATUS_LIVE,
        },
    );
    let background_ready = ocentra_tracking_core::status::evaluate_tracking_capability_status(
        ocentra_tracking_core::status::TrackingCapabilityStatusInput {
            permission_state: TrackingPermissionState::GrantedBackground,
            platform_state: TrackingPlatformState::Android,
            background_capability_state: TrackingBackgroundCapabilityState::Ready,
            strict_background_required: false,
            service_state: TrackingRuntimeServiceState::Running,
            device_status: constants::tracking_runtime::DEVICE_STATUS_LIVE,
        },
    );
    let approximate_only = ocentra_tracking_core::status::evaluate_tracking_capability_status(
        ocentra_tracking_core::status::TrackingCapabilityStatusInput {
            permission_state: TrackingPermissionState::ApproximateOnly,
            platform_state: TrackingPlatformState::Android,
            background_capability_state: TrackingBackgroundCapabilityState::PermissionRequired,
            strict_background_required: false,
            service_state: TrackingRuntimeServiceState::Running,
            device_status: constants::tracking_runtime::DEVICE_STATUS_LIVE,
        },
    );

    assert_eq!(
        foreground_only.capability_status,
        constants::tracking_runtime::CAPABILITY_STATUS_FOREGROUND_ONLY
    );
    assert_eq!(
        background_ready.capability_status,
        constants::tracking_runtime::CAPABILITY_STATUS_BACKGROUND_READY
    );
    assert_eq!(
        approximate_only.capability_status,
        constants::tracking_runtime::CAPABILITY_STATUS_APPROXIMATE_ONLY
    );
}

#[test]
fn capability_status_requires_action_for_permission_platform_and_service_failures() {
    let permission_required = ocentra_tracking_core::status::evaluate_tracking_capability_status(
        ocentra_tracking_core::status::TrackingCapabilityStatusInput {
            permission_state: TrackingPermissionState::Denied,
            platform_state: TrackingPlatformState::Android,
            background_capability_state: TrackingBackgroundCapabilityState::PermissionRequired,
            strict_background_required: false,
            service_state: TrackingRuntimeServiceState::Running,
            device_status: constants::tracking_runtime::DEVICE_STATUS_LIVE,
        },
    );
    let platform_unsupported = ocentra_tracking_core::status::evaluate_tracking_capability_status(
        ocentra_tracking_core::status::TrackingCapabilityStatusInput {
            permission_state: TrackingPermissionState::GrantedBackground,
            platform_state: TrackingPlatformState::Unsupported,
            background_capability_state: TrackingBackgroundCapabilityState::Unsupported,
            strict_background_required: false,
            service_state: TrackingRuntimeServiceState::Running,
            device_status: constants::tracking_runtime::DEVICE_STATUS_LIVE,
        },
    );
    let adapter_error = ocentra_tracking_core::status::evaluate_tracking_capability_status(
        ocentra_tracking_core::status::TrackingCapabilityStatusInput {
            permission_state: TrackingPermissionState::GrantedBackground,
            platform_state: TrackingPlatformState::Android,
            background_capability_state: TrackingBackgroundCapabilityState::Ready,
            strict_background_required: false,
            service_state: TrackingRuntimeServiceState::AdapterError,
            device_status: constants::tracking_runtime::DEVICE_STATUS_LIVE,
        },
    );

    assert_eq!(
        permission_required.capability_status,
        constants::tracking_runtime::CAPABILITY_STATUS_PERMISSION_REQUIRED
    );
    assert!(permission_required.manual_action_required);
    assert_eq!(
        platform_unsupported.capability_status,
        constants::tracking_runtime::CAPABILITY_STATUS_PLATFORM_UNSUPPORTED
    );
    assert_eq!(
        adapter_error.capability_status,
        constants::tracking_runtime::CAPABILITY_STATUS_ADAPTER_ERROR
    );
    assert!(adapter_error.manual_action_required);
}

#[test]
fn capability_status_tracks_runtime_degradation_from_device_status() {
    let offline_last_known = ocentra_tracking_core::status::evaluate_tracking_capability_status(
        ocentra_tracking_core::status::TrackingCapabilityStatusInput {
            permission_state: TrackingPermissionState::GrantedBackground,
            platform_state: TrackingPlatformState::Android,
            background_capability_state: TrackingBackgroundCapabilityState::Ready,
            strict_background_required: false,
            service_state: TrackingRuntimeServiceState::Running,
            device_status: constants::tracking_runtime::DEVICE_STATUS_OFFLINE_LAST_KNOWN_ONLY,
        },
    );
    let battery_throttled = ocentra_tracking_core::status::evaluate_tracking_capability_status(
        ocentra_tracking_core::status::TrackingCapabilityStatusInput {
            permission_state: TrackingPermissionState::GrantedBackground,
            platform_state: TrackingPlatformState::Android,
            background_capability_state: TrackingBackgroundCapabilityState::Ready,
            strict_background_required: false,
            service_state: TrackingRuntimeServiceState::Running,
            device_status: constants::tracking_runtime::DEVICE_STATUS_BATTERY_THROTTLED,
        },
    );

    assert_eq!(
        offline_last_known.capability_status,
        constants::tracking_runtime::CAPABILITY_STATUS_OFFLINE_LAST_KNOWN_ONLY
    );
    assert_eq!(
        battery_throttled.capability_status,
        constants::tracking_runtime::CAPABILITY_STATUS_BATTERY_THROTTLED
    );
}
