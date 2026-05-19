#![forbid(unsafe_code)]

mod app;
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

    axum::serve(listener, app::router(network))
        .await
        .expect(constants::error::AGENT_SERVICE_RUNS);
}
