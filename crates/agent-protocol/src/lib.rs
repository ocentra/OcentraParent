#![forbid(unsafe_code)]

mod activity;
mod activity_capture;
mod activity_conversions;
mod activity_memory_graph;
mod activity_query;
mod activity_surface;
mod app_game;
mod app_game_authority_classifier;
mod app_game_boundary_read_model;
mod app_game_notification_readiness;
mod app_game_policy_readiness;
mod browser;
mod browser_intervention;
mod browser_intervention_parse;
mod browser_intervention_values;
mod browser_inventory;
mod browser_managed;
mod browser_policy;
mod browser_policy_catalog_values;
mod browser_policy_model;
mod browser_policy_sections;
mod browser_policy_values;
mod browser_read_model;
mod browser_unmanaged_enforcement;
mod browser_values;
mod child_agent;
pub mod constants;
mod enforcement;
mod enforcement_broad_adapter_proof;
mod enforcement_browser_domain_adapter_proof;
mod enforcement_cross_platform_capability_proof;
mod enforcement_integrity_runtime_audit;
mod enforcement_os_adapter_product_proof;
mod enforcement_policy_dispatch;
mod enforcement_product_control_spine;
mod enforcement_readiness;
mod enforcement_supported_adapter_runtime_proof;
mod host_identity;
mod integrity_alert_status_bridge;
mod journal;
mod lan_pairing;
mod lan_pairing_authority;
mod lan_pairing_browser_add_device_state;
mod lan_pairing_browser_runtime;
mod lan_pairing_provider_selection;
mod lan_pairing_support;
mod local_ai_runtime;
mod local_ai_runtime_boundary;
mod local_ai_runtime_provider_proof;
mod logging;
mod network_flow;
mod network_linux_nftables_lab_status;
mod network_windows_firewall_lab_status;
mod network_windows_wfp_gate_status;
mod notification_provider_status_boundary;
mod parent_assistant;
mod parent_controller;
mod screen_evidence;
mod social_alert_report_read_model;
mod social_audit_explanation_read_model;
mod social_dashboard_read_model;
mod social_source_custody_mutation;
mod tracking_read_model;
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
pub use app_game_authority_classifier::*;
pub use app_game_boundary_read_model::*;
pub use app_game_notification_readiness::*;
pub use app_game_policy_readiness::*;
pub use browser::*;
pub use browser_intervention::*;
pub use browser_intervention_values::*;
pub use browser_inventory::*;
pub use browser_managed::*;
pub use browser_policy::*;
pub use browser_policy_catalog_values::*;
pub use browser_policy_model::*;
pub use browser_policy_sections::*;
pub use browser_policy_values::*;
pub use browser_read_model::*;
pub use browser_unmanaged_enforcement::*;
pub use child_agent::*;
pub use enforcement::*;
pub use enforcement_broad_adapter_proof::*;
pub use enforcement_browser_domain_adapter_proof::*;
pub use enforcement_cross_platform_capability_proof::*;
pub use enforcement_integrity_runtime_audit::*;
pub use enforcement_os_adapter_product_proof::*;
pub use enforcement_policy_dispatch::*;
pub use enforcement_product_control_spine::*;
pub use enforcement_readiness::*;
pub use enforcement_supported_adapter_runtime_proof::*;
pub use host_identity::*;
pub use integrity_alert_status_bridge::*;
pub use journal::*;
pub use lan_pairing::*;
pub use lan_pairing_authority::*;
pub use lan_pairing_browser_add_device_state::*;
pub use lan_pairing_browser_runtime::*;
pub use lan_pairing_provider_selection::*;
pub use lan_pairing_support::*;
pub use local_ai_runtime::*;
pub use local_ai_runtime_boundary::*;
pub use local_ai_runtime_provider_proof::*;
pub use logging::*;
pub use network_flow::*;
pub use network_linux_nftables_lab_status::*;
pub use network_windows_firewall_lab_status::*;
pub use network_windows_wfp_gate_status::*;
pub use notification_provider_status_boundary::*;
pub use parent_assistant::*;
pub use parent_controller::*;
pub use screen_evidence::*;
pub use social_alert_report_read_model::*;
pub use social_audit_explanation_read_model::*;
pub use social_dashboard_read_model::*;
pub use social_source_custody_mutation::*;
pub use tracking_read_model::*;
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
mod app_game_authority_classifier_tests;
#[cfg(test)]
mod app_game_boundary_read_model_tests;
#[cfg(test)]
mod app_game_evidence_identity_tests;
#[cfg(test)]
mod app_game_notification_readiness_tests;
#[cfg(test)]
mod app_game_policy_readiness_tests;
#[cfg(test)]
mod app_game_tests;
#[cfg(test)]
mod browser_intervention_tests;
#[cfg(test)]
mod browser_inventory_tests;
#[cfg(test)]
mod browser_managed_tests;
#[cfg(test)]
mod browser_policy_tests;
#[cfg(test)]
mod browser_read_model_tests;
#[cfg(test)]
mod child_agent_event_tests;
#[cfg(test)]
mod enforcement_audit_boundary_tests;
#[cfg(test)]
mod enforcement_broad_adapter_proof_tests;
#[cfg(test)]
mod enforcement_browser_domain_adapter_proof_tests;
#[cfg(test)]
mod enforcement_cross_platform_capability_proof_tests;
#[cfg(test)]
mod enforcement_integrity_runtime_audit_tests;
#[cfg(test)]
mod enforcement_os_adapter_product_proof_tests;
#[cfg(test)]
mod enforcement_permission_dependency_tests;
#[cfg(test)]
mod enforcement_policy_dispatch_tests;
#[cfg(test)]
mod enforcement_product_control_spine_tests;
#[cfg(test)]
mod enforcement_readiness_tests;
#[cfg(test)]
mod enforcement_supported_adapter_runtime_proof_tests;
#[cfg(test)]
mod enforcement_tests;
#[cfg(test)]
mod enforcement_unavailable_tests;
#[cfg(test)]
mod host_identity_tests;
#[cfg(test)]
mod integrity_alert_status_bridge_tests;
#[cfg(test)]
mod journal_tests;
#[cfg(test)]
mod lan_pairing_browser_add_device_state_tests;
#[cfg(test)]
mod lan_pairing_browser_runtime_tests;
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
mod network_linux_nftables_lab_status_tests;
#[cfg(test)]
mod network_live_capture_status_tests;
#[cfg(test)]
mod network_windows_firewall_lab_status_tests;
#[cfg(test)]
mod network_windows_wfp_gate_status_tests;
#[cfg(test)]
mod notification_provider_status_boundary_tests;
#[cfg(test)]
mod parent_assistant_tests;
#[cfg(test)]
mod parent_controller_event_tests;
#[cfg(test)]
mod policy_preview_tests;
#[cfg(test)]
mod screen_evidence_tests;
#[cfg(test)]
mod social_alert_report_read_model_tests;
#[cfg(test)]
mod social_audit_explanation_read_model_tests;
#[cfg(test)]
mod social_dashboard_read_model_tests;
#[cfg(test)]
mod social_source_custody_mutation_tests;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod tracking_read_model_tests;
#[cfg(test)]
mod windows_adapter_artifact_gate_tests;
#[cfg(test)]
mod windows_adapter_artifact_ingestion_tests;
#[cfg(test)]
mod windows_adapter_capability_tests;
