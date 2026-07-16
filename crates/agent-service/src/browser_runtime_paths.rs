use std::{env, path::PathBuf};

use ocentra_parent_agent_core::{
    browser_managed_discovery::{
        installed_managed_browser_candidates, BrowserManagedInstallCandidate,
    },
    browser_managed_session::{
        create_or_repair_managed_browser_profile_store, BrowserManagedProfileStoreConfig,
        BrowserManagedProfileStoreRecord,
    },
    browser_windows_inventory_source::live_windows_browser_inventory_candidate_paths_with_limit,
};
use ocentra_parent_agent_protocol::browser::{BrowserChannel, BrowserFamily};
use ocentra_parent_agent_protocol::constants;

const BROWSER_CANDIDATE_ROOT_ENV_VARS: [&str; 3] = [
    constants::env_var::PROGRAM_FILES,
    constants::env_var::PROGRAM_FILES_X86,
    constants::env_var::LOCAL_APP_DATA,
];

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct BrowserExecutablePath(PathBuf);

impl From<BrowserExecutablePath> for PathBuf {
    fn from(value: BrowserExecutablePath) -> Self {
        value.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct BrowserProfileRootDir(PathBuf);

impl From<BrowserProfileRootDir> for PathBuf {
    fn from(value: BrowserProfileRootDir) -> Self {
        value.0
    }
}

#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub(crate) struct BrowserInstallCandidatePaths(pub(crate) Vec<PathBuf>);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct BrowserRuntimePathErrorText(pub(crate) &'static str);

pub(crate) fn managed_browser_executable_path() -> Option<BrowserExecutablePath> {
    match env::var(constants::env_var::MANAGED_BROWSER_EXECUTABLE) {
        Ok(executable) => Some(BrowserExecutablePath(PathBuf::from(executable))),
        Err(env::VarError::NotPresent) => first_installed_managed_browser()
            .map(|candidate| BrowserExecutablePath(candidate.executable_path)),
        Err(_) => None,
    }
}

pub(crate) fn managed_browser_profile_store(
) -> Result<BrowserManagedProfileStoreRecord, BrowserRuntimePathErrorText> {
    let profile_root_dir = match env::var(constants::env_var::MANAGED_BROWSER_PROFILE_DIR) {
        Ok(profile_root_dir) => BrowserProfileRootDir(PathBuf::from(profile_root_dir)),
        Err(env::VarError::NotPresent) => default_managed_browser_profile_root_dir(),
        Err(_) => {
            return Err(BrowserRuntimePathErrorText(
                constants::value::MANAGED_BROWSER_PROFILE_DIR_MISSING,
            ));
        }
    };

    let config = BrowserManagedProfileStoreConfig {
        profile_root_dir: profile_root_dir.into(),
        profile_id: constants::browser::PROFILE_ID_DEV.to_string(),
        profile_scope_id: constants::browser::PROFILE_SCOPE_ID_DEV.to_string(),
        device_id: constants::browser::PROFILE_STORE_TEST_DEVICE_ID.to_string(),
        browser_family: BrowserFamily::UnknownChromium,
        browser_channel: BrowserChannel::Unknown,
        policy_revision: constants::browser::PROFILE_POLICY_REVISION_DEV.to_string(),
        now: crate::time::timestamp_now(),
    };

    create_or_repair_managed_browser_profile_store(&config).map_err(|error| {
        let _ = error;
        BrowserRuntimePathErrorText(constants::value::MANAGED_BROWSER_PROFILE_DIR_MISSING)
    })
}

fn first_installed_managed_browser() -> Option<BrowserManagedInstallCandidate> {
    installed_managed_browser_candidates(&system_browser_candidate_paths().0)
        .into_iter()
        .next()
}

fn default_managed_browser_profile_root_dir() -> BrowserProfileRootDir {
    let base = env::var(constants::env_var::LOCAL_APP_DATA)
        .map(PathBuf::from)
        .unwrap_or_else(|_| env::temp_dir());
    BrowserProfileRootDir(
        base.join(constants::browser::PATH_SEGMENT_OCENTRA_PARENT)
            .join(constants::browser::PATH_SEGMENT_MANAGED_BROWSER),
    )
}

pub(crate) fn system_browser_candidate_paths() -> BrowserInstallCandidatePaths {
    let roots = BROWSER_CANDIDATE_ROOT_ENV_VARS
        .iter()
        .filter_map(|env_var_name| env::var(env_var_name).ok())
        .map(PathBuf::from)
        .collect::<Vec<_>>();
    BrowserInstallCandidatePaths(live_windows_browser_inventory_candidate_paths_with_limit(
        &roots,
        constants::browser::PROCESS_SCAN_LIMIT_BROWSER_DISCOVERY,
    ))
}
