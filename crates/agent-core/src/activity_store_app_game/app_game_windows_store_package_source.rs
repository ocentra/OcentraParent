use std::{
    fs,
    path::{Path, PathBuf},
};

use base64::prelude::{Engine as _, BASE64_URL_SAFE_NO_PAD};
use ocentra_parent_agent_protocol::activity::ActivityEvent;
#[cfg(windows)]
use ocentra_parent_agent_protocol::app_game::APP_GAME_WINDOWS_PATH_WINDOWS_APPS;
use ocentra_parent_agent_protocol::app_game::{
    APP_GAME_INVENTORY_ENTRY_ID_PREFIX, APP_GAME_WINDOWS_APPX_MANIFEST_FILE_NAME,
};
#[cfg(windows)]
use ocentra_parent_agent_protocol::constants;
use sha2::{Digest, Sha256};

use super::{
    app_game_journal_sqlite_ingest::{
        app_game_inventory_journal_event, AppGameJournalSqliteIngestError,
    },
    app_game_windows_store_inventory::{
        windows_store_inventory_rows_from_records, WindowsStorePackageInventoryRecord,
    },
    app_game_windows_store_package_manifest::record_from_manifest_xml,
};

#[derive(Debug, PartialEq, Eq)]
pub enum AppGameLiveStorePackageSourceError {
    StorePackageJournalEventRejected,
}

impl From<AppGameJournalSqliteIngestError> for AppGameLiveStorePackageSourceError {
    fn from(_: AppGameJournalSqliteIngestError) -> Self {
        Self::StorePackageJournalEventRejected
    }
}

pub fn live_windows_store_package_records_from_roots(
    observed_at: &str,
    roots: &[PathBuf],
    limit: usize,
) -> Vec<WindowsStorePackageInventoryRecord> {
    manifest_paths_from_roots(roots, limit)
        .iter()
        .filter_map(|path| record_from_manifest_path(observed_at, path))
        .collect()
}

pub fn live_windows_store_package_journal_events_with_limit(
    device_id: &str,
    platform: &str,
    observed_at: &str,
    limit: usize,
) -> Result<Vec<ActivityEvent>, AppGameLiveStorePackageSourceError> {
    let records = live_windows_store_package_records(observed_at, limit);
    journal_events_from_records(device_id, platform, &records)
}

pub fn live_windows_store_package_journal_events_from_roots(
    device_id: &str,
    platform: &str,
    observed_at: &str,
    roots: &[PathBuf],
    limit: usize,
) -> Result<Vec<ActivityEvent>, AppGameLiveStorePackageSourceError> {
    let records = live_windows_store_package_records_from_roots(observed_at, roots, limit);
    journal_events_from_records(device_id, platform, &records)
}

#[cfg(windows)]
fn live_windows_store_package_records(
    observed_at: &str,
    limit: usize,
) -> Vec<WindowsStorePackageInventoryRecord> {
    let roots = live_windows_store_package_roots();
    live_windows_store_package_records_from_roots(observed_at, &roots, limit)
}

#[cfg(not(windows))]
fn live_windows_store_package_records(
    _observed_at: &str,
    _limit: usize,
) -> Vec<WindowsStorePackageInventoryRecord> {
    Vec::new()
}

#[cfg(windows)]
fn live_windows_store_package_roots() -> Vec<PathBuf> {
    std::env::var_os(constants::env_var::PROGRAM_FILES)
        .map(windows_apps_root)
        .into_iter()
        .collect()
}

#[cfg(windows)]
fn windows_apps_root(root: std::ffi::OsString) -> PathBuf {
    let mut path = PathBuf::from(root);
    path.push(APP_GAME_WINDOWS_PATH_WINDOWS_APPS);
    path
}

fn journal_events_from_records(
    device_id: &str,
    platform: &str,
    records: &[WindowsStorePackageInventoryRecord],
) -> Result<Vec<ActivityEvent>, AppGameLiveStorePackageSourceError> {
    let rows = windows_store_inventory_rows_from_records(records);
    rows.iter()
        .map(|row| app_game_inventory_journal_event(device_id, platform, row).map_err(Into::into))
        .collect()
}

fn manifest_paths_from_roots(roots: &[PathBuf], limit: usize) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    for root in roots {
        collect_manifest_paths(root, limit, &mut paths);
        if paths.len() >= limit {
            break;
        }
    }
    paths
}

fn collect_manifest_paths(root: &Path, limit: usize, paths: &mut Vec<PathBuf>) {
    if paths.len() >= limit {
        return;
    }
    if is_manifest_path(root) {
        paths.push(root.to_path_buf());
        return;
    }
    let direct_manifest = root.join(APP_GAME_WINDOWS_APPX_MANIFEST_FILE_NAME);
    if direct_manifest.is_file() {
        paths.push(direct_manifest);
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
            let manifest = path.join(APP_GAME_WINDOWS_APPX_MANIFEST_FILE_NAME);
            if manifest.is_file() {
                paths.push(manifest);
            }
        } else if is_manifest_path(&path) {
            paths.push(path);
        }
        if paths.len() >= limit {
            break;
        }
    }
}

fn is_manifest_path(path: &Path) -> bool {
    path.file_name().is_some_and(|file_name| {
        file_name
            .to_string_lossy()
            .eq_ignore_ascii_case(APP_GAME_WINDOWS_APPX_MANIFEST_FILE_NAME)
    })
}

fn record_from_manifest_path(
    observed_at: &str,
    path: &Path,
) -> Option<WindowsStorePackageInventoryRecord> {
    let manifest = fs::read_to_string(path).ok()?;
    record_from_manifest_xml(observed_at, opaque_ref(path), &manifest)
}

fn opaque_ref(path: &Path) -> String {
    let digest = Sha256::digest(path.to_string_lossy().as_bytes());
    let mut reference = String::from(APP_GAME_INVENTORY_ENTRY_ID_PREFIX);
    reference.push_str(&BASE64_URL_SAFE_NO_PAD.encode(digest));
    reference
}
