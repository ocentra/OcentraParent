use axum::{
    extract::{ws::WebSocketUpgrade, State},
    http::{header::ORIGIN, HeaderMap, StatusCode},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{AgentLogSnapshot, LogFields};

use crate::{
    browser_intervention_page::serve_browser_intervention_page,
    browser_policy_runtime::BrowserPolicyRuntime,
    dev_log::write_agent_info,
    lan_pairing::LanPairingRuntime,
    lan_pairing_runtime_state::{
        mdns_advertisement::spawn_lan_mdns_advertisement_runtime,
        passive_discovery::spawn_lan_passive_discovery_runtime,
    },
    network::NetworkPolicy,
    screen_settings_runtime::ScreenSettingsRuntime,
    snapshot::build_dev_log_snapshot,
    websocket::{handle_socket, WebsocketCommandOrigin},
};

#[derive(Clone)]
pub struct AppState {
    network: NetworkPolicy,
    lan_pairing: LanPairingRuntime,
    browser_policy: BrowserPolicyRuntime,
    screen_settings: ScreenSettingsRuntime,
}

pub fn router(network: NetworkPolicy) -> Router {
    let cors_layer = network.cors_layer();
    let lan_pairing = LanPairingRuntime::from_env();
    spawn_lan_mdns_advertisement_runtime(lan_pairing.clone());
    spawn_lan_passive_discovery_runtime(lan_pairing.clone());
    let state = AppState {
        network,
        lan_pairing,
        browser_policy: BrowserPolicyRuntime::from_env(),
        screen_settings: ScreenSettingsRuntime::from_env(),
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
    let _ = tokio::task::spawn_blocking(|| {
        write_agent_info(
            constants::dev_log_message::AGENT_HEALTH_REQUESTED,
            LogFields::new(),
        )
    })
    .await;
    Json(build_dev_log_snapshot())
}

// Compatibility snapshot endpoint only: this is status/read-model output, not the primary local dev log store.
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
        handle_socket(
            socket,
            state.lan_pairing,
            state.browser_policy,
            state.screen_settings,
            WebsocketCommandOrigin(origin),
        )
    })
}
