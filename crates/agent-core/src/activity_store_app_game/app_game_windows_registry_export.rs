use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

use ocentra_parent_agent_protocol::app_game::{
    APP_GAME_WINDOWS_REGISTRY_DWORD_PREFIX, APP_GAME_WINDOWS_REGISTRY_FILE_EXTENSION,
    APP_GAME_WINDOWS_REGISTRY_UNINSTALL_PATH, APP_GAME_WINDOWS_REGISTRY_WOW6432_UNINSTALL_PATH,
};
use ocentra_parent_agent_protocol::constants;

use super::{
    app_game_windows_inventory::WindowsInstalledAppInventoryRecord,
    app_game_windows_registry_record::{
        install_entry_from_values, record_from_registry_entry, WindowsRegistryInstallEntry,
    },
};

pub(super) fn collect_records_from_registry_export_path(
    observed_at: &str,
    path: &Path,
    limit: usize,
    records: &mut Vec<WindowsInstalledAppInventoryRecord>,
) {
    if records.len() >= limit {
        return;
    }
    let Ok(export) = fs::read_to_string(path) else {
        return;
    };
    for entry in registry_install_entries_from_export(path, &export) {
        records.push(record_from_registry_entry(observed_at, &entry));
        if records.len() >= limit {
            break;
        }
    }
}

pub(super) fn registry_export_paths_from_roots(roots: &[PathBuf], limit: usize) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    for root in roots {
        collect_registry_export_paths(root, limit, &mut paths);
        if paths.len() >= limit {
            break;
        }
    }
    paths
}

fn registry_install_entries_from_export(
    path: &Path,
    export: &str,
) -> Vec<WindowsRegistryInstallEntry> {
    let mut entries = Vec::new();
    let mut current_key = None;
    let mut current_values = BTreeMap::new();
    for line in export.lines() {
        let line = line.trim();
        if let Some(key) = registry_key_from_export_line(line) {
            push_current_entry(path, current_key.take(), &mut current_values, &mut entries);
            current_key = Some(key);
        } else if let Some((name, value)) = registry_value_from_export_line(line) {
            current_values.insert(name, value);
        }
    }
    push_current_entry(path, current_key, &mut current_values, &mut entries);
    entries
}

fn push_current_entry(
    path: &Path,
    key: Option<String>,
    values: &mut BTreeMap<String, String>,
    entries: &mut Vec<WindowsRegistryInstallEntry>,
) {
    if let Some(key) = key {
        let mut source_key_ref = path.to_string_lossy().into_owned();
        source_key_ref.push(constants::delimiter::COLON);
        source_key_ref.push_str(&key);
        if let Some(entry) = install_entry_from_values(source_key_ref, values) {
            entries.push(entry);
        }
    }
    values.clear();
}

fn collect_registry_export_paths(root: &Path, limit: usize, paths: &mut Vec<PathBuf>) {
    if paths.len() >= limit {
        return;
    }
    if is_registry_export_path(root) {
        paths.push(root.to_path_buf());
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
            collect_registry_export_paths(&path, limit, paths);
        } else if is_registry_export_path(&path) {
            paths.push(path);
        }
        if paths.len() >= limit {
            break;
        }
    }
}

fn is_registry_export_path(path: &Path) -> bool {
    path.extension().is_some_and(|extension| {
        extension
            .to_string_lossy()
            .eq_ignore_ascii_case(APP_GAME_WINDOWS_REGISTRY_FILE_EXTENSION)
    })
}

fn registry_key_from_export_line(line: &str) -> Option<String> {
    if !line.starts_with(constants::delimiter::OPEN_BRACKET)
        || !line.ends_with(constants::delimiter::CLOSE_BRACKET)
    {
        return None;
    }
    let key = line
        .trim_start_matches(constants::delimiter::OPEN_BRACKET)
        .trim_end_matches(constants::delimiter::CLOSE_BRACKET);
    if !is_uninstall_registry_key(key) {
        return None;
    }
    Some(key.to_string())
}

fn is_uninstall_registry_key(key: &str) -> bool {
    key.contains(APP_GAME_WINDOWS_REGISTRY_UNINSTALL_PATH)
        || key.contains(APP_GAME_WINDOWS_REGISTRY_WOW6432_UNINSTALL_PATH)
}

fn registry_value_from_export_line(line: &str) -> Option<(String, String)> {
    let line = line.strip_prefix(constants::delimiter::QUOTE)?;
    if let Some((name, value)) = registry_string_value_from_export_line(line) {
        return Some((name, value));
    }
    registry_dword_value_from_export_line(line)
}

fn registry_string_value_from_export_line(line: &str) -> Option<(String, String)> {
    let assignment = registry_string_assignment();
    let (name, value) = line.split_once(&assignment)?;
    Some((
        name.to_string(),
        value
            .trim_end_matches(constants::delimiter::QUOTE)
            .to_string(),
    ))
}

fn registry_dword_value_from_export_line(line: &str) -> Option<(String, String)> {
    let assignment = registry_dword_assignment();
    let (name, value) = line.split_once(&assignment)?;
    Some((name.to_string(), value.to_string()))
}

fn registry_string_assignment() -> String {
    let mut assignment = String::new();
    assignment.push(constants::delimiter::QUOTE);
    assignment.push(constants::delimiter::EQUALS);
    assignment.push(constants::delimiter::QUOTE);
    assignment
}

fn registry_dword_assignment() -> String {
    let mut assignment = String::new();
    assignment.push(constants::delimiter::QUOTE);
    assignment.push(constants::delimiter::EQUALS);
    assignment.push_str(APP_GAME_WINDOWS_REGISTRY_DWORD_PREFIX);
    assignment
}
