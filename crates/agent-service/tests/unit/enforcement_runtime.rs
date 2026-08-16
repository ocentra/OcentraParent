#![forbid(unsafe_code)]

extern crate ocentra_parent_agent_service as agent_service_lib;
extern crate self as ocentra_parent_agent_service;

#[path = "../support/command_dispatch_test_support.rs"]
pub mod test_support;
#[path = "../support/test_text.rs"]
mod test_text;

#[path = "../support/activity_capture_mod.rs"]
mod activity_capture;
mod activity_api {
    pub(crate) struct GeneratedAtText(pub(crate) String);
}
#[path = "../../src/activity_store_path.rs"]
mod activity_store_path;
#[path = "../../src/app_game_dispatch_evidence.rs"]
mod app_game_dispatch_evidence;
#[path = "../../src/dev_log.rs"]
mod dev_log;
#[path = "enforcement_broad_adapter_proof_read_model_tests.rs"]
mod enforcement_broad_adapter_proof_read_model_tests;
#[path = "../../src/enforcement_browser_domain_adapter_app_control_proof_states.rs"]
mod enforcement_browser_domain_adapter_app_control_proof_states;
#[path = "../../src/enforcement_browser_domain_adapter_proof_read_model.rs"]
mod enforcement_browser_domain_adapter_proof_read_model;
#[path = "enforcement_browser_domain_adapter_proof_read_model_tests.rs"]
mod enforcement_browser_domain_adapter_proof_read_model_tests;
#[path = "../../src/enforcement_capability.rs"]
mod enforcement_capability;
#[path = "../../src/enforcement_cross_platform_capability_proof_read_model.rs"]
mod enforcement_cross_platform_capability_proof_read_model;
#[path = "enforcement_cross_platform_capability_proof_read_model_tests.rs"]
mod enforcement_cross_platform_capability_proof_read_model_tests;
#[path = "enforcement_eventing_retry_production_tests.rs"]
mod enforcement_eventing_retry_production_tests;
#[path = "enforcement_integrity_runtime_audit_proof.rs"]
mod enforcement_integrity_runtime_audit_proof;
#[path = "enforcement_integrity_runtime_audit_read_model_tests.rs"]
mod enforcement_integrity_runtime_audit_read_model_tests;
#[path = "../../src/enforcement_os_adapter_product_proof_read_model.rs"]
mod enforcement_os_adapter_product_proof_read_model;
#[path = "enforcement_os_adapter_product_proof_read_model_tests.rs"]
mod enforcement_os_adapter_product_proof_read_model_tests;
#[path = "../../src/enforcement_payload.rs"]
mod enforcement_payload;
#[path = "../../src/enforcement_policy_dispatch_read_model.rs"]
mod enforcement_policy_dispatch_read_model;
#[path = "enforcement_policy_dispatch_read_model_tests.rs"]
mod enforcement_policy_dispatch_read_model_tests;
#[path = "enforcement_rejection_journal_tests.rs"]
mod enforcement_rejection_journal_tests;
#[path = "enforcement_supported_adapter_runtime_proof_read_model_tests.rs"]
mod enforcement_supported_adapter_runtime_proof_read_model_tests;
#[path = "../../src/enforcement_timer_api.rs"]
mod enforcement_timer_api;
#[path = "enforcement_timer_expiry_tests.rs"]
mod enforcement_timer_expiry_tests;
#[path = "../../src/enforcement_timer_payload.rs"]
mod enforcement_timer_payload;
#[path = "../../src/enforcement_timer_report.rs"]
mod enforcement_timer_report;
#[path = "../../src/enforcement_timer_state_file.rs"]
mod enforcement_timer_state_file;
#[path = "../../src/enforcement_timer_state_path.rs"]
mod enforcement_timer_state_path;
#[path = "enforcement_timer_tests.rs"]
mod enforcement_timer_tests;
#[path = "../../src/event_builder.rs"]
mod event_builder;
#[path = "../../src/fields.rs"]
mod fields;
#[path = "../../src/host_identity_read_model.rs"]
mod host_identity_read_model;
#[path = "integrity_alert_status_bridge_read_model_tests.rs"]
mod integrity_alert_status_bridge_read_model_tests;
#[path = "../../src/json_contract.rs"]
mod json_contract;
#[path = "notification_provider_status_boundary_read_model_tests.rs"]
mod notification_provider_status_boundary_read_model_tests;
#[path = "production_enforcement_api/mod.rs"]
mod production_enforcement_api;
#[path = "../support/test_invariants.rs"]
mod test_invariants;
#[path = "../../src/time.rs"]
mod time;
#[path = "../../src/windows_adapter_artifact_gate_read_model.rs"]
mod windows_adapter_artifact_gate_read_model;
#[path = "../../src/windows_adapter_capability_read_model.rs"]
mod windows_adapter_capability_read_model;

#[path = "../../src/enforcement_api.rs"]
pub(crate) mod enforcement_api;

#[test]
fn link_runtime_helpers_used_by_the_current_harness() {
    test_invariants::require_ok::<_, ()>(Ok(()), "link");
    test_invariants::require_some(Some(()), "link");
    let _ = enforcement_api::build_enforcement_audit_report;
    let _ = enforcement_api::build_enforcement_product_control_spine_report;
    let _ = enforcement_api::build_enforcement_policy_dispatch_report;
    let _ = enforcement_api::enforcement_broad_adapter_proof_report::build_enforcement_broad_adapter_proof_report;
    let _ = enforcement_api::enforcement_supported_adapter_runtime_proof_report::build_enforcement_supported_adapter_runtime_proof_report;
    let _ = activity_capture::startup_activity_capture_enabled;
    let _ = activity_capture::startup_activity_capture_enabled_for_value;
    let _ = activity_capture::record_activity_capture_once;
    let _ = activity_capture::record_activity_capture_to_paths;
    let _ = activity_capture::record_activity_capture_to_paths_at;
    let _ = activity_store_path::activity_db_path;
    let _ = activity_store_path::activity_journal_path;
    let _ = activity_store_path::activity_journal_key_path;
    let _: fn(
        dev_log::AgentLogMessageRef<'static>,
        ocentra_parent_agent_protocol::logging::LogFields,
    ) -> std::io::Result<()> = dev_log::write_agent_info;
    let _: fn(
        dev_log::AgentLogMessageRef<'static>,
        ocentra_parent_agent_protocol::logging::LogFields,
    ) -> std::io::Result<()> = dev_log::write_agent_warn;
    let _: fn(
        dev_log::AgentLogMessageRef<'static>,
        ocentra_parent_agent_protocol::logging::LogFields,
    ) -> std::io::Result<()> = dev_log::write_agent_error;
    let _: fn(
        dev_log::AgentLogMessageRef<'static>,
        ocentra_parent_agent_protocol::logging::LogFields,
    ) -> std::io::Result<()> = dev_log::write_agent_debug;
    let _ = event_builder::portal_peer();
    let sample_json = serde_json::json!({ "link": true });
    let _ = json_contract::serialize_json_string(&sample_json);
    let _ = json_contract::serialize_json_value(sample_json.clone());
    let decoded: serde_json::Value =
        test_invariants::require_json_decode(sample_json.to_string(), "link");
    let log_field =
        ocentra_parent_agent_protocol::logging::LogFieldValue::String(String::from("value"));
    let _ = test_invariants::require_log_string_field(Some(&log_field), "link");
    let _ = test_invariants::serialize_test_json(&decoded);
    let _ = enforcement_timer_api::build_enforcement_timer_report;
    let _ = enforcement_api::build_enforcement_audit_report_with_app_game_session;
    let _ = enforcement_api::enforcement_pre_action_journal::eventing_journal::append_enforcement_audit_journal_event_phase;
    let _ = enforcement_api::enforcement_pre_action_journal::eventing_journal::EnforcementEventingJournalPath {
        path: std::path::PathBuf::new(),
    };
    let _ = app_game_dispatch_evidence::AppGameDispatchEvidenceRejection::log_value;
    let _ = app_game_dispatch_evidence::validate_app_game_dispatch_evidence;
    let _ = app_game_dispatch_evidence::validate_app_game_timer_session;
    let _: fn(u64, u64) -> String = time::timestamp_after_epoch_seconds;
    let _: fn(u64, u64) -> String = time::timestamp_after_epoch_seconds;
}
