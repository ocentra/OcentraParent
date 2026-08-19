use std::path::Path;

use ocentra_parent_agent_protocol::constants;
use sysinfo::{Pid, ProcessRefreshKind, ProcessesToUpdate, System};

use super::{ManagedBrowserCdpCaptureError, authority::LaunchBinding, binding};
use crate::browser_bridge_poll::BrowserBridgePollError;

pub(super) fn revalidate(binding: &LaunchBinding) -> Result<(), ManagedBrowserCdpCaptureError> {
    binding::validate(binding)?;
    super::verify_managed_browser_cdp_endpoint(
        binding.endpoint,
        binding.process_id,
        &binding.executable_path,
    )
    .map_err(ManagedBrowserCdpCaptureError::from)?;
    let system = refreshed_process(binding.process_id)?;
    let process = system
        .process(Pid::from_u32(binding.process_id))
        .ok_or(BrowserBridgePollError::UntrustedProcess)?;
    if !profile_argument_matches(process, binding) {
        return Err(BrowserBridgePollError::UntrustedProfile.into());
    }
    Ok(())
}

pub(super) fn retire(binding: &LaunchBinding) -> bool {
    let Ok(system) = refreshed_process(binding.process_id) else {
        return true;
    };
    let pid = Pid::from_u32(binding.process_id);
    let Some(process) = system.process(pid) else {
        return true;
    };
    let executable_matches = process
        .exe()
        .is_some_and(|path| paths_match(path, &binding.executable_path));
    if !executable_matches || !profile_argument_matches(process, binding) {
        return false;
    }
    process.kill()
}

pub(super) fn verify_process_executable(
    process_id: u32,
    expected_path: &Path,
) -> Result<(), BrowserBridgePollError> {
    let system = refreshed_process(process_id)?;
    let process = system
        .process(Pid::from_u32(process_id))
        .ok_or(BrowserBridgePollError::UntrustedProcess)?;
    let executable = process
        .exe()
        .ok_or(BrowserBridgePollError::UntrustedProcess)?;
    if !paths_match(executable, expected_path) {
        return Err(BrowserBridgePollError::UntrustedProcess);
    }
    Ok(())
}

fn refreshed_process(process_id: u32) -> Result<System, BrowserBridgePollError> {
    let mut system = System::new();
    let pid = Pid::from_u32(process_id);
    let pids = [pid];
    system.refresh_processes_specifics(
        ProcessesToUpdate::Some(&pids),
        true,
        ProcessRefreshKind::everything(),
    );
    if system.process(pid).is_none() {
        return Err(BrowserBridgePollError::UntrustedProcess);
    }
    Ok(system)
}

fn profile_argument_matches(process: &sysinfo::Process, binding: &LaunchBinding) -> bool {
    process
        .cmd()
        .iter()
        .filter_map(|argument| argument.to_str())
        .any(|argument| {
            argument
                .strip_prefix(constants::browser::CHROMIUM_ARG_USER_DATA_DIR_PREFIX)
                .map(|profile| {
                    paths_match(Path::new(profile.trim_matches('"')), &binding.profile_path)
                })
                .unwrap_or(false)
        })
}

fn paths_match(left: &Path, right: &Path) -> bool {
    let left = std::fs::canonicalize(left).unwrap_or_else(|_error| left.to_path_buf());
    let right = std::fs::canonicalize(right).unwrap_or_else(|_error| right.to_path_buf());
    left.to_string_lossy()
        .eq_ignore_ascii_case(right.to_string_lossy().as_ref())
}
