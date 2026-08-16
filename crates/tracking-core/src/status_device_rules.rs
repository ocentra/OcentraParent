use super::status_device_battery_rules::battery_throttled_device_reasons;
use super::status_device_reason_rules::{
    offline_device_reasons, pending_upload_device_status_decision, stale_device_reasons,
};
use super::status_device_service_rules::device_status_for_service_state;
use super::{
    constants, tracking_device_status_decision, ManualReviewState, TrackingDeviceStatusDecision,
    TrackingDeviceStatusInput,
};

pub(super) fn evaluate_tracking_device_status(
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
