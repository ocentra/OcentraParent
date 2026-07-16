use super::{
    constants, tracking_device_status_decision, ManualReviewState, TrackingDeviceStatusDecision,
    TrackingRuntimeServiceState,
};

pub(super) fn device_status_for_service_state(
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
