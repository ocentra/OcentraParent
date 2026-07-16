use ocentra_evidence::ManualReviewState;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::tracking::identifiers::TrackingChildDeviceId;

#[path = "status_capability_grants.rs"]
mod status_capability_grants;
#[path = "status_capability_rules.rs"]
mod status_capability_rules;
#[path = "status_capability_state_rules.rs"]
mod status_capability_state_rules;
#[path = "status_device_battery_rules.rs"]
mod status_device_battery_rules;
#[path = "status_device_reason_rules.rs"]
mod status_device_reason_rules;
#[path = "status_device_rules.rs"]
mod status_device_rules;
#[path = "status_device_service_rules.rs"]
mod status_device_service_rules;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TrackingLowPowerModeState {
    Active,
    Inactive,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TrackingChargingState {
    Charging,
    Discharging,
    Full,
    Unknown,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TrackingConnectivityState {
    Online,
    Offline,
    CaptiveNetwork,
    Metered,
    Unknown,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TrackingRadioState {
    Enabled,
    Disabled,
    Unknown,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TrackingRuntimeServiceState {
    Running,
    ServiceDisabled,
    Unavailable,
    AdapterError,
    DisabledByParent,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TrackingPermissionState {
    GrantedForeground,
    GrantedBackground,
    ApproximateOnly,
    Denied,
    Restricted,
    NotRequested,
    ServiceDisabled,
    Unsupported,
    Unavailable,
    ManualRequired,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TrackingPlatformState {
    Android,
    Ios,
    Windows,
    Macos,
    Linux,
    ManagedDevice,
    Unsupported,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TrackingBackgroundCapabilityState {
    Ready,
    PermissionRequired,
    ManagedDeviceRequired,
    Unsupported,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TrackingCapabilityAvailabilityState {
    Available,
    Limited,
    Unavailable,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackingDeviceStatusInput {
    pub child_device_id: TrackingChildDeviceId,
    pub last_heartbeat_age_seconds: u32,
    pub last_location_sample_age_seconds: u32,
    pub last_parent_sync_age_seconds: u32,
    pub battery_percentage: Option<u8>,
    pub charging_state: TrackingChargingState,
    pub low_power_mode_state: TrackingLowPowerModeState,
    pub connectivity_state: TrackingConnectivityState,
    pub radio_state: TrackingRadioState,
    pub pending_upload_count: u32,
    pub service_state: TrackingRuntimeServiceState,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackingDeviceStatusDecision {
    pub device_status: &'static str,
    pub manual_review_state: ManualReviewState,
    pub degraded_reasons: Vec<&'static str>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackingCapabilityStatusInput {
    pub permission_state: TrackingPermissionState,
    pub platform_state: TrackingPlatformState,
    pub background_capability_state: TrackingBackgroundCapabilityState,
    pub strict_background_required: bool,
    pub service_state: TrackingRuntimeServiceState,
    pub device_status: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackingCapabilityStatusDecision {
    pub capability_status: &'static str,
    pub foreground_availability_state: TrackingCapabilityAvailabilityState,
    pub background_availability_state: TrackingCapabilityAvailabilityState,
    pub manual_action_required: bool,
    pub degraded_reasons: Vec<&'static str>,
}

const TRACKING_HEARTBEAT_STALE_AFTER_SECONDS: u32 = 300;
const TRACKING_HEARTBEAT_OFFLINE_AFTER_SECONDS: u32 = 900;
const TRACKING_LOCATION_STALE_AFTER_SECONDS: u32 = 300;
const TRACKING_LOCATION_OFFLINE_AFTER_SECONDS: u32 = 900;
const TRACKING_PARENT_SYNC_LATE_AFTER_SECONDS: u32 = 900;
const TRACKING_LOW_BATTERY_THRESHOLD_PERCENT: u8 = 15;

fn tracking_device_status_decision(
    device_status: &'static str,
    manual_review_state: ManualReviewState,
    degraded_reasons: Vec<&'static str>,
) -> TrackingDeviceStatusDecision {
    TrackingDeviceStatusDecision {
        device_status,
        manual_review_state,
        degraded_reasons,
    }
}

fn tracking_capability_status_decision(
    capability_status: &'static str,
    foreground_availability_state: TrackingCapabilityAvailabilityState,
    background_availability_state: TrackingCapabilityAvailabilityState,
    manual_action_required: bool,
    degraded_reasons: Vec<&'static str>,
) -> TrackingCapabilityStatusDecision {
    TrackingCapabilityStatusDecision {
        capability_status,
        foreground_availability_state,
        background_availability_state,
        manual_action_required,
        degraded_reasons,
    }
}

pub fn evaluate_tracking_device_status(
    input: TrackingDeviceStatusInput,
) -> TrackingDeviceStatusDecision {
    status_device_rules::evaluate_tracking_device_status(input)
}

pub fn evaluate_tracking_capability_status(
    input: TrackingCapabilityStatusInput,
) -> TrackingCapabilityStatusDecision {
    status_capability_rules::evaluate_tracking_capability_status(input)
}
