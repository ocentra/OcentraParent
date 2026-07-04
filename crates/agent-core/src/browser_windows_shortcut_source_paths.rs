use std::{
    env,
    path::{Path, PathBuf},
};

use ocentra_parent_agent_protocol::constants;

use crate::browser_windows_inventory::windows_browser_executable_identity;

pub(crate) fn live_windows_shortcut_roots() -> Vec<PathBuf> {
    [
        constants::env_var::PROGRAM_DATA,
        constants::env_var::APP_DATA,
    ]
    .iter()
    .filter_map(env::var_os)
    .map(start_menu_programs_root)
    .collect()
}

pub(crate) fn start_menu_programs_root(root: std::ffi::OsString) -> PathBuf {
    let mut path = PathBuf::from(root);
    path.push(constants::browser::PATH_SEGMENT_MICROSOFT);
    path.push(constants::browser::PATH_SEGMENT_WINDOWS);
    path.push(constants::browser::PATH_SEGMENT_START_MENU);
    path.push(constants::browser::PATH_SEGMENT_PROGRAMS);
    path
}

pub(crate) fn executable_target_path(target: &str) -> Option<PathBuf> {
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

pub(crate) fn expand_leading_windows_env_var(path: &str) -> String {
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

pub(crate) fn unquoted_known_executable_target(target: &str) -> &str {
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

pub(crate) fn push_install_location_candidates(paths: &mut Vec<PathBuf>, install_location: &Path) {
    if windows_browser_executable_identity(install_location).product_name
        != constants::browser::FAMILY_UNKNOWN
    {
        paths.push(install_location.to_path_buf());
    }
    for executable in known_browser_executable_names() {
        paths.push(install_location.join(executable));
        paths.push(
            install_location
                .join(constants::browser::PATH_SEGMENT_APPLICATION)
                .join(executable),
        );
    }
}

pub(crate) fn push_known_executable_target(paths: &mut Vec<PathBuf>, target: &str) {
    let Some(path) = executable_target_path(target) else {
        return;
    };
    if windows_browser_executable_identity(&path).product_name != constants::browser::FAMILY_UNKNOWN
    {
        paths.push(path);
    }
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
