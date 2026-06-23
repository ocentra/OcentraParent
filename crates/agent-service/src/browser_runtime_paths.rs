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

pub(crate) fn managed_browser_executable_path() -> Option<PathBuf> {
    match env::var(constants::env_var::MANAGED_BROWSER_EXECUTABLE) {
        Ok(executable) => Some(PathBuf::from(executable)),
        Err(env::VarError::NotPresent) => {
            first_installed_managed_browser().map(|candidate| candidate.executable_path)
        }
        Err(_) => None,
    }
}

pub(crate) fn managed_browser_profile_store(
) -> Result<BrowserManagedProfileStoreRecord, &'static str> {
    let profile_root_dir = match env::var(constants::env_var::MANAGED_BROWSER_PROFILE_DIR) {
        Ok(profile_root_dir) => PathBuf::from(profile_root_dir),
        Err(env::VarError::NotPresent) => default_managed_browser_profile_root_dir(),
        Err(_) => return Err(constants::value::MANAGED_BROWSER_PROFILE_DIR_MISSING),
    };

    let config = BrowserManagedProfileStoreConfig {
        profile_root_dir,
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
        constants::value::MANAGED_BROWSER_PROFILE_DIR_MISSING
    })
}

fn first_installed_managed_browser() -> Option<BrowserManagedInstallCandidate> {
    installed_managed_browser_candidates(&system_browser_candidate_paths())
        .into_iter()
        .next()
}

fn default_managed_browser_profile_root_dir() -> PathBuf {
    let base = env::var(constants::env_var::LOCAL_APP_DATA)
        .map(PathBuf::from)
        .unwrap_or_else(|_| env::temp_dir());
    base.join(constants::browser::PATH_SEGMENT_OCENTRA_PARENT)
        .join(constants::browser::PATH_SEGMENT_MANAGED_BROWSER)
}

pub(crate) fn system_browser_candidate_paths() -> Vec<PathBuf> {
    let mut roots = Vec::new();
    if let Ok(root) = env::var(constants::env_var::PROGRAM_FILES) {
        roots.push(PathBuf::from(root));
    }
    if let Ok(root) = env::var(constants::env_var::PROGRAM_FILES_X86) {
        roots.push(PathBuf::from(root));
    }
    if let Ok(root) = env::var(constants::env_var::LOCAL_APP_DATA) {
        roots.push(PathBuf::from(root));
    }
    live_windows_browser_inventory_candidate_paths_with_limit(
        &roots,
        constants::browser::PROCESS_SCAN_LIMIT_BROWSER_DISCOVERY,
    )
}
