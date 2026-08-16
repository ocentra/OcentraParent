use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};
use ocentra_parent_agent_protocol::transport::{AgentCommandName, AgentEventName};

use crate::test_invariants::{
    log_field, require_json_decode, require_log_string_field, require_ok, require_some,
    serialize_test_json,
};

#[tokio::test]
async fn parent_assistant_runtime_helpers_are_linked() {
    let event = crate::parent_assistant_runtime::build_parent_assistant_answer_report(
        super::command_with_payload(Default::default()),
    )
    .await;

    assert_eq!(
        event.event,
        AgentEventName::AgentParentAssistantAnswerReported
    );

    assert_parent_assistant_json_linkage();
    exercise_parent_assistant_api_routes();
    link_activity_surface_helpers();
    link_local_ai_helpers();
}

fn exercise_parent_assistant_api_routes() {
    let cases = [
        (
            AgentCommandName::AgentParentAssistantThreadCreate,
            AgentEventName::AgentParentAssistantThreadUpdated,
        ),
        (
            AgentCommandName::AgentParentAssistantProviderStatusGet,
            AgentEventName::AgentParentAssistantProviderDegraded,
        ),
        (
            AgentCommandName::AgentParentAssistantRunCancel,
            AgentEventName::AgentParentAssistantErrorReported,
        ),
        (
            AgentCommandName::AgentParentAssistantActionPreview,
            AgentEventName::AgentParentAssistantActionPreviewed,
        ),
        (
            AgentCommandName::AgentParentAssistantActionConfirm,
            AgentEventName::AgentParentAssistantActionConfirmed,
        ),
        (
            AgentCommandName::AgentParentAssistantAnswerGenerate,
            AgentEventName::AgentParentAssistantErrorReported,
        ),
    ];

    for (command_name, expected_event) in cases {
        let mut command = super::command_with_payload(Default::default());
        command.command = command_name;
        let event = crate::parent_assistant_api::build_parent_assistant_scaffold_event(command);
        assert_eq!(event.event, expected_event);
    }
}

fn assert_parent_assistant_json_linkage() {
    let encoded = serialize_test_json(&serde_json::json!({"parent_assistant": true}));
    let decoded: serde_json::Value =
        require_json_decode(&encoded, constants::error::AGENT_EVENT_SERIALIZES);
    assert!(require_some(
        decoded
            .get("parent_assistant")
            .and_then(serde_json::Value::as_bool),
        constants::error::AGENT_EVENT_SERIALIZES,
    ));

    let field = LogFieldValue::String(encoded);
    assert_eq!(
        require_log_string_field(Some(&field), constants::error::AGENT_EVENT_SERIALIZES),
        "{\"parent_assistant\":true}"
    );
    let _: () = require_ok(
        Ok::<(), std::io::Error>(()),
        constants::error::AGENT_EVENT_SERIALIZES,
    );

    let mut fields = LogFields::new();
    fields.insert(
        constants::field::ONLINE.to_string(),
        LogFieldValue::Boolean(true),
    );
    assert_eq!(
        log_field(
            &fields,
            constants::field::ONLINE,
            constants::error::AGENT_EVENT_SERIALIZES,
        ),
        LogFieldValue::Boolean(true)
    );
}

fn link_activity_surface_helpers() {
    let _ = crate::event_builder::portal_peer();
    let _ = crate::json_contract::serialize_json_value(serde_json::json!({
        "parent_assistant": true
    }));
    let _ = crate::json_contract::serialize_json_string(&serde_json::json!({
        "parent_assistant": true
    }));
    let _ = crate::activity_store_path::activity_db_path;
    let _ = crate::activity_store_path::activity_journal_path;
    let _ = crate::activity_store_path::activity_journal_key_path;
    let _ = crate::activity_surface_report_store::draft_metadata_for_report;
    let _ = crate::activity_surface_request::report_request_from_command;
    let _ = crate::activity_surface_store::load_browser_model;
    let _ = crate::activity_surface_store::load_browser_model_from_path;
    let _ = crate::activity_surface_store::load_network_model;
    let _ = crate::activity_surface_store::load_network_model_from_path;
    let _ = crate::activity_surface_store::load_app_game_model;
    let _ = crate::activity_surface_store::load_app_game_model_from_path;
    let _ = crate::activity_surface_store::load_screen_summary;
    let _ = crate::activity_surface_store::load_screen_summary_from_path;
    let _ = crate::activity_surface_store::local_store_snapshot;
    let _ = crate::activity_surface_store::local_store_snapshot_from_path;
    let snapshot = crate::activity_surface_store::ActivitySurfaceStoreSnapshot {
        device_id: crate::activity_surface_store::ActivitySurfaceDeviceRefText(String::new()),
        recent_returned: 0,
        last_event_id: None,
        last_observed_at: None,
        browser_returned: 0,
        network_returned: 0,
        games_returned: 0,
        screen_returned: 0,
    };
    let _ = snapshot.device_id;
    let _: String = crate::time::timestamp_from_epoch_seconds(0);
    let _: String = crate::time::timestamp_after_epoch_seconds(0, 1);
    let _ = crate::activity_surface_request::surface_request_from_command;
    let _ = crate::activity_surface_report_file_name::report_file_name;
}

fn link_local_ai_helpers() {
    let _ = crate::local_ai_cache_root::local_ai_cache_root;
    let _ = crate::local_ai_chat_generation::build_local_ai_chat_generation_report;
    let _ = crate::local_ai_chat_generation_args::llama_acceleration_args;
    let _ = crate::local_ai_chat_generation_request::parse_generation_request;
    let _ = crate::local_ai_chat_generation_result::result_id(
        constants::parent_assistant::DEFAULT_MESSAGE_ID,
    );
    let _ = |request: crate::local_ai_chat_generation_request::LocalAiChatGenerationRequest,
             config: &crate::local_ai_runtime_config::LocalAiRuntimeConfigSnapshot| {
        std::mem::drop(
            crate::local_ai_chat_generation_runner::run_local_ai_chat_generation(
                constants::parent_assistant::DEFAULT_MESSAGE_ID,
                request,
                config,
            ),
        );
    };
    let _ = |config: &crate::local_ai_runtime_config::LocalAiRuntimeConfigSnapshot,
             reason: crate::local_ai_runtime_config_values::LocalAiUnavailableReason| {
        crate::local_ai_chat_generation_runner::unavailable_result_for_command(
            constants::parent_assistant::DEFAULT_MESSAGE_ID,
            config,
            reason,
        )
    };
    let _ = crate::local_ai_generation_payload::local_ai_chat_generation_payload;
    let _ = crate::local_ai_model_registry::known_model_for_id;
    let _ = crate::local_ai_provider_scheduler::local_ai_provider_scheduler;
    let _ = crate::local_ai_runtime_acceleration_config::gpu_layers_request_acceleration;
    let _ = |config: &crate::local_ai_runtime_config::LocalAiRuntimeConfigSnapshot| {
        crate::local_ai_runtime_cache_status::local_ai_model_cache_status_from_config(
            constants::local_ai_runtime::TEST_CHECKED_AT,
            config,
        )
    };
    let _ = crate::local_ai_runtime_config::LocalAiRuntimeConfigSnapshot::unconfigured;
    let _ = crate::local_ai_runtime_config_environment::runtime_config_from_environment;
    let _ = crate::local_ai_runtime_config_path::ConfiguredLocalPath::from_path;
    let _ = crate::local_ai_runtime_config_values::validation::is_safe_local_ai_model_id;
    let _ = |config: &crate::local_ai_runtime_config::LocalAiRuntimeConfigSnapshot| {
        crate::local_ai_runtime_configured_status::configured_local_ai_runtime_status(
            constants::local_ai_runtime::TEST_CHECKED_AT,
            config,
        )
    };
    let _ = || {
        crate::local_ai_runtime_distribution::select_llama_runtime_distribution(
            crate::local_ai_runtime_distribution::LocalAiRuntimeTarget::current(),
            crate::local_ai_runtime_distribution::LlamaRuntimeAcceleration::Cpu,
            String::new(),
        )
    };
    let _ = crate::local_ai_runtime_distribution_assets::asset_name;
    let _ =
        |config: &crate::local_ai_runtime_acceleration_config::LocalAiRuntimeAccelerationConfig| {
            crate::local_ai_runtime_install_plan::default_install_plan_from_environment(
                String::new(),
                config,
            )
        };
    let _ = crate::local_ai_runtime_model_selection::requested_model_unavailable_reason;
    let _ = crate::local_ai_runtime_payload::local_ai_runtime_status_payload;
    let _ = |status: &ocentra_parent_agent_protocol::local_ai_runtime::scheduler::LocalAiProviderSchedulerStatus| {
        crate::local_ai_runtime_provider_proof_read_model::local_ai_runtime_provider_proof_read_model(
            String::new(),
            status,
        )
    };
    let _ = crate::local_ai_runtime_readiness::runtime_configuration_unavailable_reason;
    let _ = || crate::local_ai_runtime_status::unavailable_local_ai_runtime_status(String::new());
    let _ = crate::local_ai_runtime_status::build_local_ai_runtime_status_report;
}
