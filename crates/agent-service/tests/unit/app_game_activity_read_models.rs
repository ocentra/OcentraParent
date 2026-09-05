#![forbid(unsafe_code)]

extern crate self as ocentra_parent_agent_service;

#[path = "../support/activity_report_env_lock.rs"]
mod activity_report_env_lock;
#[path = "../support/test_invariants/require_json_decode.rs"]
mod test_require_json_decode;
#[path = "../support/test_invariants/require_log_string_field.rs"]
mod test_require_log_string_field;
#[path = "../support/test_invariants/require_ok.rs"]
mod test_require_ok;

#[path = "../../src/activity_store_path/activity_db.rs"]
mod activity_store_path;
#[path = "../support/activity_surface_app_game_store.rs"]
mod activity_surface_store;
#[path = "../../src/event_builder/build.rs"]
mod event_builder;
#[path = "../../src/fields.rs"]
mod fields;
#[path = "../../src/time/now.rs"]
mod time;

#[path = "../../src/activity_api/app_game_boundary_read_model_payload.rs"]
mod app_game_boundary_read_model_payload;
#[path = "../../src/activity_api/app_game_boundary_read_model_payload_rows.rs"]
mod app_game_boundary_read_model_payload_rows;
#[path = "../../src/activity_api/app_game_child_runtime_transport_receipt_payload.rs"]
mod app_game_child_runtime_transport_receipt_payload;

#[path = "app_game_boundary_read_model_payload_tests.rs"]
mod app_game_boundary_read_model_payload_tests;
#[path = "app_game_child_runtime_transport_receipt_payload_tests.rs"]
mod app_game_child_runtime_transport_receipt_payload_tests;
