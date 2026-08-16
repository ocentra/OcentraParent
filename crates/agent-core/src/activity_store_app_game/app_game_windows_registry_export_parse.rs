use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use ocentra_parent_agent_protocol::app_game::{
    APP_GAME_WINDOWS_REGISTRY_DWORD_PREFIX, APP_GAME_WINDOWS_REGISTRY_UNINSTALL_PATH,
    APP_GAME_WINDOWS_REGISTRY_WOW6432_UNINSTALL_PATH,
};
use ocentra_parent_agent_protocol::constants;

use crate::activity_store_app_game::app_game_windows_inventory::WindowsInstalledAppInventoryRecord;
use crate::activity_store_app_game::app_game_windows_registry_record::{
    install_entry_from_values, record_from_registry_entry, WindowsRegistryInstallEntry,
};

pub(crate) fn collect_records_from_registry_export_path(
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
