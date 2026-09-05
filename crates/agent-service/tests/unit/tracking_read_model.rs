#![forbid(unsafe_code)]

extern crate ocentra_parent_agent_service as agent_service_lib;
extern crate self as ocentra_parent_agent_service;

#[path = "../../src/event_builder/build.rs"]
mod event_builder;
#[path = "../../src/fields.rs"]
mod fields;
#[path = "../../src/time/now.rs"]
mod time;
#[path = "../../src/websocket/tracking_retention_settings_write.rs"]
mod tracking_retention_settings_write;

#[path = "tracking_retention_settings_write_tests.rs"]
mod tracking_retention_settings_write_tests;
