#![forbid(unsafe_code)]

#[path = "../../src/event_builder.rs"]
mod event_builder;
#[path = "../../src/fields.rs"]
mod fields;
#[path = "../../src/json_contract.rs"]
mod json_contract;
#[path = "../../src/local_ai_cache_root.rs"]
mod local_ai_cache_root;
#[path = "../../src/local_ai_chat_generation.rs"]
mod local_ai_chat_generation;
#[path = "../../src/local_ai_chat_generation_args.rs"]
mod local_ai_chat_generation_args;
#[path = "../../src/local_ai_chat_generation_request.rs"]
mod local_ai_chat_generation_request;
#[path = "local_ai_chat_generation_request_tests.rs"]
mod local_ai_chat_generation_request_tests;
#[path = "../../src/local_ai_chat_generation_result.rs"]
mod local_ai_chat_generation_result;
#[path = "../../src/local_ai_chat_generation_runner.rs"]
mod local_ai_chat_generation_runner;
#[path = "local_ai_chat_generation_tests.rs"]
mod local_ai_chat_generation_tests;
#[path = "../../src/local_ai_generation_payload.rs"]
mod local_ai_generation_payload;
#[path = "../../src/local_ai_model_registry.rs"]
mod local_ai_model_registry;
#[path = "local_ai_model_registry_tests.rs"]
mod local_ai_model_registry_tests;
#[path = "local_ai_model_request_status_tests.rs"]
mod local_ai_model_request_status_tests;
#[path = "../../src/local_ai_provider_scheduler.rs"]
mod local_ai_provider_scheduler;
#[path = "../../src/local_ai_provider_scheduler_queue.rs"]
mod local_ai_provider_scheduler_queue;
#[path = "../../src/local_ai_provider_scheduler_state.rs"]
mod local_ai_provider_scheduler_state;
#[path = "local_ai_provider_scheduler_tests.rs"]
mod local_ai_provider_scheduler_tests;
#[path = "../../src/local_ai_runtime_acceleration_config.rs"]
mod local_ai_runtime_acceleration_config;
#[path = "local_ai_runtime_acceleration_tests.rs"]
mod local_ai_runtime_acceleration_tests;
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
#[path = "local_ai_runtime_distribution_tests.rs"]
mod local_ai_runtime_distribution_tests;
#[path = "../../src/local_ai_runtime_install_plan.rs"]
mod local_ai_runtime_install_plan;
#[path = "local_ai_runtime_install_plan_tests.rs"]
mod local_ai_runtime_install_plan_tests;
#[path = "../../src/local_ai_runtime_model_selection.rs"]
mod local_ai_runtime_model_selection;
#[path = "../../src/local_ai_runtime_payload.rs"]
mod local_ai_runtime_payload;
#[path = "local_ai_runtime_payload_tests.rs"]
mod local_ai_runtime_payload_tests;
#[path = "../../src/local_ai_runtime_provider_proof_read_model.rs"]
mod local_ai_runtime_provider_proof_read_model;
#[path = "local_ai_runtime_provider_proof_read_model_tests.rs"]
mod local_ai_runtime_provider_proof_read_model_tests;
#[path = "../../src/local_ai_runtime_readiness.rs"]
mod local_ai_runtime_readiness;
#[path = "../../src/local_ai_runtime_status.rs"]
mod local_ai_runtime_status;
#[path = "local_ai_runtime_status_tests.rs"]
mod local_ai_runtime_status_tests;
#[path = "../../src/local_ai_runtime_status_unavailable.rs"]
mod local_ai_runtime_status_unavailable;
#[path = "../support/test_invariants.rs"]
mod test_invariants;
#[path = "../../src/time.rs"]
mod time;

#[cfg(test)]
mod clippy_linkage {
    use crate::test_invariants::{
        require_json_decode, require_log_string_field, serialize_test_json,
    };
    use ocentra_parent_agent_protocol::constants;
    use ocentra_parent_agent_protocol::logging::LogFieldValue;

    #[test]
    fn local_ai_runtime_helpers_are_linked() {
        let _ = crate::event_builder::portal_peer();
        let payload = serde_json::json!({
            "local_ai_runtime": true,
        });
        let serialized = serialize_test_json(&payload);
        let _: serde_json::Value =
            require_json_decode(&serialized, constants::error::AGENT_EVENT_SERIALIZES);
        let field = LogFieldValue::String(serialized);
        let _ = require_log_string_field(Some(&field), constants::error::AGENT_EVENT_SERIALIZES);
        let _ = crate::json_contract::serialize_json_string(&payload);
        let _ = crate::json_contract::serialize_json_value(payload);
        let _: String = crate::time::timestamp_after_epoch_seconds(1, 0);
        let _: String = crate::time::timestamp_after_epoch_seconds(1, 1);
    }
}

#[test]
fn local_ai_runtime_smoke_uses_event_builder_json_and_invariants_helpers() {
    use ocentra_parent_agent_protocol::constants;
    use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields, LogLevel};
    use ocentra_parent_agent_protocol::transport::AgentEventName;

    let payload = crate::fields::fields_from_pairs(vec![(
        constants::field::ONLINE,
        LogFieldValue::Boolean(true),
    )]);
    let event = event_builder::build_event(
        constants::event_id::HEALTH_REPORTED,
        constants::event_id::HEALTH_REPORTED,
        event_builder::portal_peer(),
        AgentEventName::AgentHealthReported,
        LogLevel::Info,
        payload,
        None,
    );
    let epoch: String = time::timestamp_after_epoch_seconds(0, 0);
    let later: String = time::timestamp_after_epoch_seconds(0, 1);
    let serialized = crate::json_contract::serialize_json_string(&serde_json::json!({
        "event_id": &event.event_id,
        "peer_id": &event.target.peer_id,
    }));
    let decoded: serde_json::Value = crate::test_invariants::require_json_decode(
        &serialized,
        constants::error::AGENT_EVENT_SERIALIZES,
    );
    let log_field = LogFieldValue::String(serialized.0.clone());
    let mut helper_fields = LogFields::new();
    helper_fields.insert(
        constants::field::ONLINE.to_string(),
        LogFieldValue::Boolean(true),
    );

    assert_eq!(event.target.peer_id, constants::peer::PORTAL_DEV);
    assert_eq!(
        crate::json_contract::serialize_json_value(decoded.clone()),
        decoded
    );
    assert_eq!(
        crate::test_invariants::require_log_string_field(
            Some(&log_field),
            constants::error::AGENT_EVENT_SERIALIZES,
        ),
        serialized.0.as_str()
    );
    assert_eq!(
        crate::test_invariants::log_field(
            &helper_fields,
            constants::field::ONLINE,
            constants::error::AGENT_EVENT_SERIALIZES,
        ),
        LogFieldValue::Boolean(true)
    );
    let roundtrip = crate::test_invariants::serialize_test_json(&decoded);
    let _: serde_json::Value = crate::test_invariants::require_json_decode(
        &roundtrip,
        constants::error::AGENT_EVENT_SERIALIZES,
    );
    assert_ne!(epoch, later);
}
