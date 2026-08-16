use super::{
    constants, tracking_capability_status_decision, TrackingBackgroundCapabilityState,
    TrackingCapabilityAvailabilityState, TrackingCapabilityStatusDecision, TrackingPlatformState,
};

pub(super) fn granted_foreground_capability_status_decision(
    platform_state: &TrackingPlatformState,
    background_capability_state: &TrackingBackgroundCapabilityState,
    strict_background_required: bool,
) -> TrackingCapabilityStatusDecision {
    if *background_capability_state == TrackingBackgroundCapabilityState::ManagedDeviceRequired
        || *platform_state == TrackingPlatformState::ManagedDevice
    {
        return tracking_capability_status_decision(
            constants::tracking_runtime::CAPABILITY_STATUS_MANUAL_REQUIRED,
            TrackingCapabilityAvailabilityState::Available,
            TrackingCapabilityAvailabilityState::Unavailable,
            true,
            vec![constants::tracking_runtime::REASON_MANAGED_DEVICE_PROOF_REQUIRED],
        );
    }

    if strict_background_required
        || *background_capability_state == TrackingBackgroundCapabilityState::PermissionRequired
    {
        return tracking_capability_status_decision(
            constants::tracking_runtime::CAPABILITY_STATUS_BACKGROUND_PERMISSION_REQUIRED,
            TrackingCapabilityAvailabilityState::Available,
            TrackingCapabilityAvailabilityState::Unavailable,
            true,
            vec![constants::tracking_runtime::REASON_BACKGROUND_PERMISSION_REQUIRED],
        );
    }

    if *background_capability_state == TrackingBackgroundCapabilityState::Unsupported {
        return foreground_only_capability_status_decision(vec![
            constants::tracking_runtime::REASON_BACKGROUND_PLATFORM_UNSUPPORTED,
        ]);
    }

    foreground_only_capability_status_decision(Vec::new())
}

pub(super) fn granted_background_capability_status_decision(
    background_capability_state: &TrackingBackgroundCapabilityState,
) -> TrackingCapabilityStatusDecision {
    let (
        capability_status,
        background_availability_state,
        manual_action_required,
        degraded_reasons,
    ) = match background_capability_state {
        TrackingBackgroundCapabilityState::Ready => (
            constants::tracking_runtime::CAPABILITY_STATUS_BACKGROUND_READY,
            TrackingCapabilityAvailabilityState::Available,
            false,
            Vec::new(),
        ),
        TrackingBackgroundCapabilityState::PermissionRequired => (
            constants::tracking_runtime::CAPABILITY_STATUS_BACKGROUND_PERMISSION_REQUIRED,
            TrackingCapabilityAvailabilityState::Unavailable,
            true,
            vec![constants::tracking_runtime::REASON_BACKGROUND_PERMISSION_REQUIRED],
        ),
        TrackingBackgroundCapabilityState::ManagedDeviceRequired => (
            constants::tracking_runtime::CAPABILITY_STATUS_MANUAL_REQUIRED,
            TrackingCapabilityAvailabilityState::Unavailable,
            true,
            vec![constants::tracking_runtime::REASON_MANAGED_DEVICE_PROOF_REQUIRED],
        ),
        TrackingBackgroundCapabilityState::Unsupported => (
            constants::tracking_runtime::CAPABILITY_STATUS_FOREGROUND_ONLY,
            TrackingCapabilityAvailabilityState::Unavailable,
            false,
            vec![constants::tracking_runtime::REASON_BACKGROUND_PLATFORM_UNSUPPORTED],
        ),
    };

    tracking_capability_status_decision(
        capability_status,
        TrackingCapabilityAvailabilityState::Available,
        background_availability_state,
        manual_action_required,
        degraded_reasons,
    )
}

fn foreground_only_capability_status_decision(
    degraded_reasons: Vec<&'static str>,
) -> TrackingCapabilityStatusDecision {
    tracking_capability_status_decision(
        constants::tracking_runtime::CAPABILITY_STATUS_FOREGROUND_ONLY,
        TrackingCapabilityAvailabilityState::Available,
        TrackingCapabilityAvailabilityState::Unavailable,
        false,
        degraded_reasons,
    )
}
