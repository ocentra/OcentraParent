use std::{
    path::{Path, PathBuf},
    process::Command,
};

use ocentra_parent_agent_protocol::{constants, BrowserChannel, BrowserFamily};

use crate::browser_managed_discovery::managed_browser_executable_identity;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BrowserManagedLaunchConfig {
    pub executable_path: PathBuf,
    pub profile_dir: PathBuf,
    pub bridge_port: u16,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BrowserManagedLaunchPlan {
    pub executable_path: PathBuf,
    pub args: Vec<String>,
    pub browser_family: BrowserFamily,
    pub browser_channel: BrowserChannel,
    pub profile_path_ref: String,
    pub bridge_endpoint_ref: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BrowserManagedLaunch {
    pub process_id: u32,
    pub browser_family: BrowserFamily,
    pub browser_channel: BrowserChannel,
    pub profile_path_ref: String,
    pub bridge_endpoint_ref: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BrowserManagedLaunchError {
    DefaultProfileRejected,
    UnownedProfileRejected,
    UnsupportedBrowser,
    Io,
}

impl BrowserManagedLaunchError {
    pub fn reason(&self) -> &'static str {
        match self {
            Self::DefaultProfileRejected | Self::UnownedProfileRejected => {
                constants::value::MANAGED_BROWSER_INVALID_PROFILE
            }
            Self::UnsupportedBrowser => constants::value::MANAGED_BROWSER_UNSUPPORTED_EXECUTABLE,
            Self::Io => constants::value::MANAGED_BROWSER_LAUNCH_ERROR,
        }
    }
}

pub fn managed_browser_launch_plan(
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
        browser_family: identity.browser_family,
        browser_channel: identity.browser_channel,
        profile_path_ref: constants::browser::PROFILE_PATH_REF_MANAGED.to_string(),
        bridge_endpoint_ref: constants::browser::BRIDGE_ENDPOINT_REF_LOOPBACK_DEVTOOLS.to_string(),
    })
}

pub fn launch_managed_browser(
    config: BrowserManagedLaunchConfig,
) -> Result<BrowserManagedLaunch, BrowserManagedLaunchError> {
    let plan = managed_browser_launch_plan(config)?;
    let child = Command::new(&plan.executable_path)
        .args(&plan.args)
        .spawn()
        .map_err(|_| BrowserManagedLaunchError::Io)?;

    Ok(BrowserManagedLaunch {
        process_id: child.id(),
        browser_family: plan.browser_family,
        browser_channel: plan.browser_channel,
        profile_path_ref: plan.profile_path_ref,
        bridge_endpoint_ref: plan.bridge_endpoint_ref,
    })
}

fn default_profile_path_rejected(path: &Path) -> bool {
    normalized_component_names(path).iter().any(|name| {
        name == constants::browser::PATH_SEGMENT_DEFAULT_NORMALIZED
            || name == constants::browser::PATH_SEGMENT_USER_DATA_NORMALIZED
    })
}

fn managed_profile_path_owned(path: &Path) -> bool {
    normalized_component_names(path)
        .iter()
        .any(|name| name.starts_with(constants::browser::PROFILE_ID_PREFIX_MANAGED))
}

fn normalized_component_names(path: &Path) -> Vec<String> {
    path.components()
        .map(|component| component.as_os_str().to_string_lossy().to_ascii_lowercase())
        .collect()
}
