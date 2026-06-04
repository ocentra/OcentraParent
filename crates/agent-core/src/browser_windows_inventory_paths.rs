use std::{
    env,
    path::{Path, PathBuf},
};

use ocentra_parent_agent_protocol::constants;

use crate::browser_windows_inventory::windows_browser_executable_identity;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct BrowserWindowsRegistryInstallEntry<'a> {
    pub display_icon: Option<&'a str>,
    pub install_location: Option<&'a Path>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct BrowserWindowsInventoryPathSources<'a> {
    pub roots: &'a [PathBuf],
    pub registry_entries: &'a [BrowserWindowsRegistryInstallEntry<'a>],
    pub shortcut_targets: &'a [&'a str],
}

pub fn windows_browser_inventory_candidate_paths(roots: &[PathBuf]) -> Vec<PathBuf> {
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
        push_managed_chromium_paths(&mut paths, root);
        push_manual_chromium_paths(&mut paths, root);
        push_unsupported_browser_paths(&mut paths, root);
    }
    paths.extend(windows_browser_inventory_registry_candidate_paths(
        sources.registry_entries,
    ));
    paths.extend(windows_browser_inventory_shortcut_candidate_paths(
        sources.shortcut_targets,
    ));
    deduplicated_paths(paths)
}

fn windows_browser_inventory_registry_candidate_paths(
    entries: &[BrowserWindowsRegistryInstallEntry<'_>],
) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    for entry in entries {
        if let Some(display_icon) = entry.display_icon {
            push_known_executable_target(&mut paths, display_icon);
        }
        if let Some(install_location) = entry.install_location {
            push_install_location_candidates(&mut paths, install_location);
        }
    }
    deduplicated_paths(paths)
}

fn windows_browser_inventory_shortcut_candidate_paths(targets: &[&str]) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    for target in targets {
        push_known_executable_target(&mut paths, target);
    }
    deduplicated_paths(paths)
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

fn push_managed_chromium_paths(paths: &mut Vec<PathBuf>, root: &Path) {
    push_application_path(
        paths,
        root,
        &[
            constants::browser::PATH_SEGMENT_MICROSOFT,
            constants::browser::PATH_SEGMENT_EDGE,
        ],
        constants::browser::EXECUTABLE_MSEDGE_WINDOWS,
    );
    push_application_path(
        paths,
        root,
        &[
            constants::browser::PATH_SEGMENT_MICROSOFT,
            constants::browser::PATH_SEGMENT_EDGE_BETA,
        ],
        constants::browser::EXECUTABLE_MSEDGE_WINDOWS,
    );
    push_application_path(
        paths,
        root,
        &[
            constants::browser::PATH_SEGMENT_MICROSOFT,
            constants::browser::PATH_SEGMENT_EDGE_DEV,
        ],
        constants::browser::EXECUTABLE_MSEDGE_WINDOWS,
    );
    push_application_path(
        paths,
        root,
        &[
            constants::browser::PATH_SEGMENT_MICROSOFT,
            constants::browser::PATH_SEGMENT_EDGE_SXS,
        ],
        constants::browser::EXECUTABLE_MSEDGE_WINDOWS,
    );
    push_application_path(
        paths,
        root,
        &[
            constants::browser::PATH_SEGMENT_GOOGLE,
            constants::browser::PATH_SEGMENT_CHROME,
        ],
        constants::browser::EXECUTABLE_CHROME_WINDOWS,
    );
    push_application_path(
        paths,
        root,
        &[
            constants::browser::PATH_SEGMENT_GOOGLE,
            constants::browser::PATH_SEGMENT_CHROME_FOR_TESTING,
        ],
        constants::browser::EXECUTABLE_CHROME_WINDOWS,
    );
}

fn push_manual_chromium_paths(paths: &mut Vec<PathBuf>, root: &Path) {
    push_application_path(
        paths,
        root,
        &[
            constants::browser::PATH_SEGMENT_BRAVE_SOFTWARE,
            constants::browser::PATH_SEGMENT_BRAVE_BROWSER,
        ],
        constants::browser::EXECUTABLE_BRAVE_WINDOWS,
    );
    push_application_path(
        paths,
        root,
        &[constants::browser::PATH_SEGMENT_VIVALDI],
        constants::browser::EXECUTABLE_VIVALDI_WINDOWS,
    );
    push_application_path(
        paths,
        root,
        &[
            constants::browser::PATH_SEGMENT_OPERA_SOFTWARE,
            constants::browser::PATH_SEGMENT_OPERA_STABLE,
        ],
        constants::browser::EXECUTABLE_OPERA_WINDOWS,
    );
    push_application_path(
        paths,
        root,
        &[
            constants::browser::PATH_SEGMENT_OPERA_SOFTWARE,
            constants::browser::PATH_SEGMENT_OPERA_GX_STABLE,
        ],
        constants::browser::EXECUTABLE_OPERA_WINDOWS,
    );
    push_application_path(
        paths,
        root,
        &[constants::browser::PATH_SEGMENT_CHROMIUM],
        constants::browser::EXECUTABLE_CHROME_WINDOWS,
    );
}

fn push_unsupported_browser_paths(paths: &mut Vec<PathBuf>, root: &Path) {
    push_application_path(
        paths,
        root,
        &[constants::browser::PATH_SEGMENT_MOZILLA_FIREFOX],
        constants::browser::EXECUTABLE_FIREFOX_WINDOWS,
    );
    push_application_path(
        paths,
        root,
        &[constants::browser::PATH_SEGMENT_FIREFOX_DEVELOPER_EDITION],
        constants::browser::EXECUTABLE_FIREFOX_WINDOWS,
    );
    push_application_path(
        paths,
        root,
        &[constants::browser::PATH_SEGMENT_FIREFOX_NIGHTLY],
        constants::browser::EXECUTABLE_FIREFOX_WINDOWS,
    );
    paths.push(
        root.join(constants::browser::PATH_SEGMENT_TOR_BROWSER)
            .join(constants::browser::PATH_SEGMENT_BROWSER)
            .join(constants::browser::EXECUTABLE_FIREFOX_WINDOWS),
    );
    push_application_path(
        paths,
        root,
        &[constants::browser::PATH_SEGMENT_DUCKDUCKGO],
        constants::browser::EXECUTABLE_DUCKDUCKGO_WINDOWS,
    );
    push_application_path(
        paths,
        root,
        &[constants::browser::PATH_SEGMENT_ARC],
        constants::browser::EXECUTABLE_ARC_WINDOWS,
    );
}

fn push_known_executable_target(paths: &mut Vec<PathBuf>, target: &str) {
    let Some(path) = executable_target_path(target) else {
        return;
    };
    if windows_browser_executable_identity(&path).product_name != constants::browser::FAMILY_UNKNOWN
    {
        paths.push(path);
    }
}

fn executable_target_path(target: &str) -> Option<PathBuf> {
    let trimmed = target.trim();
    if trimmed.is_empty() {
        return None;
    }
    let candidate = if let Some(unquoted) = trimmed.strip_prefix('"') {
        unquoted
            .find('"')
            .map_or(unquoted, |quote_index| &unquoted[..quote_index])
    } else {
        unquoted_known_executable_target(trimmed)
    };
    let expanded = expand_leading_windows_env_var(candidate.trim().trim_matches('"'));
    let path = expanded.trim();
    if path.is_empty() {
        None
    } else {
        Some(PathBuf::from(path))
    }
}

fn expand_leading_windows_env_var(path: &str) -> String {
    let Some(rest) = path.strip_prefix('%') else {
        return path.to_string();
    };
    let Some(end_index) = rest.find('%') else {
        return path.to_string();
    };
    let variable = &rest[..end_index];
    if variable.is_empty() {
        return path.to_string();
    }
    let Ok(value) = env::var(variable) else {
        return path.to_string();
    };
    if value.trim().is_empty() {
        return path.to_string();
    }
    let remainder = &rest[(end_index + 1)..];
    let mut expanded = PathBuf::from(value);
    for component in remainder
        .split(['/', '\\'])
        .filter(|component| !component.is_empty())
    {
        expanded.push(component);
    }
    expanded.to_string_lossy().into_owned()
}

fn unquoted_known_executable_target(target: &str) -> &str {
    let normalized_target = target.to_ascii_lowercase();
    known_browser_executable_names()
        .iter()
        .filter_map(|executable| {
            normalized_target
                .find(executable)
                .map(|index| index + executable.len())
        })
        .min()
        .map_or_else(
            || target.split(',').next().unwrap_or(target).trim(),
            |end_index| &target[..end_index],
        )
}

fn known_browser_executable_names() -> [&'static str; 11] {
    [
        constants::browser::EXECUTABLE_MSEDGE_WINDOWS,
        constants::browser::EXECUTABLE_CHROME_WINDOWS,
        constants::browser::EXECUTABLE_BRAVE_WINDOWS,
        constants::browser::EXECUTABLE_VIVALDI_WINDOWS,
        constants::browser::EXECUTABLE_OPERA_WINDOWS,
        constants::browser::EXECUTABLE_OPERA_GX_WINDOWS,
        constants::browser::EXECUTABLE_CHROMIUM_WINDOWS,
        constants::browser::EXECUTABLE_FIREFOX_WINDOWS,
        constants::browser::EXECUTABLE_TOR_WINDOWS,
        constants::browser::EXECUTABLE_DUCKDUCKGO_WINDOWS,
        constants::browser::EXECUTABLE_ARC_WINDOWS,
    ]
}

fn push_install_location_candidates(paths: &mut Vec<PathBuf>, install_location: &Path) {
    if windows_browser_executable_identity(install_location).product_name
        != constants::browser::FAMILY_UNKNOWN
    {
        paths.push(install_location.to_path_buf());
    }
    for executable in [
        constants::browser::EXECUTABLE_MSEDGE_WINDOWS,
        constants::browser::EXECUTABLE_CHROME_WINDOWS,
        constants::browser::EXECUTABLE_BRAVE_WINDOWS,
        constants::browser::EXECUTABLE_VIVALDI_WINDOWS,
        constants::browser::EXECUTABLE_OPERA_WINDOWS,
        constants::browser::EXECUTABLE_OPERA_GX_WINDOWS,
        constants::browser::EXECUTABLE_CHROMIUM_WINDOWS,
        constants::browser::EXECUTABLE_FIREFOX_WINDOWS,
        constants::browser::EXECUTABLE_TOR_WINDOWS,
        constants::browser::EXECUTABLE_DUCKDUCKGO_WINDOWS,
        constants::browser::EXECUTABLE_ARC_WINDOWS,
    ] {
        paths.push(install_location.join(executable));
        paths.push(
            install_location
                .join(constants::browser::PATH_SEGMENT_APPLICATION)
                .join(executable),
        );
    }
}

fn deduplicated_paths(paths: Vec<PathBuf>) -> Vec<PathBuf> {
    let mut unique = Vec::new();
    for path in paths {
        if path.as_os_str().is_empty() {
            continue;
        }
        if unique.iter().any(|candidate| candidate == &path) {
            continue;
        }
        unique.push(path);
    }
    unique
}

fn push_application_path(
    paths: &mut Vec<PathBuf>,
    root: &Path,
    product_segments: &[&str],
    executable: &str,
) {
    let mut path = root.to_path_buf();
    for segment in product_segments {
        path = path.join(segment);
    }
    paths.push(
        path.join(constants::browser::PATH_SEGMENT_APPLICATION)
            .join(executable),
    );
}
