use ocentra_parent_agent_protocol::constants;
use ocentra_evidence::ManualReviewState;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TrackingLowPowerModeState {
    Active,
    Inactive,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TrackingPermissionState {
    Granted,
    Missing,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TrackingPlatformBackgroundState {
    Supported,
    Unsupported,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TrackingCapabilityAvailabilityState {
    Available,
    Unavailable,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackingDeviceStatusInput {
    pub heartbeat_age_seconds: u32,
    pub battery_percentage: u8,
    pub low_power_mode_state: TrackingLowPowerModeState,
    pub pending_upload_count: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackingDeviceStatusDecision {
    pub device_status: &'static str,
    pub manual_review_state: ManualReviewState,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackingCapabilityStatusInput {
    pub foreground_permission_state: TrackingPermissionState,
    pub background_permission_state: TrackingPermissionState,
    pub platform_background_state: TrackingPlatformBackgroundState,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackingCapabilityStatusDecision {
    pub capability_status: &'static str,
    pub foreground_availability_state: TrackingCapabilityAvailabilityState,
    pub background_availability_state: TrackingCapabilityAvailabilityState,
}

pub fn evaluate_tracking_device_status(
    input: TrackingDeviceStatusInput,
) -> TrackingDeviceStatusDecision {
    if input.heartbeat_age_seconds > 900 || input.pending_upload_count > 0 {
        return TrackingDeviceStatusDecision {
            device_status: constants::tracking_runtime::DEVICE_STATUS_OFFLINE_LAST_KNOWN_ONLY,
            manual_review_state: ManualReviewState::Required,
        };
    }

    if input.low_power_mode_state == TrackingLowPowerModeState::Active
        || input.battery_percentage <= 15
    {
        return TrackingDeviceStatusDecision {
            device_status: constants::tracking_runtime::DEVICE_STATUS_BATTERY_THROTTLED,
            manual_review_state: ManualReviewState::NotRequired,
        };
    }

    TrackingDeviceStatusDecision {
        device_status: constants::tracking_runtime::DEVICE_STATUS_ONLINE,
        manual_review_state: ManualReviewState::NotRequired,
    }
}

pub fn evaluate_tracking_capability_status(
    input: TrackingCapabilityStatusInput,
) -> TrackingCapabilityStatusDecision {
    if input.foreground_permission_state == TrackingPermissionState::Missing {
        return TrackingCapabilityStatusDecision {
            capability_status: constants::tracking_runtime::CAPABILITY_STATUS_MANUAL_REQUIRED,
            foreground_availability_state: TrackingCapabilityAvailabilityState::Unavailable,
            background_availability_state: TrackingCapabilityAvailabilityState::Unavailable,
        };
    }

    if input.background_permission_state == TrackingPermissionState::Missing
        || input.platform_background_state == TrackingPlatformBackgroundState::Unsupported
    {
        return TrackingCapabilityStatusDecision {
            capability_status: constants::tracking_runtime::CAPABILITY_STATUS_DEGRADED,
            foreground_availability_state: TrackingCapabilityAvailabilityState::Available,
            background_availability_state: TrackingCapabilityAvailabilityState::Unavailable,
        };
    }

    TrackingCapabilityStatusDecision {
        capability_status: constants::tracking_runtime::CAPABILITY_STATUS_GRANTED,
        foreground_availability_state: TrackingCapabilityAvailabilityState::Available,
        background_availability_state: TrackingCapabilityAvailabilityState::Available,
    }
}
