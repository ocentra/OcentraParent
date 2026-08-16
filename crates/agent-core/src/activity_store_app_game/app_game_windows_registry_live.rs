use std::collections::BTreeMap;

use ocentra_parent_agent_protocol::app_game::{
    APP_GAME_WINDOWS_REGISTRY_CURRENT_USER_HIVE, APP_GAME_WINDOWS_REGISTRY_DISPLAY_ICON_VALUE,
    APP_GAME_WINDOWS_REGISTRY_DISPLAY_NAME_VALUE, APP_GAME_WINDOWS_REGISTRY_INSTALL_LOCATION_VALUE,
    APP_GAME_WINDOWS_REGISTRY_LOCAL_MACHINE_HIVE,
    APP_GAME_WINDOWS_REGISTRY_QUIET_UNINSTALL_STRING_VALUE,
    APP_GAME_WINDOWS_REGISTRY_SYSTEM_COMPONENT_VALUE, APP_GAME_WINDOWS_REGISTRY_UNINSTALL_PATH,
    APP_GAME_WINDOWS_REGISTRY_UNINSTALL_STRING_VALUE,
    APP_GAME_WINDOWS_REGISTRY_WOW6432_UNINSTALL_PATH,
};
use ocentra_parent_agent_protocol::constants;
use winreg::{
    enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE},
    RegKey,
};

use super::app_game_windows_registry_record::{
    install_entry_from_values, WindowsRegistryInstallEntry,
};

pub(super) fn registry_install_entries(limit: usize) -> Vec<WindowsRegistryInstallEntry> {
    let roots = [
        RegistryRoot::new(
            RegKey::predef(HKEY_LOCAL_MACHINE),
            APP_GAME_WINDOWS_REGISTRY_LOCAL_MACHINE_HIVE,
            APP_GAME_WINDOWS_REGISTRY_UNINSTALL_PATH,
        ),
        RegistryRoot::new(
            RegKey::predef(HKEY_LOCAL_MACHINE),
            APP_GAME_WINDOWS_REGISTRY_LOCAL_MACHINE_HIVE,
            APP_GAME_WINDOWS_REGISTRY_WOW6432_UNINSTALL_PATH,
        ),
        RegistryRoot::new(
            RegKey::predef(HKEY_CURRENT_USER),
            APP_GAME_WINDOWS_REGISTRY_CURRENT_USER_HIVE,
            APP_GAME_WINDOWS_REGISTRY_UNINSTALL_PATH,
        ),
    ];
    let mut entries = Vec::new();
    for root in roots {
        collect_registry_entries_from_root(&root, limit, &mut entries);
        if entries.len() >= limit {
            break;
        }
    }
    entries
}

struct RegistryRoot {
    hive: RegKey,
    hive_label: &'static str,
    key_path: &'static str,
}

impl RegistryRoot {
    fn new(hive: RegKey, hive_label: &'static str, key_path: &'static str) -> Self {
        Self {
            hive,
            hive_label,
            key_path,
        }
    }
}

fn collect_registry_entries_from_root(
    root: &RegistryRoot,
    limit: usize,
    entries: &mut Vec<WindowsRegistryInstallEntry>,
) {
    if entries.len() >= limit {
        return;
    }
    let Ok(uninstall_root) = root.hive.open_subkey(root.key_path) else {
        return;
    };
    for subkey in uninstall_root.enum_keys().flatten() {
        let Ok(app_key) = uninstall_root.open_subkey(&subkey) else {
            continue;
        };
        let source_key_ref = registry_key_ref(root.hive_label, root.key_path, &subkey);
        let values = values_from_registry_key(&app_key);
        if let Some(entry) = install_entry_from_values(source_key_ref, &values) {
            entries.push(entry);
        }
        if entries.len() >= limit {
            break;
        }
    }
}

fn values_from_registry_key(key: &RegKey) -> BTreeMap<String, String> {
    let mut values = BTreeMap::new();
    insert_registry_string_value(
        key,
        &mut values,
        APP_GAME_WINDOWS_REGISTRY_DISPLAY_NAME_VALUE,
    );
    insert_registry_string_value(
        key,
        &mut values,
        APP_GAME_WINDOWS_REGISTRY_INSTALL_LOCATION_VALUE,
    );
    insert_registry_string_value(
        key,
        &mut values,
        APP_GAME_WINDOWS_REGISTRY_DISPLAY_ICON_VALUE,
    );
    insert_registry_string_value(
        key,
        &mut values,
        APP_GAME_WINDOWS_REGISTRY_UNINSTALL_STRING_VALUE,
    );
    insert_registry_string_value(
        key,
        &mut values,
        APP_GAME_WINDOWS_REGISTRY_QUIET_UNINSTALL_STRING_VALUE,
    );
    if let Ok(system_component) =
        key.get_value::<u32, _>(APP_GAME_WINDOWS_REGISTRY_SYSTEM_COMPONENT_VALUE)
    {
        values.insert(
            APP_GAME_WINDOWS_REGISTRY_SYSTEM_COMPONENT_VALUE.to_string(),
            system_component.to_string(),
        );
    }
    values
}

fn insert_registry_string_value(key: &RegKey, values: &mut BTreeMap<String, String>, name: &str) {
    if let Ok(value) = key.get_value::<String, _>(name) {
        values.insert(name.to_string(), value);
    }
}

fn registry_key_ref(hive_label: &str, key_path: &str, subkey: &str) -> String {
    let mut key_ref = hive_label.to_string();
    key_ref.push(constants::delimiter::BACKSLASH);
    key_ref.push_str(key_path);
    key_ref.push(constants::delimiter::BACKSLASH);
    key_ref.push_str(subkey);
    key_ref
}
