#![forbid(unsafe_code)]

extern crate ocentra_parent_agent_service as agent_service_lib;
extern crate self as ocentra_parent_agent_service;

#[path = "../support/activity_report_env_lock.rs"]
mod activity_report_env_lock;
#[path = "../../src/activity_store_path/activity_db.rs"]
mod activity_store_path;
#[path = "../support/activity_surface_app_game_store.rs"]
mod activity_surface_store;
#[path = "../../src/activity_api/app_game_child_runtime_transport_receipt_payload.rs"]
mod app_game_child_runtime_transport_receipt_payload;
#[path = "../../src/activity_api/app_game_timer_parent_preference_setup_request.rs"]
mod app_game_timer_parent_preference_setup_request;
#[path = "../../src/activity_api/app_game_timer_parent_preference_setup_request_outbox.rs"]
mod app_game_timer_parent_preference_setup_request_outbox;
#[path = "../../src/activity_api/app_game_timer_parent_preference_setup_request_persistence.rs"]
mod app_game_timer_parent_preference_setup_request_persistence;
#[path = "../support/app_game_timer_parent_preference_setup_request_status.rs"]
mod app_game_timer_parent_preference_setup_request_status;
#[path = "app_game_timer_parent_preference_setup_request_tests.rs"]
mod app_game_timer_parent_preference_setup_request_tests;
#[path = "../../src/event_builder/build.rs"]
mod event_builder;
#[path = "../../src/fields.rs"]
mod fields;
#[path = "../../src/json_contract/string.rs"]
mod json_contract;
#[path = "../support/test_invariants/log_field.rs"]
mod test_log_field;
#[path = "../support/test_invariants/require_json_decode.rs"]
mod test_require_json_decode;
#[path = "../support/test_invariants/require_log_string_field.rs"]
mod test_require_log_string_field;
#[path = "../support/test_invariants/require_ok.rs"]
mod test_require_ok;
#[path = "../support/test_invariants/require_some.rs"]
mod test_require_some;
#[path = "../support/test_invariants/serialize_test_json.rs"]
mod test_serialize_json;
#[path = "../../src/time/now.rs"]
mod time;

use ocentra_parent_agent_protocol::transport::{AgentCommandEnvelope, AgentEventEnvelope};

pub(crate) async fn build_child_runtime_receipt_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    app_game_child_runtime_transport_receipt_payload::build_activity_app_game_child_runtime_transport_receipt_report(command).await
}

pub(crate) async fn build_timer_preference_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    app_game_timer_parent_preference_setup_request::build_activity_app_game_timer_parent_preference_setup_request_report(command).await
}

pub(crate) async fn build_timer_preference_report_for_store_path(
    command: AgentCommandEnvelope,
    store_path: app_game_timer_parent_preference_setup_request::AppGameTimerSetupStorePath,
) -> AgentEventEnvelope {
    app_game_timer_parent_preference_setup_request::build_activity_app_game_timer_parent_preference_setup_request_report_for_store_path(command, store_path).await
}
