use std::path::PathBuf;

use ocentra_parent_agent_protocol::{
    constants, BrowserCapabilityStatus, BrowserExactUrlCapability, BrowserFamily,
    BrowserInventoryInstallState, BrowserInventoryRunningState, BrowserManagementTier,
    BrowserSupportTier,
};

use crate::{
    windows_browser_executable_identity, windows_browser_inventory_candidate_paths,
    windows_browser_inventory_observations, ProcessObservation,
};

#[test]
fn windows_browser_inventory_classifies_supported_managed_candidates() {
    let root = temp_inventory_root(1);
    let edge = root
        .join(constants::browser::PATH_SEGMENT_MICROSOFT)
        .join(constants::browser::PATH_SEGMENT_EDGE_BETA)
        .join(constants::browser::PATH_SEGMENT_APPLICATION)
        .join(constants::browser::EXECUTABLE_MSEDGE_WINDOWS);
    let chrome_testing = root
        .join(constants::browser::PATH_SEGMENT_GOOGLE)
        .join(constants::browser::PATH_SEGMENT_CHROME_FOR_TESTING)
        .join(constants::browser::PATH_SEGMENT_APPLICATION)
        .join(constants::browser::EXECUTABLE_CHROME_WINDOWS);
    create_executable_fixture(&edge);
    create_executable_fixture(&chrome_testing);

    let observations =
        windows_browser_inventory_observations(&[edge.clone(), chrome_testing.clone()], &[], None);

    assert_eq!(observations.len(), 2);
    assert!(observations.iter().any(|observation| {
        observation.browser_family == BrowserFamily::Edge
            && observation.management_tier == BrowserManagementTier::Managed
            && observation.support_tier == BrowserSupportTier::Candidate
            && observation.exact_url_capability == BrowserExactUrlCapability::Unavailable
            && observation.capability_status == BrowserCapabilityStatus::ManagedProfileMissing
    }));
    assert!(observations.iter().any(|observation| {
        observation.product_name == constants::browser::PRODUCT_NAME_CHROME_FOR_TESTING
            && observation.install_state == BrowserInventoryInstallState::Installed
    }));

    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn windows_browser_inventory_marks_unproved_chromium_forks_manual_required() {
    let root = temp_inventory_root(2);
    let brave = root
        .join(constants::browser::PATH_SEGMENT_BRAVE_SOFTWARE)
        .join(constants::browser::PATH_SEGMENT_BRAVE_BROWSER)
        .join(constants::browser::PATH_SEGMENT_APPLICATION)
        .join(constants::browser::EXECUTABLE_BRAVE_WINDOWS);
    create_executable_fixture(&brave);

    let observations = windows_browser_inventory_observations(&[brave], &[], None);

    assert_eq!(observations.len(), 1);
    assert_eq!(observations[0].browser_family, BrowserFamily::Brave);
    assert_eq!(
        observations[0].management_tier,
        BrowserManagementTier::ManualRequired
    );
    assert_eq!(observations[0].support_tier, BrowserSupportTier::Candidate);
    assert_eq!(
        observations[0].exact_url_capability,
        BrowserExactUrlCapability::ManualRequired
    );

    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn windows_browser_inventory_keeps_firefox_unsupported_until_later_adapter() {
    let root = temp_inventory_root(3);
    let firefox = root
        .join(constants::browser::PATH_SEGMENT_MOZILLA_FIREFOX)
        .join(constants::browser::PATH_SEGMENT_APPLICATION)
        .join(constants::browser::EXECUTABLE_FIREFOX_WINDOWS);
    create_executable_fixture(&firefox);

    let observations = windows_browser_inventory_observations(&[firefox], &[], None);

    assert_eq!(observations.len(), 1);
    assert_eq!(observations[0].browser_family, BrowserFamily::Firefox);
    assert_eq!(
        observations[0].management_tier,
        BrowserManagementTier::Unsupported
    );
    assert_eq!(
        observations[0].support_tier,
        BrowserSupportTier::Unsupported
    );
    assert_eq!(
        observations[0].exact_url_capability,
        BrowserExactUrlCapability::Unsupported
    );

    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn windows_browser_inventory_reports_running_browser_processes_as_process_only() {
    let process = ProcessObservation {
        pid: constants::browser::DEVTOOLS_TEST_UNMANAGED_PROCESS_ID,
        name: constants::browser::EXECUTABLE_CHROME_WINDOWS.to_string(),
        executable_path: None,
    };

    let observations = windows_browser_inventory_observations(
        &[],
        &[process],
        Some(constants::browser::PROCESS_ID_UNKNOWN),
    );

    assert_eq!(observations.len(), 1);
    assert_eq!(observations[0].browser_family, BrowserFamily::Chrome);
    assert_eq!(
        observations[0].install_state,
        BrowserInventoryInstallState::CandidateRunning
    );
    assert_eq!(
        observations[0].running_state,
        BrowserInventoryRunningState::RunningUnmanaged
    );
    assert_eq!(
        observations[0].management_tier,
        BrowserManagementTier::Unmanaged
    );
    assert_eq!(
        observations[0].exact_url_capability,
        BrowserExactUrlCapability::NotClaimed
    );
}

#[test]
fn windows_browser_inventory_generates_known_candidate_paths_from_roots() {
    let root = PathBuf::from(constants::browser::DEVTOOLS_TEST_WINDOWS_BROWSER_INVENTORY_DIR);
    let paths = windows_browser_inventory_candidate_paths(&[root]);

    assert!(paths.iter().any(|path| {
        path.ends_with(
            PathBuf::from(constants::browser::PATH_SEGMENT_MICROSOFT)
                .join(constants::browser::PATH_SEGMENT_EDGE)
                .join(constants::browser::PATH_SEGMENT_APPLICATION)
                .join(constants::browser::EXECUTABLE_MSEDGE_WINDOWS),
        )
    }));
    assert!(paths.iter().any(|path| {
        path.ends_with(
            PathBuf::from(constants::browser::PATH_SEGMENT_BRAVE_SOFTWARE)
                .join(constants::browser::PATH_SEGMENT_BRAVE_BROWSER)
                .join(constants::browser::PATH_SEGMENT_APPLICATION)
                .join(constants::browser::EXECUTABLE_BRAVE_WINDOWS),
        )
    }));
    assert!(paths.iter().any(|path| {
        path.ends_with(
            PathBuf::from(constants::browser::PATH_SEGMENT_MOZILLA_FIREFOX)
                .join(constants::browser::PATH_SEGMENT_APPLICATION)
                .join(constants::browser::EXECUTABLE_FIREFOX_WINDOWS),
        )
    }));
}

#[test]
fn managed_discovery_identity_uses_windows_inventory_identity() {
    let identity = windows_browser_executable_identity(
        PathBuf::from(constants::browser::DEVTOOLS_TEST_MSEDGE_BETA_PATH).as_path(),
    );

    assert_eq!(identity.browser_family, BrowserFamily::Edge);
    assert!(identity.supports_managed_cdp);
}

fn temp_inventory_root(suffix: u32) -> PathBuf {
    let root = std::env::temp_dir()
        .join(constants::browser::DEVTOOLS_TEST_WINDOWS_BROWSER_INVENTORY_DIR)
        .join(std::process::id().to_string())
        .join(suffix.to_string());
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
