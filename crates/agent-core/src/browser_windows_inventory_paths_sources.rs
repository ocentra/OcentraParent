use std::path::PathBuf;

use crate::browser_windows_inventory_paths::{
    BrowserWindowsInventoryPathSources, BrowserWindowsRegistryInstallEntry,
};

pub(crate) fn windows_browser_inventory_candidate_paths(roots: &[PathBuf]) -> Vec<PathBuf> {
    windows_browser_inventory_candidate_paths_from_sources(BrowserWindowsInventoryPathSources {
        roots,
        registry_entries: &[],
        shortcut_targets: &[],
    })
}

pub(crate) fn windows_browser_inventory_candidate_paths_from_sources(
    sources: BrowserWindowsInventoryPathSources<'_>,
) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    for root in unique_roots(sources.roots) {
        super::browser_windows_inventory_paths_managed::push_managed_chromium_paths(
            &mut paths, root,
        );
        super::browser_windows_inventory_paths_managed::push_manual_chromium_paths(
            &mut paths, root,
        );
        super::browser_windows_inventory_paths_managed::push_unsupported_browser_paths(
            &mut paths, root,
        );
    }
    paths.extend(windows_browser_inventory_registry_candidate_paths(
        sources.registry_entries,
    ));
    paths.extend(windows_browser_inventory_shortcut_candidate_paths(
        sources.shortcut_targets,
    ));
    super::browser_windows_inventory_paths_normalize::deduplicated_paths(paths)
}

fn windows_browser_inventory_registry_candidate_paths(
    entries: &[BrowserWindowsRegistryInstallEntry<'_>],
) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    for entry in entries {
        if let Some(display_icon) = entry.display_icon {
            super::browser_windows_inventory_paths_normalize::push_known_executable_target(
                &mut paths,
                display_icon,
            );
        }
        if let Some(install_location) = entry.install_location {
            super::browser_windows_inventory_paths_managed::push_install_location_candidates(
                &mut paths,
                install_location,
            );
        }
    }
    super::browser_windows_inventory_paths_normalize::deduplicated_paths(paths)
}

fn windows_browser_inventory_shortcut_candidate_paths(targets: &[&str]) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    for target in targets {
        super::browser_windows_inventory_paths_normalize::push_known_executable_target(
            &mut paths, target,
        );
    }
    super::browser_windows_inventory_paths_normalize::deduplicated_paths(paths)
}

fn unique_roots(roots: &[PathBuf]) -> Vec<&PathBuf> {
    let mut unique = Vec::new();
    for root in roots {
        if root.as_os_str().is_empty() {
            continue;
        }
        if unique.iter().any(|candidate| candidate == &root) {
            continue;
        }
        unique.push(root);
    }
    unique
}
