#[cfg(test)]
use std::path::PathBuf;

use ocentra_parent_agent_core::{
    live_windows_foreground_window_journal_event, live_windows_inventory_journal_events_with_limit,
    live_windows_process_snapshot_journal_events_with_limit,
    live_windows_registry_inventory_journal_events_with_limit,
    live_windows_store_package_journal_events_with_limit,
};
#[cfg(test)]
use ocentra_parent_agent_core::{
    live_windows_inventory_journal_events_from_roots,
    live_windows_registry_inventory_journal_events_from_roots,
    live_windows_store_package_journal_events_from_roots,
};
use ocentra_parent_agent_protocol::{constants, ActivityEvent};

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

#[cfg(test)]
pub(super) fn live_inventory_events_from_roots(
    observed_at: &str,
    roots: &[PathBuf],
    limit: usize,
) -> Result<Vec<ActivityEvent>, ActivityCaptureError> {
    Ok(live_windows_inventory_journal_events_from_roots(
        constants::activity_surface::DEFAULT_DEVICE_ID,
        std::env::consts::OS,
        observed_at,
        roots,
        limit,
    )?)
}

#[cfg(test)]
pub(super) fn live_registry_inventory_events_from_roots(
    observed_at: &str,
    roots: &[PathBuf],
    limit: usize,
) -> Result<Vec<ActivityEvent>, ActivityCaptureError> {
    Ok(live_windows_registry_inventory_journal_events_from_roots(
        constants::activity_surface::DEFAULT_DEVICE_ID,
        std::env::consts::OS,
        observed_at,
        roots,
        limit,
    )?)
}

#[cfg(test)]
pub(super) fn live_store_package_events_from_roots(
    observed_at: &str,
    roots: &[PathBuf],
    limit: usize,
) -> Result<Vec<ActivityEvent>, ActivityCaptureError> {
    Ok(live_windows_store_package_journal_events_from_roots(
        constants::activity_surface::DEFAULT_DEVICE_ID,
        std::env::consts::OS,
        observed_at,
        roots,
        limit,
    )?)
}
