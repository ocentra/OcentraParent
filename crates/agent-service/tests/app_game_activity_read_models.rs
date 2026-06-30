#![forbid(unsafe_code)]

extern crate ocentra_parent_agent_service as agent_service_lib;
extern crate self as ocentra_parent_agent_service;

#[path = "support/command_dispatch_test_support.rs"]
pub mod test_support;

#[path = "../src/activity_capture.rs"]
mod activity_capture;
#[path = "support/activity_report_env_lock.rs"]
mod activity_report_env_lock;
#[path = "../src/activity_store_path.rs"]
mod activity_store_path;
#[path = "../src/activity_surface_store.rs"]
mod activity_surface_store;
#[path = "../src/dev_log.rs"]
mod dev_log;
#[path = "../src/enforcement_api.rs"]
mod enforcement_api;
#[path = "../src/enforcement_browser_domain_adapter_app_control_proof_states.rs"]
mod enforcement_browser_domain_adapter_app_control_proof_states;
#[path = "../src/enforcement_browser_domain_adapter_proof_read_model.rs"]
mod enforcement_browser_domain_adapter_proof_read_model;
#[path = "../src/enforcement_capability.rs"]
mod enforcement_capability;
#[path = "../src/enforcement_cross_platform_capability_proof_read_model.rs"]
mod enforcement_cross_platform_capability_proof_read_model;
#[path = "../src/enforcement_os_adapter_product_proof_read_model.rs"]
mod enforcement_os_adapter_product_proof_read_model;
#[path = "../src/enforcement_payload.rs"]
mod enforcement_payload;
#[path = "../src/enforcement_policy_dispatch_read_model.rs"]
mod enforcement_policy_dispatch_read_model;
#[path = "../src/enforcement_timer_state_file.rs"]
mod enforcement_timer_state_file;
#[path = "../src/enforcement_timer_state_path.rs"]
mod enforcement_timer_state_path;
#[path = "../src/event_builder.rs"]
mod event_builder;
#[path = "../src/fields.rs"]
mod fields;
#[path = "../src/host_identity_read_model.rs"]
mod host_identity_read_model;
#[path = "../src/json_contract.rs"]
mod json_contract;
#[path = "support/test_invariants.rs"]
mod test_invariants;
#[path = "../src/time.rs"]
mod time;
#[path = "../src/windows_adapter_artifact_gate_read_model.rs"]
mod windows_adapter_artifact_gate_read_model;
#[path = "../src/windows_adapter_capability_read_model.rs"]
mod windows_adapter_capability_read_model;

#[path = "../src/activity_api/app_game_adapter_dispatch_execute_payload.rs"]
mod app_game_adapter_dispatch_execute_payload;
#[path = "../src/activity_api/app_game_adapter_dispatch_preflight_payload.rs"]
mod app_game_adapter_dispatch_preflight_payload;
#[path = "../src/activity_api/app_game_adapter_dispatch_result_payload.rs"]
mod app_game_adapter_dispatch_result_payload;
#[path = "../src/activity_api/app_game_adapter_execution_readiness_payload.rs"]
mod app_game_adapter_execution_readiness_payload;
#[path = "../src/activity_api/app_game_adapter_host_capabilities.rs"]
mod app_game_adapter_host_capabilities;
#[path = "../src/activity_api/app_game_boundary_read_model_payload.rs"]
mod app_game_boundary_read_model_payload;
#[path = "../src/activity_api/app_game_child_runtime_transport_receipt_payload.rs"]
mod app_game_child_runtime_transport_receipt_payload;
#[path = "../src/activity_api/app_game_notification_readiness_payload.rs"]
mod app_game_notification_readiness_payload;
#[path = "../src/activity_api/app_game_platform_proof_status_payload.rs"]
mod app_game_platform_proof_status_payload;
#[path = "../src/activity_api/app_game_policy_readiness_payload.rs"]
mod app_game_policy_readiness_payload;
#[path = "../src/activity_api/app_game_policy_readiness_sources.rs"]
mod app_game_policy_readiness_sources;

#[path = "unit/app_game_activity_read_models_clippy_linkage_tests.rs"]
mod app_game_activity_read_models_clippy_linkage_tests;
#[path = "unit/app_game_adapter_dispatch_preflight_payload_tests.rs"]
mod app_game_adapter_dispatch_preflight_payload_tests;
#[path = "unit/app_game_adapter_dispatch_preflight_service_tests.rs"]
mod app_game_adapter_dispatch_preflight_service_tests;
#[path = "unit/app_game_adapter_dispatch_result_payload_tests.rs"]
mod app_game_adapter_dispatch_result_payload_tests;
#[path = "unit/app_game_adapter_dispatch_result_service_tests.rs"]
mod app_game_adapter_dispatch_result_service_tests;
#[path = "unit/app_game_adapter_execution_readiness_payload_tests.rs"]
mod app_game_adapter_execution_readiness_payload_tests;
#[path = "unit/app_game_adapter_execution_readiness_service_tests.rs"]
mod app_game_adapter_execution_readiness_service_tests;
#[path = "unit/app_game_boundary_read_model_payload_tests.rs"]
mod app_game_boundary_read_model_payload_tests;
#[path = "unit/app_game_boundary_read_model_service_tests.rs"]
mod app_game_boundary_read_model_service_tests;
#[path = "unit/app_game_child_runtime_transport_receipt_payload_tests.rs"]
mod app_game_child_runtime_transport_receipt_payload_tests;
#[path = "unit/app_game_child_runtime_transport_receipt_service_tests.rs"]
mod app_game_child_runtime_transport_receipt_service_tests;
#[path = "unit/app_game_notification_readiness_payload_tests.rs"]
mod app_game_notification_readiness_payload_tests;
#[path = "unit/app_game_notification_readiness_service_tests.rs"]
mod app_game_notification_readiness_service_tests;
#[path = "unit/app_game_platform_proof_status_payload_tests.rs"]
mod app_game_platform_proof_status_payload_tests;
#[path = "unit/app_game_platform_proof_status_service_tests.rs"]
mod app_game_platform_proof_status_service_tests;
#[path = "unit/app_game_policy_readiness_payload_tests.rs"]
mod app_game_policy_readiness_payload_tests;
#[path = "unit/app_game_policy_readiness_service_tests.rs"]
mod app_game_policy_readiness_service_tests;
