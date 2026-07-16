#[cfg(windows)]
use std::{collections::BTreeMap, process::Command};

#[cfg(not(windows))]
use ocentra_parent_agent_protocol::activity_capture::ActivityCaptureCapabilityStatus;
#[cfg(windows)]
use ocentra_parent_agent_protocol::constants;
#[cfg(windows)]
use sysinfo::{ProcessesToUpdate, System};

use crate::network_capture::NetworkObservation;
#[cfg(windows)]
use crate::network_capture_netstat::netstat_observations;

#[cfg(windows)]
pub fn platform_network_snapshot(limit: usize) -> Result<Vec<NetworkObservation>, ()> {
    let output = Command::new(constants::activity_capture::NETSTAT_COMMAND)
        .args(constants::activity_capture::NETSTAT_ARGS)
        .output()
        .map_err(|_error| ())?;
    if !output.status.success() {
        return Err(());
    }
    let process_names = process_names_by_pid();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut observations = netstat_observations(&stdout, &process_names);
    observations.truncate(limit);
    Ok(observations)
}

#[cfg(windows)]
fn process_names_by_pid() -> BTreeMap<u32, String> {
    let mut system = System::new();
    system.refresh_processes(ProcessesToUpdate::All, true);
    system
        .processes()
        .values()
        .map(|process| {
            (
                process.pid().as_u32(),
                process.name().to_string_lossy().into_owned(),
            )
        })
        .collect()
}

#[cfg(not(windows))]
pub fn platform_network_snapshot(_: usize) -> Result<Vec<NetworkObservation>, ()> {
    Ok(vec![NetworkObservation::degraded(
        ActivityCaptureCapabilityStatus::Unavailable,
    )])
}
