use super::{
    constants, TrackingChargingState, TrackingConnectivityState, TrackingLowPowerModeState,
    TRACKING_LOW_BATTERY_THRESHOLD_PERCENT,
};

pub(super) fn battery_throttled_device_reasons(
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
