use std::path::PathBuf;

use ocentra_parent_agent_protocol::{constants, BrowserChannel, BrowserFamily};

use crate::{
    managed_browser_executable_identity, managed_browser_launch_plan, unmanaged_browser_processes,
    BrowserManagedLaunchConfig, BrowserManagedLaunchError, ProcessObservation,
};

#[test]
fn managed_browser_launch_plan_uses_owned_profile_and_loopback_bridge() {
    let config = BrowserManagedLaunchConfig {
        executable_path: PathBuf::from(constants::browser::DEVTOOLS_TEST_MSEDGE_BETA_PATH),
        profile_dir: PathBuf::from(constants::browser::PROFILE_ID_DEV),
        bridge_port: constants::browser::DEVTOOLS_TEST_BRIDGE_PORT,
    };

    let plan =
        managed_browser_launch_plan(config).expect(constants::error::BROWSER_BRIDGE_MAPS_TARGET);

    assert_eq!(
        plan.profile_path_ref,
        constants::browser::PROFILE_PATH_REF_MANAGED
    );
    assert_eq!(
        plan.bridge_endpoint_ref,
        constants::browser::BRIDGE_ENDPOINT_REF_LOOPBACK_DEVTOOLS
    );
    assert_eq!(plan.browser_family, BrowserFamily::Edge);
    assert_eq!(plan.browser_channel, BrowserChannel::Beta);
    assert!(plan.args.iter().any(|arg| {
        arg.contains(constants::browser::CHROMIUM_ARG_REMOTE_DEBUGGING_ADDRESS_PREFIX)
            && arg.contains(constants::browser::CHROMIUM_REMOTE_DEBUGGING_LOOPBACK)
    }));
    assert!(plan
        .args
        .iter()
        .any(|arg| arg.contains(constants::browser::CHROMIUM_ARG_REMOTE_DEBUGGING_PORT_PREFIX)));
    assert!(plan.args.iter().any(|arg| {
        arg.contains(constants::browser::CHROMIUM_ARG_USER_DATA_DIR_PREFIX)
            && arg.contains(constants::browser::PROFILE_ID_DEV)
    }));
    assert!(plan.args.iter().any(|arg| {
        arg.contains(constants::browser::CHROMIUM_ARG_PROFILE_DIRECTORY_PREFIX)
            && arg.contains(constants::browser::PROFILE_DIRECTORY_MANAGED_CHILD)
    }));
}

#[test]
fn managed_browser_launch_plan_rejects_default_browser_profile() {
    let config = BrowserManagedLaunchConfig {
        executable_path: PathBuf::from(constants::browser::EXECUTABLE_CHROME_WINDOWS),
        profile_dir: PathBuf::from(constants::browser::PATH_SEGMENT_USER_DATA)
            .join(constants::browser::PATH_SEGMENT_DEFAULT),
        bridge_port: constants::browser::DEVTOOLS_TEST_BRIDGE_PORT,
    };

    let error = managed_browser_launch_plan(config)
        .expect_err(constants::error::BROWSER_BRIDGE_REJECTS_INVALID_URL);

    assert_eq!(error, BrowserManagedLaunchError::DefaultProfileRejected);
    assert_eq!(
        error.reason(),
        constants::value::MANAGED_BROWSER_INVALID_PROFILE
    );
}

#[test]
fn managed_browser_launch_plan_rejects_unowned_profile_path() {
    let config = BrowserManagedLaunchConfig {
        executable_path: PathBuf::from(constants::browser::EXECUTABLE_CHROME_WINDOWS),
        profile_dir: PathBuf::from(constants::browser::DEVTOOLS_TEST_UNOWNED_PROFILE_DIR),
        bridge_port: constants::browser::DEVTOOLS_TEST_BRIDGE_PORT,
    };

    let error = managed_browser_launch_plan(config)
        .expect_err(constants::error::BROWSER_BRIDGE_REJECTS_INVALID_URL);

    assert_eq!(error, BrowserManagedLaunchError::UnownedProfileRejected);
    assert_eq!(
        error.reason(),
        constants::value::MANAGED_BROWSER_INVALID_PROFILE
    );
}

#[test]
fn managed_browser_launch_plan_rejects_unsupported_browser_executable() {
    let config = BrowserManagedLaunchConfig {
        executable_path: PathBuf::from(
            constants::browser::DEVTOOLS_TEST_UNSUPPORTED_EXECUTABLE_PATH,
        ),
        profile_dir: PathBuf::from(constants::browser::PROFILE_ID_DEV),
        bridge_port: constants::browser::DEVTOOLS_TEST_BRIDGE_PORT,
    };

    let error = managed_browser_launch_plan(config)
        .expect_err(constants::error::BROWSER_BRIDGE_REJECTS_INVALID_URL);

    assert_eq!(error, BrowserManagedLaunchError::UnsupportedBrowser);
    assert_eq!(
        error.reason(),
        constants::value::MANAGED_BROWSER_UNSUPPORTED_EXECUTABLE
    );
}

#[test]
fn managed_browser_executable_identity_classifies_chrome_and_edge_channels() {
    let edge_identity = managed_browser_executable_identity(
        PathBuf::from(constants::browser::DEVTOOLS_TEST_MSEDGE_BETA_PATH).as_path(),
    );
    let chrome_identity = managed_browser_executable_identity(
        PathBuf::from(constants::browser::EXECUTABLE_CHROME_WINDOWS).as_path(),
    );

    assert_eq!(edge_identity.browser_family, BrowserFamily::Edge);
    assert_eq!(edge_identity.browser_channel, BrowserChannel::Beta);
    assert!(edge_identity.supports_managed_cdp);
    assert_eq!(chrome_identity.browser_family, BrowserFamily::Chrome);
    assert_eq!(chrome_identity.browser_channel, BrowserChannel::Stable);
    assert!(chrome_identity.supports_managed_cdp);
}

#[test]
fn unmanaged_browser_processes_detects_supported_unmanaged_browser_processes() {
    let observations = vec![
        ProcessObservation {
            pid: constants::browser::DEVTOOLS_TEST_UNMANAGED_PROCESS_ID,
            name: constants::browser::EXECUTABLE_CHROME_WINDOWS.to_string(),
        },
        ProcessObservation {
            pid: constants::browser::PROCESS_ID_UNKNOWN,
            name: constants::browser::DEVTOOLS_TEST_UNSUPPORTED_EXECUTABLE_PATH.to_string(),
        },
    ];

    let unmanaged =
        unmanaged_browser_processes(&observations, Some(constants::browser::PROCESS_ID_UNKNOWN));

    assert_eq!(unmanaged.len(), 1);
    assert_eq!(
        unmanaged[0].process_id,
        constants::browser::DEVTOOLS_TEST_UNMANAGED_PROCESS_ID
    );
    assert_eq!(unmanaged[0].browser_family, BrowserFamily::Chrome);
    assert_eq!(unmanaged[0].browser_channel, BrowserChannel::Stable);
}
