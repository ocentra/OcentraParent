use std::{env, path::PathBuf};

use ocentra_parent_agent_protocol::constants;

pub(crate) fn live_windows_shortcut_roots() -> Vec<PathBuf> {
    [
        constants::env_var::PROGRAM_DATA,
        constants::env_var::APP_DATA,
    ]
    .iter()
    .filter_map(env::var_os)
    .map(start_menu_programs_root)
    .collect()
}

pub(crate) fn start_menu_programs_root(root: std::ffi::OsString) -> PathBuf {
    let mut path = PathBuf::from(root);
    path.push(constants::browser::PATH_SEGMENT_MICROSOFT);
    path.push(constants::browser::PATH_SEGMENT_WINDOWS);
    path.push(constants::browser::PATH_SEGMENT_START_MENU);
    path.push(constants::browser::PATH_SEGMENT_PROGRAMS);
    path
}
