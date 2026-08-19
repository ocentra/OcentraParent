use ocentra_parent_agent_core::{
    browser_managed_discovery::{BrowserUnmanagedProcessObservation, unmanaged_browser_processes},
    browser_managed_session::{
        BrowserManagedLaunchConfig, launch_managed_browser, managed_browser_launch_plan,
        reserve_managed_browser_bridge_port,
    },
    process_capture::collect_process_snapshot,
};
use ocentra_parent_agent_protocol::browser_managed::BrowserManagedSessionStatus;
use ocentra_parent_agent_protocol::constants;

use crate::{
    browser_runtime_paths::{managed_browser_executable_path, managed_browser_profile_store},
    browser_runtime_status::{
        managed_profile_ready_status, missing_browser_status, profile_missing_status,
        status_with_error, unmanaged_browser_status,
    },
};

use super::BrowserRuntimeText;

#[derive(Clone)]
pub(super) struct ManagedBrowserRuntimeLaunch {
    pub(super) launch: ocentra_parent_agent_core::browser_managed_session::BrowserManagedLaunch,
    pub(super) profile_store_entry:
        ocentra_parent_agent_protocol::browser_managed::BrowserManagedProfileStoreEntry,
    pub(super) started_at: BrowserRuntimeText,
}

pub(super) fn managed_profile_or_missing_status(
    checked_at: BrowserRuntimeText,
) -> BrowserManagedSessionStatus {
    let checked_at = checked_at.0;
    let Some(executable) = managed_browser_executable_path() else {
        if let Some(process) = first_unmanaged_browser_process() {
            return unmanaged_browser_status(checked_at, process);
        }
        return missing_browser_status(checked_at);
    };

    let Ok(profile_store) = managed_browser_profile_store() else {
        return profile_missing_status(checked_at);
    };

    let config = BrowserManagedLaunchConfig {
        executable_path: executable.into(),
        profile_dir: profile_store.profile_dir,
        bridge_port: constants::browser::DEVTOOLS_DEFAULT_BRIDGE_PORT,
    };

    match managed_browser_launch_plan(config) {
        Ok(plan) => managed_profile_ready_status(
            checked_at,
            plan.browser_family,
            plan.browser_channel,
            profile_store.entry,
        ),
        Err(error) => status_with_error(checked_at, error.reason()),
    }
}

pub(super) fn launch_managed_browser_status(
    checked_at: BrowserRuntimeText,
) -> Result<ManagedBrowserRuntimeLaunch, BrowserManagedSessionStatus> {
    let checked_at = checked_at.0;
    let Some(executable) = managed_browser_executable_path() else {
        return Err(missing_browser_status(checked_at));
    };
    let Ok(profile_store) = managed_browser_profile_store() else {
        return Err(profile_missing_status(checked_at));
    };
    let reservation = match reserve_managed_browser_bridge_port() {
        Ok(reservation) => reservation,
        Err(error) => return Err(status_with_error(checked_at, error.reason())),
    };
    let config = BrowserManagedLaunchConfig {
        executable_path: executable.into(),
        profile_dir: profile_store.profile_dir,
        bridge_port: reservation.bridge_port,
    };

    match launch_managed_browser(config) {
        Ok(launch) => Ok(ManagedBrowserRuntimeLaunch {
            launch,
            profile_store_entry: profile_store.entry,
            started_at: BrowserRuntimeText(checked_at),
        }),
        Err(error) => Err(status_with_error(checked_at, error.reason())),
    }
}

fn first_unmanaged_browser_process() -> Option<BrowserUnmanagedProcessObservation> {
    let observations =
        collect_process_snapshot(constants::browser::PROCESS_SCAN_LIMIT_BROWSER_DISCOVERY);
    unmanaged_browser_processes(&observations, None)
        .into_iter()
        .next()
}
