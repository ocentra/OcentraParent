#![forbid(unsafe_code)]

extern crate ocentra_parent_agent_service as agent_service_lib;
extern crate self as ocentra_parent_agent_service;

#[path = "../support/app_game_policy_readiness_sources.rs"]
mod app_game_policy_readiness_sources;

#[path = "app_game_activity_read_models_clippy_linkage_tests.rs"]
mod app_game_activity_read_models_clippy_linkage_tests;
#[path = "app_game_adapter_dispatch_preflight_payload_tests.rs"]
mod app_game_adapter_dispatch_preflight_payload_tests;
#[path = "app_game_adapter_dispatch_preflight_service_tests.rs"]
mod app_game_adapter_dispatch_preflight_service_tests;
#[path = "app_game_adapter_dispatch_result_payload_tests.rs"]
mod app_game_adapter_dispatch_result_payload_tests;
#[path = "app_game_adapter_dispatch_result_service_tests.rs"]
mod app_game_adapter_dispatch_result_service_tests;
#[path = "app_game_adapter_execution_readiness_payload_tests.rs"]
mod app_game_adapter_execution_readiness_payload_tests;
#[path = "app_game_adapter_execution_readiness_service_tests.rs"]
mod app_game_adapter_execution_readiness_service_tests;
#[path = "app_game_boundary_read_model_payload_tests.rs"]
mod app_game_boundary_read_model_payload_tests;
#[path = "app_game_boundary_read_model_service_tests.rs"]
mod app_game_boundary_read_model_service_tests;
#[path = "app_game_child_runtime_transport_receipt_payload_tests.rs"]
mod app_game_child_runtime_transport_receipt_payload_tests;
#[path = "app_game_child_runtime_transport_receipt_service_tests.rs"]
mod app_game_child_runtime_transport_receipt_service_tests;
#[path = "app_game_notification_readiness_payload_tests.rs"]
mod app_game_notification_readiness_payload_tests;
#[path = "app_game_notification_readiness_service_tests.rs"]
mod app_game_notification_readiness_service_tests;
#[path = "app_game_platform_proof_status_payload_tests.rs"]
mod app_game_platform_proof_status_payload_tests;
#[path = "app_game_platform_proof_status_service_tests.rs"]
mod app_game_platform_proof_status_service_tests;
#[path = "app_game_policy_readiness_payload_tests.rs"]
mod app_game_policy_readiness_payload_tests;
#[path = "app_game_policy_readiness_service_tests.rs"]
mod app_game_policy_readiness_service_tests;
