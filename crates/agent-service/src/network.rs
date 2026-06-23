use std::{env, net::SocketAddr};

use axum::http::{header::ORIGIN, HeaderMap, HeaderValue, Method};
use ocentra_parent_agent_protocol::constants;
use tower_http::cors::{AllowOrigin, CorsLayer};

#[derive(Clone, Debug)]
pub struct NetworkPolicy {
    bind_address: SocketAddr,
    allowed_origins: Vec<HeaderValue>,
}

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
        .map(|value| parse_allowed_origins(&value))
        .unwrap_or_else(|_| {
            parse_allowed_origins(
                &constants::bind::DEFAULT_ALLOWED_ORIGINS
                    .join(&constants::delimiter::LIST.to_string()),
            )
        })
}

fn default_bind_address() -> SocketAddr {
    constants::bind::DEFAULT_AGENT_ADDR
        .parse::<SocketAddr>()
        .unwrap_or_else(|_error| SocketAddr::from(([127, 0, 0, 1], 4477)))
}

fn parse_allowed_origins(input: &str) -> Vec<HeaderValue> {
    let allowed_origins = input
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

#[cfg(test)]
mod tests {
    use std::net::{Ipv4Addr, SocketAddr};

    use axum::http::{header::ORIGIN, HeaderMap, HeaderValue};
    use ocentra_parent_agent_protocol::constants;

    use super::NetworkPolicy;

    #[test]
    fn loopback_bind_does_not_require_lan_flag() {
        let policy = NetworkPolicy::from_parts(
            SocketAddr::from((Ipv4Addr::LOCALHOST, 4477)),
            false,
            allowed_origins(),
        );

        assert_eq!(policy.bind_address().ip(), Ipv4Addr::LOCALHOST);
    }

    #[test]
    fn lan_bind_without_flag_falls_back_to_loopback() {
        let policy = NetworkPolicy::from_parts(
            SocketAddr::from((Ipv4Addr::UNSPECIFIED, 4477)),
            false,
            allowed_origins(),
        );

        assert_eq!(policy.bind_address().ip(), Ipv4Addr::LOCALHOST);
    }

    #[test]
    fn origin_policy_allows_only_configured_browser_origins() {
        let policy = NetworkPolicy::from_parts(
            SocketAddr::from((Ipv4Addr::LOCALHOST, 4477)),
            false,
            allowed_origins(),
        );
        let mut allowed = HeaderMap::new();
        allowed.insert(ORIGIN, allowed_origins()[0].clone());
        let mut rejected = HeaderMap::new();
        rejected.insert(
            ORIGIN,
            HeaderValue::from_static(constants::bind::DEFAULT_ALLOWED_ORIGINS[1]),
        );

        assert!(policy.allows_headers(&allowed));
        assert!(!policy.allows_headers(&rejected));
    }

    fn allowed_origins() -> Vec<HeaderValue> {
        vec![HeaderValue::from_static(
            constants::bind::DEFAULT_ALLOWED_ORIGINS[0],
        )]
    }
}
