use std::path::{Path, PathBuf};

use crate::browser_windows_inventory_paths::{
    windows_browser_inventory_candidate_paths_from_sources, BrowserWindowsInventoryPathSources,
    BrowserWindowsRegistryInstallEntry,
};
use crate::browser_windows_shortcut_source::live_windows_browser_shortcut_targets_with_limit;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BrowserWindowsLiveRegistryInstallEntry {
    pub display_icon: Option<String>,
    pub install_location: Option<PathBuf>,
}

pub fn live_windows_browser_inventory_candidate_paths_with_limit(
    roots: &[PathBuf],
    limit: usize,
) -> Vec<PathBuf> {
    let registry_entries = live_windows_browser_registry_install_entries(limit);
    let shortcut_scan_limit = limit.min(
        ocentra_parent_agent_protocol::constants::browser::SHORTCUT_SCAN_LIMIT_BROWSER_DISCOVERY,
    );
    let shortcut_targets = live_windows_browser_shortcut_targets_with_limit(shortcut_scan_limit)
        .into_iter()
        .map(|target| target.target)
        .collect::<Vec<_>>();
    browser_windows_inventory_candidate_paths_from_live_sources(
        roots,
        &registry_entries,
        &shortcut_targets,
    )
}

pub fn browser_windows_inventory_candidate_paths_from_live_sources(
    roots: &[PathBuf],
    registry_entries: &[BrowserWindowsLiveRegistryInstallEntry],
    shortcut_targets: &[String],
) -> Vec<PathBuf> {
    let borrowed_registry_entries = registry_entries
        .iter()
        .map(|entry| BrowserWindowsRegistryInstallEntry {
            display_icon: entry.display_icon.as_deref(),
            install_location: entry.install_location.as_deref(),
        })
        .collect::<Vec<_>>();
    let borrowed_shortcut_targets = shortcut_targets
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>();
    windows_browser_inventory_candidate_paths_from_sources(BrowserWindowsInventoryPathSources {
        roots,
        registry_entries: &borrowed_registry_entries,
        shortcut_targets: &borrowed_shortcut_targets,
    })
}

#[cfg(windows)]
fn live_windows_browser_registry_install_entries(
    limit: usize,
) -> Vec<BrowserWindowsLiveRegistryInstallEntry> {
    use ocentra_parent_agent_protocol::app_game::APP_GAME_WINDOWS_REGISTRY_CURRENT_USER_HIVE;
    use ocentra_parent_agent_protocol::app_game::APP_GAME_WINDOWS_REGISTRY_LOCAL_MACHINE_HIVE;
    use ocentra_parent_agent_protocol::app_game::APP_GAME_WINDOWS_REGISTRY_UNINSTALL_PATH;
    use ocentra_parent_agent_protocol::app_game::APP_GAME_WINDOWS_REGISTRY_WOW6432_UNINSTALL_PATH;
    use winreg::{
        enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE},
        RegKey,
    };

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

#[cfg(not(windows))]
fn live_windows_browser_registry_install_entries(
    _limit: usize,
) -> Vec<BrowserWindowsLiveRegistryInstallEntry> {
    Vec::new()
}

#[cfg(windows)]
struct RegistryRoot {
    hive: winreg::RegKey,
    _hive_label: &'static str,
    key_path: &'static str,
}

#[cfg(windows)]
impl RegistryRoot {
    fn new(hive: winreg::RegKey, hive_label: &'static str, key_path: &'static str) -> Self {
        Self {
            hive,
            _hive_label: hive_label,
            key_path,
        }
    }
}

#[cfg(windows)]
fn collect_registry_entries_from_root(
    root: &RegistryRoot,
    limit: usize,
    entries: &mut Vec<BrowserWindowsLiveRegistryInstallEntry>,
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
        if let Some(entry) = install_entry_from_registry_key(&app_key) {
            entries.push(entry);
        }
        if entries.len() >= limit {
            break;
        }
    }
}

#[cfg(windows)]
fn install_entry_from_registry_key(
    key: &winreg::RegKey,
) -> Option<BrowserWindowsLiveRegistryInstallEntry> {
    use ocentra_parent_agent_protocol::app_game::APP_GAME_WINDOWS_REGISTRY_DISPLAY_ICON_VALUE;
    use ocentra_parent_agent_protocol::app_game::APP_GAME_WINDOWS_REGISTRY_INSTALL_LOCATION_VALUE;

    let display_icon = registry_string_value(key, APP_GAME_WINDOWS_REGISTRY_DISPLAY_ICON_VALUE);
    let install_location =
        registry_string_value(key, APP_GAME_WINDOWS_REGISTRY_INSTALL_LOCATION_VALUE)
            .map(PathBuf::from);
    if display_icon.is_none() && install_location.is_none() {
        return None;
    }
    Some(BrowserWindowsLiveRegistryInstallEntry {
        display_icon,
        install_location,
    })
}

#[cfg(windows)]
fn registry_string_value(key: &winreg::RegKey, name: &str) -> Option<String> {
    key.get_value::<String, _>(name)
        .ok()
        .filter(|value| !value.trim().is_empty())
}

pub fn browser_windows_live_registry_entry(
    display_icon: Option<String>,
    install_location: Option<&Path>,
) -> BrowserWindowsLiveRegistryInstallEntry {
    BrowserWindowsLiveRegistryInstallEntry {
        display_icon,
        install_location: install_location.map(Path::to_path_buf),
    }
}
