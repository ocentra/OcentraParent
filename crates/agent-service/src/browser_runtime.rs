#[path = "browser_runtime_impl.rs"]
mod browser_runtime_impl;

use std::sync::{Arc, Mutex};

use ocentra_parent_agent_protocol::{
    browser_managed::BrowserManagedSessionStatus,
    transport::{AgentCommandEnvelope, AgentEventEnvelope},
};

use ocentra_parent_agent_core::browser_bridge_capture::{
    ManagedBrowserCdpCaptureError, ManagedBrowserCdpTargetAuthority,
};

use crate::screen_managed_browser_cdp_runtime::{
    ManagedBrowserScreenIntelligenceRequest, ManagedBrowserScreenIntelligenceRouteError,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BrowserManagedTargetId(String);

impl BrowserManagedTargetId {
    pub(crate) fn from_runtime_text(value: BrowserRuntimeText) -> Self {
        Self(value.0)
    }
}

#[derive(Clone, Debug)]
pub(crate) struct BrowserRuntimeText(pub(crate) String);

#[derive(Clone)]
pub struct BrowserManagedRuntime {
    state: Arc<Mutex<browser_runtime_impl::BrowserManagedRuntimeState>>,
}

impl BrowserManagedRuntime {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(
                browser_runtime_impl::BrowserManagedRuntimeState::new(),
            )),
        }
    }

    pub async fn build_status_report(&self, command: AgentCommandEnvelope) -> AgentEventEnvelope {
        browser_runtime_impl::build_browser_managed_status_report(self.clone(), command).await
    }

    pub(crate) fn resolve_status(&self) -> BrowserManagedSessionStatus {
        browser_runtime_impl::resolve_browser_managed_status(self)
    }

    pub fn authorize_target(
        &self,
        target_id: BrowserManagedTargetId,
    ) -> Result<ManagedBrowserCdpTargetAuthority, BrowserManagedRuntimeTargetError> {
        let _ = (self, target_id);
        Err(BrowserManagedRuntimeTargetError::Unavailable)
    }

    pub fn plan_screen_route(
        &self,
        target_id: BrowserManagedTargetId,
        input: ManagedBrowserScreenIntelligenceRequest,
    ) -> Result<
        ocentra_screen_ai_core::screen_intelligence_router::ScreenIntelligenceRouteDecision,
        BrowserManagedRuntimeScreenRouteError,
    > {
        let _ = (self, target_id, input);
        Err(BrowserManagedRuntimeScreenRouteError::Unavailable)
    }
}

#[derive(Debug)]
pub enum BrowserManagedRuntimeTargetError {
    Unavailable,
    NoActiveLaunch,
    Capture(ManagedBrowserCdpCaptureError),
}

#[derive(Debug)]
pub enum BrowserManagedRuntimeScreenRouteError {
    Unavailable,
    Target(BrowserManagedRuntimeTargetError),
    Screen(ManagedBrowserScreenIntelligenceRouteError),
}

impl Default for BrowserManagedRuntime {
    fn default() -> Self {
        Self::new()
    }
}

pub async fn build_browser_managed_status_report(
    runtime: BrowserManagedRuntime,
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    runtime.build_status_report(command).await
}
