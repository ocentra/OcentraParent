use ocentra_parent_agent_protocol::constants;
use ocentra_tracking_core::status::{
    TrackingBackgroundCapabilityState, TrackingPermissionState, TrackingPlatformState,
    TrackingRuntimeServiceState,
};

#[test]
fn tracking_capability_distinguishes_background_ready_and_permission_required_states() {
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
    let background_permission_required =
        ocentra_tracking_core::status::evaluate_tracking_capability_status(
            ocentra_tracking_core::status::TrackingCapabilityStatusInput {
                permission_state: TrackingPermissionState::GrantedForeground,
                platform_state: TrackingPlatformState::Android,
                background_capability_state: TrackingBackgroundCapabilityState::PermissionRequired,
                strict_background_required: true,
                service_state: TrackingRuntimeServiceState::Running,
                device_status: constants::tracking_runtime::DEVICE_STATUS_LIVE,
            },
        );
    let manual_required = ocentra_tracking_core::status::evaluate_tracking_capability_status(
        ocentra_tracking_core::status::TrackingCapabilityStatusInput {
            permission_state: TrackingPermissionState::ManualRequired,
            platform_state: TrackingPlatformState::ManagedDevice,
            background_capability_state: TrackingBackgroundCapabilityState::ManagedDeviceRequired,
            strict_background_required: false,
            service_state: TrackingRuntimeServiceState::Running,
            device_status: constants::tracking_runtime::DEVICE_STATUS_LIVE,
        },
    );

    assert_eq!(
        background_ready.capability_status,
        constants::tracking_runtime::CAPABILITY_STATUS_BACKGROUND_READY
    );
    assert_eq!(
        background_permission_required.capability_status,
        constants::tracking_runtime::CAPABILITY_STATUS_BACKGROUND_PERMISSION_REQUIRED
    );
    assert_eq!(
        manual_required.capability_status,
        constants::tracking_runtime::CAPABILITY_STATUS_MANUAL_REQUIRED
    );
    assert!(background_permission_required.manual_action_required);
    assert!(manual_required.manual_action_required);
}
