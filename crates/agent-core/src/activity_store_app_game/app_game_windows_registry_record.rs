use std::collections::BTreeMap;

use base64::prelude::{Engine as _, BASE64_URL_SAFE_NO_PAD};
use ocentra_parent_agent_protocol::activity::ActivityEvent;
use ocentra_parent_agent_protocol::app_game::{
    APP_GAME_CONFIDENCE_OS_INSTALLED_RECORD, APP_GAME_EXECUTABLE_PATH_REF_PREFIX,
    APP_GAME_INVENTORY_CUSTODY_LOCAL_AGENT, APP_GAME_INVENTORY_ENTRY_ID_PREFIX,
    APP_GAME_INVENTORY_SOURCE_OS_INSTALLED_RECORD, APP_GAME_INVENTORY_STATE_INSTALLED,
    APP_GAME_WINDOWS_REGISTRY_DISPLAY_ICON_VALUE, APP_GAME_WINDOWS_REGISTRY_DISPLAY_NAME_VALUE,
    APP_GAME_WINDOWS_REGISTRY_DWORD_ENABLED_TEXT, APP_GAME_WINDOWS_REGISTRY_DWORD_ENABLED_VALUE,
    APP_GAME_WINDOWS_REGISTRY_INSTALL_LOCATION_VALUE,
    APP_GAME_WINDOWS_REGISTRY_QUIET_UNINSTALL_STRING_VALUE,
    APP_GAME_WINDOWS_REGISTRY_SYSTEM_COMPONENT_VALUE,
    APP_GAME_WINDOWS_REGISTRY_UNINSTALL_STRING_VALUE,
};
use ocentra_parent_agent_protocol::constants;
use sha2::{Digest, Sha256};

use super::{
    app_game_journal_sqlite_ingest::app_game_inventory_journal_event,
    app_game_windows_inventory::{
        windows_installed_inventory_rows_from_records, WindowsInstalledAppInventoryRecord,
    },
    app_game_windows_registry_source::AppGameLiveRegistryInventorySourceError,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct WindowsRegistryInstallEntry {
    pub(super) source_key_ref: String,
    display_name: String,
    executable_path: Option<String>,
}

pub(super) fn install_entry_from_values(
    source_key_ref: String,
    values: &BTreeMap<String, String>,
) -> Option<WindowsRegistryInstallEntry> {
    if is_hidden_system_component(values) {
        return None;
    }
    let display_name = non_empty_value(values, APP_GAME_WINDOWS_REGISTRY_DISPLAY_NAME_VALUE)?;
    Some(WindowsRegistryInstallEntry {
        source_key_ref,
        display_name,
        executable_path: executable_path_value(values),
    })
}

pub(super) fn record_from_registry_entry(
    observed_at: &str,
    entry: &WindowsRegistryInstallEntry,
) -> WindowsInstalledAppInventoryRecord {
    WindowsInstalledAppInventoryRecord {
        observed_at: observed_at.to_string(),
        source_kind: APP_GAME_INVENTORY_SOURCE_OS_INSTALLED_RECORD.to_string(),
        source_ref: opaque_ref(APP_GAME_INVENTORY_ENTRY_ID_PREFIX, &entry.source_key_ref),
        custody_state: APP_GAME_INVENTORY_CUSTODY_LOCAL_AGENT.to_string(),
        display_label: entry.display_name.clone(),
        identity_id: None,
        package_id: None,
        bundle_id: None,
        app_user_model_id: None,
        desktop_entry_id: None,
        executable_path_ref: entry
            .executable_path
            .as_ref()
            .map(|path| opaque_ref(APP_GAME_EXECUTABLE_PATH_REF_PREFIX, path)),
        launcher_ref: None,
        launcher_app_id: None,
        launcher_manifest_id: None,
        store_id: None,
        catalog_ref: None,
        inventory_state: APP_GAME_INVENTORY_STATE_INSTALLED.to_string(),
        confidence: APP_GAME_CONFIDENCE_OS_INSTALLED_RECORD,
        evidence: Vec::new(),
    }
}

pub(super) fn journal_events_from_records(
    device_id: &str,
    platform: &str,
    records: &[WindowsInstalledAppInventoryRecord],
) -> Result<Vec<ActivityEvent>, AppGameLiveRegistryInventorySourceError> {
    let rows = windows_installed_inventory_rows_from_records(records);
    rows.iter()
        .map(|row| app_game_inventory_journal_event(device_id, platform, row).map_err(Into::into))
        .collect()
}

fn is_hidden_system_component(values: &BTreeMap<String, String>) -> bool {
    values
        .get(APP_GAME_WINDOWS_REGISTRY_SYSTEM_COMPONENT_VALUE)
        .is_some_and(|value| {
            value == APP_GAME_WINDOWS_REGISTRY_DWORD_ENABLED_VALUE
                || value == APP_GAME_WINDOWS_REGISTRY_DWORD_ENABLED_TEXT
                || value == constants::value::TRUE
        })
}

fn executable_path_value(values: &BTreeMap<String, String>) -> Option<String> {
    non_empty_value(values, APP_GAME_WINDOWS_REGISTRY_INSTALL_LOCATION_VALUE)
        .or_else(|| non_empty_value(values, APP_GAME_WINDOWS_REGISTRY_DISPLAY_ICON_VALUE))
        .or_else(|| non_empty_value(values, APP_GAME_WINDOWS_REGISTRY_UNINSTALL_STRING_VALUE))
        .or_else(|| {
            non_empty_value(
                values,
                APP_GAME_WINDOWS_REGISTRY_QUIET_UNINSTALL_STRING_VALUE,
            )
        })
}

fn non_empty_value(values: &BTreeMap<String, String>, key: &str) -> Option<String> {
    let value = values.get(key)?.trim();
    if value.is_empty() {
        return None;
    }
    Some(value.to_string())
}

fn opaque_ref(prefix: &str, value: &str) -> String {
    let digest = Sha256::digest(value.as_bytes());
    let mut reference = String::from(prefix);
    reference.push_str(&BASE64_URL_SAFE_NO_PAD.encode(digest));
    reference
}
