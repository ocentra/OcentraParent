use std::{
    env,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};

use axum::{
    extract::Json,
    http::{HeaderValue, Method, StatusCode, Uri},
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
    dispatch_parent_ui_action_with_service_health, load_parent_route_snapshot_with_service_health,
    parent_agent_service_health_for_address,
};
use ocentra_schema::parent_ui_bridge::{
    ParentRouteContext, ParentRouteId, ParentRouteSnapshot, ParentUiAction, ParentUiActionResult,
    PARENT_DEV_BRIDGE_DISPATCH_PATH, PARENT_DEV_BRIDGE_LOAD_ROUTE_PATH,
};
use serde::Deserialize;
use tower_http::cors::{AllowOrigin, Any, CorsLayer};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParentDevBridgeFailure(String);

impl ParentDevBridgeFailure {
    pub fn from_display(value: impl std::fmt::Display) -> Self {
        Self(value.to_string())
    }
}

impl std::fmt::Display for ParentDevBridgeFailure {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParentDevBridgeErrorMessage {
    Bind,
    Run,
}

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

struct ParentDevBridgeAgentAddress(String);

#[derive(Clone, Copy)]
struct AllowedOriginsTextRef<'a>(&'a str);

#[derive(Clone, Copy)]
struct AllowedOriginTextRef<'a>(&'a str);

const HTTP_SCHEME: &str = "http";
const HTTPS_SCHEME: &str = "https";
const NULL_ORIGIN: &str = "null";
const NULL_AUTHORITY_PREFIX: &str = "null:";
const WILDCARD_ORIGIN: &str = "*";
const URI_ROOT_PATH: &str = "/";

pub fn configured_parent_dev_bridge_address() -> Option<SocketAddr> {
    let port = std::env::var(constants::env_var::PARENT_DEV_BRIDGE_PORT)
        .ok()
        .and_then(|value| value.parse::<u16>().ok());
    let local_network = std::env::var(constants::env_var::DEV_NETWORK_MODE)
        .ok()
        .as_deref()
        == Some(constants::value::LOCAL_NETWORK_MODE);
    parent_dev_bridge_address_from_configuration(port, local_network)
}

pub fn parent_dev_bridge_address_from_configuration(
    port: Option<u16>,
    local_network: bool,
) -> Option<SocketAddr> {
    let port = port?;
    let host = if local_network {
        IpAddr::V4(Ipv4Addr::UNSPECIFIED)
    } else {
        IpAddr::V4(Ipv4Addr::LOCALHOST)
    };
    Some(SocketAddr::new(host, port))
}

pub async fn serve_parent_dev_bridge(address: SocketAddr) -> Result<(), ParentDevBridgeFailure> {
    let listener = tokio::net::TcpListener::bind(address)
        .await
        .map_err(|error| {
            let reason = ParentDevBridgeFailure::from_display(error);
            log_parent_dev_bridge_error(ParentDevBridgeErrorMessage::Bind, Some(address), &reason);
            reason
        })?;

    axum::serve(listener, parent_dev_bridge_router())
        .await
        .map_err(|error| {
            let reason = ParentDevBridgeFailure::from_display(error);
            log_parent_dev_bridge_error(ParentDevBridgeErrorMessage::Run, Some(address), &reason);
            reason
        })
}

async fn parent_dev_bridge_load_route(
    Json(request): Json<ParentDevBridgeLoadRouteRequest>,
) -> Result<Json<ParentRouteSnapshot>, StatusCode> {
    let route = request.route;
    let context = request.context;
    let agent_address = configured_agent_address();
    tokio::task::spawn_blocking(move || {
        let service_health = parent_agent_service_health_for_address(&agent_address.0);
        load_parent_route_snapshot_with_service_health(route, context.as_ref(), &service_health)
    })
    .await
    .map(Json)
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn parent_dev_bridge_dispatch(
    Json(request): Json<ParentDevBridgeDispatchRequest>,
) -> Result<Json<ParentUiActionResult>, StatusCode> {
    let action = request.action;
    let agent_address = configured_agent_address();
    tokio::task::spawn_blocking(move || {
        let service_health = parent_agent_service_health_for_address(&agent_address.0);
        dispatch_parent_ui_action_with_service_health(&action, &service_health)
    })
    .await
    .map(Json)
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

fn configured_agent_address() -> ParentDevBridgeAgentAddress {
    ParentDevBridgeAgentAddress(
        std::env::var(constants::env_var::AGENT_ADDR)
            .ok()
            .filter(|value| !value.is_empty())
            .unwrap_or_else(|| constants::bind::DEFAULT_AGENT_ADDR.to_string()),
    )
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
        .layer(parent_dev_bridge_cors_layer())
}

fn parent_dev_bridge_cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(AllowOrigin::list(read_allowed_origins()))
        .allow_methods([Method::POST, Method::OPTIONS])
        .allow_headers(Any)
}

fn read_allowed_origins() -> Vec<HeaderValue> {
    env::var(constants::env_var::AGENT_ALLOWED_ORIGINS)
        .map(|value| parse_allowed_origins(AllowedOriginsTextRef(&value)))
        .unwrap_or_else(|error| {
            matches!(error, env::VarError::NotPresent)
                .then(default_allowed_origins)
                .unwrap_or_default()
        })
}

fn parse_allowed_origins(input: AllowedOriginsTextRef<'_>) -> Vec<HeaderValue> {
    let origins = input
        .0
        .split(constants::delimiter::LIST)
        .map(str::trim)
        .map(AllowedOriginTextRef)
        .collect::<Vec<_>>();

    let origins_are_valid = !origins.is_empty()
        && origins
            .iter()
            .all(|origin| is_valid_allowed_origin(*origin));
    origins_are_valid
        .then(|| {
            origins
                .into_iter()
                .map(|origin| HeaderValue::from_str(origin.0))
                .collect::<Result<Vec<_>, _>>()
                .unwrap_or_default()
        })
        .unwrap_or_default()
}

fn is_valid_allowed_origin(origin: AllowedOriginTextRef<'_>) -> bool {
    let Ok(uri) = origin.0.parse::<Uri>() else {
        return false;
    };
    let Some(scheme) = uri.scheme_str() else {
        return false;
    };
    if !scheme.eq_ignore_ascii_case(HTTP_SCHEME) && !scheme.eq_ignore_ascii_case(HTTPS_SCHEME) {
        return false;
    }

    let Some(authority) = uri.authority() else {
        return false;
    };
    let authority_text = authority.as_str();
    let Some(host) = uri.host() else {
        return false;
    };
    if host.is_empty()
        || host.eq_ignore_ascii_case(NULL_ORIGIN)
        || authority_text.contains('@')
        || authority_text.contains(WILDCARD_ORIGIN)
        || authority_text.contains('%')
        || authority_text.eq_ignore_ascii_case(NULL_ORIGIN)
        || authority_text
            .to_ascii_lowercase()
            .starts_with(NULL_AUTHORITY_PREFIX)
    {
        return false;
    }

    let has_explicit_port = if authority_text.starts_with('[') {
        authority_text
            .find(']')
            .map(|index| authority_text[index + 1..].starts_with(':'))
            .unwrap_or(false)
    } else {
        authority_text.rsplit_once(':').is_some()
    };
    if has_explicit_port && authority.port_u16().is_none() {
        return false;
    }

    uri.path() == URI_ROOT_PATH
        && !origin.0.ends_with(URI_ROOT_PATH)
        && !origin.0.contains('?')
        && !origin.0.contains('#')
}

fn default_allowed_origins() -> Vec<HeaderValue> {
    constants::bind::DEFAULT_ALLOWED_ORIGINS
        .iter()
        .map(|origin| HeaderValue::from_static(origin))
        .collect()
}

pub fn log_parent_dev_bridge_error(
    message: ParentDevBridgeErrorMessage,
    address: Option<SocketAddr>,
    reason: &ParentDevBridgeFailure,
) {
    let message = match message {
        ParentDevBridgeErrorMessage::Bind => constants::error::PARENT_DEV_BRIDGE_BINDS,
        ParentDevBridgeErrorMessage::Run => constants::error::PARENT_DEV_BRIDGE_RUNS,
    };

    let _ = DevLogger::from_env(LogSource::LocalApi)
        .and_then(|logger| logger.error(message, parent_dev_bridge_log_fields(address, reason)));
}

pub fn parent_dev_bridge_log_fields(
    address: Option<SocketAddr>,
    reason: &ParentDevBridgeFailure,
) -> LogFields {
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
        LogFieldValue::String(reason.to_string()),
    );
    fields
}
