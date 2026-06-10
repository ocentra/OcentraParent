use std::{
    fs,
    path::{Path, PathBuf},
};

use ocentra_parent_agent_protocol::constants;

use crate::{
    browser_windows_inventory_candidate_paths_from_live_sources,
    browser_windows_live_registry_entry, browser_windows_shortcut_target_from_bytes,
    live_windows_browser_package_entries_from_roots,
    live_windows_browser_shortcut_targets_from_roots, windows_browser_inventory_observations,
    windows_browser_package_observations,
};

#[test]
fn browser_windows_live_sources_feed_registry_display_icon_and_install_location_candidates() {
    let root = temp_inventory_source_root(1);
    let edge = root
        .join(constants::browser::PATH_SEGMENT_MICROSOFT)
        .join(constants::browser::PATH_SEGMENT_EDGE)
        .join(constants::browser::PATH_SEGMENT_APPLICATION)
        .join(constants::browser::EXECUTABLE_MSEDGE_WINDOWS);
    let chrome_root = root
        .join(constants::browser::PATH_SEGMENT_GOOGLE)
        .join(constants::browser::PATH_SEGMENT_CHROME);
    let chrome = chrome_root
        .join(constants::browser::PATH_SEGMENT_APPLICATION)
        .join(constants::browser::EXECUTABLE_CHROME_WINDOWS);
    create_executable_fixture(&edge);
    create_executable_fixture(&chrome);
    let display_icon = quoted_display_icon(&edge);
    let entries = [browser_windows_live_registry_entry(
        Some(display_icon),
        Some(chrome_root.as_path()),
    )];

    let paths = browser_windows_inventory_candidate_paths_from_live_sources(&[], &entries, &[]);
    let observations = windows_browser_inventory_observations(&paths, &[], None);

    assert!(paths.iter().any(|path| path == &edge));
    assert!(paths.iter().any(|path| path == &chrome));
    assert_eq!(observations.len(), 2);
    assert!(observations
        .iter()
        .all(|observation| observation.executable_path.is_some()));

    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn browser_windows_live_sources_feed_shortcut_targets_without_url_claims() {
    let root = temp_inventory_source_root(2);
    let edge = root
        .join(constants::browser::PATH_SEGMENT_MICROSOFT)
        .join(constants::browser::PATH_SEGMENT_EDGE)
        .join(constants::browser::PATH_SEGMENT_APPLICATION)
        .join(constants::browser::EXECUTABLE_MSEDGE_WINDOWS);
    create_executable_fixture(&edge);
    let shortcut_targets = [quoted_display_icon(&edge)];

    let paths =
        browser_windows_inventory_candidate_paths_from_live_sources(&[], &[], &shortcut_targets);
    let observations = windows_browser_inventory_observations(&paths, &[], None);

    assert_eq!(paths, vec![edge.clone()]);
    assert_eq!(observations.len(), 1);
    assert_eq!(
        observations[0].reason_code,
        constants::browser::INVENTORY_REASON_WINDOWS_MANAGED_PROFILE_REQUIRED
    );

    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn browser_windows_live_sources_parse_lnk_targets_without_url_claims() {
    let root = temp_inventory_source_root(4);
    let edge = root
        .join(constants::browser::PATH_SEGMENT_MICROSOFT)
        .join(constants::browser::PATH_SEGMENT_EDGE)
        .join(constants::browser::PATH_SEGMENT_APPLICATION)
        .join(constants::browser::EXECUTABLE_MSEDGE_WINDOWS);
    let shortcut = root.join(constants::browser::DEVTOOLS_TEST_EDGE_SHORTCUT_FILE_NAME);
    create_executable_fixture(&edge);
    write_shortcut(&shortcut, &edge);

    let parsed_target = browser_windows_shortcut_target_from_bytes(
        &fs::read(&shortcut).expect(constants::error::ACTIVITY_CAPTURE_RECORDS),
    );
    let shortcut_targets = live_windows_browser_shortcut_targets_from_roots(
        std::slice::from_ref(&root),
        constants::browser::SHORTCUT_SCAN_LIMIT_BROWSER_DISCOVERY,
    );
    let target_strings = shortcut_targets
        .iter()
        .map(|target| target.target.clone())
        .collect::<Vec<_>>();
    let paths =
        browser_windows_inventory_candidate_paths_from_live_sources(&[], &[], &target_strings);
    let observations = windows_browser_inventory_observations(&paths, &[], None);

    assert_eq!(
        parsed_target.as_deref(),
        Some(edge.to_string_lossy().as_ref())
    );
    assert_eq!(shortcut_targets.len(), 1);
    assert_eq!(paths, vec![edge]);
    assert_eq!(observations.len(), 1);
    assert_eq!(
        observations[0].reason_code,
        constants::browser::INVENTORY_REASON_WINDOWS_MANAGED_PROFILE_REQUIRED
    );
    assert_eq!(
        observations[0].exact_url_capability,
        ocentra_parent_agent_protocol::BrowserExactUrlCapability::Unavailable
    );

    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn browser_windows_live_sources_feed_packaged_browser_manifest_without_url_claims() {
    let root = temp_inventory_source_root(3);
    let package_root = root
        .join(constants::browser::PATH_SEGMENT_WINDOWS_APPS)
        .join(constants::browser::DEVTOOLS_TEST_EDGE_STORE_PACKAGE_NAME);
    write_manifest(
        &package_root,
        constants::browser::DEVTOOLS_TEST_EDGE_STORE_PACKAGE_MANIFEST_XML,
    );

    let packages = live_windows_browser_package_entries_from_roots(
        &[root.join(constants::browser::PATH_SEGMENT_WINDOWS_APPS)],
        constants::browser::PACKAGE_SCAN_LIMIT_BROWSER_DISCOVERY,
    );
    let observations = windows_browser_package_observations(&packages);

    assert_eq!(packages.len(), 1);
    assert_eq!(
        packages[0].package_name,
        constants::browser::DEVTOOLS_TEST_EDGE_STORE_PACKAGE_NAME
    );
    assert_eq!(
        packages[0].app_user_model_id.as_deref(),
        Some(constants::browser::DEVTOOLS_TEST_EDGE_STORE_PACKAGE_USER_MODEL_ID)
    );
    assert_eq!(observations.len(), 1);
    assert_eq!(observations[0].executable_path, None);
    assert_eq!(
        observations[0].install_state,
        ocentra_parent_agent_protocol::BrowserInventoryInstallState::Packaged
    );
    assert_eq!(
        observations[0].exact_url_capability,
        ocentra_parent_agent_protocol::BrowserExactUrlCapability::ManualRequired
    );
    assert_eq!(
        observations[0].reason_code,
        constants::browser::INVENTORY_REASON_WINDOWS_PACKAGE_MANUAL_REQUIRED
    );

    let _ = std::fs::remove_dir_all(root);
}

fn temp_inventory_source_root(index: u32) -> PathBuf {
    let root = std::env::temp_dir()
        .join(constants::browser::DEVTOOLS_TEST_WINDOWS_BROWSER_INVENTORY_SOURCE_DIR)
        .join(std::process::id().to_string())
        .join(index.saturating_add(1000).to_string());
    let _ = std::fs::remove_dir_all(&root);
    root
}

fn create_executable_fixture(path: &PathBuf) {
    std::fs::create_dir_all(
        path.parent()
            .expect(constants::error::BROWSER_BRIDGE_MAPS_TARGET),
    )
    .expect(constants::error::BROWSER_BRIDGE_MAPS_TARGET);
    std::fs::write(path, []).expect(constants::error::BROWSER_BRIDGE_MAPS_TARGET);
}

fn write_manifest(root: &Path, manifest: &str) {
    fs::create_dir_all(root).expect(constants::error::ACTIVITY_CAPTURE_RECORDS);
    fs::write(
        root.join(ocentra_parent_agent_protocol::APP_GAME_WINDOWS_APPX_MANIFEST_FILE_NAME),
        manifest,
    )
    .expect(constants::error::ACTIVITY_CAPTURE_RECORDS);
}

fn write_shortcut(path: &Path, target: &Path) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect(constants::error::ACTIVITY_CAPTURE_RECORDS);
    }
    fs::write(path, shortcut_bytes(target)).expect(constants::error::ACTIVITY_CAPTURE_RECORDS);
}

fn shortcut_bytes(target: &Path) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&constants::browser::SHORTCUT_LINK_HEADER_SIZE.to_le_bytes());
    bytes.extend_from_slice(&[
        0x01, 0x14, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x46,
    ]);
    bytes.extend_from_slice(&constants::browser::SHORTCUT_LINK_FLAGS_HAS_LINK_INFO.to_le_bytes());
    while bytes.len() < constants::browser::SHORTCUT_LINK_INFO_SECTION_OFFSET {
        bytes.push(0);
    }
    let mut target_bytes = target.to_string_lossy().as_bytes().to_vec();
    target_bytes.push(0);
    let link_info_size =
        constants::browser::SHORTCUT_LINK_INFO_MIN_SIZE as u32 + target_bytes.len() as u32;
    bytes.extend_from_slice(&link_info_size.to_le_bytes());
    bytes.extend_from_slice(&constants::browser::SHORTCUT_LINK_INFO_HEADER_SIZE.to_le_bytes());
    bytes.extend_from_slice(
        &constants::browser::SHORTCUT_LINK_INFO_LOCAL_BASE_PATH_FLAG.to_le_bytes(),
    );
    bytes.extend_from_slice(&0u32.to_le_bytes());
    bytes
        .extend_from_slice(&(constants::browser::SHORTCUT_LINK_INFO_MIN_SIZE as u32).to_le_bytes());
    bytes.extend_from_slice(&0u32.to_le_bytes());
    bytes.extend_from_slice(&0u32.to_le_bytes());
    bytes.extend_from_slice(&target_bytes);
    bytes
}

fn quoted_display_icon(path: &Path) -> String {
    let mut display_icon = String::new();
    display_icon.push(constants::delimiter::QUOTE);
    display_icon.push_str(path.to_string_lossy().as_ref());
    display_icon.push(constants::delimiter::QUOTE);
    display_icon.push(constants::delimiter::LIST);
    display_icon.push('0');
    display_icon
}
