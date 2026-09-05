use std::{env, path::PathBuf};

use ocentra_parent_agent_core::browser_windows_inventory_source::live_windows_browser_inventory_candidate_paths_with_limit;
use ocentra_parent_agent_protocol::constants;

const BROWSER_CANDIDATE_ROOT_ENV_VARS: [&str; 3] = [
    constants::env_var::PROGRAM_FILES,
    constants::env_var::PROGRAM_FILES_X86,
    constants::env_var::LOCAL_APP_DATA,
];

#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub(crate) struct BrowserInstallCandidatePaths(pub(crate) Vec<PathBuf>);

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
