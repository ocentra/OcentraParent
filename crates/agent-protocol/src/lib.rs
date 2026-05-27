#![forbid(unsafe_code)]

mod activity;
mod activity_capture;
mod activity_conversions;
mod activity_memory_graph;
mod activity_query;
mod app_game;
mod browser;
mod browser_intervention;
mod browser_intervention_parse;
mod browser_intervention_values;
mod browser_managed;
mod browser_read_model;
mod browser_unmanaged_enforcement;
mod browser_values;
pub mod constants;
mod enforcement;
mod journal;
mod lan_pairing;
mod lan_pairing_authority;
mod lan_pairing_support;
mod local_ai_runtime;
mod local_ai_runtime_boundary;
mod logging;
mod network_flow;
mod screen_evidence;
mod transport;

pub use activity::*;
pub use activity_capture::*;
pub use activity_memory_graph::*;
pub use activity_query::*;
pub use app_game::*;
pub use browser::*;
pub use browser_intervention::*;
pub use browser_intervention_values::*;
pub use browser_managed::*;
pub use browser_read_model::*;
pub use browser_unmanaged_enforcement::*;
pub use enforcement::*;
pub use journal::*;
pub use lan_pairing::*;
pub use lan_pairing_authority::*;
pub use lan_pairing_support::*;
pub use local_ai_runtime::*;
pub use local_ai_runtime_boundary::*;
pub use logging::*;
pub use network_flow::*;
pub use screen_evidence::*;
pub use transport::*;

pub const CRATE_NAME: &str = "ocentra-parent-agent-protocol";
pub const LOG_SCHEMA_VERSION: u16 = 1;
pub const AGENT_PROTOCOL_SCHEMA_VERSION: u16 = 1;
pub const ACTIVITY_SCHEMA_VERSION: u16 = 1;
pub const ACTIVITY_JOURNAL_SCHEMA_VERSION: u16 = 1;
pub const ACTIVITY_QUERY_SCHEMA_VERSION: u16 = 1;
pub const ACTIVITY_MEMORY_GRAPH_SCHEMA_VERSION: u16 = 1;
pub const BROWSER_EVIDENCE_SCHEMA_VERSION: u16 = 1;
pub const BROWSER_INTERVENTION_SCHEMA_VERSION: u16 = 1;
pub const NETWORK_FLOW_SCHEMA_VERSION: u16 = 1;
pub const LAN_PAIRING_SCHEMA_VERSION: u16 = 1;
pub const SCREEN_EVIDENCE_SCHEMA_VERSION: u16 = 1;
pub const ENFORCEMENT_SCHEMA_VERSION: u16 = 1;

pub fn crate_name() -> &'static str {
    CRATE_NAME
}

#[cfg(test)]
mod activity_memory_graph_tests;
#[cfg(test)]
mod activity_query_tests;
#[cfg(test)]
mod activity_tests;
#[cfg(test)]
mod app_game_tests;
#[cfg(test)]
mod browser_intervention_tests;
#[cfg(test)]
mod browser_managed_tests;
#[cfg(test)]
mod browser_read_model_tests;
#[cfg(test)]
mod enforcement_audit_boundary_tests;
#[cfg(test)]
mod enforcement_permission_dependency_tests;
#[cfg(test)]
mod enforcement_tests;
#[cfg(test)]
mod enforcement_unavailable_tests;
#[cfg(test)]
mod journal_tests;
#[cfg(test)]
mod lan_pairing_tests;
#[cfg(test)]
mod local_ai_chat_generation_protocol_tests;
#[cfg(test)]
mod local_ai_model_cache_tests;
#[cfg(test)]
mod local_ai_provider_scheduler_tests;
#[cfg(test)]
mod local_ai_runtime_tests;
#[cfg(test)]
mod local_provider_adapter_readiness_tests;
#[cfg(test)]
mod network_flow_tests;
#[cfg(test)]
mod policy_preview_tests;
#[cfg(test)]
mod screen_evidence_tests;
#[cfg(test)]
mod tests;
