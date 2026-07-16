#![forbid(unsafe_code)]

#[macro_use]
#[path = "../support/unit_root_basic_harness.rs"]
mod unit_root_basic_harness;
declare_agent_service_unit_root_basic_harness!();

#[path = "../support/activity_report_env_lock.rs"]
mod activity_report_env_lock;
#[path = "parent_assistant_api_tests.rs"]
mod parent_assistant_api_tests;

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
        assert_parent_assistant_json_linkage();
        link_activity_surface_helpers();
        link_local_ai_helpers();

        let event =
            parent_assistant_runtime::build_parent_assistant_answer_report(command_envelope())
                .await;

        assert_eq!(
            event.event,
            AgentEventName::AgentParentAssistantAnswerReported
        );
    }

    fn assert_parent_assistant_json_linkage() {
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
    }

    fn link_activity_surface_helpers() {
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
        let snapshot = empty_activity_surface_snapshot();
        let _ = snapshot.device_id;
        let _ = time::timestamp_from_epoch_seconds;
        let _ = time::timestamp_after_epoch_seconds;
        let _ = activity_surface_request::surface_request_from_command;
        let _ = activity_surface_report_file_name::report_file_name;
    }

    fn empty_activity_surface_snapshot() -> activity_surface_store::ActivitySurfaceStoreSnapshot {
        activity_surface_store::ActivitySurfaceStoreSnapshot {
            device_id: String::new(),
            recent_returned: 0,
            last_event_id: None,
            last_observed_at: None,
            browser_returned: 0,
            network_returned: 0,
            games_returned: 0,
            screen_returned: 0,
        }
    }

    fn link_local_ai_helpers() {
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
        let _ = local_ai_runtime_config_values::validation::is_safe_local_ai_model_id;
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
    }

    fn command_envelope() -> AgentCommandEnvelope {
        AgentCommandEnvelope {
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
        }
    }
}
