use std::{env, path::PathBuf};

use ocentra_parent_agent_core::{
    browser_managed_discovery::{
        installed_managed_browser_candidates, BrowserManagedInstallCandidate,
    },
    browser_windows_inventory_source::live_windows_browser_inventory_candidate_paths_with_limit,
};
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

#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub(crate) struct BrowserInstallCandidatePaths(pub(crate) Vec<PathBuf>);

pub(crate) fn managed_browser_executable_path() -> Option<BrowserExecutablePath> {
    match env::var(constants::env_var::MANAGED_BROWSER_EXECUTABLE) {
        Ok(executable) => Some(BrowserExecutablePath(PathBuf::from(executable))),
        Err(env::VarError::NotPresent) => first_installed_managed_browser()
            .map(|candidate| BrowserExecutablePath(candidate.executable_path)),
        Err(_) => None,
    }
}

fn first_installed_managed_browser() -> Option<BrowserManagedInstallCandidate> {
    installed_managed_browser_candidates(&system_browser_candidate_paths().0)
        .into_iter()
        .next()
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
