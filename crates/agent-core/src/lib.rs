#![forbid(unsafe_code)]

pub mod activity_store;
pub mod activity_store_app_game;
mod activity_store_app_game_observation;
mod activity_store_app_game_rows;
mod activity_store_browser;
mod activity_store_browser_intervention;
mod activity_store_connection;
mod activity_store_enforcement_audit;
pub mod activity_store_error;
mod activity_store_memory_graph;
mod activity_store_memory_graph_builder;
mod activity_store_memory_graph_index;
mod activity_store_memory_graph_index_persist;
mod activity_store_memory_graph_index_query;
mod activity_store_memory_graph_nodes;
mod activity_store_memory_graph_rows;
mod activity_store_network_flow;
mod activity_store_network_flow_rows;
mod activity_store_parent_rule_context;
mod activity_store_policy_preview;
mod activity_store_policy_preview_parent_rules;
mod activity_store_policy_preview_rows;
mod activity_store_policy_preview_targets;
mod activity_store_rows;
mod activity_store_screen_evidence;
pub mod browser_bridge_event;
mod browser_bridge_fields;
mod browser_bridge_http;
mod browser_bridge_ids;
#[cfg(test)]
mod browser_bridge_native_host;
pub mod browser_bridge_poll;
pub mod browser_event_runtime;
mod browser_event_runtime_refs;
#[cfg(test)]
mod browser_event_runtime_tests;
pub mod browser_intervention_event;
pub mod browser_managed_discovery;
pub mod browser_managed_session;
pub mod browser_windows_inventory;
pub mod browser_windows_inventory_paths;
pub mod browser_windows_inventory_source;
pub mod browser_windows_package_inventory;
pub mod browser_windows_package_source;
pub mod browser_windows_shortcut_source;
pub mod enforcement_adapter;
pub mod enforcement_app_time_limit;
pub mod enforcement_boundary;
pub mod enforcement_policy_dispatch;
pub mod enforcement_readiness;
pub mod enforcement_timer_state;
pub mod household_ai_provider_route;
mod household_ai_provider_route_labels;
pub mod household_ai_provider_route_state;
pub mod household_mesh_bridge_runtime;
mod household_mesh_bridge_runtime_refs;
mod household_mesh_bridge_runtime_source;
mod household_mesh_bridge_runtime_state;
pub mod household_mesh_event_bridge;
#[cfg(test)]
mod household_mesh_event_bridge_tests;
pub mod journal;
pub mod journal_crypto;
pub mod journal_error;
mod journal_rotation;
pub mod network_capture;
mod network_capture_adapter;
pub mod network_capture_event;
mod network_capture_event_fields;
#[cfg(windows)]
mod network_capture_netstat;
pub mod network_event_runtime;
mod network_event_runtime_state;
pub mod parent_child_event_runtime;
mod policy_dry_run_evaluator;
pub mod process_capture;
pub mod screen_event_runtime;
pub mod screen_event_runtime_input;
mod screen_event_runtime_metadata;
mod screen_event_runtime_refs;
mod screen_event_runtime_state;
pub mod screen_evidence_queue;
pub mod screen_household_mesh_runtime;
mod screen_household_mesh_runtime_refs;
mod screen_household_mesh_runtime_state;
pub mod tracking;
pub mod trusted_device_registry;
mod trusted_device_registry_selection;
pub mod window_capture;
pub mod window_capture_event;

use activity_store::ActivityStore;
use activity_store_error::ActivityStoreError;
use browser_bridge_event::{browser_tab_observation_event, BrowserBridgeTargetObservation};
use browser_windows_package_inventory::BrowserWindowsPackageIdentity;
use household_mesh_event_bridge::HouseholdMeshLocalEventKind;
use journal::ActivityJournal;
use journal_error::JournalError;
use network_capture::NetworkObservation;
use trusted_device_registry::TrustedDeviceRegistry;

#[cfg(test)]
use self::{
    browser_bridge_event::BrowserBridgeEventError,
    browser_bridge_poll::{
        poll_chromium_bridge, BrowserBridgeExpectedCustody, BrowserBridgePollConfig,
        BrowserBridgePollError,
    },
    browser_intervention_event::{
        browser_intervention_applied_event, BrowserInterventionObservation,
    },
    browser_managed_discovery::{
        installed_managed_browser_candidates, managed_browser_executable_identity,
        unmanaged_browser_processes,
    },
    browser_managed_session::{
        create_or_repair_managed_browser_profile_store, delete_managed_browser_profile_store,
        launch_managed_browser, load_managed_browser_profile_store, managed_browser_launch_plan,
        reserve_managed_browser_bridge_port, BrowserManagedLaunchConfig, BrowserManagedLaunchError,
        BrowserManagedProfileStoreConfig, BrowserManagedProfileStoreError,
    },
    browser_windows_inventory::{
        windows_browser_executable_identity, windows_browser_inventory_observations,
    },
    browser_windows_inventory_paths::windows_browser_inventory_candidate_paths,
    browser_windows_inventory_source::{
        browser_windows_inventory_candidate_paths_from_live_sources,
        browser_windows_live_registry_entry,
    },
    browser_windows_package_inventory::windows_browser_package_observations,
    browser_windows_package_source::live_windows_browser_package_entries_from_roots,
    browser_windows_shortcut_source::{
        browser_windows_shortcut_target_from_bytes,
        live_windows_browser_shortcut_targets_from_roots,
    },
    enforcement_adapter::{
        app_block_control_capability, managed_browser_control_capability,
        network_control_capability, process_control_capability, terminate_owned_process,
        timer_control_capability, unavailable_adapter_outcome, EnforcementAdapterOutcome,
        OwnedProcessTerminationTarget,
    },
    enforcement_app_time_limit::{
        app_time_limit_capability, app_time_limit_target_from_action,
        expire_app_time_limit_for_owned_process, unavailable_app_time_limit_outcome,
        AppTimeLimitAdapterTarget,
    },
    enforcement_boundary::{
        authorize_enforcement_boundary, evaluate_enforcement_boundary, EnforcementBoundaryInput,
        EnforcementBoundaryOutcome,
    },
    household_ai_provider_route::{
        select_household_ai_provider_route, HouseholdAiProviderCandidate, HouseholdAiRouteRequest,
    },
    household_ai_provider_route_state::{
        HouseholdAiProviderClass, HouseholdAiProviderResourceState, HouseholdAiProviderTrustState,
        HouseholdAiRouteDecisionState, HouseholdAiRouteRejectionReason,
    },
    household_mesh_event_bridge::{
        export_selected_local_event, validate_incoming_lan_message,
        HouseholdMeshAuthenticationState, HouseholdMeshBridgeRejection,
        HouseholdMeshExportDecision, HouseholdMeshImportDecision, HouseholdMeshLanMessage,
        HouseholdMeshPolicyAuthority,
    },
    journal_crypto::{JournalKey, JOURNAL_KEY_BYTES},
    network_capture::collect_network_snapshot,
    network_capture_event::network_observation_event,
    process_capture::{collect_process_snapshot, process_observation_event, ProcessObservation},
    window_capture::ForegroundWindowObservation,
    window_capture_event::foreground_window_observation_event,
};

pub fn crate_name() -> &'static str {
    env!("CARGO_PKG_NAME")
}

#[cfg(test)]
mod activity_store_app_game_tests;
#[cfg(test)]
mod activity_store_browser_intervention_tests;
#[cfg(test)]
mod activity_store_browser_tests;
#[cfg(test)]
mod activity_store_enforcement_audit_tests;
#[cfg(test)]
mod activity_store_memory_graph_index_test_api;
#[cfg(test)]
mod activity_store_memory_graph_index_tests;
#[cfg(test)]
mod activity_store_memory_graph_tests;
#[cfg(test)]
mod activity_store_network_flow_tests;
#[cfg(test)]
mod activity_store_policy_preview_parent_rule_tests;
#[cfg(test)]
mod activity_store_policy_preview_test_fixture;
#[cfg(test)]
mod activity_store_policy_preview_tests;
#[cfg(test)]
mod activity_store_screen_evidence_tests;
#[cfg(test)]
mod activity_store_tests;
#[cfg(test)]
mod browser_bridge_cdp_adapter_tests;
#[cfg(test)]
mod browser_bridge_native_host_tests;
#[cfg(test)]
mod browser_bridge_poll_security_tests;
#[cfg(test)]
mod browser_bridge_poll_test_support;
#[cfg(test)]
mod browser_bridge_poll_tests;
#[cfg(test)]
mod browser_bridge_tests;
#[cfg(test)]
mod browser_managed_session_tests;
#[cfg(test)]
mod browser_windows_inventory_source_tests;
#[cfg(test)]
mod browser_windows_inventory_tests;
#[cfg(test)]
mod enforcement_app_time_limit_tests;
#[cfg(test)]
mod enforcement_approval_audit_tests;
#[cfg(test)]
mod enforcement_audit_boundary_tests;
#[cfg(test)]
mod enforcement_permission_dependency_tests;
#[cfg(test)]
mod enforcement_policy_dispatch_tests;
#[cfg(test)]
mod enforcement_readiness_tests;
#[cfg(test)]
mod enforcement_tests;
#[cfg(test)]
mod enforcement_timer_state_tests;
#[cfg(test)]
mod enforcement_timer_tests;
#[cfg(test)]
mod enforcement_unavailable_adapter_tests;
#[cfg(test)]
mod household_ai_provider_route_tests;
#[cfg(test)]
mod household_mesh_bridge_runtime_tests;
#[cfg(test)]
mod journal_tests;
#[cfg(test)]
mod network_capture_tests;
#[cfg(test)]
mod network_event_runtime_broker_delivery_tests;
#[cfg(test)]
mod network_event_runtime_cross_process_replay_tests;
#[cfg(test)]
mod network_event_runtime_delete_export_propagation_tests;
#[cfg(test)]
mod network_event_runtime_fixture_transport_tests;
#[cfg(test)]
mod network_event_runtime_queue_tests;
#[cfg(test)]
mod network_event_runtime_remote_delivery_tests;
#[cfg(test)]
mod network_event_runtime_tests;
#[cfg(test)]
mod parent_child_event_runtime_tests;
#[cfg(test)]
mod policy_dry_run_evaluator_edge_tests;
#[cfg(test)]
mod policy_dry_run_evaluator_fixture;
#[cfg(test)]
mod policy_dry_run_evaluator_rule_tests;
#[cfg(test)]
mod process_capture_tests;
#[cfg(test)]
mod screen_event_runtime_tests;
#[cfg(test)]
mod screen_evidence_queue_tests;
#[cfg(test)]
mod screen_household_mesh_runtime_tests;
#[cfg(test)]
mod trusted_device_registry_test_fixtures;
#[cfg(test)]
mod trusted_device_registry_tests;
#[cfg(test)]
mod window_capture_tests;

#[cfg(test)]
mod tests {
    use super::crate_name;

    #[test]
    fn crate_name_identifies_agent_core_boundary() {
        assert_eq!(crate_name(), env!("CARGO_PKG_NAME"));
    }
}
