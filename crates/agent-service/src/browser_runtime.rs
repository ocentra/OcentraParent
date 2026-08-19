#[path = "browser_runtime_impl.rs"]
mod browser_runtime_impl;

use ocentra_parent_agent_protocol::transport::{AgentCommandEnvelope, AgentEventEnvelope};

#[derive(Clone, Copy, Debug, Default)]
pub struct BrowserManagedRuntime;

impl BrowserManagedRuntime {
    pub const fn new() -> Self {
        Self
    }

    pub async fn build_status_report(&self, command: AgentCommandEnvelope) -> AgentEventEnvelope {
        browser_runtime_impl::build_browser_managed_status_report(self.clone(), command).await
    }

    pub(crate) fn resolve_status(
        &self,
    ) -> ocentra_parent_agent_protocol::browser_managed::BrowserManagedSessionStatus {
        browser_runtime_impl::resolve_browser_managed_status()
    }
}

pub async fn build_browser_managed_status_report(
    runtime: BrowserManagedRuntime,
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    runtime.build_status_report(command).await
}
