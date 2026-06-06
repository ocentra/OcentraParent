use axum::{
    extract::{ws::WebSocketUpgrade, State},
    http::{header::ORIGIN, HeaderMap, StatusCode},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use ocentra_parent_agent_protocol::{constants, AgentLogSnapshot, LogFields};

use crate::{
    browser_intervention_page::serve_browser_intervention_page,
    browser_policy_runtime::BrowserPolicyRuntime, dev_log::write_agent_info,
    lan_pairing::LanPairingRuntime, network::NetworkPolicy, snapshot::build_dev_log_snapshot,
    websocket::handle_socket,
};

#[derive(Clone)]
pub struct AppState {
    network: NetworkPolicy,
    lan_pairing: LanPairingRuntime,
    browser_policy: BrowserPolicyRuntime,
}

pub fn router(network: NetworkPolicy) -> Router {
    let cors_layer = network.cors_layer();
    let state = AppState {
        network,
        lan_pairing: LanPairingRuntime::from_env(),
        browser_policy: BrowserPolicyRuntime::from_env(),
    };
    Router::new()
        .route(constants::endpoint::HEALTH, get(health))
        .route(
            constants::endpoint::BROWSER_INTERVENTION_PAGE,
            get(serve_browser_intervention_page),
        )
        .route(constants::endpoint::DEV_LOG_SNAPSHOT, get(log_snapshot))
        .route(constants::endpoint::DEV_WS, get(websocket))
        .with_state(state)
        .layer(cors_layer)
}

async fn health() -> Json<AgentLogSnapshot> {
    let _ = write_agent_info(
        constants::dev_log_message::AGENT_HEALTH_REQUESTED,
        LogFields::new(),
    );
    Json(build_dev_log_snapshot())
}

async fn log_snapshot() -> Json<AgentLogSnapshot> {
    Json(build_dev_log_snapshot())
}

async fn websocket(
    State(state): State<AppState>,
    headers: HeaderMap,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    if !state.network.allows_headers(&headers) {
        return StatusCode::FORBIDDEN.into_response();
    }
    let origin = headers
        .get(ORIGIN)
        .and_then(|value| value.to_str().ok())
        .map(ToOwned::to_owned);
    ws.on_upgrade(move |socket| {
        handle_socket(socket, state.lan_pairing, state.browser_policy, origin)
    })
}
