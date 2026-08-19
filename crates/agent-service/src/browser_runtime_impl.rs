use ocentra_parent_agent_protocol::browser_managed::BrowserManagedSessionStatus;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogLevel;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventName;

use crate::{
    browser_payload::browser_managed_status_payload, browser_runtime_status::status_with_error,
    event_builder::build_event, time::timestamp_now,
};

use super::{BrowserManagedRuntime, BrowserRuntimeText};

pub(super) struct BrowserManagedRuntimeState {
    active: Option<launch::ManagedBrowserRuntimeLaunch>,
    terminal: Option<BrowserManagedSessionStatus>,
}

fn resolve_active_status(
    state: &mut BrowserManagedRuntimeState,
    checked_at: BrowserRuntimeText,
) -> Option<BrowserManagedSessionStatus> {
    let active = state.active.clone()?;
    if active.launch.expires_at_epoch_ms() <= current_epoch_millis() {
        let _ = active.launch.retire();
        state.active = None;
        let status = crate::browser_runtime_status::stopped_managed_status(
            checked_at,
            constants::value::BROWSER_BRIDGE_STALE_SESSION,
            &active.launch,
            &active.profile_store_entry,
            active.started_at,
        );
        state.terminal = Some(status.clone());
        return Some(status);
    }
    let result = bridge_poll_status(
        checked_at,
        &active.launch,
        &active.profile_store_entry,
        active.started_at.clone(),
    );
    if result.retire {
        let _ = active.launch.retire();
        state.active = None;
        let reason = result
            .status
            .degraded_reason
            .clone()
            .unwrap_or_else(|| constants::value::BROWSER_BRIDGE_STALE_SESSION.to_string());
        let status = crate::browser_runtime_status::stopped_managed_status(
            result.status.checked_at.clone(),
            reason,
            &active.launch,
            &active.profile_store_entry,
            active.started_at,
        );
        state.terminal = Some(status.clone());
        Some(status)
    } else {
        Some(result.status)
    }
}

impl BrowserManagedRuntimeState {
    pub(super) fn new() -> Self {
        Self {
            active: None,
            terminal: None,
        }
    }
}

#[path = "browser_runtime_impl/bridge.rs"]
mod bridge;
#[path = "browser_runtime_impl/config.rs"]
mod config;
#[path = "browser_runtime_impl/launch.rs"]
mod launch;

use self::bridge::bridge_poll_status;
use self::config::configured_bridge_port;
use self::launch::managed_profile_or_missing_status;

#[derive(Clone, Copy, Debug)]
struct BrowserRuntimeErrorText(&'static str);

pub async fn build_browser_managed_status_report(
    runtime: BrowserManagedRuntime,
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let status = tokio::task::spawn_blocking(move || runtime.resolve_status())
        .await
        .unwrap_or_else(|_| {
            status_with_error(
                timestamp_now::<String>(),
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

pub(super) fn resolve_browser_managed_status(
    runtime: &BrowserManagedRuntime,
) -> BrowserManagedSessionStatus {
    let checked_at: String = timestamp_now();
    let mut state = runtime
        .state
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());

    if let Some(status) = resolve_active_status(&mut state, BrowserRuntimeText(checked_at.clone()))
    {
        return status;
    }

    if let Some(status) = state.terminal.clone() {
        return status;
    }

    match configured_bridge_port() {
        Ok(Some(_)) => status_with_error(
            checked_at,
            constants::value::MANAGED_BROWSER_BRIDGE_ENDPOINT_MANUAL_REQUIRED,
        ),
        Ok(None) => managed_profile_or_missing_status(BrowserRuntimeText(checked_at)),
        Err(reason) => status_with_error(checked_at, reason.0),
    }
}

fn current_epoch_millis() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .ok()
        .and_then(|duration| u64::try_from(duration.as_millis()).ok())
        .unwrap_or(u64::MAX)
}
