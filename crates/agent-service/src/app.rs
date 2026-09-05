use axum::{
    extract::{ws::WebSocketUpgrade, ConnectInfo, State},
    http::{header::ORIGIN, HeaderMap, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use ocentra_parent_agent_protocol::constants;
use std::{net::SocketAddr, sync::Arc};

use crate::{
    activity_api::app_game_platform_probe_runtime::PlatformProbeRuntimeOwner,
    activity_api::app_game_platform_proof_status_transport::platform_probe_dispatcher,
    browser_intervention_page::serve_browser_intervention_page,
    browser_policy_runtime::BrowserPolicyRuntime,
    browser_runtime::BrowserManagedRuntime,
    lan_pairing::LanPairingRuntime,
    lan_pairing_runtime_state::{
        mdns_advertisement::spawn_lan_mdns_advertisement_runtime,
        passive_discovery::{
            start_lan_passive_discovery_service_runtime, LanPassiveDiscoveryServiceRuntime,
        },
    },
    network::NetworkPolicy,
    parent_local_bridge_admission::ParentLocalBridgeAdmission,
    screen_settings_runtime::ScreenSettingsRuntime,
    websocket::{
        handle_socket, WebsocketCommandOrigin, WebsocketPeerProvenance,
        WebsocketPlatformProbeDispatcher,
    },
};

#[path = "app/health.rs"]
mod health;

#[derive(Clone)]
pub struct AppState {
    network: NetworkPolicy,
    lan_pairing: LanPairingRuntime,
    browser_policy: BrowserPolicyRuntime,
    browser_runtime: BrowserManagedRuntime,
    screen_settings: ScreenSettingsRuntime,
    platform_probe_dispatcher: Arc<WebsocketPlatformProbeDispatcher>,
    parent_local_bridge_admission: ParentLocalBridgeAdmission,
    _platform_probe_runtime: Arc<PlatformProbeRuntimeOwner>,
    _passive_discovery_runtime: Option<LanPassiveDiscoveryServiceRuntime>,
}

pub(crate) fn router(
    network: NetworkPolicy,
    parent_local_bridge_admission: ParentLocalBridgeAdmission,
) -> Router {
    let cors_layer = network.cors_layer();
    let lan_pairing = LanPairingRuntime::from_env();
    let passive_discovery_runtime = if lan_pairing.durable_pairing_registry_available() {
        spawn_lan_mdns_advertisement_runtime(lan_pairing.clone());
        start_lan_passive_discovery_service_runtime(&lan_pairing).ok()
    } else {
        None
    };
    let platform_probe_runtime = PlatformProbeRuntimeOwner::start();
    let state = AppState {
        network,
        lan_pairing,
        browser_policy: BrowserPolicyRuntime::from_env(),
        browser_runtime: BrowserManagedRuntime::new(),
        screen_settings: ScreenSettingsRuntime::from_env(),
        platform_probe_dispatcher: platform_probe_dispatcher(platform_probe_runtime.cache()),
        parent_local_bridge_admission,
        _platform_probe_runtime: platform_probe_runtime,
        _passive_discovery_runtime: passive_discovery_runtime,
    };
    Router::new()
        .route(constants::endpoint::HEALTH, get(health::handle))
        .route(
            constants::endpoint::BROWSER_INTERVENTION_PAGE,
            get(serve_browser_intervention_page),
        )
        .route(constants::endpoint::DEV_WS, get(websocket))
        .with_state(state)
        .layer(cors_layer)
}

async fn websocket(
    State(state): State<AppState>,
    ConnectInfo(peer_addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    if !state.parent_local_bridge_admission.is_ready() {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    }
    if !state.network.allows_headers(&headers) {
        return StatusCode::FORBIDDEN.into_response();
    }
    let origin = headers
        .get(ORIGIN)
        .and_then(|value| value.to_str().ok())
        .map(ToOwned::to_owned);
    let provenance = if peer_addr.ip().is_loopback() {
        WebsocketPeerProvenance::Loopback
    } else {
        WebsocketPeerProvenance::LocalNetwork
    };
    ws.on_upgrade(move |socket| {
        handle_socket(
            socket,
            crate::websocket::WebsocketSocketRuntime {
                command: crate::websocket::WebsocketCommandRuntime {
                    lan_pairing: state.lan_pairing,
                    browser_policy: state.browser_policy,
                    browser_runtime: state.browser_runtime,
                    screen_settings: state.screen_settings,
                    origin: WebsocketCommandOrigin(origin),
                    probe_dispatcher: state.platform_probe_dispatcher,
                    provenance,
                },
                admission: state.parent_local_bridge_admission,
            },
        )
    })
}
