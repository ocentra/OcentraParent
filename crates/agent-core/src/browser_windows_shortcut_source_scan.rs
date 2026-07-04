use std::path::Path;

use super::BrowserWindowsLiveShortcutTarget;

mod browser_windows_shortcut_source_scan_walk;

pub(crate) fn collect_shortcut_targets(
    root: &Path,
    limit: usize,
    targets: &mut Vec<BrowserWindowsLiveShortcutTarget>,
) {
    browser_windows_shortcut_source_scan_walk::collect_shortcut_targets(root, limit, targets)
}

pub(crate) fn is_shortcut_path(path: &Path) -> bool {
    browser_windows_shortcut_source_scan_walk::is_shortcut_path(path)
}
