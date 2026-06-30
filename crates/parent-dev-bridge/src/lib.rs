use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use axum::{
    extract::Json,
    http::{Method, StatusCode},
    routing::post,
    Router,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_logging_core::{
    dev_log::DevLogger,
    field::{LogFieldValue, LogFields},
    source::LogSource,
};
use ocentra_parent_runtime_core::parent_ui_bridge::{
    dispatch_parent_ui_action, load_parent_route_snapshot,
};
use ocentra_schema::parent_ui_bridge::{
    ParentRouteContext, ParentRouteId, ParentRouteSnapshot, ParentUiAction, ParentUiActionResult,
    PARENT_DEV_BRIDGE_DISPATCH_PATH, PARENT_DEV_BRIDGE_LOAD_ROUTE_PATH,
};
use serde::Deserialize;
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ParentDevBridgeLoadRouteRequest {
    route: ParentRouteId,
    context: Option<ParentRouteContext>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ParentDevBridgeDispatchRequest {
    action: ParentUiAction,
}

pub fn configured_parent_dev_bridge_address() -> Option<SocketAddr> {
    configured_parent_dev_bridge_address_for(
        std::env::var(constants::env_var::PARENT_DEV_BRIDGE_PORT)
            .ok()
            .as_deref(),
        std::env::var(constants::env_var::DEV_NETWORK_MODE)
            .ok()
            .as_deref(),
    )
}

pub async fn serve_parent_dev_bridge(address: SocketAddr) -> Result<(), String> {
    let listener = tokio::net::TcpListener::bind(address)
        .await
        .map_err(|error| {
            let reason = error.to_string();
            log_parent_dev_bridge_error(
                constants::error::PARENT_DEV_BRIDGE_BINDS,
                Some(address),
                reason.clone(),
            );
            reason
        })?;

    axum::serve(listener, parent_dev_bridge_router())
        .await
        .map_err(|error| {
            let reason = error.to_string();
            log_parent_dev_bridge_error(
                constants::error::PARENT_DEV_BRIDGE_RUNS,
                Some(address),
                reason.clone(),
            );
            reason
        })
}

async fn parent_dev_bridge_load_route(
    Json(request): Json<ParentDevBridgeLoadRouteRequest>,
) -> Result<Json<ParentRouteSnapshot>, StatusCode> {
    let route = request.route;
    let context = request.context;
    tokio::task::spawn_blocking(move || load_parent_route_snapshot(route, context.as_ref()))
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn parent_dev_bridge_dispatch(
    Json(request): Json<ParentDevBridgeDispatchRequest>,
) -> Result<Json<ParentUiActionResult>, StatusCode> {
    let action = request.action;
    tokio::task::spawn_blocking(move || dispatch_parent_ui_action(&action))
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

fn parent_dev_bridge_router() -> Router {
    Router::new()
        .route(
            PARENT_DEV_BRIDGE_LOAD_ROUTE_PATH,
            post(parent_dev_bridge_load_route),
        )
        .route(
            PARENT_DEV_BRIDGE_DISPATCH_PATH,
            post(parent_dev_bridge_dispatch),
        )
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([Method::POST, Method::OPTIONS])
                .allow_headers(Any),
        )
}

pub fn log_parent_dev_bridge_error(message: &str, address: Option<SocketAddr>, reason: String) {
    let _ = DevLogger::from_env(LogSource::LocalApi)
        .and_then(|logger| logger.error(message, parent_dev_bridge_log_fields(address, reason)));
}

pub fn parent_dev_bridge_log_fields(address: Option<SocketAddr>, reason: String) -> LogFields {
    let mut fields = LogFields::new();
    if let Some(address) = address {
        fields.insert(
            constants::field::LOCAL_PORT.to_string(),
            LogFieldValue::Number(f64::from(address.port())),
        );
        fields.insert(
            constants::field::BRIDGE_ENDPOINT_REF.to_string(),
            LogFieldValue::String(address.to_string()),
        );
    }
    fields.insert(
        constants::field::REASON.to_string(),
        LogFieldValue::String(reason),
    );
    fields
}

pub fn configured_parent_dev_bridge_address_for(
    port: Option<&str>,
    dev_network_mode: Option<&str>,
) -> Option<SocketAddr> {
    let port = port?.parse::<u16>().ok()?;
    let host = if dev_network_mode == Some(constants::value::LOCAL_NETWORK_MODE) {
        IpAddr::V4(Ipv4Addr::UNSPECIFIED)
    } else {
        IpAddr::V4(Ipv4Addr::LOCALHOST)
    };
    Some(SocketAddr::new(host, port))
}
