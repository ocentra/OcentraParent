use ocentra_parent_agent_protocol::transport::AgentRoute;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParentAgentServiceHealthState {
    Ready,
    Unavailable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParentAgentServiceAuthenticationState {
    Unauthenticated,
    Unavailable,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParentAgentServiceHealth {
    pub state: ParentAgentServiceHealthState,
    pub route: Option<AgentRoute>,
    pub protocol_schema_version: Option<u16>,
    pub service_version: Option<String>,
    pub transport: Option<String>,
    pub authentication_state: ParentAgentServiceAuthenticationState,
}

impl ParentAgentServiceHealth {
    pub fn unavailable() -> Self {
        Self {
            state: ParentAgentServiceHealthState::Unavailable,
            route: None,
            protocol_schema_version: None,
            service_version: None,
            transport: None,
            authentication_state: ParentAgentServiceAuthenticationState::Unavailable,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.state == ParentAgentServiceHealthState::Ready
    }
}
