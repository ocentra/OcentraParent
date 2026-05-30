#![forbid(unsafe_code)]

mod activity;
mod activity_capture;
mod activity_conversions;
mod activity_memory_graph;
mod activity_query;
mod activity_surface;
mod app_game;
mod browser;
mod browser_intervention;
mod browser_intervention_parse;
mod browser_intervention_values;
mod browser_managed;
mod browser_policy;
mod browser_policy_catalog_values;
mod browser_policy_model;
mod browser_policy_sections;
mod browser_policy_values;
mod browser_read_model;
mod browser_unmanaged_enforcement;
mod browser_values;
pub mod constants;
mod enforcement;
mod enforcement_os_adapter_product_proof;
mod enforcement_readiness;
mod host_identity;
mod journal;
mod lan_pairing;
mod lan_pairing_authority;
mod lan_pairing_provider_selection;
mod lan_pairing_support;
mod local_ai_runtime;
mod local_ai_runtime_boundary;
mod local_ai_runtime_provider_proof;
mod logging;
mod network_flow;
mod parent_assistant;
mod screen_evidence;
mod transport;
mod windows_adapter_artifact_gate;
mod windows_adapter_artifact_ingestion;
mod windows_adapter_capability;

pub use activity::*;
pub use activity_capture::*;
pub use activity_memory_graph::*;
pub use activity_query::*;
pub use activity_surface::*;
pub use app_game::*;
pub use browser::*;
pub use browser_intervention::*;
pub use browser_intervention_values::*;
pub use browser_managed::*;
pub use browser_policy::*;
pub use browser_policy_catalog_values::*;
pub use browser_policy_model::*;
pub use browser_policy_sections::*;
pub use browser_policy_values::*;
pub use browser_read_model::*;
pub use browser_unmanaged_enforcement::*;
pub use enforcement::*;
pub use enforcement_os_adapter_product_proof::*;
pub use enforcement_readiness::*;
pub use host_identity::*;
pub use journal::*;
pub use lan_pairing::*;
pub use lan_pairing_authority::*;
pub use lan_pairing_provider_selection::*;
pub use lan_pairing_support::*;
pub use local_ai_runtime::*;
pub use local_ai_runtime_boundary::*;
pub use local_ai_runtime_provider_proof::*;
pub use logging::*;
pub use network_flow::*;
pub use parent_assistant::*;
pub use screen_evidence::*;
pub use transport::*;
pub use windows_adapter_artifact_gate::*;
pub use windows_adapter_artifact_ingestion::*;
pub use windows_adapter_capability::*;

pub const CRATE_NAME: &str = "ocentra-parent-agent-protocol";
pub const LOG_SCHEMA_VERSION: u16 = 1;
pub const AGENT_PROTOCOL_SCHEMA_VERSION: u16 = 1;
pub const ACTIVITY_SCHEMA_VERSION: u16 = 1;
pub const ACTIVITY_JOURNAL_SCHEMA_VERSION: u16 = 1;
pub const ACTIVITY_QUERY_SCHEMA_VERSION: u16 = 1;
pub const ACTIVITY_MEMORY_GRAPH_SCHEMA_VERSION: u16 = 1;
pub const ACTIVITY_SURFACE_SCHEMA_VERSION: u16 = 1;
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
mod activity_surface_tests;
#[cfg(test)]
mod activity_tests;
#[cfg(test)]
mod app_game_tests;
#[cfg(test)]
mod browser_intervention_tests;
#[cfg(test)]
mod browser_managed_tests;
#[cfg(test)]
mod browser_policy_tests;
#[cfg(test)]
mod browser_read_model_tests;
#[cfg(test)]
mod enforcement_audit_boundary_tests;
#[cfg(test)]
mod enforcement_os_adapter_product_proof_tests;
#[cfg(test)]
mod enforcement_permission_dependency_tests;
#[cfg(test)]
mod enforcement_readiness_tests;
#[cfg(test)]
mod enforcement_tests;
#[cfg(test)]
mod enforcement_unavailable_tests;
#[cfg(test)]
mod host_identity_tests;
#[cfg(test)]
mod journal_tests;
#[cfg(test)]
mod lan_pairing_provider_selection_tests;
#[cfg(test)]
mod lan_pairing_tests;
#[cfg(test)]
mod local_ai_chat_generation_protocol_tests;
#[cfg(test)]
mod local_ai_model_cache_tests;
#[cfg(test)]
mod local_ai_provider_scheduler_tests;
#[cfg(test)]
mod local_ai_runtime_provider_proof_tests;
#[cfg(test)]
mod local_ai_runtime_tests;
#[cfg(test)]
mod local_provider_adapter_readiness_tests;
#[cfg(test)]
mod network_flow_tests;
#[cfg(test)]
mod parent_assistant_tests;
#[cfg(test)]
mod policy_preview_tests;
#[cfg(test)]
mod screen_evidence_tests;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod windows_adapter_artifact_gate_tests;
#[cfg(test)]
mod windows_adapter_artifact_ingestion_tests;
#[cfg(test)]
mod windows_adapter_capability_tests;
