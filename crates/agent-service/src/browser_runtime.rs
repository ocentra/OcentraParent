use std::{
    env,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::PathBuf,
};

use ocentra_parent_agent_core::{
    collect_process_snapshot, launch_managed_browser, poll_chromium_bridge,
    unmanaged_browser_processes, BrowserBridgePollConfig, BrowserManagedLaunchConfig,
    BrowserUnmanagedProcessObservation,
};
use ocentra_parent_agent_protocol::{
    constants, AgentCommandEnvelope, AgentEventEnvelope, AgentEventName, BrowserCapabilityStatus,
    BrowserChannel, BrowserFamily, BrowserManagedSessionStatus, LogLevel,
};

use crate::{
    activity_capture::record_activity_events_to_paths,
    activity_store_path::{activity_db_path, activity_journal_key_path, activity_journal_path},
    browser_payload::browser_managed_status_payload,
    browser_runtime_status::{
        bridge_disconnected_status, connected_status, missing_browser_status,
        profile_missing_status, running_managed_status, status_with_error,
        unmanaged_browser_status,
    },
    event_builder::build_event,
    time::timestamp_now,
};

pub async fn build_browser_managed_status_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let status = tokio::task::spawn_blocking(resolve_browser_managed_status)
        .await
        .unwrap_or_else(|_| {
            status_with_error(
                timestamp_now(),
                constants::value::MANAGED_BROWSER_LAUNCH_ERROR,
            )
        });

    build_event(
        constants::event_id::BROWSER_MANAGED_STATUS_REPORTED,
        &command.message_id,
        command.source,
        AgentEventName::AgentBrowserManagedStatusReported,
        LogLevel::Info,
        browser_managed_status_payload(&status),
        None,
    )
}

fn resolve_browser_managed_status() -> BrowserManagedSessionStatus {
    let checked_at = timestamp_now();
    match configured_bridge_port() {
        Ok(Some(port)) => bridge_poll_status(checked_at, port),
        Ok(None) => launch_or_missing_status(checked_at),
        Err(reason) => status_with_error(checked_at, reason),
    }
}

fn bridge_poll_status(checked_at: String, port: u16) -> BrowserManagedSessionStatus {
    let config = BrowserBridgePollConfig {
        endpoint: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), port),
        managed_browser_session_id: constants::browser::SESSION_ID_DEV.to_string(),
        profile_id: constants::browser::PROFILE_ID_DEV.to_string(),
        process_id: constants::browser::PROCESS_ID_UNKNOWN,
        browser_family: BrowserFamily::UnknownChromium,
        browser_channel: BrowserChannel::Unknown,
    };

    match poll_chromium_bridge(config, &checked_at, &checked_at) {
        Ok(snapshot) => {
            if let Err(error) = record_activity_events_to_paths(
                &activity_journal_path(),
                &activity_journal_key_path(),
                &activity_db_path(),
                &snapshot.events,
            ) {
                return connected_status(
                    checked_at,
                    snapshot.browser_version,
                    BrowserCapabilityStatus::AdapterError,
                    Some(error.reason().to_string()),
                );
            }
            connected_status(
                checked_at,
                snapshot.browser_version,
                BrowserCapabilityStatus::TabListOnly,
                browser_target_degraded_reason(snapshot.page_target_count),
            )
        }
        Err(error) => bridge_disconnected_status(checked_at, error.reason()),
    }
}

fn launch_or_missing_status(checked_at: String) -> BrowserManagedSessionStatus {
    let Ok(executable) = env::var(constants::env_var::MANAGED_BROWSER_EXECUTABLE) else {
        if let Some(process) = first_unmanaged_browser_process() {
            return unmanaged_browser_status(
                checked_at,
                process.process_id,
                process.browser_family,
                process.browser_channel,
            );
        }
        return missing_browser_status(checked_at);
    };
    let Ok(profile_dir) = env::var(constants::env_var::MANAGED_BROWSER_PROFILE_DIR) else {
        return profile_missing_status(checked_at);
    };

    let config = BrowserManagedLaunchConfig {
        executable_path: PathBuf::from(executable),
        profile_dir: PathBuf::from(profile_dir),
        bridge_port: constants::browser::DEVTOOLS_DEFAULT_BRIDGE_PORT,
    };

    match launch_managed_browser(config) {
        Ok(launch) => running_managed_status(
            checked_at,
            launch.process_id,
            launch.browser_family,
            launch.browser_channel,
        ),
        Err(error) => status_with_error(checked_at, error.reason()),
    }
}

fn browser_target_degraded_reason(page_target_count: usize) -> Option<String> {
    if page_target_count == 0 {
        return Some(constants::value::BROWSER_BRIDGE_NO_PAGE_TARGETS.to_string());
    }
    None
}

fn first_unmanaged_browser_process() -> Option<BrowserUnmanagedProcessObservation> {
    let observations =
        collect_process_snapshot(constants::browser::PROCESS_SCAN_LIMIT_BROWSER_DISCOVERY);
    unmanaged_browser_processes(&observations, None)
        .into_iter()
        .next()
}

fn configured_bridge_port() -> Result<Option<u16>, &'static str> {
    match env::var(constants::env_var::MANAGED_BROWSER_BRIDGE_PORT) {
        Ok(port) => port
            .parse::<u16>()
            .map(Some)
            .map_err(|_| constants::value::MANAGED_BROWSER_INVALID_BRIDGE_PORT),
        Err(env::VarError::NotPresent) => Ok(None),
        Err(_) => Err(constants::value::MANAGED_BROWSER_INVALID_BRIDGE_PORT),
    }
}
