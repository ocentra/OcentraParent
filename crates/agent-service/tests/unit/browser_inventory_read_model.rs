#![forbid(unsafe_code)]

extern crate ocentra_parent_agent_service as agent_service_lib;
extern crate self as ocentra_parent_agent_service;

#[path = "../support/activity_report_env_lock.rs"]
mod activity_report_env_lock;
#[path = "../../src/browser_inventory_read_model.rs"]
mod browser_inventory_read_model;
#[path = "../support/browser_inventory_test_support.rs"]
mod browser_inventory_test_support;
#[path = "../../src/browser_payload.rs"]
mod browser_payload;
#[path = "../../src/browser_runtime_paths.rs"]
mod browser_runtime_paths;
#[path = "../../src/browser_runtime_status.rs"]
mod browser_runtime_status;
#[path = "../../src/fields.rs"]
mod fields;
#[path = "../support/test_invariants/require_ok.rs"]
mod test_require_ok;
#[path = "../support/test_invariants/require_some.rs"]
mod test_require_some;

#[path = "browser_inventory_read_model_tests.rs"]
mod browser_inventory_read_model_tests;
