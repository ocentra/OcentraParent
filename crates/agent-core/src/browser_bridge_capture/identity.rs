use std::net::SocketAddr;

use ocentra_parent_agent_protocol::constants;
use serde_json::Value;

use super::{authority::LaunchBinding, ManagedBrowserCdpCaptureError};
use crate::browser_bridge_http::read_devtools_body;
use sha2::{Digest, Sha256};

pub(super) fn verify_browser_identity(
    binding: &LaunchBinding,
) -> Result<String, ManagedBrowserCdpCaptureError> {
    let body = read_devtools_body(&binding.endpoint, constants::browser::HTTP_GET_JSON_VERSION)?;
    let value: Value = serde_json::from_str(&body)
        .map_err(|_error| ManagedBrowserCdpCaptureError::InvalidResponse)?;
    let browser = value
        .get(constants::browser::DEVTOOLS_FIELD_BROWSER)
        .and_then(Value::as_str)
        .filter(|value| !value.trim().is_empty())
        .ok_or(crate::browser_bridge_poll::BrowserBridgePollError::UntrustedBrowserIdentity)?;
    if !super::identity_match::browser_identity_matches(
        browser,
        binding.browser_family,
        binding.browser_channel,
    ) {
        return Err(
            crate::browser_bridge_poll::BrowserBridgePollError::UntrustedBrowserIdentity.into(),
        );
    }
    let debugger_url = value
        .get(constants::browser::DEVTOOLS_FIELD_WEB_SOCKET_DEBUGGER_URL)
        .and_then(Value::as_str)
        .ok_or(ManagedBrowserCdpCaptureError::InvalidWebSocketEndpoint)?;
    validate_websocket_endpoint(debugger_url, binding.endpoint)?;
    Ok(text_digest(browser))
}

pub(super) fn validate_websocket_endpoint(
    url: &str,
    endpoint: SocketAddr,
) -> Result<(), ManagedBrowserCdpCaptureError> {
    let remainder = url
        .strip_prefix("ws://")
        .ok_or(ManagedBrowserCdpCaptureError::InvalidWebSocketEndpoint)?;
    let (authority, path) = remainder
        .split_once('/')
        .ok_or(ManagedBrowserCdpCaptureError::InvalidWebSocketEndpoint)?;
    if path.is_empty() {
        return Err(ManagedBrowserCdpCaptureError::InvalidWebSocketEndpoint);
    }
    let websocket_endpoint: SocketAddr = authority
        .parse()
        .map_err(|_error| ManagedBrowserCdpCaptureError::InvalidWebSocketEndpoint)?;
    if websocket_endpoint != endpoint || !websocket_endpoint.ip().is_loopback() {
        return Err(ManagedBrowserCdpCaptureError::InvalidWebSocketEndpoint);
    }
    Ok(())
}

fn text_digest(value: &str) -> String {
    let mut digest = String::new();
    for byte in Sha256::digest(value.as_bytes()) {
        digest.push_str(&format!("{byte:02x}"));
    }
    digest
}
