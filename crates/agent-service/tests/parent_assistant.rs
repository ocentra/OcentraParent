#![forbid(unsafe_code)]

#[path = "support/activity_report_env_lock.rs"]
mod activity_report_env_lock;
#[path = "../src/activity_store_path.rs"]
mod activity_store_path;
#[path = "../src/activity_surface_report_file_name.rs"]
mod activity_surface_report_file_name;
#[path = "../src/activity_surface_report_store.rs"]
mod activity_surface_report_store;
#[path = "../src/activity_surface_request.rs"]
mod activity_surface_request;
#[path = "../src/activity_surface_store.rs"]
mod activity_surface_store;
#[path = "../src/event_builder.rs"]
mod event_builder;
#[path = "../src/fields.rs"]
mod fields;
#[path = "../src/json_contract.rs"]
mod json_contract;
#[path = "../src/local_ai_cache_root.rs"]
mod local_ai_cache_root;
#[path = "../src/local_ai_chat_generation_args.rs"]
mod local_ai_chat_generation_args;
#[path = "../src/local_ai_chat_generation_request.rs"]
mod local_ai_chat_generation_request;
#[path = "../src/local_ai_chat_generation_result.rs"]
mod local_ai_chat_generation_result;
#[path = "../src/local_ai_chat_generation_runner.rs"]
mod local_ai_chat_generation_runner;
#[path = "../src/local_ai_generation_payload.rs"]
mod local_ai_generation_payload;
#[path = "../src/local_ai_model_registry.rs"]
mod local_ai_model_registry;
#[path = "../src/local_ai_provider_scheduler.rs"]
mod local_ai_provider_scheduler;
#[path = "../src/local_ai_provider_scheduler_queue.rs"]
mod local_ai_provider_scheduler_queue;
#[path = "../src/local_ai_provider_scheduler_state.rs"]
mod local_ai_provider_scheduler_state;
#[path = "../src/local_ai_runtime_acceleration_config.rs"]
mod local_ai_runtime_acceleration_config;
#[path = "../src/local_ai_runtime_cache_status.rs"]
mod local_ai_runtime_cache_status;
#[path = "../src/local_ai_runtime_config.rs"]
mod local_ai_runtime_config;
#[path = "../src/local_ai_runtime_config_environment.rs"]
mod local_ai_runtime_config_environment;
#[path = "../src/local_ai_runtime_config_parts.rs"]
mod local_ai_runtime_config_parts;
#[path = "../src/local_ai_runtime_config_path.rs"]
mod local_ai_runtime_config_path;
#[path = "../src/local_ai_runtime_config_values.rs"]
mod local_ai_runtime_config_values;
#[path = "../src/local_ai_runtime_configured_status.rs"]
mod local_ai_runtime_configured_status;
#[path = "../src/local_ai_runtime_distribution.rs"]
mod local_ai_runtime_distribution;
#[path = "../src/local_ai_runtime_distribution_assets.rs"]
mod local_ai_runtime_distribution_assets;
#[path = "../src/local_ai_runtime_install_plan.rs"]
mod local_ai_runtime_install_plan;
#[path = "../src/local_ai_runtime_model_selection.rs"]
mod local_ai_runtime_model_selection;
#[path = "../src/local_ai_runtime_payload.rs"]
mod local_ai_runtime_payload;
#[path = "../src/local_ai_runtime_provider_proof_read_model.rs"]
mod local_ai_runtime_provider_proof_read_model;
#[path = "../src/local_ai_runtime_readiness.rs"]
mod local_ai_runtime_readiness;
#[path = "../src/local_ai_runtime_status.rs"]
mod local_ai_runtime_status;
#[path = "../src/local_ai_runtime_status_unavailable.rs"]
mod local_ai_runtime_status_unavailable;
#[path = "../src/parent_assistant_api.rs"]
mod parent_assistant_api;
#[path = "unit/parent_assistant_api_tests.rs"]
mod parent_assistant_api_tests;
#[path = "../src/parent_assistant_evidence_context.rs"]
mod parent_assistant_evidence_context;
#[path = "../src/parent_assistant_payload.rs"]
mod parent_assistant_payload;
#[path = "../src/parent_assistant_report_history.rs"]
mod parent_assistant_report_history;
#[path = "../src/parent_assistant_runtime.rs"]
mod parent_assistant_runtime;
#[path = "unit/parent_assistant_runtime_tests.rs"]
mod parent_assistant_runtime_tests;
#[path = "support/test_invariants.rs"]
mod test_invariants;
#[path = "../src/time.rs"]
mod time;

#[cfg(test)]
mod clippy_linkage {
    use super::*;
    use crate::test_invariants::{
        require_json_decode, require_log_string_field, require_ok, require_some,
        serialize_test_json,
    };
    use ocentra_parent_agent_protocol::constants;
    use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};
    use ocentra_parent_agent_protocol::transport::{
        AgentCommandEnvelope, AgentCommandName, AgentEventName, AgentMessageTarget, AgentPeer,
        AgentPeerRole, AgentRoute,
    };
    use ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION;

    #[tokio::test]
    async fn parent_assistant_runtime_and_helpers_are_linked() {
        let encoded = serialize_test_json(&serde_json::json!({
            "parent_assistant": true
        }));
        let decoded: serde_json::Value =
            require_json_decode(&encoded, "parent_assistant linkage json");
        assert!(require_some(
            decoded
                .get("parent_assistant")
                .and_then(|value| value.as_bool()),
            "parent_assistant linkage bool",
        ));
        let field = LogFieldValue::String(encoded);
        let text = require_log_string_field(Some(&field), "parent_assistant linkage field");
        let _: serde_json::Value = require_json_decode(text, "parent_assistant linkage field json");
        let _: () = require_ok(Ok::<(), std::io::Error>(()), "parent_assistant linkage ok");

        let _ = event_builder::portal_peer();
        let _ = json_contract::serialize_json_value(serde_json::json!({
            "parent_assistant": true
        }));
        let _ = activity_store_path::activity_journal_path();
        let _ = activity_store_path::activity_journal_key_path();
        let _ = activity_surface_report_store::draft_metadata_for_report;
        let _ = activity_surface_request::report_request_from_command;
        let _ = activity_surface_store::load_browser_model;
        let _ = activity_surface_store::load_browser_model_from_path;
        let _ = activity_surface_store::load_network_model;
        let _ = activity_surface_store::load_network_model_from_path;
        let _ = activity_surface_store::load_app_game_model;
        let _ = activity_surface_store::load_app_game_model_from_path;
        let _ = activity_surface_store::load_screen_summary;
        let _ = activity_surface_store::load_screen_summary_from_path;
        let snapshot = activity_surface_store::ActivitySurfaceStoreSnapshot {
            device_id: String::new(),
            recent_returned: 0,
            last_event_id: None,
            last_observed_at: None,
            browser_returned: 0,
            network_returned: 0,
            games_returned: 0,
            screen_returned: 0,
        };
        let _ = snapshot.device_id;
        let _ = time::timestamp_from_epoch_seconds;
        let _ = time::timestamp_after_epoch_seconds;
        let _ = activity_surface_request::surface_request_from_command;
        let _ = activity_surface_report_file_name::report_file_name;
        let _ = local_ai_cache_root::local_ai_cache_root;
        let _ = local_ai_chat_generation_args::llama_acceleration_args;
        let _ = local_ai_chat_generation_request::parse_generation_request;
        let _ = local_ai_chat_generation_result::result_id;
        let _ = local_ai_chat_generation_runner::run_local_ai_chat_generation;
        let _ = local_ai_generation_payload::local_ai_chat_generation_payload;
        let _ = local_ai_model_registry::known_model_for_id;
        let _ = local_ai_provider_scheduler::local_ai_provider_scheduler;
        let _ = local_ai_runtime_acceleration_config::gpu_layers_request_acceleration;
        let _ = local_ai_runtime_cache_status::local_ai_model_cache_status_from_config;
        let _ = local_ai_runtime_config::LocalAiRuntimeConfigSnapshot::unconfigured;
        let _ = local_ai_runtime_config_environment::runtime_config_from_environment;
        let _ = local_ai_runtime_config_path::ConfiguredLocalPath::from_path;
        let _ = local_ai_runtime_config_values::is_safe_local_ai_model_id;
        let _ = local_ai_runtime_configured_status::configured_local_ai_runtime_status;
        let _ = local_ai_runtime_distribution::select_llama_runtime_distribution;
        let _ = local_ai_runtime_distribution_assets::asset_name;
        let _ = local_ai_runtime_install_plan::default_install_plan_from_environment;
        let _ = local_ai_runtime_model_selection::requested_model_unavailable_reason;
        let _ = local_ai_runtime_payload::local_ai_runtime_status_payload;
        let _ =
            local_ai_runtime_provider_proof_read_model::local_ai_runtime_provider_proof_read_model;
        let _ = local_ai_runtime_readiness::runtime_configuration_unavailable_reason;
        let _ = local_ai_runtime_status::unavailable_local_ai_runtime_status;
        let _ = local_ai_runtime_status::build_local_ai_runtime_status_report;
        let _ = local_ai_chat_generation_runner::unavailable_result_for_command;

        let event =
            parent_assistant_runtime::build_parent_assistant_answer_report(AgentCommandEnvelope {
                schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
                message_id: "cmd-parent-assistant-clippy".to_string(),
                sent_at: "2026-06-29T00:00:00Z".to_string(),
                source: AgentPeer {
                    peer_id: constants::peer::PORTAL_DEV.to_string(),
                    role: AgentPeerRole::Portal,
                },
                target: AgentMessageTarget {
                    device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
                    platform: "windows".to_string(),
                    route: AgentRoute::Localhost,
                },
                command: AgentCommandName::AgentParentAssistantAnswerGenerate,
                payload: LogFields::new(),
            })
            .await;

        assert_eq!(
            event.event,
            AgentEventName::AgentParentAssistantAnswerReported
        );
    }
}
