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
        if !local_network_enabled && !bind_address.ip().is_loopback() {
            std::panic::panic_any(constants::error::LAN_BIND_REQUIRES_FLAG);
        }

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
        .unwrap_or_else(|_| constants::bind::DEFAULT_AGENT_ADDR.to_string())
        .parse::<SocketAddr>()
        .expect(constants::error::AGENT_ADDR_SOCKET_ADDRESS)
}

fn read_local_network_enabled() -> bool {
    env::var(constants::env_var::AGENT_LOCAL_NETWORK_ENABLED)
        .map(|value| value == constants::value::TRUE)
        .unwrap_or(false)
}

fn read_allowed_origins() -> Vec<HeaderValue> {
    env::var(constants::env_var::AGENT_ALLOWED_ORIGINS)
        .map(parse_allowed_origins)
        .unwrap_or_else(|_| {
            parse_allowed_origins(
                constants::bind::DEFAULT_ALLOWED_ORIGINS
                    .join(&constants::delimiter::LIST.to_string()),
            )
        })
}

fn parse_allowed_origins(input: String) -> Vec<HeaderValue> {
    input
        .split(constants::delimiter::LIST)
        .map(str::trim)
        .filter(|origin| !origin.is_empty())
        .map(|origin| {
            HeaderValue::from_str(origin).expect(constants::error::AGENT_ORIGIN_HEADER_VALID)
        })
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
    fn lan_bind_requires_explicit_flag() {
        let result = std::panic::catch_unwind(|| {
            NetworkPolicy::from_parts(
                SocketAddr::from((Ipv4Addr::UNSPECIFIED, 4477)),
                false,
                allowed_origins(),
            );
        });

        assert!(result.is_err());
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
