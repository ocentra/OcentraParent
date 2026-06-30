#[cfg(windows)]
use ocentra_parent_agent_core::activity_store_app_game::{
    live_windows_foreground_window_journal_event, live_windows_inventory_journal_events_with_limit,
    live_windows_process_snapshot_journal_events_with_limit,
    live_windows_registry_inventory_journal_events_with_limit,
    live_windows_store_package_journal_events_with_limit,
};
use ocentra_parent_agent_protocol::activity::ActivityEvent;
#[cfg(windows)]
use ocentra_parent_agent_protocol::constants;

use super::ActivityCaptureError;

#[cfg(windows)]
pub(super) fn live_process_events(
    observed_at: &str,
    limit: usize,
) -> Result<Vec<ActivityEvent>, ActivityCaptureError> {
    Ok(live_windows_process_snapshot_journal_events_with_limit(
        constants::activity_surface::DEFAULT_DEVICE_ID,
        std::env::consts::OS,
        observed_at,
        limit,
    )?)
}

#[cfg(not(windows))]
pub(super) fn live_process_events(
    _observed_at: &str,
    _limit: usize,
) -> Result<Vec<ActivityEvent>, ActivityCaptureError> {
    Ok(Vec::new())
}

#[cfg(windows)]
pub(super) fn live_foreground_event(
    observed_at: &str,
) -> Result<Option<ActivityEvent>, ActivityCaptureError> {
    Ok(live_windows_foreground_window_journal_event(
        constants::activity_surface::DEFAULT_DEVICE_ID,
        std::env::consts::OS,
        observed_at,
    )?)
}

#[cfg(not(windows))]
pub(super) fn live_foreground_event(
    _observed_at: &str,
) -> Result<Option<ActivityEvent>, ActivityCaptureError> {
    Ok(None)
}

#[cfg(windows)]
pub(super) fn live_inventory_events(
    observed_at: &str,
    limit: usize,
) -> Result<Vec<ActivityEvent>, ActivityCaptureError> {
    Ok(live_windows_inventory_journal_events_with_limit(
        constants::activity_surface::DEFAULT_DEVICE_ID,
        std::env::consts::OS,
        observed_at,
        limit,
    )?)
}

#[cfg(not(windows))]
pub(super) fn live_inventory_events(
    _observed_at: &str,
    _limit: usize,
) -> Result<Vec<ActivityEvent>, ActivityCaptureError> {
    Ok(Vec::new())
}

#[cfg(windows)]
pub(super) fn live_store_package_events(
    observed_at: &str,
    limit: usize,
) -> Result<Vec<ActivityEvent>, ActivityCaptureError> {
    Ok(live_windows_store_package_journal_events_with_limit(
        constants::activity_surface::DEFAULT_DEVICE_ID,
        std::env::consts::OS,
        observed_at,
        limit,
    )?)
}

#[cfg(not(windows))]
pub(super) fn live_store_package_events(
    _observed_at: &str,
    _limit: usize,
) -> Result<Vec<ActivityEvent>, ActivityCaptureError> {
    Ok(Vec::new())
}

#[cfg(windows)]
pub(super) fn live_registry_inventory_events(
    observed_at: &str,
    limit: usize,
) -> Result<Vec<ActivityEvent>, ActivityCaptureError> {
    Ok(live_windows_registry_inventory_journal_events_with_limit(
        constants::activity_surface::DEFAULT_DEVICE_ID,
        std::env::consts::OS,
        observed_at,
        limit,
    )?)
}

#[cfg(not(windows))]
pub(super) fn live_registry_inventory_events(
    _observed_at: &str,
    _limit: usize,
) -> Result<Vec<ActivityEvent>, ActivityCaptureError> {
    Ok(Vec::new())
}
