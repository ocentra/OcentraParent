#![forbid(unsafe_code)]

mod activity_store;
mod activity_store_app_game;
mod activity_store_app_game_observation;
mod activity_store_app_game_rows;
mod activity_store_browser;
mod activity_store_browser_intervention;
mod activity_store_connection;
mod activity_store_error;
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
mod activity_store_tracking;
mod activity_store_tracking_rows;
mod browser_bridge_event;
mod browser_bridge_fields;
mod browser_bridge_http;
mod browser_bridge_ids;
#[cfg(test)]
mod browser_bridge_native_host;
mod browser_bridge_poll;
mod browser_intervention_event;
mod browser_managed_discovery;
mod browser_managed_session;
#[cfg(test)]
mod browser_performance_health;
mod browser_windows_inventory;
mod browser_windows_inventory_paths;
mod enforcement_adapter;
mod enforcement_app_time_limit;
mod enforcement_boundary;
mod enforcement_policy_dispatch;
mod enforcement_readiness;
mod enforcement_timer_state;
mod journal;
mod journal_crypto;
mod journal_error;
mod journal_rotation;
mod network_capture;
mod network_capture_adapter;
mod network_capture_event;
mod network_capture_event_fields;
#[cfg(windows)]
mod network_capture_netstat;
mod network_event_runtime;
mod network_event_runtime_phase;
mod network_event_runtime_state;
mod parent_child_event_runtime;
mod parent_child_event_runtime_phase;
mod policy_dry_run_evaluator;
mod process_capture;
mod screen_evidence_queue;
mod trusted_device_registry;
mod trusted_device_registry_selection;
mod window_capture;
mod window_capture_event;

pub use activity_store::ActivityStore;
pub use activity_store_app_game::{
    live_windows_foreground_window_journal_event, live_windows_inventory_journal_events_from_roots,
    live_windows_inventory_journal_events_with_limit,
    live_windows_process_snapshot_journal_events_with_limit,
    live_windows_registry_inventory_journal_events_from_roots,
    live_windows_registry_inventory_journal_events_with_limit,
    live_windows_store_package_journal_events_from_roots,
    live_windows_store_package_journal_events_with_limit, AppGameLiveForegroundWindowError,
    AppGameLiveInventorySourceError, AppGameLiveProcessSnapshotError,
    AppGameLiveRegistryInventorySourceError, AppGameLiveStorePackageSourceError,
};
pub use activity_store_error::ActivityStoreError;
pub use activity_store_tracking::tracking_read_model_for_store;
pub use browser_bridge_event::{
    browser_tab_observation_event, BrowserBridgeEventError, BrowserBridgeTargetObservation,
};
pub use browser_bridge_poll::{
    poll_chromium_bridge, BrowserBridgeExpectedCustody, BrowserBridgePollConfig,
    BrowserBridgePollError, BrowserBridgePollSnapshot,
};
pub use browser_intervention_event::{
    browser_intervention_applied_event, BrowserInterventionObservation,
};
pub use browser_managed_discovery::{
    installed_managed_browser_candidates, managed_browser_executable_identity,
    unmanaged_browser_processes, BrowserManagedExecutableIdentity, BrowserManagedInstallCandidate,
    BrowserUnmanagedProcessObservation,
};
pub use browser_managed_session::{
    create_or_repair_managed_browser_profile_store, delete_managed_browser_profile_store,
    launch_managed_browser, load_managed_browser_profile_store, managed_browser_launch_plan,
    reserve_managed_browser_bridge_port, BrowserManagedBridgePortReservation, BrowserManagedLaunch,
    BrowserManagedLaunchConfig, BrowserManagedLaunchError, BrowserManagedLaunchPlan,
    BrowserManagedProfileStoreConfig, BrowserManagedProfileStoreError,
    BrowserManagedProfileStoreRecord,
};
pub use browser_windows_inventory::{
    windows_browser_executable_identity, windows_browser_inventory_observations,
    BrowserWindowsExecutableIdentity, BrowserWindowsInventoryObservation,
};
pub use browser_windows_inventory_paths::windows_browser_inventory_candidate_paths;
pub use enforcement_adapter::{
    app_block_control_capability, managed_browser_control_capability, network_control_capability,
    process_control_capability, terminate_owned_process, timer_control_capability,
    unavailable_adapter_outcome, EnforcementAdapterOutcome, OwnedProcessTerminationTarget,
};
pub use enforcement_app_time_limit::{
    app_time_limit_capability, app_time_limit_target_from_action,
    expire_app_time_limit_for_owned_process, unavailable_app_time_limit_outcome,
    AppTimeLimitAdapterTarget, AppTimeLimitTargetRejection,
};
pub use enforcement_boundary::{
    authorize_enforcement_boundary, evaluate_enforcement_boundary, EnforcementAdapterRequest,
    EnforcementAuthorizationOutcome, EnforcementBoundaryInput, EnforcementBoundaryOutcome,
    EnforcementBoundaryRejection,
};
pub use enforcement_policy_dispatch::{
    validate_enforcement_policy_dispatch_read_model, EnforcementPolicyDispatchValidation,
};
pub use enforcement_readiness::broad_os_adapter_readiness;
pub use enforcement_timer_state::{
    active_timer_state_from_outcome, cancelled_timer_outcome, expired_timer_outcome,
    restart_recovered_timer_outcome, EnforcementTimerTransitionIds,
};
pub use journal::ActivityJournal;
pub use journal_crypto::{JournalKey, JOURNAL_KEY_BYTES};
pub use journal_error::JournalError;
pub use network_capture::{collect_network_snapshot, NetworkObservation};
pub use network_capture_event::{network_observation_event, network_snapshot_events};
pub use network_event_runtime::{
    publish_network_runtime_chain_for_observation, NetworkRuntimeEventPayload, NetworkRuntimeReport,
};
pub use network_event_runtime_phase::NetworkRuntimePhase;
pub use network_event_runtime_state::{
    NetworkAiAuditState, NetworkEvidenceGrade, NetworkEvidenceScope, NetworkInterventionState,
    NetworkRiskBudgetState, NetworkRuntimeClaimBoundary,
};
pub use parent_child_event_runtime::{
    publish_parent_child_runtime_for_validated_intent, ParentChildRuntimeEventPayload,
    ParentChildRuntimeInput, ParentChildRuntimeReport,
};
pub use parent_child_event_runtime_phase::ParentChildRuntimePhase;
pub use policy_dry_run_evaluator::{evaluate_policy_dry_run, PolicyDryRunEvaluationInput};
pub use process_capture::{
    collect_process_snapshot, process_observation_event, process_snapshot_events,
    ProcessObservation,
};
pub use screen_evidence_queue::{
    ScreenEvidenceExpiredQueueEntry, ScreenEvidenceQueue, ScreenEvidenceQueueSweep,
};
pub use trusted_device_registry::TrustedDeviceRegistry;
pub use window_capture::{collect_foreground_window_observation, ForegroundWindowObservation};
pub use window_capture_event::{foreground_window_event, foreground_window_observation_event};

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
mod activity_store_tracking_tests;
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
mod browser_performance_health_tests;
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
mod journal_tests;
#[cfg(test)]
mod network_capture_tests;
#[cfg(test)]
mod network_event_runtime_broker_delivery_tests;
#[cfg(test)]
mod network_event_runtime_queue_tests;
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
mod screen_evidence_queue_tests;
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
