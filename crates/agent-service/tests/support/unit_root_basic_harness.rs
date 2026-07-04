macro_rules! declare_agent_service_unit_root_basic_harness {
    () => {
        #[path = "../support/activity_api/mod.rs"]
        mod activity_api;
        #[path = "../support/activity_capture/mod.rs"]
        mod activity_capture;
        #[path = "../../src/activity_network_flow_payload.rs"]
        mod activity_network_flow_payload;
        #[path = "../../src/activity_payload.rs"]
        mod activity_payload;
        #[path = "../support/activity_report_env_lock.rs"]
        mod activity_report_env_lock;
        #[path = "../../src/activity_store_path.rs"]
        mod activity_store_path;
        #[path = "../../src/activity_surface_read_model_states.rs"]
        mod activity_surface_read_model_states;
        #[path = "../../src/activity_surface_report_file_name.rs"]
        mod activity_surface_report_file_name;
        #[path = "../support/activity_surface_report_store/mod.rs"]
        mod activity_surface_report_store;
        #[path = "../../src/activity_surface_request.rs"]
        mod activity_surface_request;
        #[path = "../../src/activity_surface_store.rs"]
        mod activity_surface_store;
        #[path = "../../src/browser_evidence_payload.rs"]
        mod browser_evidence_payload;
        #[path = "../../src/browser_inventory_read_model.rs"]
        mod browser_inventory_read_model;
        #[path = "../../src/browser_payload.rs"]
        mod browser_payload;
        #[path = "../../src/browser_runtime_delivery.rs"]
        mod browser_runtime_delivery;
        #[path = "../../src/browser_runtime_paths.rs"]
        mod browser_runtime_paths;
        #[path = "../../src/browser_runtime_stream_api.rs"]
        mod browser_runtime_stream_api;
        #[path = "../../src/browser_runtime_stream_events.rs"]
        mod browser_runtime_stream_events;
        #[path = "../../src/browser_runtime_stream_payload.rs"]
        mod browser_runtime_stream_payload;
        #[path = "../../src/browser_runtime_stream_request.rs"]
        mod browser_runtime_stream_request;
        #[path = "../../src/event_builder.rs"]
        mod event_builder;
        #[path = "../../src/fields.rs"]
        mod fields;
        #[path = "../../src/json_contract.rs"]
        mod json_contract;
        #[path = "../../src/network_flow_digest.rs"]
        mod network_flow_digest;
        #[path = "../../src/network_flow_digest_indicators.rs"]
        mod network_flow_digest_indicators;
        #[path = "../../src/network_flow_digest_rollups.rs"]
        mod network_flow_digest_rollups;
        #[path = "../../src/network_runtime_stream_event_payloads.rs"]
        mod network_runtime_stream_event_payloads;
        #[path = "../../src/network_runtime_stream_event_values.rs"]
        mod network_runtime_stream_event_values;
        #[path = "../../src/local_ai_cache_root.rs"]
        mod local_ai_cache_root;
        #[path = "../../src/local_ai_chat_generation.rs"]
        mod local_ai_chat_generation;
        #[path = "../../src/local_ai_chat_generation_args.rs"]
        mod local_ai_chat_generation_args;
        #[path = "../../src/local_ai_chat_generation_request.rs"]
        mod local_ai_chat_generation_request;
        #[path = "../../src/local_ai_chat_generation_result.rs"]
        mod local_ai_chat_generation_result;
        #[path = "../../src/local_ai_chat_generation_runner.rs"]
        mod local_ai_chat_generation_runner;
        #[path = "../../src/local_ai_generation_payload.rs"]
        mod local_ai_generation_payload;
        #[path = "../../src/local_ai_model_registry.rs"]
        mod local_ai_model_registry;
        #[path = "../../src/local_ai_provider_scheduler.rs"]
        mod local_ai_provider_scheduler;
        #[path = "../../src/local_ai_provider_scheduler_queue.rs"]
        mod local_ai_provider_scheduler_queue;
        #[path = "../../src/local_ai_provider_scheduler_state.rs"]
        mod local_ai_provider_scheduler_state;
        #[path = "../../src/local_ai_runtime_acceleration_config.rs"]
        mod local_ai_runtime_acceleration_config;
        #[path = "../../src/local_ai_runtime_cache_status.rs"]
        mod local_ai_runtime_cache_status;
        #[path = "../../src/local_ai_runtime_config.rs"]
        mod local_ai_runtime_config;
        #[path = "../../src/local_ai_runtime_config_environment.rs"]
        mod local_ai_runtime_config_environment;
        #[path = "../../src/local_ai_runtime_config_parts.rs"]
        mod local_ai_runtime_config_parts;
        #[path = "../../src/local_ai_runtime_config_path.rs"]
        mod local_ai_runtime_config_path;
        #[path = "../../src/local_ai_runtime_config_values.rs"]
        mod local_ai_runtime_config_values;
        #[path = "../../src/local_ai_runtime_configured_status.rs"]
        mod local_ai_runtime_configured_status;
        #[path = "../../src/local_ai_runtime_distribution.rs"]
        mod local_ai_runtime_distribution;
        #[path = "../../src/local_ai_runtime_distribution_assets.rs"]
        mod local_ai_runtime_distribution_assets;
        #[path = "../../src/local_ai_runtime_install_plan.rs"]
        mod local_ai_runtime_install_plan;
        #[path = "../../src/local_ai_runtime_model_selection.rs"]
        mod local_ai_runtime_model_selection;
        #[path = "../../src/local_ai_runtime_payload.rs"]
        mod local_ai_runtime_payload;
        #[path = "../../src/local_ai_runtime_provider_proof_read_model.rs"]
        mod local_ai_runtime_provider_proof_read_model;
        #[path = "../../src/local_ai_runtime_readiness.rs"]
        mod local_ai_runtime_readiness;
        #[path = "../../src/local_ai_runtime_status.rs"]
        mod local_ai_runtime_status;
        #[path = "../../src/local_ai_runtime_status_unavailable.rs"]
        mod local_ai_runtime_status_unavailable;
        #[path = "../../src/network_product_path_bridge.rs"]
        mod network_product_path_bridge;
        #[path = "../../src/network_runtime_delivery.rs"]
        mod network_runtime_delivery;
        #[path = "../../src/network_runtime_stream_events.rs"]
        mod network_runtime_stream_events;
        #[path = "../../src/network_runtime_stream_payload.rs"]
        mod network_runtime_stream_payload;
        #[path = "../../src/parent_assistant_api.rs"]
        mod parent_assistant_api;
        #[path = "../../src/parent_assistant_evidence_context.rs"]
        mod parent_assistant_evidence_context;
        #[path = "../../src/parent_assistant_payload.rs"]
        mod parent_assistant_payload;
        #[path = "../../src/parent_assistant_report_history.rs"]
        mod parent_assistant_report_history;
        #[path = "../../src/parent_assistant_runtime.rs"]
        mod parent_assistant_runtime;
        #[path = "../../src/policy_preview_api.rs"]
        mod policy_preview_api;
        #[path = "../../src/policy_preview_payload.rs"]
        mod policy_preview_payload;
        #[path = "../support/test_invariants.rs"]
        mod test_invariants;
        #[path = "../support/test_text.rs"]
        mod test_text;
        #[path = "../../src/time.rs"]
        mod time;
    };
}
