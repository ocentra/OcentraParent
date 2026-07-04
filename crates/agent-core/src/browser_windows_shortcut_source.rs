use std::{
    fs,
    path::{Path, PathBuf},
};

use ocentra_parent_agent_protocol::constants;

#[path = "browser_windows_shortcut_source_parse.rs"]
mod browser_windows_shortcut_source_parse;
#[path = "browser_windows_shortcut_source_paths.rs"]
mod browser_windows_shortcut_source_paths;
#[path = "browser_windows_shortcut_source_scan.rs"]
mod browser_windows_shortcut_source_scan;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BrowserWindowsLiveShortcutTarget {
    pub shortcut_path: PathBuf,
    pub target: String,
}

pub fn live_windows_browser_shortcut_targets_with_limit(
    limit: usize,
) -> Vec<BrowserWindowsLiveShortcutTarget> {
    live_windows_browser_shortcut_targets_from_roots(&live_windows_shortcut_roots(), limit)
}

pub fn live_windows_browser_shortcut_targets_from_roots(
    roots: &[PathBuf],
    limit: usize,
) -> Vec<BrowserWindowsLiveShortcutTarget> {
    let mut targets = Vec::new();
    for root in roots {
        collect_shortcut_targets(root, limit, &mut targets);
        if targets.len() >= limit {
            break;
        }
    }
    targets
}

pub fn browser_windows_shortcut_target_from_bytes(bytes: &[u8]) -> Option<String> {
    if read_u32(bytes, 0)? != constants::browser::SHORTCUT_LINK_HEADER_SIZE {
        return None;
    }
    let link_flags = read_u32(bytes, constants::browser::SHORTCUT_LINK_FLAGS_OFFSET)?;
    if link_flags & constants::browser::SHORTCUT_LINK_FLAGS_HAS_LINK_INFO == 0 {
        return None;
    }
    link_info_target(bytes, constants::browser::SHORTCUT_LINK_INFO_SECTION_OFFSET)
}

fn live_windows_shortcut_roots() -> Vec<PathBuf> {
    browser_windows_shortcut_source_paths::live_windows_shortcut_roots()
}

fn start_menu_programs_root(root: std::ffi::OsString) -> PathBuf {
    browser_windows_shortcut_source_paths::start_menu_programs_root(root)
}

fn collect_shortcut_targets(
    root: &Path,
    limit: usize,
    targets: &mut Vec<BrowserWindowsLiveShortcutTarget>,
) {
    browser_windows_shortcut_source_scan::collect_shortcut_targets(root, limit, targets)
}

fn is_shortcut_path(path: &Path) -> bool {
    browser_windows_shortcut_source_scan::is_shortcut_path(path)
}

fn shortcut_target_from_path(path: &Path) -> Option<String> {
    browser_windows_shortcut_source_parse::shortcut_target_from_path(path)
}

fn link_info_target(bytes: &[u8], offset: usize) -> Option<String> {
    browser_windows_shortcut_source_parse::link_info_target(bytes, offset)
}

fn read_u32(bytes: &[u8], offset: usize) -> Option<u32> {
    browser_windows_shortcut_source_parse::read_u32(bytes, offset)
}

fn read_null_terminated_ansi(bytes: &[u8]) -> Option<String> {
    browser_windows_shortcut_source_parse::read_null_terminated_ansi(bytes)
}

fn executable_target_path(target: &str) -> Option<PathBuf> {
    browser_windows_shortcut_source_paths::executable_target_path(target)
}

fn expand_leading_windows_env_var(path: &str) -> String {
    browser_windows_shortcut_source_paths::expand_leading_windows_env_var(path)
}

fn unquoted_known_executable_target(target: &str) -> &str {
    browser_windows_shortcut_source_paths::unquoted_known_executable_target(target)
}

fn push_install_location_candidates(paths: &mut Vec<PathBuf>, install_location: &Path) {
    browser_windows_shortcut_source_paths::push_install_location_candidates(paths, install_location)
}

fn push_known_executable_target(paths: &mut Vec<PathBuf>, target: &str) {
    browser_windows_shortcut_source_paths::push_known_executable_target(paths, target)
}
