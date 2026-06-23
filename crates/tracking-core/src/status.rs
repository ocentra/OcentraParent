use ocentra_evidence::ManualReviewState;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::tracking::identifiers::TrackingChildDeviceId;

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

#[derive(Clone, Debug, PartialEq, Eq)]
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TrackingPlatformState {
    Android,
    Ios,
    Windows,
    Macos,
    Linux,
    ManagedDevice,
    Unsupported,
}

#[derive(Clone, Debug, PartialEq, Eq)]
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
    let TrackingDeviceStatusInput {
        child_device_id: _child_device_id,
        last_heartbeat_age_seconds,
        last_location_sample_age_seconds,
        last_parent_sync_age_seconds,
        battery_percentage,
        charging_state,
        low_power_mode_state,
        connectivity_state,
        radio_state,
        pending_upload_count,
        service_state,
    } = input;

    if let Some(decision) = device_status_for_service_state(&service_state) {
        return decision;
    }

    let degraded_reasons = offline_device_reasons(
        last_heartbeat_age_seconds,
        last_location_sample_age_seconds,
        &connectivity_state,
        &radio_state,
    );
    if !degraded_reasons.is_empty() {
        return tracking_device_status_decision(
            constants::tracking_runtime::DEVICE_STATUS_OFFLINE_LAST_KNOWN_ONLY,
            ManualReviewState::Required,
            degraded_reasons,
        );
    }

    if pending_upload_count > 0 {
        return pending_upload_device_status_decision(last_parent_sync_age_seconds);
    }

    let degraded_reasons = stale_device_reasons(
        last_heartbeat_age_seconds,
        last_location_sample_age_seconds,
        last_parent_sync_age_seconds,
    );
    if !degraded_reasons.is_empty() {
        return tracking_device_status_decision(
            constants::tracking_runtime::DEVICE_STATUS_STALE,
            ManualReviewState::NotRequired,
            degraded_reasons,
        );
    }

    let degraded_reasons = battery_throttled_device_reasons(
        battery_percentage,
        &charging_state,
        &low_power_mode_state,
        &connectivity_state,
    );
    if !degraded_reasons.is_empty() {
        return tracking_device_status_decision(
            constants::tracking_runtime::DEVICE_STATUS_BATTERY_THROTTLED,
            ManualReviewState::NotRequired,
            degraded_reasons,
        );
    }

    tracking_device_status_decision(
        constants::tracking_runtime::DEVICE_STATUS_LIVE,
        ManualReviewState::NotRequired,
        Vec::new(),
    )
}

pub fn evaluate_tracking_capability_status(
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

fn device_status_for_service_state(
    service_state: &TrackingRuntimeServiceState,
) -> Option<TrackingDeviceStatusDecision> {
    match service_state {
        TrackingRuntimeServiceState::DisabledByParent => Some(tracking_device_status_decision(
            constants::tracking_runtime::DEVICE_STATUS_SERVICE_DISABLED,
            ManualReviewState::NotRequired,
            vec![constants::tracking_runtime::REASON_DISABLED_BY_PARENT],
        )),
        TrackingRuntimeServiceState::ServiceDisabled => Some(tracking_device_status_decision(
            constants::tracking_runtime::DEVICE_STATUS_SERVICE_DISABLED,
            ManualReviewState::Required,
            vec![constants::tracking_runtime::REASON_SERVICE_DISABLED],
        )),
        TrackingRuntimeServiceState::Unavailable => Some(tracking_device_status_decision(
            constants::tracking_runtime::DEVICE_STATUS_UNAVAILABLE,
            ManualReviewState::Required,
            vec![constants::tracking_runtime::REASON_TRACKING_RUNTIME_UNAVAILABLE],
        )),
        TrackingRuntimeServiceState::AdapterError => Some(tracking_device_status_decision(
            constants::tracking_runtime::DEVICE_STATUS_UNAVAILABLE,
            ManualReviewState::Required,
            vec![constants::tracking_runtime::REASON_ADAPTER_ERROR],
        )),
        TrackingRuntimeServiceState::Running => None,
    }
}

fn offline_device_reasons(
    last_heartbeat_age_seconds: u32,
    last_location_sample_age_seconds: u32,
    connectivity_state: &TrackingConnectivityState,
    radio_state: &TrackingRadioState,
) -> Vec<&'static str> {
    let mut degraded_reasons = Vec::new();
    if last_heartbeat_age_seconds > TRACKING_HEARTBEAT_OFFLINE_AFTER_SECONDS {
        degraded_reasons.push(constants::tracking_runtime::REASON_TRACKING_HEARTBEAT_STALE);
    }
    if last_location_sample_age_seconds > TRACKING_LOCATION_OFFLINE_AFTER_SECONDS {
        degraded_reasons.push(constants::tracking_runtime::REASON_LAST_LOCATION_SAMPLE_STALE);
    }
    if *connectivity_state == TrackingConnectivityState::Offline {
        degraded_reasons.push(constants::tracking_runtime::REASON_CONNECTIVITY_OFFLINE);
    }
    if *radio_state == TrackingRadioState::Disabled {
        degraded_reasons.push(constants::tracking_runtime::REASON_RADIO_DISABLED);
    }
    degraded_reasons
}

fn pending_upload_device_status_decision(
    last_parent_sync_age_seconds: u32,
) -> TrackingDeviceStatusDecision {
    let mut degraded_reasons = vec![constants::tracking_runtime::REASON_PENDING_UPLOAD_BACKLOG];
    if last_parent_sync_age_seconds > TRACKING_PARENT_SYNC_LATE_AFTER_SECONDS {
        degraded_reasons.push(constants::tracking_runtime::REASON_PARENT_SYNC_LATE);
    }
    tracking_device_status_decision(
        constants::tracking_runtime::DEVICE_STATUS_PENDING_UPLOAD,
        ManualReviewState::NotRequired,
        degraded_reasons,
    )
}

fn stale_device_reasons(
    last_heartbeat_age_seconds: u32,
    last_location_sample_age_seconds: u32,
    last_parent_sync_age_seconds: u32,
) -> Vec<&'static str> {
    let mut degraded_reasons = Vec::new();
    if last_heartbeat_age_seconds > TRACKING_HEARTBEAT_STALE_AFTER_SECONDS {
        degraded_reasons.push(constants::tracking_runtime::REASON_TRACKING_HEARTBEAT_STALE);
    }
    if last_location_sample_age_seconds > TRACKING_LOCATION_STALE_AFTER_SECONDS {
        degraded_reasons.push(constants::tracking_runtime::REASON_LAST_LOCATION_SAMPLE_STALE);
    }
    if last_parent_sync_age_seconds > TRACKING_PARENT_SYNC_LATE_AFTER_SECONDS {
        degraded_reasons.push(constants::tracking_runtime::REASON_PARENT_SYNC_LATE);
    }
    degraded_reasons
}

fn battery_throttled_device_reasons(
    battery_percentage: Option<u8>,
    charging_state: &TrackingChargingState,
    low_power_mode_state: &TrackingLowPowerModeState,
    connectivity_state: &TrackingConnectivityState,
) -> Vec<&'static str> {
    let low_battery = battery_percentage
        .is_some_and(|percent| percent <= TRACKING_LOW_BATTERY_THRESHOLD_PERCENT)
        && *charging_state == TrackingChargingState::Discharging;
    let mut degraded_reasons = Vec::new();
    if *low_power_mode_state == TrackingLowPowerModeState::Active
        || low_battery
        || *connectivity_state == TrackingConnectivityState::Metered
    {
        degraded_reasons.push(constants::tracking_runtime::REASON_BATTERY_THROTTLED);
    }
    if *low_power_mode_state == TrackingLowPowerModeState::Active {
        degraded_reasons.push(constants::tracking_runtime::REASON_LOW_POWER_MODE);
    }
    if low_battery {
        degraded_reasons.push(constants::tracking_runtime::REASON_BATTERY_LOW);
    }
    if *connectivity_state == TrackingConnectivityState::Metered {
        degraded_reasons.push(constants::tracking_runtime::REASON_CONNECTIVITY_METERED);
    }
    degraded_reasons
}

fn capability_status_for_service_state(
    service_state: &TrackingRuntimeServiceState,
) -> Option<TrackingCapabilityStatusDecision> {
    match service_state {
        TrackingRuntimeServiceState::DisabledByParent => Some(tracking_capability_status_decision(
            constants::tracking_runtime::CAPABILITY_STATUS_DISABLED_BY_PARENT,
            TrackingCapabilityAvailabilityState::Unavailable,
            TrackingCapabilityAvailabilityState::Unavailable,
            false,
            vec![constants::tracking_runtime::REASON_DISABLED_BY_PARENT],
        )),
        TrackingRuntimeServiceState::AdapterError => Some(tracking_capability_status_decision(
            constants::tracking_runtime::CAPABILITY_STATUS_ADAPTER_ERROR,
            TrackingCapabilityAvailabilityState::Unavailable,
            TrackingCapabilityAvailabilityState::Unavailable,
            true,
            vec![constants::tracking_runtime::REASON_ADAPTER_ERROR],
        )),
        TrackingRuntimeServiceState::ServiceDisabled => Some(tracking_capability_status_decision(
            constants::tracking_runtime::CAPABILITY_STATUS_SERVICE_DISABLED,
            TrackingCapabilityAvailabilityState::Unavailable,
            TrackingCapabilityAvailabilityState::Unavailable,
            true,
            vec![constants::tracking_runtime::REASON_SERVICE_DISABLED],
        )),
        TrackingRuntimeServiceState::Unavailable => Some(tracking_capability_status_decision(
            constants::tracking_runtime::CAPABILITY_STATUS_UNAVAILABLE,
            TrackingCapabilityAvailabilityState::Unavailable,
            TrackingCapabilityAvailabilityState::Unavailable,
            true,
            vec![constants::tracking_runtime::REASON_TRACKING_RUNTIME_UNAVAILABLE],
        )),
        TrackingRuntimeServiceState::Running => None,
    }
}

fn capability_status_for_device_status(
    device_status: &'static str,
) -> Option<TrackingCapabilityStatusDecision> {
    if device_status == constants::tracking_runtime::DEVICE_STATUS_OFFLINE_LAST_KNOWN_ONLY {
        return Some(tracking_capability_status_decision(
            constants::tracking_runtime::CAPABILITY_STATUS_OFFLINE_LAST_KNOWN_ONLY,
            TrackingCapabilityAvailabilityState::Limited,
            TrackingCapabilityAvailabilityState::Limited,
            false,
            vec![constants::tracking_runtime::REASON_TRACKING_HEARTBEAT_STALE],
        ));
    }
    if device_status == constants::tracking_runtime::DEVICE_STATUS_BATTERY_THROTTLED {
        return Some(tracking_capability_status_decision(
            constants::tracking_runtime::CAPABILITY_STATUS_BATTERY_THROTTLED,
            TrackingCapabilityAvailabilityState::Limited,
            TrackingCapabilityAvailabilityState::Limited,
            false,
            vec![constants::tracking_runtime::REASON_BATTERY_THROTTLED],
        ));
    }
    None
}

fn granted_foreground_capability_status_decision(
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

fn granted_background_capability_status_decision(
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
