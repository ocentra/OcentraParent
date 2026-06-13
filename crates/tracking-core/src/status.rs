use ocentra_evidence::ManualReviewState;
use ocentra_parent_agent_protocol::{constants, TrackingChildDeviceId};

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

pub fn evaluate_tracking_device_status(
    input: TrackingDeviceStatusInput,
) -> TrackingDeviceStatusDecision {
    match input.service_state {
        TrackingRuntimeServiceState::DisabledByParent => {
            return TrackingDeviceStatusDecision {
                device_status: constants::tracking_runtime::DEVICE_STATUS_SERVICE_DISABLED,
                manual_review_state: ManualReviewState::NotRequired,
                degraded_reasons: vec![constants::tracking_runtime::REASON_DISABLED_BY_PARENT],
            };
        }
        TrackingRuntimeServiceState::ServiceDisabled => {
            return TrackingDeviceStatusDecision {
                device_status: constants::tracking_runtime::DEVICE_STATUS_SERVICE_DISABLED,
                manual_review_state: ManualReviewState::Required,
                degraded_reasons: vec![constants::tracking_runtime::REASON_SERVICE_DISABLED],
            };
        }
        TrackingRuntimeServiceState::Unavailable => {
            return TrackingDeviceStatusDecision {
                device_status: constants::tracking_runtime::DEVICE_STATUS_UNAVAILABLE,
                manual_review_state: ManualReviewState::Required,
                degraded_reasons: vec![
                    constants::tracking_runtime::REASON_TRACKING_RUNTIME_UNAVAILABLE,
                ],
            };
        }
        TrackingRuntimeServiceState::AdapterError => {
            return TrackingDeviceStatusDecision {
                device_status: constants::tracking_runtime::DEVICE_STATUS_UNAVAILABLE,
                manual_review_state: ManualReviewState::Required,
                degraded_reasons: vec![constants::tracking_runtime::REASON_ADAPTER_ERROR],
            };
        }
        TrackingRuntimeServiceState::Running => {}
    }

    let mut offline_reasons = Vec::new();
    if input.last_heartbeat_age_seconds > TRACKING_HEARTBEAT_OFFLINE_AFTER_SECONDS {
        offline_reasons.push(constants::tracking_runtime::REASON_TRACKING_HEARTBEAT_STALE);
    }
    if input.last_location_sample_age_seconds > TRACKING_LOCATION_OFFLINE_AFTER_SECONDS {
        offline_reasons.push(constants::tracking_runtime::REASON_LAST_LOCATION_SAMPLE_STALE);
    }
    if input.connectivity_state == TrackingConnectivityState::Offline {
        offline_reasons.push(constants::tracking_runtime::REASON_CONNECTIVITY_OFFLINE);
    }
    if input.radio_state == TrackingRadioState::Disabled {
        offline_reasons.push(constants::tracking_runtime::REASON_RADIO_DISABLED);
    }
    if !offline_reasons.is_empty() {
        return TrackingDeviceStatusDecision {
            device_status: constants::tracking_runtime::DEVICE_STATUS_OFFLINE_LAST_KNOWN_ONLY,
            manual_review_state: ManualReviewState::Required,
            degraded_reasons: offline_reasons,
        };
    }

    if input.pending_upload_count > 0 {
        let mut degraded_reasons = vec![constants::tracking_runtime::REASON_PENDING_UPLOAD_BACKLOG];
        if input.last_parent_sync_age_seconds > TRACKING_PARENT_SYNC_LATE_AFTER_SECONDS {
            degraded_reasons.push(constants::tracking_runtime::REASON_PARENT_SYNC_LATE);
        }
        return TrackingDeviceStatusDecision {
            device_status: constants::tracking_runtime::DEVICE_STATUS_PENDING_UPLOAD,
            manual_review_state: ManualReviewState::NotRequired,
            degraded_reasons,
        };
    }

    let heartbeat_stale = input.last_heartbeat_age_seconds > TRACKING_HEARTBEAT_STALE_AFTER_SECONDS;
    let location_stale =
        input.last_location_sample_age_seconds > TRACKING_LOCATION_STALE_AFTER_SECONDS;
    let parent_sync_late =
        input.last_parent_sync_age_seconds > TRACKING_PARENT_SYNC_LATE_AFTER_SECONDS;
    if heartbeat_stale || location_stale || parent_sync_late {
        let mut degraded_reasons = Vec::new();
        if heartbeat_stale {
            degraded_reasons.push(constants::tracking_runtime::REASON_TRACKING_HEARTBEAT_STALE);
        }
        if location_stale {
            degraded_reasons.push(constants::tracking_runtime::REASON_LAST_LOCATION_SAMPLE_STALE);
        }
        if parent_sync_late {
            degraded_reasons.push(constants::tracking_runtime::REASON_PARENT_SYNC_LATE);
        }
        return TrackingDeviceStatusDecision {
            device_status: constants::tracking_runtime::DEVICE_STATUS_STALE,
            manual_review_state: ManualReviewState::NotRequired,
            degraded_reasons,
        };
    }

    let low_battery = input
        .battery_percentage
        .is_some_and(|percent| percent <= TRACKING_LOW_BATTERY_THRESHOLD_PERCENT)
        && input.charging_state == TrackingChargingState::Discharging;
    if input.low_power_mode_state == TrackingLowPowerModeState::Active
        || low_battery
        || input.connectivity_state == TrackingConnectivityState::Metered
    {
        let mut degraded_reasons = vec![constants::tracking_runtime::REASON_BATTERY_THROTTLED];
        if input.low_power_mode_state == TrackingLowPowerModeState::Active {
            degraded_reasons.push(constants::tracking_runtime::REASON_LOW_POWER_MODE);
        }
        if low_battery {
            degraded_reasons.push(constants::tracking_runtime::REASON_BATTERY_LOW);
        }
        if input.connectivity_state == TrackingConnectivityState::Metered {
            degraded_reasons.push(constants::tracking_runtime::REASON_CONNECTIVITY_METERED);
        }
        return TrackingDeviceStatusDecision {
            device_status: constants::tracking_runtime::DEVICE_STATUS_BATTERY_THROTTLED,
            manual_review_state: ManualReviewState::NotRequired,
            degraded_reasons,
        };
    }

    TrackingDeviceStatusDecision {
        device_status: constants::tracking_runtime::DEVICE_STATUS_LIVE,
        manual_review_state: ManualReviewState::NotRequired,
        degraded_reasons: Vec::new(),
    }
}

pub fn evaluate_tracking_capability_status(
    input: TrackingCapabilityStatusInput,
) -> TrackingCapabilityStatusDecision {
    match input.service_state {
        TrackingRuntimeServiceState::DisabledByParent => {
            return TrackingCapabilityStatusDecision {
                capability_status:
                    constants::tracking_runtime::CAPABILITY_STATUS_DISABLED_BY_PARENT,
                foreground_availability_state: TrackingCapabilityAvailabilityState::Unavailable,
                background_availability_state: TrackingCapabilityAvailabilityState::Unavailable,
                manual_action_required: false,
                degraded_reasons: vec![constants::tracking_runtime::REASON_DISABLED_BY_PARENT],
            };
        }
        TrackingRuntimeServiceState::AdapterError => {
            return TrackingCapabilityStatusDecision {
                capability_status: constants::tracking_runtime::CAPABILITY_STATUS_ADAPTER_ERROR,
                foreground_availability_state: TrackingCapabilityAvailabilityState::Unavailable,
                background_availability_state: TrackingCapabilityAvailabilityState::Unavailable,
                manual_action_required: true,
                degraded_reasons: vec![constants::tracking_runtime::REASON_ADAPTER_ERROR],
            };
        }
        TrackingRuntimeServiceState::ServiceDisabled => {
            return TrackingCapabilityStatusDecision {
                capability_status: constants::tracking_runtime::CAPABILITY_STATUS_SERVICE_DISABLED,
                foreground_availability_state: TrackingCapabilityAvailabilityState::Unavailable,
                background_availability_state: TrackingCapabilityAvailabilityState::Unavailable,
                manual_action_required: true,
                degraded_reasons: vec![constants::tracking_runtime::REASON_SERVICE_DISABLED],
            };
        }
        TrackingRuntimeServiceState::Unavailable => {
            return TrackingCapabilityStatusDecision {
                capability_status: constants::tracking_runtime::CAPABILITY_STATUS_UNAVAILABLE,
                foreground_availability_state: TrackingCapabilityAvailabilityState::Unavailable,
                background_availability_state: TrackingCapabilityAvailabilityState::Unavailable,
                manual_action_required: true,
                degraded_reasons: vec![
                    constants::tracking_runtime::REASON_TRACKING_RUNTIME_UNAVAILABLE,
                ],
            };
        }
        TrackingRuntimeServiceState::Running => {}
    }

    if input.platform_state == TrackingPlatformState::Unsupported {
        return TrackingCapabilityStatusDecision {
            capability_status: constants::tracking_runtime::CAPABILITY_STATUS_PLATFORM_UNSUPPORTED,
            foreground_availability_state: TrackingCapabilityAvailabilityState::Unavailable,
            background_availability_state: TrackingCapabilityAvailabilityState::Unavailable,
            manual_action_required: false,
            degraded_reasons: vec![constants::tracking_runtime::REASON_PLATFORM_UNSUPPORTED],
        };
    }

    if input.device_status == constants::tracking_runtime::DEVICE_STATUS_OFFLINE_LAST_KNOWN_ONLY {
        return TrackingCapabilityStatusDecision {
            capability_status:
                constants::tracking_runtime::CAPABILITY_STATUS_OFFLINE_LAST_KNOWN_ONLY,
            foreground_availability_state: TrackingCapabilityAvailabilityState::Limited,
            background_availability_state: TrackingCapabilityAvailabilityState::Limited,
            manual_action_required: false,
            degraded_reasons: vec![constants::tracking_runtime::REASON_TRACKING_HEARTBEAT_STALE],
        };
    }
    if input.device_status == constants::tracking_runtime::DEVICE_STATUS_BATTERY_THROTTLED {
        return TrackingCapabilityStatusDecision {
            capability_status: constants::tracking_runtime::CAPABILITY_STATUS_BATTERY_THROTTLED,
            foreground_availability_state: TrackingCapabilityAvailabilityState::Limited,
            background_availability_state: TrackingCapabilityAvailabilityState::Limited,
            manual_action_required: false,
            degraded_reasons: vec![constants::tracking_runtime::REASON_BATTERY_THROTTLED],
        };
    }

    match input.permission_state {
        TrackingPermissionState::ManualRequired => TrackingCapabilityStatusDecision {
            capability_status: constants::tracking_runtime::CAPABILITY_STATUS_MANUAL_REQUIRED,
            foreground_availability_state: TrackingCapabilityAvailabilityState::Unavailable,
            background_availability_state: TrackingCapabilityAvailabilityState::Unavailable,
            manual_action_required: true,
            degraded_reasons: vec![
                constants::tracking_runtime::REASON_MANAGED_DEVICE_PROOF_REQUIRED,
            ],
        },
        TrackingPermissionState::Denied
        | TrackingPermissionState::Restricted
        | TrackingPermissionState::NotRequested => TrackingCapabilityStatusDecision {
            capability_status: constants::tracking_runtime::CAPABILITY_STATUS_PERMISSION_REQUIRED,
            foreground_availability_state: TrackingCapabilityAvailabilityState::Unavailable,
            background_availability_state: TrackingCapabilityAvailabilityState::Unavailable,
            manual_action_required: true,
            degraded_reasons: vec![
                constants::tracking_runtime::REASON_FOREGROUND_PERMISSION_REQUIRED,
            ],
        },
        TrackingPermissionState::ServiceDisabled => TrackingCapabilityStatusDecision {
            capability_status: constants::tracking_runtime::CAPABILITY_STATUS_SERVICE_DISABLED,
            foreground_availability_state: TrackingCapabilityAvailabilityState::Unavailable,
            background_availability_state: TrackingCapabilityAvailabilityState::Unavailable,
            manual_action_required: true,
            degraded_reasons: vec![constants::tracking_runtime::REASON_SERVICE_DISABLED],
        },
        TrackingPermissionState::Unsupported | TrackingPermissionState::Unavailable => {
            TrackingCapabilityStatusDecision {
                capability_status: constants::tracking_runtime::CAPABILITY_STATUS_UNAVAILABLE,
                foreground_availability_state: TrackingCapabilityAvailabilityState::Unavailable,
                background_availability_state: TrackingCapabilityAvailabilityState::Unavailable,
                manual_action_required: true,
                degraded_reasons: vec![
                    constants::tracking_runtime::REASON_TRACKING_RUNTIME_UNAVAILABLE,
                ],
            }
        }
        TrackingPermissionState::ApproximateOnly => TrackingCapabilityStatusDecision {
            capability_status: constants::tracking_runtime::CAPABILITY_STATUS_APPROXIMATE_ONLY,
            foreground_availability_state: TrackingCapabilityAvailabilityState::Limited,
            background_availability_state: TrackingCapabilityAvailabilityState::Unavailable,
            manual_action_required: false,
            degraded_reasons: vec![
                constants::tracking_runtime::REASON_PRECISE_LOCATION_UNAVAILABLE,
            ],
        },
        TrackingPermissionState::GrantedForeground => {
            if input.background_capability_state
                == TrackingBackgroundCapabilityState::ManagedDeviceRequired
                || input.platform_state == TrackingPlatformState::ManagedDevice
            {
                return TrackingCapabilityStatusDecision {
                    capability_status:
                        constants::tracking_runtime::CAPABILITY_STATUS_MANUAL_REQUIRED,
                    foreground_availability_state: TrackingCapabilityAvailabilityState::Available,
                    background_availability_state: TrackingCapabilityAvailabilityState::Unavailable,
                    manual_action_required: true,
                    degraded_reasons: vec![
                        constants::tracking_runtime::REASON_MANAGED_DEVICE_PROOF_REQUIRED,
                    ],
                };
            }

            if input.strict_background_required
                || input.background_capability_state
                    == TrackingBackgroundCapabilityState::PermissionRequired
            {
                return TrackingCapabilityStatusDecision {
                    capability_status: constants::tracking_runtime::CAPABILITY_STATUS_BACKGROUND_PERMISSION_REQUIRED,
                    foreground_availability_state: TrackingCapabilityAvailabilityState::Available,
                    background_availability_state: TrackingCapabilityAvailabilityState::Unavailable,
                    manual_action_required: true,
                    degraded_reasons: vec![constants::tracking_runtime::REASON_BACKGROUND_PERMISSION_REQUIRED],
                };
            }

            if input.background_capability_state == TrackingBackgroundCapabilityState::Unsupported {
                return TrackingCapabilityStatusDecision {
                    capability_status:
                        constants::tracking_runtime::CAPABILITY_STATUS_FOREGROUND_ONLY,
                    foreground_availability_state: TrackingCapabilityAvailabilityState::Available,
                    background_availability_state: TrackingCapabilityAvailabilityState::Unavailable,
                    manual_action_required: false,
                    degraded_reasons: vec![
                        constants::tracking_runtime::REASON_BACKGROUND_PLATFORM_UNSUPPORTED,
                    ],
                };
            }

            TrackingCapabilityStatusDecision {
                capability_status: constants::tracking_runtime::CAPABILITY_STATUS_FOREGROUND_ONLY,
                foreground_availability_state: TrackingCapabilityAvailabilityState::Available,
                background_availability_state: TrackingCapabilityAvailabilityState::Unavailable,
                manual_action_required: false,
                degraded_reasons: Vec::new(),
            }
        }
        TrackingPermissionState::GrantedBackground => {
            let (
                capability_status,
                background_availability_state,
                manual_action_required,
                degraded_reasons,
            ) = match input.background_capability_state {
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

            TrackingCapabilityStatusDecision {
                capability_status,
                foreground_availability_state: TrackingCapabilityAvailabilityState::Available,
                background_availability_state,
                manual_action_required,
                degraded_reasons,
            }
        }
    }
}
