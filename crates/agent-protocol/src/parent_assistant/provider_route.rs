use serde::{Deserialize, Serialize};

use super::{ParentAssistantApiProviderAccessState, ParentAssistantProviderState};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParentAssistantProviderSelection {
    #[serde(rename = "local")]
    Local,
    #[serde(rename = "api")]
    Api,
    #[serde(rename = "none")]
    None,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParentAssistantProviderRoutingState {
    #[serde(rename = "local-provider-ready")]
    LocalProviderReady,
    #[serde(rename = "local-provider-degraded")]
    LocalProviderDegraded,
    #[serde(rename = "local-provider-unavailable")]
    LocalProviderUnavailable,
    #[serde(rename = "api-provider-authorized-unavailable")]
    ApiProviderAuthorizedUnavailable,
    #[serde(rename = "api-provider-authorized-degraded")]
    ApiProviderAuthorizedDegraded,
    #[serde(rename = "no-provider-available")]
    NoProviderAvailable,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentAssistantProviderRoute {
    pub routing_state: ParentAssistantProviderRoutingState,
    pub selected_provider: ParentAssistantProviderSelection,
    pub local_provider_state: ParentAssistantProviderState,
    pub api_provider_state: ParentAssistantProviderState,
    pub api_access_state: ParentAssistantApiProviderAccessState,
    pub evidence_citation_required: bool,
    pub remote_ai_optional: bool,
    pub child_safety_or_enforcement_use_allowed: bool,
    pub reason: String,
}
