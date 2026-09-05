#![forbid(unsafe_code)]

extern crate ocentra_parent_agent_service as agent_service_lib;
extern crate self as ocentra_parent_agent_service;

#[path = "../../src/activity_store_path/activity_db_value.rs"]
mod activity_store_path;
#[path = "../../src/websocket/policy_request_resolution/persistence.rs"]
mod policy_request_resolution_persistence;

#[path = "policy_request_confirm_tests.rs"]
mod policy_request_confirm_tests;
