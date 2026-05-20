#![forbid(unsafe_code)]

mod activity_api;
mod activity_capture;
#[cfg(test)]
mod activity_capture_tests;
mod activity_payload;
mod activity_store_path;
mod app;
mod dev_log;
mod event_builder;
mod fields;
mod network;
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
    activity_capture::spawn_startup_process_snapshot_capture();

    axum::serve(listener, app::router(network))
        .await
        .expect(constants::error::AGENT_SERVICE_RUNS);
}
