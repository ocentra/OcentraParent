use std::{fs, path::Path};

use super::BrowserWindowsLiveShortcutTarget;

pub(crate) fn collect_shortcut_targets(
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
        if collect_shortcut_target_entry(&entry, limit, targets) {
            break;
        }
    }
}

pub(crate) fn is_shortcut_path(path: &Path) -> bool {
    path.extension().is_some_and(|extension| {
        extension.to_string_lossy().eq_ignore_ascii_case(
            ocentra_parent_agent_protocol::constants::browser::WINDOWS_SHORTCUT_EXTENSION,
        )
    })
}

fn collect_shortcut_target_entry(
    entry: &fs::DirEntry,
    limit: usize,
    targets: &mut Vec<BrowserWindowsLiveShortcutTarget>,
) -> bool {
    let path = entry.path();
    let Ok(file_type) = entry.file_type() else {
        return targets.len() >= limit;
    };
    if file_type.is_dir() {
        collect_shortcut_targets(&path, limit, targets);
    } else if is_shortcut_path(&path) {
        if let Some(target) = super::super::shortcut_target_from_path(&path) {
            targets.push(BrowserWindowsLiveShortcutTarget {
                shortcut_path: path,
                target,
            });
        }
    }
    targets.len() >= limit
}
