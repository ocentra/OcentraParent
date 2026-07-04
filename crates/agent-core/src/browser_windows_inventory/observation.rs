use std::path::{Path, PathBuf};

use super::{
    BrowserWindowsExecutableIdentity, BrowserWindowsInventoryObservation, BrowserWindowsSupportKind,
};

#[path = "observation_paths.rs"]
mod observation_paths;
#[path = "observation_processes.rs"]
mod observation_processes;

pub(super) fn windows_browser_inventory_observations(
    candidate_paths: &[PathBuf],
    process_observations: &[crate::process_capture::ProcessObservation],
    managed_process_id: Option<u32>,
) -> Vec<BrowserWindowsInventoryObservation> {
    observation_paths::windows_browser_inventory_observations(
        candidate_paths,
        process_observations,
        managed_process_id,
    )
}

pub(super) fn windows_browser_executable_identity(path: &Path) -> BrowserWindowsExecutableIdentity {
    super::windows_browser_executable_identity(path)
}

pub(super) fn process_identity_path(process: &crate::process_capture::ProcessObservation) -> &Path {
    super::process_identity_path(process)
}

pub(super) fn install_state_from_path(
    path: &Path,
) -> ocentra_parent_agent_protocol::browser_inventory::BrowserInventoryInstallState {
    super::install_state_from_path(path)
}
