use std::{
    fs,
    path::{Path, PathBuf},
};

use ocentra_parent_agent_protocol::constants;

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
    [
        constants::env_var::PROGRAM_DATA,
        constants::env_var::APP_DATA,
    ]
    .iter()
    .filter_map(std::env::var_os)
    .map(start_menu_programs_root)
    .collect()
}

fn start_menu_programs_root(root: std::ffi::OsString) -> PathBuf {
    let mut path = PathBuf::from(root);
    path.push(constants::browser::PATH_SEGMENT_MICROSOFT);
    path.push(constants::browser::PATH_SEGMENT_WINDOWS);
    path.push(constants::browser::PATH_SEGMENT_START_MENU);
    path.push(constants::browser::PATH_SEGMENT_PROGRAMS);
    path
}

fn collect_shortcut_targets(
    root: &Path,
    limit: usize,
    targets: &mut Vec<BrowserWindowsLiveShortcutTarget>,
) {
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
                targets.push(BrowserWindowsLiveShortcutTarget {
                    shortcut_path: path,
                    target,
                });
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
    browser_windows_shortcut_target_from_bytes(&bytes)
}

fn link_info_target(bytes: &[u8], offset: usize) -> Option<String> {
    let size = read_u32(
        bytes,
        offset + constants::browser::SHORTCUT_LINK_INFO_SIZE_OFFSET,
    )? as usize;
    if size < constants::browser::SHORTCUT_LINK_INFO_MIN_SIZE {
        return None;
    }
    let end = offset.checked_add(size)?;
    if end > bytes.len() {
        return None;
    }
    let flags = read_u32(
        bytes,
        offset + constants::browser::SHORTCUT_LINK_INFO_FLAGS_OFFSET,
    )?;
    if flags & constants::browser::SHORTCUT_LINK_INFO_LOCAL_BASE_PATH_FLAG == 0 {
        return None;
    }
    let local_base_path_offset = read_u32(
        bytes,
        offset + constants::browser::SHORTCUT_LINK_INFO_LOCAL_BASE_PATH_OFFSET,
    )? as usize;
    let target_offset = offset.checked_add(local_base_path_offset)?;
    if target_offset >= end {
        return None;
    }
    read_null_terminated_ansi(&bytes[target_offset..end])
}

fn read_u32(bytes: &[u8], offset: usize) -> Option<u32> {
    let slice = bytes.get(offset..offset.checked_add(4)?)?;
    Some(u32::from_le_bytes(slice.try_into().ok()?))
}

fn read_null_terminated_ansi(bytes: &[u8]) -> Option<String> {
    let end = bytes.iter().position(|byte| *byte == 0)?;
    if end == 0 {
        return None;
    }
    String::from_utf8(bytes[..end].to_vec()).ok()
}
