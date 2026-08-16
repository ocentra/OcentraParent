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
