extern crate ocentra_parent_agent_service as agent_service_lib;
extern crate self as ocentra_parent_agent_service;

#[path = "../../src/activity_family_sources.rs"]
mod activity_family_sources;
#[path = "../support/activity_report_env_lock.rs"]
mod activity_report_env_lock;
#[path = "../../src/activity_store_path.rs"]
mod activity_store_path;
#[path = "../support/activity_surface_app_game_boundary_fixtures.rs"]
mod activity_surface_app_game_boundary_fixtures;
#[path = "../support/activity_surface_app_game_model_fixtures.rs"]
mod activity_surface_app_game_model_fixtures;
#[path = "../support/activity_surface_common_fixtures.rs"]
mod activity_surface_common_fixtures;
#[path = "../../src/activity_surface_payload.rs"]
mod activity_surface_payload;
#[path = "../../src/activity_surface_read_model_states.rs"]
mod activity_surface_read_model_states;
#[path = "../../src/activity_surface_read_models.rs"]
mod activity_surface_read_models;
#[path = "../../src/activity_surface_report.rs"]
mod activity_surface_report;
#[path = "../../src/activity_surface_report_file_name.rs"]
mod activity_surface_report_file_name;
#[path = "../../src/activity_surface_report_store.rs"]
mod activity_surface_report_store;
#[path = "../../src/activity_surface_request.rs"]
mod activity_surface_request;
#[path = "../../src/activity_surface_store.rs"]
mod activity_surface_store;
#[path = "../../src/fields.rs"]
mod fields;
#[path = "../support/test_invariants.rs"]
mod test_invariants;
#[path = "../support/activity_surface_test_support.rs"]
mod test_support;
#[path = "../support/test_text.rs"]
mod test_text;
#[path = "../../src/time.rs"]
mod time;

#[path = "activity_surface_read_models_direct_tests.rs"]
mod activity_surface_read_models_direct_tests;
#[path = "activity_surface_report_command_tests.rs"]
mod activity_surface_report_command_tests;
