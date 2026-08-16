use std::fmt;

use ocentra_eventing::error::EventingError;
use ocentra_parent_agent_protocol::child_domain_runtime::ChildDomainObservedEvent;

use crate::child_domain_runtime::ChildDomainRuntimeEventFlow;

#[derive(Debug)]
pub enum ChildAgentServiceError {
    Runtime(EventingError),
    Shutdown(std::io::Error),
}

impl fmt::Display for ChildAgentServiceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Runtime(error) => write!(formatter, "child runtime initialization failed: {error}"),
            Self::Shutdown(error) => write!(formatter, "child runtime shutdown signal failed: {error}"),
        }
    }
}

impl std::error::Error for ChildAgentServiceError {}

impl From<EventingError> for ChildAgentServiceError {
    fn from(error: EventingError) -> Self {
        Self::Runtime(error)
    }
}

pub struct ChildAgentService {
    domain_flows: Vec<ChildDomainRuntimeEventFlow>,
}

impl ChildAgentService {
    pub async fn initialize() -> Result<Self, ChildAgentServiceError> {
        let domain_flows = default_child_domain_observed_events()
            .iter()
            .map(|event| ChildDomainRuntimeEventFlow::for_event(event))
            .collect::<Vec<_>>();

        let mut initialized_flows = Vec::with_capacity(domain_flows.len());
        for flow in domain_flows {
            initialized_flows.push(flow.await?);
        }

        Ok(Self {
            domain_flows: initialized_flows,
        })
    }

    pub fn domain_flow_count(&self) -> usize {
        self.domain_flows.len()
    }

    pub async fn run_until_shutdown(self) -> Result<(), ChildAgentServiceError> {
        tokio::signal::ctrl_c()
            .await
            .map_err(ChildAgentServiceError::Shutdown)
    }
}

pub async fn run_child_agent_service() -> Result<(), ChildAgentServiceError> {
    ChildAgentService::initialize()
        .await?
        .run_until_shutdown()
        .await
}

fn default_child_domain_observed_events() -> Vec<ChildDomainObservedEvent> {
    vec![
        ocentra_app_core::default_app_observed_event(),
        ocentra_app_game_core::default_app_game_observed_event(),
        ocentra_browser_core::default_browser_observed_event(),
        ocentra_lan_core::lan_pairing::default_lan_observed_event(),
        ocentra_network_core::network_runtime::default_network_observed_event(),
        ocentra_screen_core::default_screen_observed_event(),
        ocentra_screen_live_view_core::default_screen_live_view_observed_event(),
    ]
}
