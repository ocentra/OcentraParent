use std::{env, net::SocketAddr};

use axum::http::{header::ORIGIN, HeaderMap, HeaderValue, Method};
use ocentra_parent_agent_protocol::constants;
use tower_http::cors::{AllowOrigin, CorsLayer};

#[derive(Clone, Debug)]
pub struct NetworkPolicy {
    bind_address: SocketAddr,
    allowed_origins: Vec<HeaderValue>,
}

#[derive(Clone, Copy)]
struct AllowedOriginsTextRef<'a>(&'a str);

impl NetworkPolicy {
    pub fn from_environment() -> Self {
        let bind_address = read_bind_address();
        let local_network_enabled = read_local_network_enabled();
        let allowed_origins = read_allowed_origins();
        Self::from_parts(bind_address, local_network_enabled, allowed_origins)
    }

    pub fn from_parts(
        bind_address: SocketAddr,
        local_network_enabled: bool,
        allowed_origins: Vec<HeaderValue>,
    ) -> Self {
        let bind_address = if !local_network_enabled && !bind_address.ip().is_loopback() {
            SocketAddr::from(([127, 0, 0, 1], bind_address.port()))
        } else {
            bind_address
        };

        Self {
            bind_address,
            allowed_origins,
        }
    }

    pub fn bind_address(&self) -> SocketAddr {
        self.bind_address
    }

    pub fn cors_layer(&self) -> CorsLayer {
        CorsLayer::new()
            .allow_methods([Method::GET])
            .allow_origin(AllowOrigin::list(self.allowed_origins.clone()))
    }

    pub fn allows_headers(&self, headers: &HeaderMap) -> bool {
        match headers.get(ORIGIN) {
            Some(origin) => self.allowed_origins.iter().any(|allowed| allowed == origin),
            None => true,
        }
    }
}

fn read_bind_address() -> SocketAddr {
    env::var(constants::env_var::AGENT_ADDR)
        .ok()
        .and_then(|value| value.parse::<SocketAddr>().ok())
        .unwrap_or_else(default_bind_address)
}

fn read_local_network_enabled() -> bool {
    env::var(constants::env_var::AGENT_LOCAL_NETWORK_ENABLED)
        .map(|value| value == constants::value::TRUE)
        .unwrap_or(false)
}

fn read_allowed_origins() -> Vec<HeaderValue> {
    env::var(constants::env_var::AGENT_ALLOWED_ORIGINS)
        .map(|value| parse_allowed_origins(AllowedOriginsTextRef(&value)))
        .unwrap_or_else(|_| {
            let default_allowed_origins = constants::bind::DEFAULT_ALLOWED_ORIGINS
                .join(&constants::delimiter::LIST.to_string());
            parse_allowed_origins(AllowedOriginsTextRef(&default_allowed_origins))
        })
}

fn default_bind_address() -> SocketAddr {
    constants::bind::DEFAULT_AGENT_ADDR
        .parse::<SocketAddr>()
        .unwrap_or_else(|_error| SocketAddr::from(([127, 0, 0, 1], 4477)))
}

fn parse_allowed_origins(input: AllowedOriginsTextRef<'_>) -> Vec<HeaderValue> {
    let allowed_origins = input
        .0
        .split(constants::delimiter::LIST)
        .map(str::trim)
        .filter(|origin| !origin.is_empty())
        .filter_map(|origin| HeaderValue::from_str(origin).ok())
        .collect::<Vec<_>>();

    if allowed_origins.is_empty() {
        return default_allowed_origins();
    }

    allowed_origins
}

fn default_allowed_origins() -> Vec<HeaderValue> {
    constants::bind::DEFAULT_ALLOWED_ORIGINS
        .iter()
        .map(|origin| HeaderValue::from_static(origin))
        .collect()
}
