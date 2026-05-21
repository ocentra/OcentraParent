use std::path::PathBuf;

use ocentra_parent_agent_protocol::constants;

use crate::{managed_browser_launch_plan, BrowserManagedLaunchConfig, BrowserManagedLaunchError};

#[test]
fn managed_browser_launch_plan_uses_owned_profile_and_loopback_bridge() {
    let config = BrowserManagedLaunchConfig {
        executable_path: PathBuf::from(constants::browser::DEVTOOLS_TEST_EXECUTABLE_PATH),
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
    assert!(plan
        .args
        .iter()
        .any(|arg| arg.contains(constants::browser::CHROMIUM_ARG_REMOTE_DEBUGGING_PORT_PREFIX)));
    assert!(plan.args.iter().any(|arg| {
        arg.contains(constants::browser::CHROMIUM_ARG_USER_DATA_DIR_PREFIX)
            && arg.contains(constants::browser::PROFILE_ID_DEV)
    }));
}

#[test]
fn managed_browser_launch_plan_rejects_default_browser_profile() {
    let config = BrowserManagedLaunchConfig {
        executable_path: PathBuf::from(constants::browser::DEVTOOLS_TEST_EXECUTABLE_PATH),
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
