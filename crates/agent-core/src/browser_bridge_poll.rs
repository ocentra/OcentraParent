use std::net::SocketAddr;

use ocentra_parent_agent_protocol::activity::ActivityEvent;
use ocentra_parent_agent_protocol::browser::{
    BrowserCapabilityStatus, BrowserChannel, BrowserFamily,
};
use ocentra_parent_agent_protocol::constants;

use crate::browser_bridge_http::read_devtools_body;

#[path = "browser_bridge_poll/custody.rs"]
mod custody;
#[path = "browser_bridge_poll/error.rs"]
mod error;
#[path = "browser_bridge_poll/parse.rs"]
mod parse;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BrowserBridgePollConfig {
    pub endpoint: SocketAddr,
    pub managed_browser_session_id: String,
    pub profile_id: String,
    pub process_id: u32,
    pub browser_family: BrowserFamily,
    pub browser_channel: BrowserChannel,
    pub expected_custody: BrowserBridgeExpectedCustody,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BrowserBridgeExpectedCustody {
    pub bridge_port: u16,
    pub managed_browser_session_id: String,
    pub profile_id: String,
    pub process_id: u32,
    pub browser_family: BrowserFamily,
    pub browser_channel: BrowserChannel,
    pub session_fresh_until: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BrowserBridgePollSnapshot {
    pub browser_version: Option<String>,
    pub page_target_count: usize,
    pub events: Vec<ActivityEvent>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BrowserBridgePollError {
    NonLoopbackEndpoint,
    StaleSession,
    Timeout,
    UntrustedBridgePort,
    UntrustedBrowserIdentity,
    UntrustedProcess,
    UntrustedProfile,
    UntrustedSession,
    Io,
    InvalidHttpResponse,
    InvalidJson,
    InvalidTargetPayload,
    ResponseTooLarge,
}

impl BrowserBridgePollError {
    pub fn reason(&self) -> &'static str {
        error::browser_bridge_poll_error_reason(self)
    }

    pub fn capability_status(&self) -> BrowserCapabilityStatus {
        error::browser_bridge_poll_error_capability_status(self)
    }
}

pub(crate) fn validate_bridge_custody(
    config: &BrowserBridgePollConfig,
    observed_at: &str,
) -> Result<(), BrowserBridgePollError> {
    custody::validate_bridge_custody(config, observed_at)
}

pub fn poll_chromium_bridge(
    config: &BrowserBridgePollConfig,
    observed_at: &str,
    fresh_until: &str,
) -> Result<BrowserBridgePollSnapshot, BrowserBridgePollError> {
    if !config.endpoint.ip().is_loopback() {
        return Err(BrowserBridgePollError::NonLoopbackEndpoint);
    }
    custody::validate_bridge_custody(config, observed_at)?;

    let version_body =
        read_devtools_body(&config.endpoint, constants::browser::HTTP_GET_JSON_VERSION)?;
    let list_body = read_devtools_body(&config.endpoint, constants::browser::HTTP_GET_JSON_LIST)?;
    let browser_version = parse::parse_browser_version(&version_body)?;
    let events = parse::parse_target_events(config, &list_body, observed_at, fresh_until)?;

    Ok(BrowserBridgePollSnapshot {
        browser_version,
        page_target_count: events.len(),
        events,
    })
}
