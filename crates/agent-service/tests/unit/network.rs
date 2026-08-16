#[path = "../../src/network.rs"]
mod network;

use std::net::{Ipv4Addr, SocketAddr};

use axum::http::{header::ORIGIN, HeaderMap, HeaderValue};
use ocentra_parent_agent_protocol::constants;

#[test]
fn loopback_bind_does_not_require_lan_flag() {
    let policy = network::NetworkPolicy::from_parts(
        SocketAddr::from((Ipv4Addr::LOCALHOST, 4477)),
        false,
        allowed_origins(),
    );

    assert_eq!(policy.bind_address().ip(), Ipv4Addr::LOCALHOST);
}

#[test]
fn lan_bind_without_flag_falls_back_to_loopback() {
    let policy = network::NetworkPolicy::from_parts(
        SocketAddr::from((Ipv4Addr::UNSPECIFIED, 4477)),
        false,
        allowed_origins(),
    );

    assert_eq!(policy.bind_address().ip(), Ipv4Addr::LOCALHOST);
}

#[test]
fn origin_policy_allows_only_configured_browser_origins() {
    let policy = network::NetworkPolicy::from_parts(
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

#[test]
fn environment_policy_builds_cors_layer_and_loopback_bind() {
    let policy = network::NetworkPolicy::from_environment();
    let _cors = policy.cors_layer();

    assert!(policy.bind_address().port() > 0);
    assert!(policy.allows_headers(&HeaderMap::new()));
}

fn allowed_origins() -> Vec<HeaderValue> {
    vec![HeaderValue::from_static(
        constants::bind::DEFAULT_ALLOWED_ORIGINS[0],
    )]
}
