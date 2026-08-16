use std::net::SocketAddr;

use crate::browser_bridge_poll::BrowserBridgePollError;

#[path = "browser_bridge_http/helpers.rs"]
mod helpers;
#[path = "browser_bridge_http/transport.rs"]
mod transport;

pub fn read_devtools_body(
    endpoint: &SocketAddr,
    request_line: &str,
) -> Result<String, BrowserBridgePollError> {
    transport::read_devtools_body(endpoint, request_line)
}
