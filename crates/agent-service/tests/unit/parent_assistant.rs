#![forbid(unsafe_code)]

#[macro_use]
#[path = "../support/unit_root_basic_harness.rs"]
mod unit_root_basic_harness;
declare_agent_service_unit_root_basic_harness!();

#[path = "../support/activity_report_env_lock.rs"]
mod activity_report_env_lock;
#[path = "parent_assistant_api_tests.rs"]
mod parent_assistant_api_tests;
