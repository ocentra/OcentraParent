use super::{
    constants, tracking_device_status_decision, ManualReviewState, TrackingConnectivityState,
    TrackingDeviceStatusDecision, TrackingRadioState, TRACKING_HEARTBEAT_OFFLINE_AFTER_SECONDS,
    TRACKING_HEARTBEAT_STALE_AFTER_SECONDS, TRACKING_LOCATION_OFFLINE_AFTER_SECONDS,
    TRACKING_LOCATION_STALE_AFTER_SECONDS, TRACKING_PARENT_SYNC_LATE_AFTER_SECONDS,
};

pub(super) fn offline_device_reasons(
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

pub(super) fn pending_upload_device_status_decision(
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

pub(super) fn stale_device_reasons(
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
