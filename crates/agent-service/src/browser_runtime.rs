#[path = "browser_runtime_impl.rs"]
mod browser_runtime_impl;

use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;

pub async fn build_browser_managed_status_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    browser_runtime_impl::build_browser_managed_status_report(command).await
}
