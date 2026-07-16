use std::path::PathBuf;

#[path = "browser_windows_inventory_paths_managed.rs"]
mod browser_windows_inventory_paths_managed;
#[path = "browser_windows_inventory_paths_normalize.rs"]
mod browser_windows_inventory_paths_normalize;
#[path = "browser_windows_inventory_paths_sources.rs"]
mod browser_windows_inventory_paths_sources;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BrowserWindowsRegistryInstallEntry<'a> {
    pub display_icon: Option<&'a str>,
    pub install_location: Option<&'a std::path::Path>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BrowserWindowsInventoryPathSources<'a> {
    pub roots: &'a [PathBuf],
    pub registry_entries: &'a [BrowserWindowsRegistryInstallEntry<'a>],
    pub shortcut_targets: &'a [&'a str],
}

pub fn windows_browser_inventory_candidate_paths(roots: &[PathBuf]) -> Vec<PathBuf> {
    browser_windows_inventory_paths_sources::windows_browser_inventory_candidate_paths(roots)
}

pub fn windows_browser_inventory_candidate_paths_from_sources(
    sources: BrowserWindowsInventoryPathSources<'_>,
) -> Vec<PathBuf> {
    browser_windows_inventory_paths_sources::windows_browser_inventory_candidate_paths_from_sources(
        sources,
    )
}
