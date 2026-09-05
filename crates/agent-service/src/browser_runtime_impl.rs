use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogLevel;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventName;

use crate::{
    browser_payload::browser_managed_status_payload, browser_runtime_status::status_with_error,
    event_builder::build_event, time::timestamp_now,
};

use super::BrowserManagedRuntime;

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
) -> ocentra_parent_agent_protocol::browser_managed::BrowserManagedSessionStatus {
    let checked_at: String = timestamp_now();
    status_with_error(
        checked_at,
        constants::value::MANAGED_BROWSER_BRIDGE_ENDPOINT_MANUAL_REQUIRED,
    )
}
