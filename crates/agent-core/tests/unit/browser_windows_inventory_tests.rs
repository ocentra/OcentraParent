use std::path::{Path, PathBuf};

use ocentra_parent_agent_protocol::browser::{BrowserCapabilityStatus, BrowserFamily};
use ocentra_parent_agent_protocol::browser_inventory::BrowserExactUrlCapability;
use ocentra_parent_agent_protocol::browser_inventory::BrowserInventoryInstallState;
use ocentra_parent_agent_protocol::browser_inventory::BrowserInventoryRunningState;
use ocentra_parent_agent_protocol::browser_inventory::BrowserManagementTier;
use ocentra_parent_agent_protocol::browser_inventory::BrowserSupportTier;
use ocentra_parent_agent_protocol::constants;

use crate::{
    browser_windows_inventory_paths::{
        windows_browser_inventory_candidate_paths_from_sources, BrowserWindowsInventoryPathSources,
        BrowserWindowsRegistryInstallEntry,
    },
    test_text::{test_ok as ok, test_some as some, TestResult, TestText},
    windows_browser_executable_identity, windows_browser_inventory_candidate_paths,
    windows_browser_inventory_observations, ProcessObservation,
};

#[test]
fn windows_browser_inventory_classifies_supported_managed_candidates() -> TestResult {
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
    create_executable_fixture(&edge)?;
    create_executable_fixture(&chrome_testing)?;

    let observations = windows_browser_inventory_observations(&[edge, chrome_testing], &[], None);

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

    Ok(())
}

#[test]
fn windows_browser_inventory_marks_unproved_chromium_forks_manual_required() -> TestResult {
    let root = temp_inventory_root(2);
    let brave = root
        .join(constants::browser::PATH_SEGMENT_BRAVE_SOFTWARE)
        .join(constants::browser::PATH_SEGMENT_BRAVE_BROWSER)
        .join(constants::browser::PATH_SEGMENT_APPLICATION)
        .join(constants::browser::EXECUTABLE_BRAVE_WINDOWS);
    create_executable_fixture(&brave)?;

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

    Ok(())
}

#[test]
fn windows_browser_inventory_keeps_firefox_unsupported_until_later_adapter() -> TestResult {
    let root = temp_inventory_root(3);
    let firefox = root
        .join(constants::browser::PATH_SEGMENT_MOZILLA_FIREFOX)
        .join(constants::browser::PATH_SEGMENT_APPLICATION)
        .join(constants::browser::EXECUTABLE_FIREFOX_WINDOWS);
    create_executable_fixture(&firefox)?;

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

    Ok(())
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
fn windows_browser_inventory_uses_process_executable_path_for_identity() {
    let process = ProcessObservation {
        pid: constants::browser::DEVTOOLS_TEST_UNMANAGED_PROCESS_ID,
        name: constants::browser::EXECUTABLE_CHROME_WINDOWS.to_string(),
        executable_path: Some(
            PathBuf::from(constants::browser::DEVTOOLS_TEST_WINDOWS_BROWSER_INVENTORY_DIR)
                .join(constants::browser::PATH_SEGMENT_GOOGLE)
                .join(constants::browser::PATH_SEGMENT_CHROME_FOR_TESTING)
                .join(constants::browser::PATH_SEGMENT_APPLICATION)
                .join(constants::browser::EXECUTABLE_CHROME_WINDOWS),
        ),
    };

    let observations = windows_browser_inventory_observations(&[], &[process], None);

    assert_eq!(observations.len(), 1);
    assert_eq!(
        observations[0].product_name,
        constants::browser::PRODUCT_NAME_CHROME_FOR_TESTING
    );
    assert_eq!(
        observations[0].install_state,
        BrowserInventoryInstallState::CandidateRunning
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
fn windows_browser_inventory_collapses_installed_candidate_and_running_process() -> TestResult {
    let root = temp_inventory_root(5);
    let edge = root
        .join(constants::browser::PATH_SEGMENT_MICROSOFT)
        .join(constants::browser::PATH_SEGMENT_EDGE)
        .join(constants::browser::PATH_SEGMENT_APPLICATION)
        .join(constants::browser::EXECUTABLE_MSEDGE_WINDOWS);
    create_executable_fixture(&edge)?;
    let process = ProcessObservation {
        pid: constants::browser::DEVTOOLS_TEST_UNMANAGED_PROCESS_ID,
        name: constants::browser::EXECUTABLE_MSEDGE_WINDOWS.to_string(),
        executable_path: Some(edge.clone()),
    };

    let observations =
        windows_browser_inventory_observations(std::slice::from_ref(&edge), &[process], None);

    assert_eq!(observations.len(), 1);
    assert_eq!(
        observations[0].executable_path.as_deref(),
        Some(edge.as_path())
    );
    assert_eq!(
        observations[0].install_state,
        BrowserInventoryInstallState::Installed
    );
    assert_eq!(
        observations[0].running_state,
        BrowserInventoryRunningState::RunningUnmanaged
    );
    assert_eq!(
        observations[0].process_id,
        Some(constants::browser::DEVTOOLS_TEST_UNMANAGED_PROCESS_ID)
    );
    assert_eq!(
        observations[0].management_tier,
        BrowserManagementTier::Unmanaged
    );
    assert_eq!(
        observations[0].exact_url_capability,
        BrowserExactUrlCapability::NotClaimed
    );
    assert_eq!(
        observations[0].capability_status,
        BrowserCapabilityStatus::UnmanagedBrowser
    );

    let _ = std::fs::remove_dir_all(root);

    Ok(())
}

#[test]
fn windows_browser_inventory_keeps_tor_process_path_unsupported() {
    let process = ProcessObservation {
        pid: constants::browser::DEVTOOLS_TEST_UNMANAGED_PROCESS_ID,
        name: constants::browser::EXECUTABLE_FIREFOX_WINDOWS.to_string(),
        executable_path: Some(
            PathBuf::from(constants::browser::DEVTOOLS_TEST_WINDOWS_BROWSER_INVENTORY_DIR)
                .join(constants::browser::PATH_SEGMENT_TOR_BROWSER)
                .join(constants::browser::PATH_SEGMENT_BROWSER)
                .join(constants::browser::EXECUTABLE_FIREFOX_WINDOWS),
        ),
    };

    let observations = windows_browser_inventory_observations(&[], &[process], None);

    assert_eq!(observations.len(), 1);
    assert_eq!(
        observations[0].product_name,
        constants::browser::PRODUCT_NAME_TOR_BROWSER
    );
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
fn windows_browser_inventory_deduplicates_candidate_roots() {
    let root = PathBuf::from(constants::browser::DEVTOOLS_TEST_WINDOWS_BROWSER_INVENTORY_DIR)
        .join(constants::browser::PATH_SEGMENT_DEFAULT);
    let paths = windows_browser_inventory_candidate_paths(&[root.clone(), root.clone()]);
    let edge_stable_count = paths
        .iter()
        .filter(|path| {
            path == &&root
                .join(constants::browser::PATH_SEGMENT_MICROSOFT)
                .join(constants::browser::PATH_SEGMENT_EDGE)
                .join(constants::browser::PATH_SEGMENT_APPLICATION)
                .join(constants::browser::EXECUTABLE_MSEDGE_WINDOWS)
        })
        .count();

    assert_eq!(edge_stable_count, 1);
}

#[test]
fn windows_browser_inventory_generates_candidates_from_multiple_roots() {
    let machine_root =
        PathBuf::from(constants::browser::DEVTOOLS_TEST_WINDOWS_BROWSER_INVENTORY_DIR)
            .join(constants::browser::PATH_SEGMENT_APPLICATION);
    let user_root = PathBuf::from(constants::browser::DEVTOOLS_TEST_WINDOWS_BROWSER_INVENTORY_DIR)
        .join(constants::browser::PATH_SEGMENT_USER_DATA);
    let paths =
        windows_browser_inventory_candidate_paths(&[machine_root.clone(), user_root.clone()]);

    assert!(paths.iter().any(|path| {
        path == &machine_root
            .join(constants::browser::PATH_SEGMENT_MICROSOFT)
            .join(constants::browser::PATH_SEGMENT_EDGE)
            .join(constants::browser::PATH_SEGMENT_APPLICATION)
            .join(constants::browser::EXECUTABLE_MSEDGE_WINDOWS)
    }));
    assert!(paths.iter().any(|path| {
        path == &user_root
            .join(constants::browser::PATH_SEGMENT_GOOGLE)
            .join(constants::browser::PATH_SEGMENT_CHROME)
            .join(constants::browser::PATH_SEGMENT_APPLICATION)
            .join(constants::browser::EXECUTABLE_CHROME_WINDOWS)
    }));
}

#[test]
fn windows_browser_inventory_derives_registry_candidates_from_display_icon_and_install_location(
) -> TestResult {
    let root = temp_inventory_root(6);
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
    create_executable_fixture(&edge)?;
    create_executable_fixture(&chrome)?;
    let mut display_icon = String::new();
    display_icon.push('"');
    display_icon.push_str(edge.to_string_lossy().as_ref());
    display_icon.push('"');
    display_icon.push(',');
    display_icon.push('0');
    let entries = [BrowserWindowsRegistryInstallEntry {
        display_icon: Some(display_icon.as_str()),
        install_location: Some(chrome_root.as_path()),
    }];

    let paths = windows_browser_inventory_candidate_paths_from_sources(
        BrowserWindowsInventoryPathSources {
            roots: &[],
            registry_entries: &entries,
            shortcut_targets: &[],
        },
    );
    let observations = windows_browser_inventory_observations(&paths, &[], None);

    assert!(observations.iter().any(|observation| {
        observation.browser_family == BrowserFamily::Edge
            && observation.install_state == BrowserInventoryInstallState::Installed
            && observation.exact_url_capability == BrowserExactUrlCapability::Unavailable
    }));
    assert!(observations.iter().any(|observation| {
        observation.browser_family == BrowserFamily::Chrome
            && observation.product_name == constants::browser::PRODUCT_NAME_GOOGLE_CHROME
            && observation.management_tier == BrowserManagementTier::Managed
    }));

    let _ = std::fs::remove_dir_all(root);

    Ok(())
}

#[test]
fn windows_browser_inventory_derives_shortcut_target_candidates_without_url_claims() -> TestResult {
    let root = temp_inventory_root(7);
    let brave = root
        .join(constants::browser::PATH_SEGMENT_BRAVE_SOFTWARE)
        .join(constants::browser::PATH_SEGMENT_BRAVE_BROWSER)
        .join(constants::browser::PATH_SEGMENT_APPLICATION)
        .join(constants::browser::EXECUTABLE_BRAVE_WINDOWS);
    create_executable_fixture(&brave)?;
    let mut target = String::new();
    target.push('"');
    target.push_str(brave.to_string_lossy().as_ref());
    target.push('"');

    let shortcut_targets = [target.as_str()];
    let paths = windows_browser_inventory_candidate_paths_from_sources(
        BrowserWindowsInventoryPathSources {
            roots: &[],
            registry_entries: &[],
            shortcut_targets: &shortcut_targets,
        },
    );
    let observations = windows_browser_inventory_observations(&paths, &[], None);

    assert_eq!(observations.len(), 1);
    assert_eq!(observations[0].browser_family, BrowserFamily::Brave);
    assert_eq!(
        observations[0].management_tier,
        BrowserManagementTier::ManualRequired
    );
    assert_eq!(
        observations[0].exact_url_capability,
        BrowserExactUrlCapability::ManualRequired
    );
    assert_eq!(
        observations[0].running_state,
        BrowserInventoryRunningState::NotRunning
    );

    let _ = std::fs::remove_dir_all(root);

    Ok(())
}

#[test]
fn windows_browser_inventory_derives_unquoted_command_targets_without_url_claims() -> TestResult {
    let root = temp_inventory_root(8);
    let chrome = root
        .join(constants::browser::PATH_SEGMENT_GOOGLE)
        .join(constants::browser::PATH_SEGMENT_CHROME)
        .join(constants::browser::PATH_SEGMENT_APPLICATION)
        .join(constants::browser::EXECUTABLE_CHROME_WINDOWS);
    create_executable_fixture(&chrome)?;
    let mut target = String::new();
    target.push_str(chrome.to_string_lossy().as_ref());
    target.push(' ');
    target.push_str(constants::browser::CHROMIUM_ARG_PROFILE_DIRECTORY_PREFIX);
    target.push_str(constants::browser::PATH_SEGMENT_DEFAULT);

    let shortcut_targets = [target.as_str()];
    let paths = windows_browser_inventory_candidate_paths_from_sources(
        BrowserWindowsInventoryPathSources {
            roots: &[],
            registry_entries: &[],
            shortcut_targets: &shortcut_targets,
        },
    );
    let observations = windows_browser_inventory_observations(&paths, &[], None);

    assert_eq!(paths.as_slice(), std::slice::from_ref(&chrome));
    assert_eq!(observations.len(), 1);
    assert_eq!(observations[0].browser_family, BrowserFamily::Chrome);
    assert_eq!(
        observations[0].install_state,
        BrowserInventoryInstallState::Installed
    );
    assert_eq!(
        observations[0].exact_url_capability,
        BrowserExactUrlCapability::Unavailable
    );
    assert_eq!(
        observations[0].running_state,
        BrowserInventoryRunningState::NotRunning
    );

    let _ = std::fs::remove_dir_all(root);

    Ok(())
}

#[test]
fn windows_browser_inventory_expands_env_var_targets_without_url_claims() -> TestResult {
    let root = temp_inventory_root(9);
    let chrome = root
        .join(constants::browser::PATH_SEGMENT_GOOGLE)
        .join(constants::browser::PATH_SEGMENT_CHROME)
        .join(constants::browser::PATH_SEGMENT_APPLICATION)
        .join(constants::browser::EXECUTABLE_CHROME_WINDOWS);
    create_executable_fixture(&chrome)?;
    let env_var_name = constants::env_var::AGENT_BROWSER_POLICY_STORE_PATH;
    let previous_env_var_value = std::env::var_os(env_var_name);
    std::env::set_var(env_var_name, &root);
    let mut target = String::new();
    target.push('%');
    target.push_str(constants::env_var::AGENT_BROWSER_POLICY_STORE_PATH);
    target.push('%');
    target.push('\\');
    target.push_str(constants::browser::PATH_SEGMENT_GOOGLE);
    target.push('\\');
    target.push_str(constants::browser::PATH_SEGMENT_CHROME);
    target.push('\\');
    target.push_str(constants::browser::PATH_SEGMENT_APPLICATION);
    target.push('\\');
    target.push_str(constants::browser::EXECUTABLE_CHROME_WINDOWS);
    target.push(' ');
    target.push_str(constants::browser::CHROMIUM_ARG_PROFILE_DIRECTORY_PREFIX);
    target.push_str(constants::browser::PATH_SEGMENT_DEFAULT);

    let shortcut_targets = [target.as_str()];
    let paths = windows_browser_inventory_candidate_paths_from_sources(
        BrowserWindowsInventoryPathSources {
            roots: &[],
            registry_entries: &[],
            shortcut_targets: &shortcut_targets,
        },
    );
    let observations = windows_browser_inventory_observations(&paths, &[], None);

    assert_eq!(paths.as_slice(), std::slice::from_ref(&chrome));
    assert_eq!(observations.len(), 1);
    assert_eq!(observations[0].browser_family, BrowserFamily::Chrome);
    assert_eq!(
        observations[0].install_state,
        BrowserInventoryInstallState::Installed
    );
    assert_eq!(
        observations[0].exact_url_capability,
        BrowserExactUrlCapability::Unavailable
    );
    assert_eq!(
        observations[0].running_state,
        BrowserInventoryRunningState::NotRunning
    );

    if let Some(previous) = previous_env_var_value {
        std::env::set_var(env_var_name, previous);
    } else {
        std::env::remove_var(env_var_name);
    }
    let _ = std::fs::remove_dir_all(root);

    Ok(())
}

#[test]
fn windows_browser_inventory_marks_windowsapps_path_as_packaged() -> TestResult {
    let test_root = temp_inventory_root(4);
    let root = test_root
        .join(constants::browser::PATH_SEGMENT_WINDOWS_APPS)
        .join(constants::browser::PATH_SEGMENT_MICROSOFT)
        .join(constants::browser::PATH_SEGMENT_EDGE)
        .join(constants::browser::PATH_SEGMENT_APPLICATION);
    let edge = root.join(constants::browser::EXECUTABLE_MSEDGE_WINDOWS);
    create_executable_fixture(&edge)?;

    let observations =
        windows_browser_inventory_observations(std::slice::from_ref(&edge), &[], None);

    assert_eq!(observations.len(), 1);
    assert_eq!(
        observations[0].install_state,
        BrowserInventoryInstallState::Packaged
    );
    assert_eq!(observations[0].browser_family, BrowserFamily::Edge);
    assert_eq!(
        observations[0].exact_url_capability,
        BrowserExactUrlCapability::Unavailable
    );

    let _ = std::fs::remove_dir_all(test_root);

    Ok(())
}

#[test]
fn managed_discovery_identity_uses_windows_inventory_identity() {
    let identity = windows_browser_executable_identity(
        PathBuf::from(constants::browser::DEVTOOLS_TEST_MSEDGE_BETA_PATH).as_path(),
    );

    assert_eq!(identity.browser_family, BrowserFamily::Edge);
    assert!(identity.supports_managed_cdp);
}

fn temp_inventory_root(suffix: u32) -> TestText {
    let root = std::env::temp_dir()
        .join(constants::browser::DEVTOOLS_TEST_WINDOWS_BROWSER_INVENTORY_DIR)
        .join(std::process::id().to_string())
        .join(suffix.to_string());
    let _ = std::fs::remove_dir_all(&root);
    TestText::from_display(root.display())
}

fn create_executable_fixture(path: impl AsRef<Path>) -> TestResult {
    let path = path.as_ref();
    ok(
        std::fs::create_dir_all(some(
            path.parent(),
            constants::error::BROWSER_BRIDGE_MAPS_TARGET,
        )?),
        constants::error::BROWSER_BRIDGE_MAPS_TARGET,
    )?;
    ok(
        std::fs::write(path, []),
        constants::error::BROWSER_BRIDGE_MAPS_TARGET,
    )?;

    Ok(())
}
