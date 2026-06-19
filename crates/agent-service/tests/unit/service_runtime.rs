use std::net::{Ipv4Addr, SocketAddr};

use axum::http::HeaderValue;
use ocentra_parent_agent_protocol::{constants, LogFieldValue};

use crate::network::NetworkPolicy;

use super::startup_log_fields;

#[test]
fn startup_log_fields_include_context_and_bound_port() {
    let network = NetworkPolicy::from_parts(
        SocketAddr::from((Ipv4Addr::LOCALHOST, 4477)),
        false,
        allowed_origins(),
    );

    let fields = startup_log_fields(&network);

    assert_eq!(
        fields.get("context"),
        Some(&LogFieldValue::String("startup".to_string()))
    );
    assert_eq!(
        fields.get(constants::field::LOCAL_PORT),
        Some(&LogFieldValue::Number(4477.0))
    );
}

fn allowed_origins() -> Vec<HeaderValue> {
    vec![HeaderValue::from_static(
        constants::bind::DEFAULT_ALLOWED_ORIGINS[0],
    )]
}
