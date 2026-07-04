use std::{
    env,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};

use ocentra_parent_agent_core::{
    browser_bridge_poll::{
        poll_chromium_bridge, BrowserBridgeExpectedCustody, BrowserBridgePollConfig,
    },
    browser_managed_discovery::{unmanaged_browser_processes, BrowserUnmanagedProcessObservation},
    browser_managed_session::{
        launch_managed_browser, managed_browser_launch_plan, reserve_managed_browser_bridge_port,
        BrowserManagedLaunchConfig,
    },
    process_capture::collect_process_snapshot,
};
use ocentra_parent_agent_protocol::browser::BrowserCapabilityStatus;
use ocentra_parent_agent_protocol::browser::BrowserChannel;
use ocentra_parent_agent_protocol::browser::BrowserFamily;
use ocentra_parent_agent_protocol::browser_managed::BrowserManagedSessionStatus;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogLevel;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventName;

use crate::{
    activity_capture::record_activity_events_to_paths,
    activity_store_path::{activity_db_path, activity_journal_key_path, activity_journal_path},
    browser_payload::browser_managed_status_payload,
    browser_runtime_paths::{managed_browser_executable_path, managed_browser_profile_store},
    browser_runtime_status::{
        bridge_disconnected_status, connected_status, managed_profile_ready_status,
        missing_browser_status, profile_missing_status, running_managed_status, status_with_error,
        unmanaged_browser_status,
    },
    event_builder::build_event,
    time::timestamp_now,
};

#[derive(Clone, Debug)]
struct BrowserRuntimeText(String);

#[derive(Clone, Copy, Debug)]
struct BrowserRuntimeErrorText(&'static str);

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
        Ok(Some(port)) => bridge_poll_status(BrowserRuntimeText(checked_at.clone()), port),
        Ok(None) => {
            if launch_on_status_enabled() {
                return launch_managed_browser_status(BrowserRuntimeText(checked_at.clone()));
            }
            managed_profile_or_missing_status(BrowserRuntimeText(checked_at.clone()))
        }
        Err(reason) => status_with_error(checked_at, reason.0),
    }
}

fn bridge_poll_status(checked_at: BrowserRuntimeText, port: u16) -> BrowserManagedSessionStatus {
    let checked_at = checked_at.0;
    let config = BrowserBridgePollConfig {
        endpoint: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), port),
        managed_browser_session_id: constants::browser::SESSION_ID_DEV.to_string(),
        profile_id: constants::browser::PROFILE_ID_DEV.to_string(),
        process_id: constants::browser::PROCESS_ID_UNKNOWN,
        browser_family: BrowserFamily::UnknownChromium,
        browser_channel: BrowserChannel::Unknown,
        expected_custody: BrowserBridgeExpectedCustody {
            bridge_port: port,
            managed_browser_session_id: constants::browser::SESSION_ID_DEV.to_string(),
            profile_id: constants::browser::PROFILE_ID_DEV.to_string(),
            process_id: constants::browser::PROCESS_ID_UNKNOWN,
            browser_family: BrowserFamily::UnknownChromium,
            browser_channel: BrowserChannel::Unknown,
            session_fresh_until: checked_at.clone(),
        },
    };

    match poll_chromium_bridge(&config, &checked_at, &checked_at) {
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
                browser_target_degraded_reason(snapshot.page_target_count).map(|reason| reason.0),
            )
        }
        Err(error) => bridge_disconnected_status(checked_at, error.reason()),
    }
}

fn managed_profile_or_missing_status(
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
        executable_path: executable,
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

fn launch_managed_browser_status(checked_at: BrowserRuntimeText) -> BrowserManagedSessionStatus {
    let checked_at = checked_at.0;
    let Some(executable) = managed_browser_executable_path() else {
        return missing_browser_status(checked_at);
    };
    let Ok(profile_store) = managed_browser_profile_store() else {
        return profile_missing_status(checked_at);
    };
    let reservation = match reserve_managed_browser_bridge_port() {
        Ok(reservation) => reservation,
        Err(error) => return status_with_error(checked_at, error.reason()),
    };
    let config = BrowserManagedLaunchConfig {
        executable_path: executable,
        profile_dir: profile_store.profile_dir,
        bridge_port: reservation.bridge_port,
    };

    match launch_managed_browser(config) {
        Ok(launch) => {
            running_managed_status(checked_at.clone(), launch, profile_store.entry, checked_at)
        }
        Err(error) => status_with_error(checked_at, error.reason()),
    }
}

fn browser_target_degraded_reason(page_target_count: usize) -> Option<BrowserRuntimeText> {
    if page_target_count == 0 {
        return Some(BrowserRuntimeText(
            constants::value::BROWSER_BRIDGE_NO_PAGE_TARGETS.to_string(),
        ));
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

fn configured_bridge_port() -> Result<Option<u16>, BrowserRuntimeErrorText> {
    match env::var(constants::env_var::MANAGED_BROWSER_BRIDGE_PORT) {
        Ok(port) => port.parse::<u16>().map(Some).map_err(|error| {
            let _ = error;
            BrowserRuntimeErrorText(constants::value::MANAGED_BROWSER_INVALID_BRIDGE_PORT)
        }),
        Err(env::VarError::NotPresent) => Ok(None),
        Err(_) => Err(BrowserRuntimeErrorText(
            constants::value::MANAGED_BROWSER_INVALID_BRIDGE_PORT,
        )),
    }
}

fn launch_on_status_enabled() -> bool {
    env::var(constants::env_var::MANAGED_BROWSER_LAUNCH_ON_STATUS)
        .map(|value| value == constants::value::TRUE)
        .unwrap_or(false)
}
