use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener},
    path::Path,
};

use ocentra_parent_agent_protocol::constants;

use crate::browser_managed_discovery::managed_browser_executable_identity;

use super::{
    BrowserManagedBridgePortReservation, BrowserManagedLaunchConfig, BrowserManagedLaunchError,
    BrowserManagedLaunchPlan,
};

pub(crate) fn launch_error_reason(error: &BrowserManagedLaunchError) -> &'static str {
    match error {
        BrowserManagedLaunchError::DefaultProfileRejected
        | BrowserManagedLaunchError::UnownedProfileRejected => {
            constants::value::MANAGED_BROWSER_INVALID_PROFILE
        }
        BrowserManagedLaunchError::BridgePortUnavailable => {
            constants::value::MANAGED_BROWSER_BRIDGE_PORT_UNAVAILABLE
        }
        BrowserManagedLaunchError::UnsupportedBrowser => {
            constants::value::MANAGED_BROWSER_UNSUPPORTED_EXECUTABLE
        }
        BrowserManagedLaunchError::ManualRequired => {
            constants::value::MANAGED_BROWSER_BRIDGE_ENDPOINT_MANUAL_REQUIRED
        }
        BrowserManagedLaunchError::Io => constants::value::MANAGED_BROWSER_LAUNCH_ERROR,
    }
}

pub(crate) fn reserve_managed_browser_bridge_port(
) -> Result<BrowserManagedBridgePortReservation, BrowserManagedLaunchError> {
    let endpoint = SocketAddr::new(
        IpAddr::V4(Ipv4Addr::LOCALHOST),
        constants::browser::DEVTOOLS_PORT_UNRESERVED,
    );
    let listener = TcpListener::bind(endpoint)
        .map_err(|_error| BrowserManagedLaunchError::BridgePortUnavailable)?;
    let reserved_endpoint = listener
        .local_addr()
        .map_err(|_error| BrowserManagedLaunchError::BridgePortUnavailable)?;
    if !reserved_endpoint.ip().is_loopback()
        || reserved_endpoint.port() == constants::browser::DEVTOOLS_PORT_UNRESERVED
    {
        return Err(BrowserManagedLaunchError::BridgePortUnavailable);
    }
    drop(listener);

    Ok(BrowserManagedBridgePortReservation {
        endpoint: reserved_endpoint,
        bridge_port: reserved_endpoint.port(),
    })
}

pub(crate) fn managed_browser_launch_plan(
    config: BrowserManagedLaunchConfig,
) -> Result<BrowserManagedLaunchPlan, BrowserManagedLaunchError> {
    let identity = managed_browser_executable_identity(&config.executable_path);
    if !identity.supports_managed_cdp {
        return Err(BrowserManagedLaunchError::UnsupportedBrowser);
    }
    if default_profile_path_rejected(&config.profile_dir) {
        return Err(BrowserManagedLaunchError::DefaultProfileRejected);
    }
    if !managed_profile_path_owned(&config.profile_dir) {
        return Err(BrowserManagedLaunchError::UnownedProfileRejected);
    }
    if config.bridge_port == constants::browser::DEVTOOLS_PORT_UNRESERVED {
        return Err(BrowserManagedLaunchError::BridgePortUnavailable);
    }

    let profile = config.profile_dir.to_string_lossy();
    let mut debugging_address =
        String::from(constants::browser::CHROMIUM_ARG_REMOTE_DEBUGGING_ADDRESS_PREFIX);
    debugging_address.push_str(constants::browser::CHROMIUM_REMOTE_DEBUGGING_LOOPBACK);
    let mut debugging = String::from(constants::browser::CHROMIUM_ARG_REMOTE_DEBUGGING_PORT_PREFIX);
    debugging.push_str(&config.bridge_port.to_string());
    let mut user_data = String::from(constants::browser::CHROMIUM_ARG_USER_DATA_DIR_PREFIX);
    user_data.push_str(&profile);
    let mut profile_directory =
        String::from(constants::browser::CHROMIUM_ARG_PROFILE_DIRECTORY_PREFIX);
    profile_directory.push_str(constants::browser::PROFILE_DIRECTORY_MANAGED_CHILD);

    Ok(BrowserManagedLaunchPlan {
        executable_path: config.executable_path,
        args: vec![
            debugging_address,
            debugging,
            user_data,
            profile_directory,
            constants::browser::CHROMIUM_ARG_NO_FIRST_RUN.to_string(),
            constants::browser::CHROMIUM_ARG_NO_DEFAULT_BROWSER_CHECK.to_string(),
            constants::browser::CHROMIUM_DEFAULT_URL.to_string(),
        ],
        bridge_port: config.bridge_port,
        browser_family: identity.browser_family,
        browser_channel: identity.browser_channel,
        profile_path_ref: constants::browser::PROFILE_PATH_REF_MANAGED.to_string(),
        bridge_endpoint_ref: constants::browser::BRIDGE_ENDPOINT_REF_LOOPBACK_DEVTOOLS.to_string(),
    })
}

pub(crate) fn launch_managed_browser(
    config: BrowserManagedLaunchConfig,
) -> Result<super::BrowserManagedLaunch, BrowserManagedLaunchError> {
    super::capability::launch_managed_browser(config)
}

pub(crate) fn default_profile_path_rejected(path: &Path) -> bool {
    normalized_component_names(path).iter().any(|name| {
        name == constants::browser::PATH_SEGMENT_DEFAULT_NORMALIZED
            || name == constants::browser::PATH_SEGMENT_USER_DATA_NORMALIZED
    })
}

pub(crate) fn managed_profile_path_owned(path: &Path) -> bool {
    path.file_name()
        .map(|name| {
            name.to_string_lossy()
                .to_ascii_lowercase()
                .starts_with(constants::browser::PROFILE_ID_PREFIX_MANAGED)
        })
        .unwrap_or(false)
}

pub(crate) fn normalized_component_names(path: &Path) -> Vec<String> {
    path.components()
        .map(|component| component.as_os_str().to_string_lossy().to_ascii_lowercase())
        .collect()
}
