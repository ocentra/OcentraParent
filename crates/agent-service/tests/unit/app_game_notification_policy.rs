#![forbid(unsafe_code)]

extern crate self as ocentra_parent_agent_service;

#[path = "../support/app_game_policy_readiness_sources.rs"]
mod app_game_policy_readiness_sources;
#[path = "../support/test_invariants/require_json_decode.rs"]
mod test_require_json_decode;
#[path = "../support/test_invariants/require_log_string_field.rs"]
mod test_require_log_string_field;
#[path = "../support/test_invariants/require_ok.rs"]
mod test_require_ok;
#[path = "../support/test_invariants/require_some.rs"]
mod test_require_some;

#[path = "../../src/fields.rs"]
mod fields;

#[path = "../../src/activity_store_path.rs"]
mod activity_store_path;
#[path = "../../src/activity_api/app_game_notification_readiness_payload.rs"]
mod app_game_notification_readiness_payload;
#[path = "../../src/activity_api/app_game_notification_readiness_report.rs"]
mod app_game_notification_readiness_report;
#[path = "../../src/activity_api/app_game_policy_readiness_payload.rs"]
mod app_game_policy_readiness_payload;

#[path = "app_game_notification_readiness_payload_tests.rs"]
mod app_game_notification_readiness_payload_tests;
#[path = "app_game_notification_status_handoff_fixture.rs"]
mod app_game_notification_status_handoff_fixture;
#[path = "app_game_notification_status_handoff_tests.rs"]
mod app_game_notification_status_handoff_tests;
#[path = "app_game_policy_readiness_payload_tests.rs"]
mod app_game_policy_readiness_payload_tests;
