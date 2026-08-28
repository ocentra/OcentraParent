use ocentra_parent_agent_core::activity_store::ActivityStore;
use ocentra_parent_agent_core::activity_store_app_game_rows;
use ocentra_parent_agent_core::activity_store_memory_graph_index_persist;
use ocentra_parent_agent_core::browser_bridge_event::{
    browser_tab_observation_event, BrowserBridgeEventError, BrowserBridgeTargetObservation,
};
use ocentra_parent_agent_core::browser_bridge_poll::{
    poll_chromium_bridge, BrowserBridgePollError,
};
use ocentra_parent_agent_core::browser_intervention_event::{
    browser_intervention_applied_event, BrowserInterventionObservation,
};
use ocentra_parent_agent_core::browser_managed_discovery::{
    installed_managed_browser_candidates, managed_browser_executable_identity,
    unmanaged_browser_processes,
};
use ocentra_parent_agent_core::browser_managed_session::{
    launch_managed_browser, managed_browser_launch_plan, reserve_managed_browser_bridge_port,
    BrowserManagedLaunchConfig, BrowserManagedLaunchError,
};
use ocentra_parent_agent_core::browser_windows_inventory::{
    windows_browser_executable_identity, windows_browser_inventory_observations,
};
use ocentra_parent_agent_core::browser_windows_inventory_paths;
use ocentra_parent_agent_core::browser_windows_inventory_paths::windows_browser_inventory_candidate_paths;
use ocentra_parent_agent_core::browser_windows_inventory_source::{
    browser_windows_inventory_candidate_paths_from_live_sources,
    browser_windows_live_registry_entry,
};
use ocentra_parent_agent_core::browser_windows_package_inventory::windows_browser_package_observations;
use ocentra_parent_agent_core::browser_windows_package_source::live_windows_browser_package_entries_from_roots;
use ocentra_parent_agent_core::browser_windows_shortcut_source::{
    browser_windows_shortcut_target_from_bytes, live_windows_browser_shortcut_targets_from_roots,
};
use ocentra_parent_agent_core::enforcement_adapter::{
    app_block_control_capability, managed_browser_control_capability, network_control_capability,
    process_control_capability, terminate_owned_process, timer_control_capability,
    unavailable_adapter_outcome, EnforcementAdapterOutcome, OwnedProcessTerminationTarget,
};
use ocentra_parent_agent_core::enforcement_app_time_limit::{
    app_time_limit_capability, app_time_limit_target_from_action,
    expire_app_time_limit_for_owned_process, unavailable_app_time_limit_outcome,
    AppTimeLimitAdapterTarget,
};
use ocentra_parent_agent_core::enforcement_boundary::{
    authorize_enforcement_boundary, evaluate_enforcement_boundary, EnforcementBoundaryInput,
    EnforcementBoundaryOutcome,
};
use ocentra_parent_agent_core::household_ai_provider_route::{
    select_household_ai_provider_route, HouseholdAiProviderCandidate, HouseholdAiRouteRequest,
};
use ocentra_parent_agent_core::household_ai_provider_route_state::{
    HouseholdAiProviderClass, HouseholdAiProviderResourceState, HouseholdAiProviderTrustState,
    HouseholdAiRouteDecisionState, HouseholdAiRouteRejectionReason,
};
use ocentra_parent_agent_core::household_mesh_event_bridge::{
    export_selected_local_event, HouseholdMeshAuthenticationState, HouseholdMeshBridgeRejection,
    HouseholdMeshExportDecision, HouseholdMeshLocalEventKind, HouseholdMeshPolicyAuthority,
};
use ocentra_parent_agent_core::journal::ActivityJournal;
use ocentra_parent_agent_core::journal_crypto::{JournalKey, JOURNAL_KEY_BYTES};
use ocentra_parent_agent_core::journal_error;
#[cfg(windows)]
use ocentra_parent_agent_core::network_capture::collect_network_snapshot;
use ocentra_parent_agent_core::network_capture::NetworkObservation;
use ocentra_parent_agent_core::network_capture_event::network_observation_event;
#[cfg(windows)]
use ocentra_parent_agent_core::network_capture_netstat;
use ocentra_parent_agent_core::process_capture::{
    collect_process_snapshot, process_observation_event, ProcessObservation,
};
use ocentra_parent_agent_core::tracking;
use ocentra_parent_agent_core::window_capture::ForegroundWindowObservation;
use ocentra_parent_agent_core::window_capture_event::foreground_window_observation_event;

mod activity_store {
    pub(crate) type ActivityStore = ocentra_parent_agent_core::activity_store::ActivityStore;
}

mod journal {
    pub(crate) type ActivityJournal = ocentra_parent_agent_core::journal::ActivityJournal;
}

mod journal_crypto {
    pub(crate) type JournalKey = ocentra_parent_agent_core::journal_crypto::JournalKey;
    pub(crate) const JOURNAL_KEY_BYTES: usize =
        ocentra_parent_agent_core::journal_crypto::JOURNAL_KEY_BYTES;
}

#[path = "../support/activity_store_policy_preview_support.rs"]
mod activity_store_policy_preview_support;

#[path = "../support/browser_bridge_poll_support.rs"]
mod browser_bridge_poll_support;

#[path = "../support/policy_dry_run_evaluator_support.rs"]
mod policy_dry_run_evaluator_support;

#[path = "../support/trusted_device_registry_support.rs"]
mod trusted_device_registry_support;

#[path = "../support/test_text.rs"]
mod test_text;

#[path = "activity_store_app_game.rs"]
mod activity_store_app_game;
#[path = "activity_store_app_game_tests.rs"]
mod activity_store_app_game_tests;
#[path = "activity_store_browser_intervention_tests.rs"]
mod activity_store_browser_intervention_tests;
#[path = "activity_store_browser_tests.rs"]
mod activity_store_browser_tests;
#[path = "activity_store_enforcement_audit_tests.rs"]
mod activity_store_enforcement_audit_tests;
#[path = "activity_store_memory_graph_index_tests.rs"]
mod activity_store_memory_graph_index_tests;
#[path = "activity_store_memory_graph_tests.rs"]
mod activity_store_memory_graph_tests;
#[path = "activity_store_network_flow_tests.rs"]
mod activity_store_network_flow_tests;
#[path = "activity_store_policy_preview_parent_rule_tests.rs"]
mod activity_store_policy_preview_parent_rule_tests;
#[path = "activity_store_policy_preview_tests.rs"]
mod activity_store_policy_preview_tests;
#[path = "activity_store_screen_evidence_tests.rs"]
mod activity_store_screen_evidence_tests;
#[path = "activity_store_tests.rs"]
mod activity_store_tests;
#[path = "authenticated_delivery_execution.rs"]
mod authenticated_delivery_execution;
#[path = "authenticated_delivery_grant.rs"]
mod authenticated_delivery_grant;
#[path = "authenticated_delivery_grant_rejection_retention.rs"]
mod authenticated_delivery_grant_rejection_retention;
#[path = "browser_bridge_cdp_adapter_tests.rs"]
mod browser_bridge_cdp_adapter_tests;
#[path = "browser_bridge_native_host_tests.rs"]
mod browser_bridge_native_host_tests;
#[path = "browser_bridge_poll_security_tests.rs"]
mod browser_bridge_poll_security_tests;
#[path = "browser_bridge_poll_tests.rs"]
mod browser_bridge_poll_tests;
#[path = "browser_bridge_tests.rs"]
mod browser_bridge_tests;
#[path = "browser_event_runtime_tests.rs"]
mod browser_event_runtime_tests;
#[path = "browser_managed_session_tests.rs"]
mod browser_managed_session_tests;
#[path = "browser_windows_inventory_source_tests.rs"]
mod browser_windows_inventory_source_tests;
#[path = "browser_windows_inventory_tests.rs"]
mod browser_windows_inventory_tests;
#[path = "crate_name_tests.rs"]
mod crate_name_tests;
#[path = "enforcement_app_time_limit_tests.rs"]
mod enforcement_app_time_limit_tests;
#[path = "enforcement_approval_audit_tests.rs"]
mod enforcement_approval_audit_tests;
#[path = "enforcement_audit_boundary_tests.rs"]
mod enforcement_audit_boundary_tests;
#[path = "enforcement_permission_dependency_tests.rs"]
mod enforcement_permission_dependency_tests;
#[path = "enforcement_policy_dispatch_tests.rs"]
mod enforcement_policy_dispatch_tests;
#[path = "enforcement_readiness_tests.rs"]
mod enforcement_readiness_tests;
#[path = "enforcement_tests.rs"]
mod enforcement_tests;
#[path = "enforcement_timer_state_tests.rs"]
mod enforcement_timer_state_tests;
#[path = "enforcement_timer_tests.rs"]
mod enforcement_timer_tests;
#[path = "enforcement_unavailable_adapter_tests.rs"]
mod enforcement_unavailable_adapter_tests;
#[path = "household_ai_provider_route_tests.rs"]
mod household_ai_provider_route_tests;
#[path = "household_mesh_event_bridge_tests.rs"]
mod household_mesh_event_bridge_tests;
#[path = "journal_tests.rs"]
mod journal_tests;
#[path = "network_capture_tests.rs"]
mod network_capture_tests;
#[path = "network_event_runtime.rs"]
mod network_event_runtime;
#[path = "network_event_runtime_broker_delivery_tests.rs"]
mod network_event_runtime_broker_delivery_tests;
#[path = "network_event_runtime_cross_process_replay_tests.rs"]
mod network_event_runtime_cross_process_replay_tests;
#[path = "network_event_runtime_delete_export_propagation_tests.rs"]
mod network_event_runtime_delete_export_propagation_tests;
#[path = "network_event_runtime_fixture_transport_tests.rs"]
mod network_event_runtime_fixture_transport_tests;
#[path = "network_event_runtime_queue_tests.rs"]
mod network_event_runtime_queue_tests;
#[path = "network_event_runtime_remote_delivery_tests.rs"]
mod network_event_runtime_remote_delivery_tests;
#[path = "network_event_runtime_tests.rs"]
mod network_event_runtime_tests;
#[path = "parent_child_event_runtime_tests.rs"]
mod parent_child_event_runtime_tests;
#[path = "policy_dry_run_evaluator_edge_tests.rs"]
mod policy_dry_run_evaluator_edge_tests;
#[path = "policy_dry_run_evaluator_rule_tests.rs"]
mod policy_dry_run_evaluator_rule_tests;
#[path = "process_capture_tests.rs"]
mod process_capture_tests;
#[path = "screen_event_runtime_tests.rs"]
mod screen_event_runtime_tests;
#[path = "screen_evidence_queue_tests.rs"]
mod screen_evidence_queue_tests;
#[path = "screen_household_mesh_runtime_tests.rs"]
mod screen_household_mesh_runtime_tests;
#[path = "tracking_read_model.rs"]
mod tracking_read_model;
#[path = "trusted_device_registry.rs"]
mod trusted_device_registry;
#[path = "window_capture_tests.rs"]
mod window_capture_tests;
