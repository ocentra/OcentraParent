use std::path::PathBuf;

#[cfg(windows)]
use ocentra_parent_agent_protocol::constants;

#[cfg(windows)]
use winreg::{
    enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE},
    RegKey,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BrowserWindowsRegistryInstallSource {
    pub display_icon: Option<String>,
    pub install_location: Option<PathBuf>,
}

#[cfg(windows)]
pub(crate) fn live_registry_install_sources(
    limit: usize,
) -> Vec<BrowserWindowsRegistryInstallSource> {
    let roots = [
        RegistryRoot::new(
            RegKey::predef(HKEY_LOCAL_MACHINE),
            constants::browser::WINDOWS_REGISTRY_UNINSTALL_PATH,
        ),
        RegistryRoot::new(
            RegKey::predef(HKEY_LOCAL_MACHINE),
            constants::browser::WINDOWS_REGISTRY_WOW6432_UNINSTALL_PATH,
        ),
        RegistryRoot::new(
            RegKey::predef(HKEY_CURRENT_USER),
            constants::browser::WINDOWS_REGISTRY_UNINSTALL_PATH,
        ),
    ];
    let mut entries = Vec::new();
    for root in roots {
        collect_registry_install_sources_from_root(&root, limit, &mut entries);
        if entries.len() >= limit {
            break;
        }
    }
    entries
}

#[cfg(not(windows))]
pub(crate) fn live_registry_install_sources(
    _limit: usize,
) -> Vec<BrowserWindowsRegistryInstallSource> {
    Vec::new()
}

#[cfg(windows)]
struct RegistryRoot {
    hive: RegKey,
    key_path: &'static str,
}

#[cfg(windows)]
impl RegistryRoot {
    fn new(hive: RegKey, key_path: &'static str) -> Self {
        Self { hive, key_path }
    }
}

#[cfg(windows)]
fn collect_registry_install_sources_from_root(
    root: &RegistryRoot,
    limit: usize,
    entries: &mut Vec<BrowserWindowsRegistryInstallSource>,
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
        if let Some(entry) = registry_install_source_from_key(&app_key) {
            entries.push(entry);
        }
        if entries.len() >= limit {
            break;
        }
    }
}

#[cfg(windows)]
fn registry_install_source_from_key(key: &RegKey) -> Option<BrowserWindowsRegistryInstallSource> {
    let display_icon = key
        .get_value::<String, _>(constants::browser::WINDOWS_REGISTRY_DISPLAY_ICON_VALUE)
        .ok()
        .filter(|value| !value.trim().is_empty());
    let install_location = key
        .get_value::<String, _>(constants::browser::WINDOWS_REGISTRY_INSTALL_LOCATION_VALUE)
        .ok()
        .filter(|value| !value.trim().is_empty())
        .map(PathBuf::from);
    if display_icon.is_none() && install_location.is_none() {
        return None;
    }
    Some(BrowserWindowsRegistryInstallSource {
        display_icon,
        install_location,
    })
}
