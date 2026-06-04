use std::path::{Path, PathBuf};

use ocentra_parent_agent_protocol::constants;

pub(crate) fn temp_inventory_root(suffix: u32) -> PathBuf {
    let root = std::env::temp_dir()
        .join(constants::browser::DEVTOOLS_TEST_WINDOWS_BROWSER_INVENTORY_DIR)
        .join(std::process::id().to_string())
        .join(suffix.to_string());
    let _ = std::fs::remove_dir_all(&root);
    root
}

pub(crate) fn create_executable_fixture(path: &Path) {
    std::fs::create_dir_all(
        path.parent()
            .expect(constants::error::BROWSER_BRIDGE_MAPS_TARGET),
    )
    .expect(constants::error::BROWSER_BRIDGE_MAPS_TARGET);
    std::fs::write(path, []).expect(constants::error::BROWSER_BRIDGE_MAPS_TARGET);
}

pub(crate) fn write_utf16_shortcut_fixture(path: &Path, target: &Path) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect(constants::error::BROWSER_BRIDGE_MAPS_TARGET);
    }
    let bytes = target
        .to_string_lossy()
        .encode_utf16()
        .flat_map(u16::to_le_bytes)
        .collect::<Vec<_>>();
    std::fs::write(path, bytes).expect(constants::error::BROWSER_BRIDGE_MAPS_TARGET);
}
