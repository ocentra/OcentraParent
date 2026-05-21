use std::{env, fs, path::PathBuf};

use ocentra_parent_agent_core::{
    installed_managed_browser_candidates, BrowserManagedInstallCandidate,
};
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

pub(crate) fn managed_browser_profile_dir() -> Result<PathBuf, &'static str> {
    let profile_dir = match env::var(constants::env_var::MANAGED_BROWSER_PROFILE_DIR) {
        Ok(profile_dir) => PathBuf::from(profile_dir),
        Err(env::VarError::NotPresent) => default_managed_browser_profile_dir(),
        Err(_) => return Err(constants::value::MANAGED_BROWSER_PROFILE_DIR_MISSING),
    };

    fs::create_dir_all(&profile_dir)
        .map(|()| profile_dir)
        .map_err(|_| constants::value::MANAGED_BROWSER_PROFILE_DIR_MISSING)
}

fn first_installed_managed_browser() -> Option<BrowserManagedInstallCandidate> {
    installed_managed_browser_candidates(&system_browser_candidate_paths())
        .into_iter()
        .next()
}

fn default_managed_browser_profile_dir() -> PathBuf {
    let base = env::var(constants::env_var::LOCAL_APP_DATA)
        .map(PathBuf::from)
        .unwrap_or_else(|_| env::temp_dir());
    base.join(constants::browser::PATH_SEGMENT_OCENTRA_PARENT)
        .join(constants::browser::PATH_SEGMENT_MANAGED_BROWSER)
        .join(constants::browser::PROFILE_ID_DEV)
}

fn system_browser_candidate_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();
    if let Ok(root) = env::var(constants::env_var::PROGRAM_FILES) {
        push_windows_browser_candidates(&mut paths, PathBuf::from(root));
    }
    if let Ok(root) = env::var(constants::env_var::PROGRAM_FILES_X86) {
        push_windows_browser_candidates(&mut paths, PathBuf::from(root));
    }
    if let Ok(root) = env::var(constants::env_var::LOCAL_APP_DATA) {
        push_windows_browser_candidates(&mut paths, PathBuf::from(root));
    }
    paths
}

fn push_windows_browser_candidates(paths: &mut Vec<PathBuf>, root: PathBuf) {
    paths.push(
        root.join(constants::browser::PATH_SEGMENT_MICROSOFT)
            .join(constants::browser::PATH_SEGMENT_EDGE)
            .join(constants::browser::PATH_SEGMENT_APPLICATION)
            .join(constants::browser::EXECUTABLE_MSEDGE_WINDOWS),
    );
    paths.push(
        root.join(constants::browser::PATH_SEGMENT_GOOGLE)
            .join(constants::browser::PATH_SEGMENT_CHROME)
            .join(constants::browser::PATH_SEGMENT_APPLICATION)
            .join(constants::browser::EXECUTABLE_CHROME_WINDOWS),
    );
}
