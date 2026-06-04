use std::{
    fs,
    path::{Path, PathBuf},
};

#[cfg(windows)]
use std::env;

use ocentra_parent_agent_protocol::constants;

use crate::browser_windows_inventory_paths::known_browser_executable_names;

pub(crate) fn live_start_menu_shortcut_targets(limit: usize) -> Vec<String> {
    let roots = live_start_menu_roots();
    windows_browser_shortcut_targets_from_roots(&roots, limit)
}

#[cfg(windows)]
fn live_start_menu_roots() -> Vec<PathBuf> {
    [
        constants::env_var::PROGRAM_DATA,
        constants::env_var::APP_DATA,
    ]
    .iter()
    .filter_map(env::var_os)
    .map(start_menu_programs_root)
    .collect()
}

#[cfg(not(windows))]
fn live_start_menu_roots() -> Vec<PathBuf> {
    Vec::new()
}

#[cfg(windows)]
fn start_menu_programs_root(root: std::ffi::OsString) -> PathBuf {
    let mut path = PathBuf::from(root);
    path.push(constants::browser::WINDOWS_PATH_MICROSOFT);
    path.push(constants::browser::WINDOWS_PATH_WINDOWS);
    path.push(constants::browser::WINDOWS_PATH_START_MENU);
    path.push(constants::browser::WINDOWS_PATH_PROGRAMS);
    path
}

pub(crate) fn windows_browser_shortcut_targets_from_roots(
    roots: &[PathBuf],
    limit: usize,
) -> Vec<String> {
    let mut targets = Vec::new();
    for root in roots {
        collect_shortcut_targets(root, limit, &mut targets);
        if targets.len() >= limit {
            break;
        }
    }
    targets
}

fn collect_shortcut_targets(root: &Path, limit: usize, targets: &mut Vec<String>) {
    if targets.len() >= limit {
        return;
    }
    let Ok(entries) = fs::read_dir(root) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let Ok(file_type) = entry.file_type() else {
            continue;
        };
        if file_type.is_dir() {
            collect_shortcut_targets(&path, limit, targets);
        } else if is_shortcut_path(&path) {
            if let Some(target) = shortcut_target_from_path(&path) {
                targets.push(target);
            }
        }
        if targets.len() >= limit {
            break;
        }
    }
}

fn is_shortcut_path(path: &Path) -> bool {
    path.extension().is_some_and(|extension| {
        extension
            .to_string_lossy()
            .eq_ignore_ascii_case(constants::browser::WINDOWS_SHORTCUT_EXTENSION)
    })
}

fn shortcut_target_from_path(path: &Path) -> Option<String> {
    let bytes = fs::read(path).ok()?;
    shortcut_target_from_bytes(&bytes)
}

fn shortcut_target_from_bytes(bytes: &[u8]) -> Option<String> {
    let utf16 = bytes
        .chunks_exact(2)
        .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
        .collect::<Vec<_>>();
    let utf16_text = String::from_utf16_lossy(&utf16);
    target_from_text(&utf16_text).or_else(|| {
        let utf8_text = String::from_utf8_lossy(bytes);
        target_from_text(utf8_text.as_ref())
    })
}

fn target_from_text(text: &str) -> Option<String> {
    let normalized = text.to_ascii_lowercase();
    known_browser_executable_names()
        .iter()
        .filter_map(|executable| normalized.find(executable).map(|start| (start, executable)))
        .min_by_key(|(start, _)| *start)
        .map(|(start, executable)| {
            let end = start + executable.len();
            let path_start = target_path_start(text, start);
            text[path_start..end]
                .trim_matches(char::from(0))
                .to_string()
        })
        .filter(|target| !target.trim().is_empty())
}

fn target_path_start(text: &str, executable_start: usize) -> usize {
    let bytes = text.as_bytes();
    for index in (0..=executable_start).rev() {
        if windows_drive_path_starts_at(bytes, index)
            || windows_unc_path_starts_at(bytes, index)
            || windows_env_path_starts_at(bytes, index)
        {
            return index;
        }
    }
    text[..executable_start]
        .rfind(['\0', '"', '\n', '\r', '\t'])
        .map_or(0, |index| index + 1)
}

fn windows_drive_path_starts_at(bytes: &[u8], index: usize) -> bool {
    bytes
        .get(index)
        .is_some_and(|value| value.is_ascii_alphabetic())
        && bytes.get(index + 1) == Some(&(constants::delimiter::COLON as u8))
        && is_windows_separator(bytes.get(index + 2))
}

fn windows_unc_path_starts_at(bytes: &[u8], index: usize) -> bool {
    is_windows_separator(bytes.get(index)) && is_windows_separator(bytes.get(index + 1))
}

fn windows_env_path_starts_at(bytes: &[u8], index: usize) -> bool {
    bytes.get(index) == Some(&b'%')
}

fn is_windows_separator(value: Option<&u8>) -> bool {
    value == Some(&(constants::delimiter::BACKSLASH as u8))
        || value == Some(&(constants::delimiter::SLASH as u8))
}
