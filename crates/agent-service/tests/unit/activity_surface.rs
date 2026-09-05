extern crate ocentra_parent_agent_service as agent_service_lib;
extern crate self as ocentra_parent_agent_service;

#[path = "../support/activity_surface_app_game_boundary_fixtures.rs"]
mod activity_surface_app_game_boundary_fixtures;
#[path = "../support/activity_surface_app_game_model_fixtures.rs"]
mod activity_surface_app_game_model_fixtures;
#[path = "../support/activity_surface_common_fixtures.rs"]
mod activity_surface_common_fixtures;
#[path = "../support/activity_surface_read_model_fixtures.rs"]
mod activity_surface_read_model_fixtures;
#[path = "../../src/activity_surface_read_model_states.rs"]
mod activity_surface_read_model_states;
#[path = "../../src/activity_surface_read_models.rs"]
mod activity_surface_read_models;
#[path = "../../src/time/now.rs"]
mod time;

#[path = "activity_surface_read_models_direct_tests.rs"]
mod activity_surface_read_models_direct_tests;
#[path = "app_game_source_status_tests.rs"]
mod app_game_source_status_tests;
