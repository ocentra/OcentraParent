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
mod browser_bridge_event;
mod browser_bridge_fields;
mod browser_bridge_http;
mod browser_bridge_ids;
mod browser_bridge_poll;
mod browser_intervention_event;
mod browser_managed_discovery;
mod browser_managed_session;
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
mod policy_dry_run_evaluator;
mod process_capture;
mod screen_evidence_queue;
mod window_capture;
mod window_capture_event;

pub use activity_store::ActivityStore;
pub use activity_store_error::ActivityStoreError;
pub use browser_bridge_event::{
    browser_tab_observation_event, BrowserBridgeEventError, BrowserBridgeTargetObservation,
};
pub use browser_bridge_poll::{
    poll_chromium_bridge, BrowserBridgePollConfig, BrowserBridgePollError,
    BrowserBridgePollSnapshot,
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
    launch_managed_browser, managed_browser_launch_plan, BrowserManagedLaunch,
    BrowserManagedLaunchConfig, BrowserManagedLaunchError, BrowserManagedLaunchPlan,
};
pub use journal::ActivityJournal;
pub use journal_crypto::{JournalKey, JOURNAL_KEY_BYTES};
pub use journal_error::JournalError;
pub use network_capture::{collect_network_snapshot, NetworkObservation};
pub use network_capture_event::{network_observation_event, network_snapshot_events};
pub use policy_dry_run_evaluator::{evaluate_policy_dry_run, PolicyDryRunEvaluationInput};
pub use process_capture::{
    collect_process_snapshot, process_observation_event, process_snapshot_events,
    ProcessObservation,
};
pub use screen_evidence_queue::ScreenEvidenceQueue;
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
mod browser_bridge_poll_tests;
#[cfg(test)]
mod browser_bridge_tests;
#[cfg(test)]
mod browser_managed_session_tests;
#[cfg(test)]
mod journal_tests;
#[cfg(test)]
mod network_capture_tests;
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
mod window_capture_tests;

#[cfg(test)]
mod tests {
    use super::crate_name;

    #[test]
    fn crate_name_identifies_agent_core_boundary() {
        assert_eq!(crate_name(), env!("CARGO_PKG_NAME"));
    }
}
