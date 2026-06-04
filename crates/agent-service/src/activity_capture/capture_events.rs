use std::{path::Path, path::PathBuf};

use ocentra_parent_agent_core::{
    foreground_window_event, network_snapshot_events, process_snapshot_events,
};
use ocentra_parent_agent_protocol::{constants, ActivityEvent, ActivityIngestStatus};

use super::{app_game, record_activity_events_to_paths, ActivityCaptureError};

pub(super) fn activity_capture_events(
    observed_at: &str,
    process_limit: usize,
    network_limit: usize,
) -> Result<Vec<ActivityEvent>, ActivityCaptureError> {
    activity_capture_events_with_inventory_roots(observed_at, process_limit, network_limit, None)
}

fn activity_capture_events_with_inventory_roots(
    observed_at: &str,
    process_limit: usize,
    network_limit: usize,
    inventory_roots: Option<&[PathBuf]>,
) -> Result<Vec<ActivityEvent>, ActivityCaptureError> {
    let mut events = process_snapshot_events(observed_at, process_limit);
    events.push(foreground_window_event(observed_at));
    events.extend(network_snapshot_events(observed_at, network_limit));
    events.extend(app_game::live_process_events(observed_at, process_limit)?);
    let inventory_events = if let Some(roots) = inventory_roots {
        app_game::live_inventory_events_from_roots(
            observed_at,
            roots,
            constants::activity_capture::APP_GAME_INVENTORY_SNAPSHOT_LIMIT,
        )?
    } else {
        app_game::live_inventory_events(
            observed_at,
            constants::activity_capture::APP_GAME_INVENTORY_SNAPSHOT_LIMIT,
        )?
    };
    events.extend(inventory_events);
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
        Some(inventory_roots),
    )?;
    record_activity_events_to_paths(journal_path, key_path, store_path, &events)
}
