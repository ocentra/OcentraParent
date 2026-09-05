extern crate ocentra_parent_agent_service as agent_service_lib;
extern crate self as ocentra_parent_agent_service;

#[path = "../../src/activity_capture/injected.rs"]
mod activity_capture;
#[path = "../../src/activity_capture/network_observation.rs"]
mod activity_capture_network_observation;
#[path = "../../src/activity_capture/app_game.rs"]
mod app_game;
#[path = "../../src/activity_capture/capture_events.rs"]
pub(crate) mod capture_events;
#[path = "../../src/activity_capture/errors.rs"]
mod errors;
pub(crate) type ActivityCaptureError = errors::ActivityCaptureError;
#[path = "../../src/activity_capture/persistence.rs"]
mod activity_capture_persistence;
#[path = "../../src/activity_network_flow_payload.rs"]
mod activity_network_flow_payload;
#[path = "../support/activity_report_env_lock.rs"]
mod activity_report_env_lock;
#[path = "../../src/activity_store_path/activity_db.rs"]
mod activity_store_path;
#[path = "../../src/fields.rs"]
mod fields;
#[path = "../../src/network_flow_digest.rs"]
mod network_flow_digest;
#[path = "../../src/network_flow_digest_indicators.rs"]
mod network_flow_digest_indicators;
#[path = "../../src/network_flow_digest_rollups.rs"]
mod network_flow_digest_rollups;
#[path = "../../src/network_runtime_delivery.rs"]
mod network_runtime_delivery;
#[path = "../../src/network_runtime_stream_event_payloads.rs"]
mod network_runtime_stream_event_payloads;
#[path = "../../src/network_runtime_stream_event_values.rs"]
mod network_runtime_stream_event_values;
#[path = "../../src/network_runtime_stream_events.rs"]
mod network_runtime_stream_events;
#[path = "../../src/network_runtime_stream_payload.rs"]
mod network_runtime_stream_payload;
#[path = "../support/network_runtime_test_support.rs"]
mod network_runtime_test_support;
#[path = "../support/activity_capture_test_support.rs"]
mod test_support;
#[path = "../support/activity_capture_test_text.rs"]
mod test_text;
#[path = "../../src/time/now.rs"]
mod time;

#[path = "activity_capture_browser_tests.rs"]
mod activity_capture_browser_tests;
#[path = "activity_capture_freshness_tests.rs"]
mod activity_capture_freshness_tests;
#[cfg(windows)]
#[path = "activity_capture_inventory_tests.rs"]
mod activity_capture_inventory_tests;
#[path = "activity_capture_tests.rs"]
mod activity_capture_tests;
#[path = "network_flow_payload_tests.rs"]
mod network_flow_payload_tests;
#[path = "network_runtime_delivery_tests.rs"]
mod network_runtime_delivery_tests;
#[path = "network_runtime_spine_tests.rs"]
mod network_runtime_spine_tests;
#[path = "network_runtime_stream_tests.rs"]
mod network_runtime_stream_tests;
