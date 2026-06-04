use std::{env, path::PathBuf};

use ocentra_parent_agent_protocol::constants;

use crate::browser_windows_inventory_paths::{
    windows_browser_inventory_candidate_paths_from_sources, BrowserWindowsInventoryPathSources,
    BrowserWindowsRegistryInstallEntry,
};
use crate::browser_windows_inventory_registry_sources::live_registry_install_sources;
use crate::browser_windows_inventory_shortcut_sources::live_start_menu_shortcut_targets;

pub use crate::browser_windows_inventory_registry_sources::BrowserWindowsRegistryInstallSource;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BrowserWindowsInventorySourceSnapshot {
    pub roots: Vec<PathBuf>,
    pub registry_entries: Vec<BrowserWindowsRegistryInstallSource>,
    pub shortcut_targets: Vec<String>,
}

pub fn windows_browser_inventory_live_candidate_paths(limit: usize) -> Vec<PathBuf> {
    let snapshot = windows_browser_inventory_live_source_snapshot(limit);
    windows_browser_inventory_candidate_paths_from_snapshot(&snapshot)
}

pub fn windows_browser_inventory_candidate_paths_from_snapshot(
    snapshot: &BrowserWindowsInventorySourceSnapshot,
) -> Vec<PathBuf> {
    let registry_entries = snapshot
        .registry_entries
        .iter()
        .map(|entry| BrowserWindowsRegistryInstallEntry {
            display_icon: entry.display_icon.as_deref(),
            install_location: entry.install_location.as_deref(),
        })
        .collect::<Vec<_>>();
    let shortcut_targets = snapshot
        .shortcut_targets
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>();
    windows_browser_inventory_candidate_paths_from_sources(BrowserWindowsInventoryPathSources {
        roots: &snapshot.roots,
        registry_entries: &registry_entries,
        shortcut_targets: &shortcut_targets,
    })
}

pub(crate) fn windows_browser_inventory_live_source_snapshot(
    limit: usize,
) -> BrowserWindowsInventorySourceSnapshot {
    BrowserWindowsInventorySourceSnapshot {
        roots: windows_browser_inventory_default_roots(),
        registry_entries: live_registry_install_sources(limit),
        shortcut_targets: live_start_menu_shortcut_targets(limit),
    }
}

pub(crate) fn windows_browser_inventory_default_roots() -> Vec<PathBuf> {
    [
        constants::env_var::PROGRAM_FILES,
        constants::env_var::PROGRAM_FILES_X86,
        constants::env_var::LOCAL_APP_DATA,
    ]
    .iter()
    .filter_map(env::var_os)
    .map(PathBuf::from)
    .collect()
}
