use std::path::PathBuf;

use ocentra_parent_agent_protocol::activity::ActivityEvent;

use super::{
    app_game_journal_sqlite_ingest::AppGameJournalSqliteIngestError,
    app_game_windows_inventory::WindowsInstalledAppInventoryRecord,
    app_game_windows_registry_export::{
        collect_records_from_registry_export_path, registry_export_paths_from_roots,
    },
    app_game_windows_registry_record::journal_events_from_records,
};

#[cfg(windows)]
use super::app_game_windows_registry_live::registry_install_entries;
#[cfg(windows)]
use super::app_game_windows_registry_record::record_from_registry_entry;

#[derive(Debug, PartialEq, Eq)]
pub enum AppGameLiveRegistryInventorySourceError {
    RegistryInventoryJournalEventRejected,
}

impl From<AppGameJournalSqliteIngestError> for AppGameLiveRegistryInventorySourceError {
    fn from(_: AppGameJournalSqliteIngestError) -> Self {
        Self::RegistryInventoryJournalEventRejected
    }
}

pub fn live_windows_registry_inventory_records_from_roots(
    observed_at: &str,
    roots: &[PathBuf],
    limit: usize,
) -> Vec<WindowsInstalledAppInventoryRecord> {
    let mut records = Vec::new();
    for path in registry_export_paths_from_roots(roots, limit) {
        collect_records_from_registry_export_path(observed_at, &path, limit, &mut records);
        if records.len() >= limit {
            break;
        }
    }
    records
}

pub fn live_windows_registry_inventory_journal_events_with_limit(
    device_id: &str,
    platform: &str,
    observed_at: &str,
    limit: usize,
) -> Result<Vec<ActivityEvent>, AppGameLiveRegistryInventorySourceError> {
    let records = live_windows_registry_inventory_records(observed_at, limit);
    journal_events_from_records(device_id, platform, &records)
}

pub fn live_windows_registry_inventory_journal_events_from_roots(
    device_id: &str,
    platform: &str,
    observed_at: &str,
    roots: &[PathBuf],
    limit: usize,
) -> Result<Vec<ActivityEvent>, AppGameLiveRegistryInventorySourceError> {
    let records = live_windows_registry_inventory_records_from_roots(observed_at, roots, limit);
    journal_events_from_records(device_id, platform, &records)
}

#[cfg(windows)]
fn live_windows_registry_inventory_records(
    observed_at: &str,
    limit: usize,
) -> Vec<WindowsInstalledAppInventoryRecord> {
    registry_install_entries(limit)
        .iter()
        .map(|entry| record_from_registry_entry(observed_at, entry))
        .collect()
}

#[cfg(not(windows))]
fn live_windows_registry_inventory_records(
    _observed_at: &str,
    _limit: usize,
) -> Vec<WindowsInstalledAppInventoryRecord> {
    Vec::new()
}
