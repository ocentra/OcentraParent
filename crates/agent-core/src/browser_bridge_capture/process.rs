use std::path::Path;

use ocentra_parent_agent_protocol::constants;
use sysinfo::{Pid, ProcessRefreshKind, ProcessesToUpdate, System};

use super::{authority::LaunchBinding, binding, ManagedBrowserCdpCaptureError};
use crate::{
    browser_bridge_poll::BrowserBridgePollError, browser_managed_session::managed_path_binding,
};

pub(super) fn revalidate(binding: &LaunchBinding) -> Result<(), ManagedBrowserCdpCaptureError> {
    binding::validate(binding)?;
    let mut system = System::new();
    let pid = Pid::from_u32(binding.process_id);
    let pids = [pid];
    system.refresh_processes_specifics(
        ProcessesToUpdate::Some(&pids),
        true,
        ProcessRefreshKind::everything(),
    );
    let process = system
        .process(pid)
        .ok_or(BrowserBridgePollError::UntrustedProcess)?;
    let executable = process
        .exe()
        .ok_or(BrowserBridgePollError::UntrustedProcess)?;
    if managed_path_binding(executable) != binding.executable_binding {
        return Err(BrowserBridgePollError::UntrustedProcess.into());
    }
    if !profile_argument_matches(process, binding) {
        return Err(BrowserBridgePollError::UntrustedProfile.into());
    }
    Ok(())
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
                    managed_path_binding(Path::new(profile.trim_matches('"')))
                        == binding.profile_binding
                })
                .unwrap_or(false)
        })
}
