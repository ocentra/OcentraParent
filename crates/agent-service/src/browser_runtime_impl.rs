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

#[path = "browser_runtime_impl/bridge.rs"]
mod bridge;
#[path = "browser_runtime_impl/config.rs"]
mod config;
#[path = "browser_runtime_impl/launch.rs"]
mod launch;

use self::bridge::bridge_poll_status;
use self::config::{configured_bridge_port, launch_on_status_enabled};
use self::launch::{launch_managed_browser_status, managed_profile_or_missing_status};

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

fn resolve_browser_managed_status() -> BrowserManagedSessionStatus {
    let checked_at: String = timestamp_now();
    match configured_bridge_port() {
        Ok(Some(port)) => bridge_poll_status(BrowserRuntimeText(checked_at), port),
        Ok(None) => {
            if launch_on_status_enabled() {
                return launch_managed_browser_status(BrowserRuntimeText(checked_at));
            }
            managed_profile_or_missing_status(BrowserRuntimeText(checked_at))
        }
        Err(reason) => status_with_error(checked_at, reason.0),
    }
}
