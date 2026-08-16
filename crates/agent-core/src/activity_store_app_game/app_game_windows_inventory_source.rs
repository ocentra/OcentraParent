use std::{
    fs,
    path::{Path, PathBuf},
};

use base64::prelude::{Engine as _, BASE64_URL_SAFE_NO_PAD};
use ocentra_parent_agent_protocol::activity::ActivityEvent;
use ocentra_parent_agent_protocol::app_game::{
    APP_GAME_CONFIDENCE_SHORTCUT_INVENTORY, APP_GAME_DESKTOP_ENTRY_ID_PREFIX,
    APP_GAME_INVENTORY_CUSTODY_LOCAL_AGENT, APP_GAME_INVENTORY_ENTRY_ID_PREFIX,
    APP_GAME_INVENTORY_SOURCE_SHORTCUT, APP_GAME_INVENTORY_STATE_INSTALLED,
    APP_GAME_WINDOWS_SHORTCUT_EXTENSION,
};
#[cfg(windows)]
use ocentra_parent_agent_protocol::app_game::{
    APP_GAME_WINDOWS_PATH_MICROSOFT, APP_GAME_WINDOWS_PATH_PROGRAMS,
    APP_GAME_WINDOWS_PATH_START_MENU, APP_GAME_WINDOWS_PATH_WINDOWS,
};
use sha2::{Digest, Sha256};

use super::{
    app_game_journal_sqlite_ingest::{
        app_game_inventory_journal_event, AppGameJournalSqliteIngestError,
    },
    app_game_windows_inventory::{
        windows_installed_inventory_rows_from_records, WindowsInstalledAppInventoryRecord,
    },
};

#[derive(Debug, PartialEq, Eq)]
pub enum AppGameLiveInventorySourceError {
    InventoryJournalEventRejected,
}

impl From<AppGameJournalSqliteIngestError> for AppGameLiveInventorySourceError {
    fn from(_: AppGameJournalSqliteIngestError) -> Self {
        Self::InventoryJournalEventRejected
    }
}

pub fn live_windows_inventory_records_from_roots(
    observed_at: &str,
    roots: &[PathBuf],
    limit: usize,
) -> Vec<WindowsInstalledAppInventoryRecord> {
    shortcut_paths_from_roots(roots, limit)
        .iter()
        .filter_map(|path| record_from_shortcut_path(observed_at, path))
        .collect()
}

pub fn live_windows_inventory_journal_events_with_limit(
    device_id: &str,
    platform: &str,
    observed_at: &str,
    limit: usize,
) -> Result<Vec<ActivityEvent>, AppGameLiveInventorySourceError> {
    let records = live_windows_inventory_records(observed_at, limit);
    journal_events_from_records(device_id, platform, &records)
}

pub fn live_windows_inventory_journal_events_from_roots(
    device_id: &str,
    platform: &str,
    observed_at: &str,
    roots: &[PathBuf],
    limit: usize,
) -> Result<Vec<ActivityEvent>, AppGameLiveInventorySourceError> {
    let records = live_windows_inventory_records_from_roots(observed_at, roots, limit);
    journal_events_from_records(device_id, platform, &records)
}

#[cfg(windows)]
fn live_windows_inventory_records(
    observed_at: &str,
    limit: usize,
) -> Vec<WindowsInstalledAppInventoryRecord> {
    let roots = live_windows_inventory_roots();
    live_windows_inventory_records_from_roots(observed_at, &roots, limit)
}

#[cfg(not(windows))]
fn live_windows_inventory_records(
    _observed_at: &str,
    _limit: usize,
) -> Vec<WindowsInstalledAppInventoryRecord> {
    Vec::new()
}

#[cfg(windows)]
fn live_windows_inventory_roots() -> Vec<PathBuf> {
    [
        constants::env_var::PROGRAM_DATA,
        constants::env_var::APP_DATA,
    ]
    .iter()
    .filter_map(std::env::var_os)
    .map(start_menu_programs_root)
    .collect()
}

#[cfg(windows)]
fn start_menu_programs_root(root: std::ffi::OsString) -> PathBuf {
    let mut path = PathBuf::from(root);
    path.push(APP_GAME_WINDOWS_PATH_MICROSOFT);
    path.push(APP_GAME_WINDOWS_PATH_WINDOWS);
    path.push(APP_GAME_WINDOWS_PATH_START_MENU);
    path.push(APP_GAME_WINDOWS_PATH_PROGRAMS);
    path
}

fn journal_events_from_records(
    device_id: &str,
    platform: &str,
    records: &[WindowsInstalledAppInventoryRecord],
) -> Result<Vec<ActivityEvent>, AppGameLiveInventorySourceError> {
    let rows = windows_installed_inventory_rows_from_records(records);
    rows.iter()
        .map(|row| app_game_inventory_journal_event(device_id, platform, row).map_err(Into::into))
        .collect()
}

fn shortcut_paths_from_roots(roots: &[PathBuf], limit: usize) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    for root in roots {
        collect_shortcut_paths(root, limit, &mut paths);
        if paths.len() >= limit {
            break;
        }
    }
    paths
}

fn collect_shortcut_paths(root: &Path, limit: usize, paths: &mut Vec<PathBuf>) {
    if paths.len() >= limit {
        return;
    }
    let Ok(entries) = fs::read_dir(root) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let Ok(file_type) = entry.file_type() else {
            continue;
        };
        if file_type.is_dir() {
            collect_shortcut_paths(&path, limit, paths);
        } else if is_shortcut_path(&path) {
            paths.push(path);
        }
        if paths.len() >= limit {
            break;
        }
    }
}

fn is_shortcut_path(path: &Path) -> bool {
    path.extension().is_some_and(|extension| {
        extension
            .to_string_lossy()
            .eq_ignore_ascii_case(APP_GAME_WINDOWS_SHORTCUT_EXTENSION)
    })
}

fn record_from_shortcut_path(
    observed_at: &str,
    path: &Path,
) -> Option<WindowsInstalledAppInventoryRecord> {
    let display_label = display_label(path)?;
    let source_ref = opaque_ref(APP_GAME_INVENTORY_ENTRY_ID_PREFIX, path);
    Some(WindowsInstalledAppInventoryRecord {
        observed_at: observed_at.to_string(),
        source_kind: APP_GAME_INVENTORY_SOURCE_SHORTCUT.to_string(),
        source_ref,
        custody_state: APP_GAME_INVENTORY_CUSTODY_LOCAL_AGENT.to_string(),
        display_label,
        identity_id: None,
        package_id: None,
        bundle_id: None,
        app_user_model_id: None,
        desktop_entry_id: Some(opaque_ref(APP_GAME_DESKTOP_ENTRY_ID_PREFIX, path)),
        executable_path_ref: None,
        launcher_ref: None,
        launcher_app_id: None,
        launcher_manifest_id: None,
        store_id: None,
        catalog_ref: None,
        inventory_state: APP_GAME_INVENTORY_STATE_INSTALLED.to_string(),
        confidence: APP_GAME_CONFIDENCE_SHORTCUT_INVENTORY,
        evidence: Vec::new(),
    })
}

fn display_label(path: &Path) -> Option<String> {
    let label = path.file_stem()?.to_string_lossy().into_owned();
    if label.is_empty() {
        return None;
    }
    Some(label)
}

fn opaque_ref(prefix: &str, path: &Path) -> String {
    let digest = Sha256::digest(path.to_string_lossy().as_bytes());
    let mut reference = String::from(prefix);
    reference.push_str(&BASE64_URL_SAFE_NO_PAD.encode(digest));
    reference
}
#[cfg(target_os = "windows")]
use ocentra_parent_agent_protocol::constants;
