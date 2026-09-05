#![forbid(unsafe_code)]

extern crate self as ocentra_parent_agent_service;

#[path = "../support/test_invariants/require_json_decode.rs"]
mod test_require_json_decode;
#[path = "../support/test_invariants/require_log_string_field.rs"]
mod test_require_log_string_field;
#[path = "../support/test_invariants/require_some.rs"]
mod test_require_some;

#[path = "../../src/activity_api/app_game_adapter_host_capabilities.rs"]
mod app_game_adapter_host_capabilities;
#[path = "../../src/activity_api/app_game_adapter_host_capabilities_linux.rs"]
mod app_game_adapter_host_capabilities_linux;
#[path = "../../src/activity_api/app_game_adapter_host_capabilities_paths.rs"]
mod app_game_adapter_host_capabilities_paths;
#[path = "../../src/activity_api/app_game_linux_docker_host_preflight.rs"]
mod app_game_linux_docker_host_preflight;
#[path = "../../src/activity_api/app_game_linux_docker_host_preflight_cleanup.rs"]
mod app_game_linux_docker_host_preflight_cleanup;
#[path = "../../src/activity_api/app_game_linux_docker_host_preflight_cleanup_owner.rs"]
mod app_game_linux_docker_host_preflight_cleanup_owner;
#[path = "../../src/activity_api/app_game_linux_docker_host_preflight_cleanup_process.rs"]
mod app_game_linux_docker_host_preflight_cleanup_process;
#[path = "../../src/activity_api/app_game_linux_docker_host_preflight_cleanup_worker.rs"]
mod app_game_linux_docker_host_preflight_cleanup_worker;
#[path = "../../src/activity_api/app_game_linux_docker_host_preflight_group.rs"]
mod app_game_linux_docker_host_preflight_group;
#[path = "../../src/activity_api/app_game_linux_docker_host_preflight_output.rs"]
mod app_game_linux_docker_host_preflight_output;
#[path = "../../src/activity_api/app_game_linux_docker_host_preflight_path_security.rs"]
mod app_game_linux_docker_host_preflight_path_security;
#[path = "../../src/activity_api/app_game_linux_docker_host_preflight_paths.rs"]
mod app_game_linux_docker_host_preflight_paths;
#[path = "../../src/activity_api/app_game_linux_docker_host_preflight_process.rs"]
mod app_game_linux_docker_host_preflight_process;
#[path = "../../src/activity_api/app_game_linux_docker_host_preflight_state.rs"]
mod app_game_linux_docker_host_preflight_state;
#[path = "../../src/activity_api/app_game_linux_docker_host_preflight_supervisor.rs"]
mod app_game_linux_docker_host_preflight_supervisor;
#[path = "../../src/activity_api/app_game_linux_docker_host_preflight_wait.rs"]
mod app_game_linux_docker_host_preflight_wait;
#[path = "../../src/activity_api/app_game_platform_probe_cache.rs"]
mod app_game_platform_probe_cache;
#[path = "../../src/activity_api/app_game_platform_proof_status_payload.rs"]
mod app_game_platform_proof_status_payload;
#[path = "../../src/fields.rs"]
mod fields;

#[path = "app_game_adapter_host_capabilities.rs"]
mod app_game_adapter_host_capabilities_tests;
#[path = "app_game_linux_capture_readiness.rs"]
mod app_game_linux_capture_readiness;
#[path = "app_game_linux_docker_host_preflight_cleanup_tests.rs"]
mod app_game_linux_docker_host_preflight_cleanup_tests;
#[path = "app_game_linux_docker_host_preflight_parser_tests.rs"]
mod app_game_linux_docker_host_preflight_parser_tests;
#[path = "app_game_linux_docker_host_preflight_path_security_tests.rs"]
mod app_game_linux_docker_host_preflight_path_security_tests;
#[path = "app_game_linux_docker_host_preflight.rs"]
mod app_game_linux_docker_host_preflight_tests;
#[path = "app_game_linux_source_preflight.rs"]
mod app_game_linux_source_preflight;
#[path = "app_game_platform_probe_cache_tests.rs"]
mod app_game_platform_probe_cache_tests;
#[path = "app_game_platform_proof_status_payload_tests.rs"]
mod app_game_platform_proof_status_payload_tests;
#[path = "app_game_platform_proof_status_route_rejection_tests.rs"]
mod app_game_platform_proof_status_route_rejection_tests;
