#![forbid(unsafe_code)]

mod activity_api;
mod activity_capture;
#[cfg(test)]
mod activity_capture_tests;
mod activity_network_flow_payload;
mod activity_payload;
mod activity_store_path;
mod app;
mod browser_evidence_payload;
mod browser_payload;
mod browser_runtime;
mod browser_runtime_paths;
mod browser_runtime_status;
#[cfg(test)]
mod browser_runtime_tests;
mod dev_log;
mod event_builder;
mod fields;
mod local_ai_chat_generation;
mod local_ai_chat_generation_request;
mod local_ai_chat_generation_result;
mod local_ai_chat_generation_runner;
#[cfg(test)]
mod local_ai_chat_generation_tests;
mod local_ai_generation_payload;
mod local_ai_runtime_cache_status;
mod local_ai_runtime_config;
mod local_ai_runtime_config_path;
mod local_ai_runtime_config_values;
mod local_ai_runtime_configured_status;
mod local_ai_runtime_payload;
#[cfg(test)]
mod local_ai_runtime_payload_tests;
mod local_ai_runtime_status;
#[cfg(test)]
mod local_ai_runtime_status_tests;
mod network;
mod network_flow_digest;
mod network_flow_digest_indicators;
mod network_flow_digest_rollups;
#[cfg(test)]
mod network_flow_digest_tests;
#[cfg(test)]
mod network_flow_payload_tests;
mod policy_preview_api;
mod policy_preview_payload;
#[cfg(test)]
mod policy_preview_tests;
mod snapshot;
mod time;
mod websocket;

use ocentra_parent_agent_protocol::constants;

use crate::network::NetworkPolicy;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let network = NetworkPolicy::from_environment();
    let listener = tokio::net::TcpListener::bind(network.bind_address())
        .await
        .expect(constants::error::LOCALHOST_BIND_SUCCEEDS);
    let _ = dev_log::write_agent_info(
        constants::dev_log_message::AGENT_SERVICE_STARTED,
        Default::default(),
    );
    activity_capture::spawn_startup_activity_capture();

    axum::serve(listener, app::router(network))
        .await
        .expect(constants::error::AGENT_SERVICE_RUNS);
}
