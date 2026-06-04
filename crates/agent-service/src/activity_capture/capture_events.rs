#[cfg(test)]
use std::path::{Path, PathBuf};

use ocentra_parent_agent_core::{
    foreground_window_event, network_snapshot_events, process_snapshot_events,
};
#[cfg(test)]
use ocentra_parent_agent_protocol::ActivityIngestStatus;
use ocentra_parent_agent_protocol::{constants, ActivityEvent};

#[cfg(test)]
use super::record_activity_events_to_paths;
use super::{app_game, ActivityCaptureError};

pub(super) fn activity_capture_events(
    observed_at: &str,
    process_limit: usize,
    network_limit: usize,
) -> Result<Vec<ActivityEvent>, ActivityCaptureError> {
    let inventory_events = app_game::live_inventory_events(
        observed_at,
        constants::activity_capture::APP_GAME_INVENTORY_SNAPSHOT_LIMIT,
    )?;
    let store_package_events = app_game::live_store_package_events(
        observed_at,
        constants::activity_capture::APP_GAME_INVENTORY_SNAPSHOT_LIMIT,
    )?;
    activity_capture_events_with_inventory(
        observed_at,
        process_limit,
        network_limit,
        inventory_events,
        store_package_events,
    )
}

#[cfg(test)]
fn activity_capture_events_with_inventory_roots(
    observed_at: &str,
    process_limit: usize,
    network_limit: usize,
    inventory_roots: &[PathBuf],
) -> Result<Vec<ActivityEvent>, ActivityCaptureError> {
    let inventory_events = app_game::live_inventory_events_from_roots(
        observed_at,
        inventory_roots,
        constants::activity_capture::APP_GAME_INVENTORY_SNAPSHOT_LIMIT,
    )?;
    activity_capture_events_with_inventory_and_store_package_events(
        observed_at,
        process_limit,
        network_limit,
        inventory_events,
        Vec::new(),
    )
}

#[cfg(test)]
fn activity_capture_events_with_store_package_roots(
    observed_at: &str,
    process_limit: usize,
    network_limit: usize,
    store_package_roots: &[PathBuf],
) -> Result<Vec<ActivityEvent>, ActivityCaptureError> {
    let store_package_events = app_game::live_store_package_events_from_roots(
        observed_at,
        store_package_roots,
        constants::activity_capture::APP_GAME_INVENTORY_SNAPSHOT_LIMIT,
    )?;
    activity_capture_events_with_inventory(
        observed_at,
        process_limit,
        network_limit,
        Vec::new(),
        store_package_events,
    )
}

fn activity_capture_events_with_inventory(
    observed_at: &str,
    process_limit: usize,
    network_limit: usize,
    inventory_events: Vec<ActivityEvent>,
    store_package_events: Vec<ActivityEvent>,
) -> Result<Vec<ActivityEvent>, ActivityCaptureError> {
    activity_capture_events_with_inventory_and_store_package_events(
        observed_at,
        process_limit,
        network_limit,
        inventory_events,
        store_package_events,
    )
}

fn activity_capture_events_with_inventory_and_store_package_events(
    observed_at: &str,
    process_limit: usize,
    network_limit: usize,
    inventory_events: Vec<ActivityEvent>,
    store_package_events: Vec<ActivityEvent>,
) -> Result<Vec<ActivityEvent>, ActivityCaptureError> {
    let mut events = process_snapshot_events(observed_at, process_limit);
    events.push(foreground_window_event(observed_at));
    events.extend(network_snapshot_events(observed_at, network_limit));
    events.extend(app_game::live_process_events(observed_at, process_limit)?);
    events.extend(inventory_events);
    events.extend(store_package_events);
    if let Some(event) = app_game::live_foreground_event(observed_at)? {
        events.push(event);
    }
    Ok(events)
}

#[cfg(test)]
pub(crate) fn record_activity_capture_to_paths_at_with_inventory_roots(
    journal_path: &Path,
    key_path: &Path,
    store_path: &Path,
    process_limit: usize,
    network_limit: usize,
    observed_at: &str,
    inventory_roots: &[PathBuf],
) -> Result<ActivityIngestStatus, ActivityCaptureError> {
    let events = activity_capture_events_with_inventory_roots(
        observed_at,
        process_limit,
        network_limit,
        inventory_roots,
    )?;
    record_activity_events_to_paths(journal_path, key_path, store_path, &events)
}

#[cfg(test)]
pub(crate) fn record_activity_capture_to_paths_at_with_store_package_roots(
    journal_path: &Path,
    key_path: &Path,
    store_path: &Path,
    process_limit: usize,
    network_limit: usize,
    observed_at: &str,
    store_package_roots: &[PathBuf],
) -> Result<ActivityIngestStatus, ActivityCaptureError> {
    let events = activity_capture_events_with_store_package_roots(
        observed_at,
        process_limit,
        network_limit,
        store_package_roots,
    )?;
    record_activity_events_to_paths(journal_path, key_path, store_path, &events)
}
