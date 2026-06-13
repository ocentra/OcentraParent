use ocentra_parent_agent_protocol::constants;
use ocentra_tracking_core::{
    TrackingPermissionState, TrackingPlatformBackgroundState,
};

#[test]
fn tracking_capability_distinguishes_granted_degraded_and_manual_required() {
    let granted = ocentra_tracking_core::evaluate_tracking_capability_status(
        ocentra_tracking_core::TrackingCapabilityStatusInput {
            foreground_permission_state: TrackingPermissionState::Granted,
            background_permission_state: TrackingPermissionState::Granted,
            platform_background_state: TrackingPlatformBackgroundState::Supported,
        },
    );
    let degraded = ocentra_tracking_core::evaluate_tracking_capability_status(
        ocentra_tracking_core::TrackingCapabilityStatusInput {
            foreground_permission_state: TrackingPermissionState::Granted,
            background_permission_state: TrackingPermissionState::Missing,
            platform_background_state: TrackingPlatformBackgroundState::Supported,
        },
    );
    let manual = ocentra_tracking_core::evaluate_tracking_capability_status(
        ocentra_tracking_core::TrackingCapabilityStatusInput {
            foreground_permission_state: TrackingPermissionState::Missing,
            background_permission_state: TrackingPermissionState::Granted,
            platform_background_state: TrackingPlatformBackgroundState::Supported,
        },
    );

    assert_eq!(
        granted.capability_status,
        constants::tracking_runtime::CAPABILITY_STATUS_GRANTED
    );
    assert_eq!(
        degraded.capability_status,
        constants::tracking_runtime::CAPABILITY_STATUS_DEGRADED
    );
    assert_eq!(
        manual.capability_status,
        constants::tracking_runtime::CAPABILITY_STATUS_MANUAL_REQUIRED
    );
}
