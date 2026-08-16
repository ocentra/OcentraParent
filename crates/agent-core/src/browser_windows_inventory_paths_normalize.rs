use std::{env, path::PathBuf};

use ocentra_parent_agent_protocol::constants;

use crate::browser_windows_inventory::windows_browser_executable_identity;

pub(crate) fn push_known_executable_target(paths: &mut Vec<PathBuf>, target: &str) {
    let Some(path) = executable_target_path(target) else {
        return;
    };
    if windows_browser_executable_identity(&path).product_name != constants::browser::FAMILY_UNKNOWN
    {
        paths.push(path);
    }
}

pub(crate) fn deduplicated_paths(paths: Vec<PathBuf>) -> Vec<PathBuf> {
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
