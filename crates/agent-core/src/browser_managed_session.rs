use std::{
    path::{Path, PathBuf},
    process::Command,
};

use ocentra_parent_agent_protocol::constants;

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
    pub profile_path_ref: String,
    pub bridge_endpoint_ref: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BrowserManagedLaunch {
    pub process_id: u32,
    pub profile_path_ref: String,
    pub bridge_endpoint_ref: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BrowserManagedLaunchError {
    DefaultProfileRejected,
    Io,
}

impl BrowserManagedLaunchError {
    pub fn reason(&self) -> &'static str {
        match self {
            Self::DefaultProfileRejected => constants::value::MANAGED_BROWSER_INVALID_PROFILE,
            Self::Io => constants::value::MANAGED_BROWSER_LAUNCH_ERROR,
        }
    }
}

pub fn managed_browser_launch_plan(
    config: BrowserManagedLaunchConfig,
) -> Result<BrowserManagedLaunchPlan, BrowserManagedLaunchError> {
    if !managed_profile_path_allowed(&config.profile_dir) {
        return Err(BrowserManagedLaunchError::DefaultProfileRejected);
    }

    let profile = config.profile_dir.to_string_lossy();
    let mut debugging = String::from(constants::browser::CHROMIUM_ARG_REMOTE_DEBUGGING_PORT_PREFIX);
    debugging.push_str(&config.bridge_port.to_string());
    let mut user_data = String::from(constants::browser::CHROMIUM_ARG_USER_DATA_DIR_PREFIX);
    user_data.push_str(&profile);

    Ok(BrowserManagedLaunchPlan {
        executable_path: config.executable_path,
        args: vec![
            debugging,
            user_data,
            constants::browser::CHROMIUM_ARG_NO_FIRST_RUN.to_string(),
            constants::browser::CHROMIUM_ARG_NO_DEFAULT_BROWSER_CHECK.to_string(),
            constants::browser::CHROMIUM_DEFAULT_URL.to_string(),
        ],
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
        profile_path_ref: plan.profile_path_ref,
        bridge_endpoint_ref: plan.bridge_endpoint_ref,
    })
}

fn managed_profile_path_allowed(path: &Path) -> bool {
    !path.components().any(|component| {
        let name = component.as_os_str().to_string_lossy();
        name == constants::browser::PATH_SEGMENT_DEFAULT
            || name == constants::browser::PATH_SEGMENT_USER_DATA
    })
}
