use axum::{
    extract::{ws::WebSocketUpgrade, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use ocentra_parent_agent_protocol::{constants, AgentLogSnapshot};

use crate::{network::NetworkPolicy, snapshot::build_dev_log_snapshot, websocket::handle_socket};

pub fn router(network: NetworkPolicy) -> Router {
    let cors_layer = network.cors_layer();
    Router::new()
        .route(constants::endpoint::HEALTH, get(health))
        .route(constants::endpoint::DEV_LOG_SNAPSHOT, get(log_snapshot))
        .route(constants::endpoint::DEV_WS, get(websocket))
        .with_state(network)
        .layer(cors_layer)
}

async fn health() -> Json<AgentLogSnapshot> {
    Json(build_dev_log_snapshot())
}

async fn log_snapshot() -> Json<AgentLogSnapshot> {
    Json(build_dev_log_snapshot())
}

async fn websocket(
    State(network): State<NetworkPolicy>,
    headers: HeaderMap,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    if !network.allows_headers(&headers) {
        return StatusCode::FORBIDDEN.into_response();
    }
    ws.on_upgrade(handle_socket)
}
