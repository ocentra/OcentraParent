use super::{
    constants, tracking_capability_status_decision, TrackingCapabilityAvailabilityState,
    TrackingCapabilityStatusDecision, TrackingRuntimeServiceState,
};

pub(super) fn capability_status_for_service_state(
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

pub(super) fn capability_status_for_device_status(
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
