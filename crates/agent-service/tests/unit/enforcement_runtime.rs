#![forbid(unsafe_code)]

extern crate ocentra_parent_agent_service as agent_service_lib;
extern crate self as ocentra_parent_agent_service;

#[path = "../support/test_text.rs"]
mod test_text;

#[path = "../../src/activity_capture/persistence.rs"]
mod activity_capture_persistence;
mod activity_api {
    pub(crate) struct GeneratedAtText(pub(crate) String);
}
#[path = "../../src/activity_store_path.rs"]
mod activity_store_path;
#[path = "../../src/activity_api/app_game_adapter_dispatch_execute_payload.rs"]
mod app_game_adapter_dispatch_execute_payload;
#[path = "../../src/activity_api/app_game_adapter_dispatch_preflight_payload.rs"]
mod app_game_adapter_dispatch_preflight_payload;
#[path = "app_game_adapter_dispatch_preflight_payload_tests.rs"]
mod app_game_adapter_dispatch_preflight_payload_tests;
#[path = "../../src/activity_api/app_game_adapter_dispatch_result_fields.rs"]
mod app_game_adapter_dispatch_result_fields;
#[path = "../../src/activity_api/app_game_adapter_dispatch_result_payload.rs"]
mod app_game_adapter_dispatch_result_payload;
#[path = "app_game_adapter_dispatch_result_payload_tests.rs"]
mod app_game_adapter_dispatch_result_payload_tests;
#[path = "app_game_adapter_dispatch_result_service_tests.rs"]
mod app_game_adapter_dispatch_result_service_tests;
#[path = "../../src/activity_api/app_game_adapter_execution_readiness_payload.rs"]
mod app_game_adapter_execution_readiness_payload;
#[path = "app_game_adapter_execution_readiness_payload_tests.rs"]
mod app_game_adapter_execution_readiness_payload_tests;
#[path = "../../src/activity_api/app_game_adapter_host_capabilities.rs"]
mod app_game_adapter_host_capabilities;
#[path = "../../src/activity_api/app_game_adapter_host_capabilities_linux.rs"]
mod app_game_adapter_host_capabilities_linux;
#[path = "../../src/activity_api/app_game_adapter_host_capabilities_paths.rs"]
mod app_game_adapter_host_capabilities_paths;
#[path = "app_game_adapter_host_capabilities.rs"]
mod app_game_adapter_host_capabilities_tests;
#[path = "../../src/app_game_dispatch_evidence.rs"]
mod app_game_dispatch_evidence;
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
#[path = "enforcement_tests.rs"]
mod enforcement_tests;
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
#[path = "../../src/event_builder/build.rs"]
mod event_builder;
#[path = "../../src/fields.rs"]
mod fields;
#[path = "../../src/host_identity_read_model.rs"]
mod host_identity_read_model;
#[path = "integrity_alert_status_bridge_read_model_tests.rs"]
mod integrity_alert_status_bridge_read_model_tests;
#[path = "notification_provider_status_boundary_read_model_tests.rs"]
mod notification_provider_status_boundary_read_model_tests;
#[path = "production_enforcement_api/mod.rs"]
mod production_enforcement_api;
#[path = "../support/test_invariants/require_json_decode.rs"]
mod test_require_json_decode;
#[path = "../support/test_invariants/require_log_string_field.rs"]
mod test_require_log_string_field;
#[path = "../support/test_invariants/require_ok.rs"]
mod test_require_ok;
#[path = "../support/test_invariants/require_some.rs"]
mod test_require_some;
#[path = "../../src/time/now.rs"]
mod time;
#[path = "../../src/windows_adapter_artifact_gate_read_model.rs"]
mod windows_adapter_artifact_gate_read_model;
#[path = "../../src/windows_adapter_capability_read_model.rs"]
mod windows_adapter_capability_read_model;

#[path = "enforcement_runtime/enforcement_api.rs"]
pub(crate) mod enforcement_api;
