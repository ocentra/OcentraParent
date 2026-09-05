#![forbid(unsafe_code)]

extern crate self as ocentra_parent_agent_service;

#[path = "../../src/fields.rs"]
mod fields;
#[path = "../support/test_invariants/require_json_decode.rs"]
mod test_require_json_decode;
#[path = "../support/test_invariants/require_log_string_field.rs"]
mod test_require_log_string_field;
#[path = "../support/test_invariants/require_ok.rs"]
mod test_require_ok;

#[path = "../../src/activity_api/app_game_timer_parent_surface_action_results.rs"]
mod app_game_timer_parent_surface_action_results;
#[path = "../support/app_game_timer_parent_surface_payload.rs"]
mod app_game_timer_parent_surface_payload;

#[path = "app_game_timer_parent_surface_payload_tests.rs"]
mod app_game_timer_parent_surface_payload_tests;
