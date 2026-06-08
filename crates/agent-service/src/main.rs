#![forbid(unsafe_code)]

mod activity_api;
mod activity_capture;
#[cfg(test)]
mod activity_capture_browser_tests;
#[cfg(test)]
mod activity_capture_tests;
mod activity_family_sources;
#[cfg(test)]
mod activity_family_sources_tests;
mod activity_memory_graph_payload;
#[cfg(test)]
mod activity_memory_graph_payload_tests;
mod activity_network_flow_payload;
mod activity_payload;
#[cfg(test)]
mod activity_report_env_lock;
mod activity_store_path;
mod activity_surface_adapter;
#[cfg(test)]
mod activity_surface_adapter_tests;
mod activity_surface_api;
#[cfg(test)]
mod activity_surface_main_backed_adapter_tests;
mod activity_surface_payload;
#[cfg(test)]
mod activity_surface_payload_tests;
mod activity_surface_read_model_states;
mod activity_surface_read_models;
mod activity_surface_report;
mod activity_surface_report_file_name;
mod activity_surface_report_store;
#[cfg(test)]
mod activity_surface_report_store_tests;
mod activity_surface_request;
mod activity_surface_store;
mod app;
mod browser_evidence_payload;
mod browser_intervention_page;
#[cfg_attr(not(test), allow(dead_code))]
mod browser_inventory_read_model;
#[cfg(test)]
mod browser_inventory_read_model_tests;
mod browser_payload;
mod browser_policy_api;
#[cfg(test)]
mod browser_policy_api_tests;
mod browser_policy_compiler;
mod browser_policy_compiler_assessment;
#[cfg(test)]
mod browser_policy_compiler_tests;
#[cfg(test)]
mod browser_policy_manifest_patch_tests;
mod browser_policy_payload;
mod browser_policy_request;
mod browser_policy_runtime;
mod browser_policy_runtime_support;
mod browser_policy_store;
mod browser_runtime;
mod browser_runtime_paths;
mod browser_runtime_status;
#[cfg(test)]
mod browser_runtime_tests;
mod dev_log;
mod enforcement_api;
#[cfg_attr(not(test), allow(dead_code))]
mod enforcement_browser_domain_adapter_app_control_proof_states;
#[cfg_attr(not(test), allow(dead_code))]
mod enforcement_browser_domain_adapter_proof_read_model;
#[cfg(test)]
mod enforcement_browser_domain_adapter_proof_read_model_tests;
mod enforcement_capability;
#[cfg_attr(not(test), allow(dead_code))]
mod enforcement_cross_platform_capability_proof_read_model;
#[cfg(test)]
mod enforcement_cross_platform_capability_proof_read_model_tests;
#[cfg_attr(not(test), allow(dead_code))]
mod enforcement_os_adapter_product_proof_read_model;
#[cfg(test)]
mod enforcement_os_adapter_product_proof_read_model_tests;
mod enforcement_payload;
mod enforcement_policy_dispatch_read_model;
#[cfg(test)]
mod enforcement_policy_dispatch_read_model_tests;
#[cfg(test)]
mod enforcement_tests;
mod enforcement_timer_api;
#[cfg(test)]
mod enforcement_timer_expiry_tests;
mod enforcement_timer_payload;
mod enforcement_timer_report;
mod enforcement_timer_state_file;
mod enforcement_timer_state_path;
#[cfg(test)]
mod enforcement_timer_tests;
mod event_builder;
mod fields;
#[cfg_attr(not(test), allow(dead_code))]
mod host_identity_read_model;
#[cfg(test)]
mod host_identity_read_model_tests;
mod lan_network_inventory;
mod lan_network_inventory_command;
mod lan_network_inventory_hardware;
mod lan_pairing;
mod lan_pairing_audit;
mod lan_pairing_browser_add_device_scan;
mod lan_pairing_browser_add_device_state;
#[cfg(test)]
mod lan_pairing_browser_add_device_state_tests;
mod lan_pairing_browser_runtime;
#[cfg(test)]
mod lan_pairing_browser_runtime_tests;
mod lan_pairing_household_device_spine;
#[cfg(test)]
mod lan_pairing_household_device_spine_test_fixtures;
#[cfg(test)]
mod lan_pairing_household_device_spine_tests;
#[cfg(test)]
mod lan_pairing_multidevice_tests;
mod lan_pairing_payload;
#[cfg_attr(not(test), allow(dead_code))]
mod lan_pairing_provider_selection_read_model;
#[cfg(test)]
mod lan_pairing_provider_selection_read_model_tests;
mod lan_pairing_runtime_state;
mod lan_pairing_status;
#[cfg(test)]
mod lan_pairing_test_assertions;
#[cfg(test)]
mod lan_pairing_test_commands;
#[cfg(test)]
mod lan_pairing_test_support;
#[cfg(test)]
mod lan_pairing_tests;
mod local_ai_cache_root;
mod local_ai_chat_generation;
mod local_ai_chat_generation_args;
mod local_ai_chat_generation_request;
#[cfg(test)]
mod local_ai_chat_generation_request_tests;
mod local_ai_chat_generation_result;
mod local_ai_chat_generation_runner;
#[cfg(test)]
mod local_ai_chat_generation_tests;
mod local_ai_generation_payload;
mod local_ai_model_registry;
#[cfg(test)]
mod local_ai_model_registry_tests;
#[cfg(test)]
mod local_ai_model_request_status_tests;
mod local_ai_provider_scheduler;
mod local_ai_provider_scheduler_queue;
mod local_ai_provider_scheduler_state;
#[cfg(test)]
mod local_ai_provider_scheduler_tests;
mod local_ai_runtime_acceleration_config;
#[cfg(test)]
mod local_ai_runtime_acceleration_tests;
mod local_ai_runtime_cache_status;
mod local_ai_runtime_config;
mod local_ai_runtime_config_environment;
mod local_ai_runtime_config_parts;
mod local_ai_runtime_config_path;
mod local_ai_runtime_config_values;
mod local_ai_runtime_configured_status;
mod local_ai_runtime_distribution;
mod local_ai_runtime_distribution_assets;
#[cfg(test)]
mod local_ai_runtime_distribution_tests;
mod local_ai_runtime_install_plan;
#[cfg(test)]
mod local_ai_runtime_install_plan_tests;
mod local_ai_runtime_model_selection;
mod local_ai_runtime_payload;
#[cfg(test)]
mod local_ai_runtime_payload_tests;
#[cfg_attr(not(test), allow(dead_code))]
mod local_ai_runtime_provider_proof_read_model;
#[cfg(test)]
mod local_ai_runtime_provider_proof_read_model_tests;
mod local_ai_runtime_readiness;
mod local_ai_runtime_status;
#[cfg(test)]
mod local_ai_runtime_status_tests;
mod local_ai_runtime_status_unavailable;
mod network;
mod network_flow_digest;
mod network_flow_digest_indicators;
mod network_flow_digest_rollups;
#[cfg(test)]
mod network_flow_digest_tests;
#[cfg(test)]
mod network_flow_payload_tests;
mod network_live_capture_readiness_bridge;
#[cfg(test)]
mod network_live_capture_readiness_bridge_tests;
mod network_product_path_bridge;
#[cfg(test)]
mod network_product_path_bridge_tests;
mod network_remote_delivery_status_cross_process;
mod network_remote_delivery_status_payload;
#[cfg(test)]
mod network_remote_delivery_status_service_tests;
mod network_runtime_delivery;
#[cfg(test)]
mod network_runtime_delivery_tests;
mod network_runtime_stream_event_payloads;
mod network_runtime_stream_event_values;
mod network_runtime_stream_events;
mod network_runtime_stream_payload;
#[cfg(test)]
mod network_runtime_stream_tests;
mod parent_assistant_api;
#[cfg(test)]
mod parent_assistant_api_tests;
mod parent_assistant_evidence_context;
mod parent_assistant_payload;
mod parent_assistant_report_history;
mod parent_assistant_runtime;
#[cfg(test)]
mod parent_assistant_runtime_tests;
mod policy_preview_api;
mod policy_preview_payload;
#[cfg(test)]
mod policy_preview_tests;
mod screen_ai_analysis_runtime;
#[cfg(test)]
mod screen_ai_analysis_runtime_tests;
mod screen_ai_cadence_runtime;
mod screen_ai_cadence_runtime_event;
#[cfg(test)]
mod screen_ai_cadence_runtime_tests;
mod screen_ai_foreground_runtime;
mod screen_ai_foreground_runtime_config;
#[cfg(test)]
mod screen_ai_foreground_runtime_tests;
mod screen_ai_retention_sweeper_runtime;
#[cfg(test)]
mod screen_ai_retention_sweeper_runtime_tests;
mod screen_ai_service_capture_event_builder;
mod snapshot;
mod time;
mod tracking_read_model_payload;
#[cfg(test)]
mod tracking_read_model_payload_tests;
#[cfg(test)]
mod tracking_read_model_service_tests;
mod websocket;
#[cfg_attr(not(test), allow(dead_code))]
mod windows_adapter_artifact_gate_read_model;
#[cfg(test)]
mod windows_adapter_artifact_gate_read_model_tests;
#[cfg_attr(not(test), allow(dead_code))]
mod windows_adapter_artifact_ingestion_read_model;
#[cfg(test)]
mod windows_adapter_artifact_ingestion_read_model_tests;
#[cfg_attr(not(test), allow(dead_code))]
mod windows_adapter_capability_read_model;
#[cfg(test)]
mod windows_adapter_capability_read_model_tests;

use ocentra_parent_agent_protocol::constants;

use crate::network::NetworkPolicy;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let network = NetworkPolicy::from_environment();
    let listener = tokio::net::TcpListener::bind(network.bind_address())
        .await
        .expect(constants::error::LOCALHOST_BIND_SUCCEEDS);
    let _ = dev_log::write_agent_info(
        constants::dev_log_message::AGENT_SERVICE_STARTED,
        Default::default(),
    );
    activity_capture::spawn_startup_activity_capture();
    screen_ai_cadence_runtime::spawn_screen_ai_cadence_runtime();
    screen_ai_foreground_runtime::spawn_screen_ai_foreground_runtime();
    screen_ai_analysis_runtime::spawn_screen_ai_analysis_runtime();
    screen_ai_retention_sweeper_runtime::spawn_screen_ai_retention_sweeper_runtime();

    axum::serve(listener, app::router(network))
        .await
        .expect(constants::error::AGENT_SERVICE_RUNS);
}
