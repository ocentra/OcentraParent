use super::status_capability_grants::{
    granted_background_capability_status_decision, granted_foreground_capability_status_decision,
};
use super::status_capability_state_rules::{
    capability_status_for_device_status, capability_status_for_service_state,
};
use super::{
    constants, tracking_capability_status_decision, TrackingCapabilityAvailabilityState,
    TrackingCapabilityStatusDecision, TrackingCapabilityStatusInput, TrackingPermissionState,
    TrackingPlatformState,
};

pub(super) fn evaluate_tracking_capability_status(
    input: TrackingCapabilityStatusInput,
) -> TrackingCapabilityStatusDecision {
    let TrackingCapabilityStatusInput {
        permission_state,
        platform_state,
        background_capability_state,
        strict_background_required,
        service_state,
        device_status,
    } = input;

    if let Some(decision) = capability_status_for_service_state(&service_state) {
        return decision;
    }

    if platform_state == TrackingPlatformState::Unsupported {
        return tracking_capability_status_decision(
            constants::tracking_runtime::CAPABILITY_STATUS_PLATFORM_UNSUPPORTED,
            TrackingCapabilityAvailabilityState::Unavailable,
            TrackingCapabilityAvailabilityState::Unavailable,
            false,
            vec![constants::tracking_runtime::REASON_PLATFORM_UNSUPPORTED],
        );
    }

    if let Some(decision) = capability_status_for_device_status(device_status) {
        return decision;
    }

    capability_status_from_permission_state(
        permission_state,
        platform_state,
        background_capability_state,
        strict_background_required,
    )
}

fn capability_status_from_permission_state(
    permission_state: TrackingPermissionState,
    platform_state: TrackingPlatformState,
    background_capability_state: super::TrackingBackgroundCapabilityState,
    strict_background_required: bool,
) -> TrackingCapabilityStatusDecision {
    match permission_state {
        TrackingPermissionState::ManualRequired => tracking_capability_status_decision(
            constants::tracking_runtime::CAPABILITY_STATUS_MANUAL_REQUIRED,
            TrackingCapabilityAvailabilityState::Unavailable,
            TrackingCapabilityAvailabilityState::Unavailable,
            true,
            vec![constants::tracking_runtime::REASON_MANAGED_DEVICE_PROOF_REQUIRED],
        ),
        TrackingPermissionState::Denied
        | TrackingPermissionState::Restricted
        | TrackingPermissionState::NotRequested => tracking_capability_status_decision(
            constants::tracking_runtime::CAPABILITY_STATUS_PERMISSION_REQUIRED,
            TrackingCapabilityAvailabilityState::Unavailable,
            TrackingCapabilityAvailabilityState::Unavailable,
            true,
            vec![constants::tracking_runtime::REASON_FOREGROUND_PERMISSION_REQUIRED],
        ),
        TrackingPermissionState::ServiceDisabled => tracking_capability_status_decision(
            constants::tracking_runtime::CAPABILITY_STATUS_SERVICE_DISABLED,
            TrackingCapabilityAvailabilityState::Unavailable,
            TrackingCapabilityAvailabilityState::Unavailable,
            true,
            vec![constants::tracking_runtime::REASON_SERVICE_DISABLED],
        ),
        TrackingPermissionState::Unsupported | TrackingPermissionState::Unavailable => {
            tracking_capability_status_decision(
                constants::tracking_runtime::CAPABILITY_STATUS_UNAVAILABLE,
                TrackingCapabilityAvailabilityState::Unavailable,
                TrackingCapabilityAvailabilityState::Unavailable,
                true,
                vec![constants::tracking_runtime::REASON_TRACKING_RUNTIME_UNAVAILABLE],
            )
        }
        TrackingPermissionState::ApproximateOnly => tracking_capability_status_decision(
            constants::tracking_runtime::CAPABILITY_STATUS_APPROXIMATE_ONLY,
            TrackingCapabilityAvailabilityState::Limited,
            TrackingCapabilityAvailabilityState::Unavailable,
            false,
            vec![constants::tracking_runtime::REASON_PRECISE_LOCATION_UNAVAILABLE],
        ),
        TrackingPermissionState::GrantedForeground => {
            granted_foreground_capability_status_decision(
                &platform_state,
                &background_capability_state,
                strict_background_required,
            )
        }
        TrackingPermissionState::GrantedBackground => {
            granted_background_capability_status_decision(&background_capability_state)
        }
    }
}
