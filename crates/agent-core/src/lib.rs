#![forbid(unsafe_code)]

mod activity_store;
mod activity_store_app_game;
mod activity_store_app_game_observation;
mod activity_store_app_game_rows;
mod activity_store_browser;
mod activity_store_browser_intervention;
mod activity_store_connection;
mod activity_store_enforcement_audit;
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
mod browser_event_runtime;
mod browser_event_runtime_phase;
mod browser_event_runtime_refs;
#[cfg(test)]
mod browser_event_runtime_tests;
mod browser_intervention_event;
mod browser_managed_discovery;
mod browser_managed_session;
#[cfg(test)]
mod browser_performance_health;
mod browser_windows_inventory;
mod browser_windows_inventory_paths;
mod browser_windows_inventory_source;
mod browser_windows_package_inventory;
mod browser_windows_package_source;
mod browser_windows_shortcut_source;
mod enforcement_adapter;
mod enforcement_app_time_limit;
mod enforcement_boundary;
mod enforcement_policy_dispatch;
mod enforcement_readiness;
mod enforcement_timer_state;
mod household_ai_provider_route;
mod household_ai_provider_route_labels;
mod household_ai_provider_route_state;
mod household_mesh_bridge_runtime;
mod household_mesh_bridge_runtime_phase;
mod household_mesh_bridge_runtime_refs;
mod household_mesh_bridge_runtime_source;
mod household_mesh_bridge_runtime_state;
mod household_mesh_event_bridge;
#[cfg(test)]
mod household_mesh_event_bridge_tests;
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
mod screen_event_runtime;
mod screen_event_runtime_input;
mod screen_event_runtime_metadata;
mod screen_event_runtime_phase;
mod screen_event_runtime_refs;
mod screen_event_runtime_state;
mod screen_evidence_queue;
mod screen_household_mesh_runtime;
mod screen_household_mesh_runtime_phase;
mod screen_household_mesh_runtime_refs;
mod screen_household_mesh_runtime_state;
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
pub use browser_event_runtime::{
    browser_runtime_action_intent_handoff_topology_manifest,
    browser_runtime_action_intent_status_topology_manifest,
    browser_runtime_chain_topology_manifest,
    browser_runtime_social_provider_receipt_status_topology_manifest,
    prove_browser_runtime_delivery_decision, publish_browser_runtime_chain_for_input,
    request_browser_runtime_action_intent_handoff_for_input,
    request_browser_runtime_action_intent_status_for_input,
    request_browser_runtime_social_provider_receipt_status_for_input,
    BrowserRuntimeActionIntentHandoffReport, BrowserRuntimeActionIntentHandoffResponse,
    BrowserRuntimeActionIntentStatusReport, BrowserRuntimeActionIntentStatusResponse,
    BrowserRuntimeDeliveryDecisionError, BrowserRuntimeDeliveryDecisionReport,
    BrowserRuntimeEventPayload, BrowserRuntimeInput, BrowserRuntimeReport,
    BrowserRuntimeSocialProviderReceiptStatusReport,
    BrowserRuntimeSocialProviderReceiptStatusResponse,
};
pub use browser_event_runtime_phase::BrowserRuntimePhase;
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
pub use browser_windows_inventory_source::{
    browser_windows_inventory_candidate_paths_from_live_sources,
    browser_windows_live_registry_entry, live_windows_browser_inventory_candidate_paths_with_limit,
    BrowserWindowsLiveRegistryInstallEntry,
};
pub use browser_windows_package_inventory::{
    windows_browser_package_observations, BrowserWindowsPackageIdentity,
};
pub use browser_windows_package_source::{
    live_windows_browser_package_entries_from_roots,
    live_windows_browser_package_entries_with_limit, BrowserWindowsLivePackageEntry,
};
pub use browser_windows_shortcut_source::{
    browser_windows_shortcut_target_from_bytes, live_windows_browser_shortcut_targets_from_roots,
    live_windows_browser_shortcut_targets_with_limit, BrowserWindowsLiveShortcutTarget,
};
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
pub use household_ai_provider_route::{
    select_household_ai_provider_route, HouseholdAiProviderCandidate,
    HouseholdAiRouteCandidateDecision, HouseholdAiRouteRequest, HouseholdAiRouteSelection,
};
pub use household_ai_provider_route_state::{
    HouseholdAiProviderClass, HouseholdAiProviderResourcePolicy, HouseholdAiProviderResourceState,
    HouseholdAiProviderTrustState, HouseholdAiRouteDecisionState, HouseholdAiRouteRejectionReason,
    HouseholdAiWorkClass,
};
pub use household_mesh_bridge_runtime::{
    publish_household_mesh_bridge_chain_for_input, validate_household_mesh_bridge_export,
    validate_household_mesh_bridge_import, HouseholdMeshBridgeEventPayload,
    HouseholdMeshBridgeExportCandidate, HouseholdMeshBridgeInboundEnvelope,
    HouseholdMeshBridgeInput, HouseholdMeshBridgeReport, HouseholdMeshBridgeValidation,
};
pub use household_mesh_bridge_runtime_phase::HouseholdMeshBridgePhase;
pub use household_mesh_bridge_runtime_state::{
    HouseholdMeshBridgeCustody, HouseholdMeshBridgeDirection, HouseholdMeshBridgeEnvelopeState,
    HouseholdMeshBridgeRejectionReason, HouseholdMeshBridgeValidationState,
};
pub use household_mesh_event_bridge::{
    export_selected_local_event, validate_incoming_lan_message, HouseholdMeshAuthenticationState,
    HouseholdMeshBridgeRejection, HouseholdMeshExportDecision, HouseholdMeshImportDecision,
    HouseholdMeshLanMessage, HouseholdMeshLocalEventKind, HouseholdMeshLocalRepublish,
    HouseholdMeshPolicyAuthority,
};
pub use journal::ActivityJournal;
pub use journal_crypto::{JournalKey, JOURNAL_KEY_BYTES};
pub use journal_error::JournalError;
pub use network_capture::{collect_network_snapshot, NetworkObservation};
pub use network_capture_event::{network_observation_event, network_snapshot_events};
pub use network_event_runtime::{
    prove_network_runtime_remote_delivery_cross_process_custody_readiness,
    prove_network_runtime_remote_delivery_cross_process_replay,
    prove_network_runtime_remote_delivery_delete_export_propagation,
    prove_network_runtime_remote_delivery_dispatch_readiness,
    prove_network_runtime_remote_delivery_durable_envelope,
    prove_network_runtime_remote_delivery_external_cross_process_transport,
    prove_network_runtime_remote_delivery_fixture_transport,
    prove_network_runtime_remote_delivery_no_enforcement_invariant,
    prove_network_runtime_remote_delivery_outbox_handoff,
    prove_network_runtime_remote_delivery_provider_child_readiness,
    prove_network_runtime_remote_delivery_transport_dispatch_state,
    publish_network_runtime_chain_for_observation, NetworkRuntimeEventPayload,
    NetworkRuntimeRemoteDeliveryBlockedDispatchRecord,
    NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessError,
    NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessRecord,
    NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessReport,
    NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessState,
    NetworkRuntimeRemoteDeliveryCrossProcessReplayError,
    NetworkRuntimeRemoteDeliveryCrossProcessReplayRecord,
    NetworkRuntimeRemoteDeliveryCrossProcessReplayReport,
    NetworkRuntimeRemoteDeliveryCrossProcessReplayState,
    NetworkRuntimeRemoteDeliveryDeleteExportPropagationError,
    NetworkRuntimeRemoteDeliveryDeleteExportPropagationRecord,
    NetworkRuntimeRemoteDeliveryDeleteExportPropagationReport,
    NetworkRuntimeRemoteDeliveryDeleteExportPropagationState,
    NetworkRuntimeRemoteDeliveryDispatchReadinessError,
    NetworkRuntimeRemoteDeliveryDispatchReadinessReport,
    NetworkRuntimeRemoteDeliveryDispatchReadinessState,
    NetworkRuntimeRemoteDeliveryDurableEnvelopeError,
    NetworkRuntimeRemoteDeliveryDurableEnvelopeReport,
    NetworkRuntimeRemoteDeliveryExternalCrossProcessTransportError,
    NetworkRuntimeRemoteDeliveryExternalCrossProcessTransportRecord,
    NetworkRuntimeRemoteDeliveryExternalCrossProcessTransportReport,
    NetworkRuntimeRemoteDeliveryExternalCrossProcessTransportState,
    NetworkRuntimeRemoteDeliveryFixtureTransportError,
    NetworkRuntimeRemoteDeliveryFixtureTransportRecord,
    NetworkRuntimeRemoteDeliveryFixtureTransportReport,
    NetworkRuntimeRemoteDeliveryFixtureTransportState,
    NetworkRuntimeRemoteDeliveryNoEnforcementInvariantError,
    NetworkRuntimeRemoteDeliveryNoEnforcementInvariantReport,
    NetworkRuntimeRemoteDeliveryNoEnforcementInvariantState,
    NetworkRuntimeRemoteDeliveryNoEnforcementStage, NetworkRuntimeRemoteDeliveryOutboxHandoffError,
    NetworkRuntimeRemoteDeliveryOutboxHandoffReport, NetworkRuntimeRemoteDeliveryOutboxState,
    NetworkRuntimeRemoteDeliveryProviderChildReadinessError,
    NetworkRuntimeRemoteDeliveryProviderChildReadinessRecord,
    NetworkRuntimeRemoteDeliveryProviderChildReadinessReport,
    NetworkRuntimeRemoteDeliveryProviderChildReadinessState,
    NetworkRuntimeRemoteDeliveryReceiptLedgerError,
    NetworkRuntimeRemoteDeliveryReceiptLedgerReport, NetworkRuntimeRemoteDeliveryState,
    NetworkRuntimeRemoteDeliveryStatusError, NetworkRuntimeRemoteDeliveryStatusReport,
    NetworkRuntimeRemoteDeliveryTransportDispatchState,
    NetworkRuntimeRemoteDeliveryTransportDispatchStateError,
    NetworkRuntimeRemoteDeliveryTransportDispatchStateReport,
    NetworkRuntimeRemoteEventChainJournalError, NetworkRuntimeReport,
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
pub use screen_event_runtime::{
    publish_screen_capture_queue_events_for_input, publish_screen_degraded_event_chain_for_input,
    publish_screen_deletion_event_for_input, publish_screen_runtime_chain_for_input,
    ScreenRuntimeEventPayload, ScreenRuntimeReport,
};
pub use screen_event_runtime_input::{
    ScreenRuntimeCaptureInput, ScreenRuntimeDegradedInput, ScreenRuntimeDeletionInput,
    ScreenRuntimeInput,
};
pub use screen_event_runtime_phase::ScreenRuntimePhase;
pub use screen_event_runtime_state::{
    ScreenActionState, ScreenAiAuditState, ScreenDeletionState, ScreenEvidenceScope,
    ScreenPolicyState, ScreenRuntimeClaimBoundary,
};
pub use screen_evidence_queue::{
    ScreenEvidenceExpiredQueueEntry, ScreenEvidenceQueue, ScreenEvidenceQueueSweep,
};
pub use screen_household_mesh_runtime::{
    publish_screen_household_mesh_chain_for_input, validate_screen_household_mesh_result,
    ScreenHouseholdMeshEventPayload, ScreenHouseholdMeshInput, ScreenHouseholdMeshReport,
    ScreenHouseholdMeshResultSubmission, ScreenHouseholdMeshResultValidation,
};
pub use screen_household_mesh_runtime_phase::ScreenHouseholdMeshPhase;
pub use screen_household_mesh_runtime_state::{
    ScreenMeshChildValidationState, ScreenMeshClaimState, ScreenMeshCustodyBoundary,
    ScreenMeshLeaseState, ScreenMeshPayloadMode, ScreenMeshPolicyState,
    ScreenMeshProviderResultState, ScreenMeshResultRejectionReason,
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
