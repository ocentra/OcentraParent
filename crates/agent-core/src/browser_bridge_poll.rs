use std::net::SocketAddr;

use ocentra_parent_agent_protocol::{
    constants, ActivityEvent, BrowserActiveTabState, BrowserCapabilityStatus, BrowserChannel,
    BrowserCustodyLabel, BrowserFamily,
};
use serde_json::Value;

use crate::{
    browser_bridge_http::read_devtools_body, browser_tab_observation_event,
    BrowserBridgeTargetObservation,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BrowserBridgePollConfig {
    pub endpoint: SocketAddr,
    pub managed_browser_session_id: String,
    pub profile_id: String,
    pub process_id: u32,
    pub browser_family: BrowserFamily,
    pub browser_channel: BrowserChannel,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BrowserBridgePollSnapshot {
    pub browser_version: Option<String>,
    pub events: Vec<ActivityEvent>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BrowserBridgePollError {
    NonLoopbackEndpoint,
    Io,
    InvalidHttpResponse,
    InvalidJson,
}

impl BrowserBridgePollError {
    pub fn reason(&self) -> &'static str {
        match self {
            Self::NonLoopbackEndpoint => constants::value::BROWSER_BRIDGE_NON_LOOPBACK_ENDPOINT,
            Self::Io => constants::value::BROWSER_BRIDGE_IO_ERROR,
            Self::InvalidHttpResponse => constants::value::BROWSER_BRIDGE_INVALID_RESPONSE,
            Self::InvalidJson => constants::value::BROWSER_BRIDGE_INVALID_JSON,
        }
    }
}

pub fn poll_chromium_bridge(
    config: BrowserBridgePollConfig,
    observed_at: &str,
    fresh_until: &str,
) -> Result<BrowserBridgePollSnapshot, BrowserBridgePollError> {
    if !config.endpoint.ip().is_loopback() {
        return Err(BrowserBridgePollError::NonLoopbackEndpoint);
    }

    let version_body =
        read_devtools_body(&config.endpoint, constants::browser::HTTP_GET_JSON_VERSION)?;
    let list_body = read_devtools_body(&config.endpoint, constants::browser::HTTP_GET_JSON_LIST)?;
    let browser_version = parse_browser_version(&version_body)?;
    let events = parse_target_events(&config, &list_body, observed_at, fresh_until)?;

    Ok(BrowserBridgePollSnapshot {
        browser_version,
        events,
    })
}

fn parse_browser_version(body: &str) -> Result<Option<String>, BrowserBridgePollError> {
    let value: Value =
        serde_json::from_str(body).map_err(|_| BrowserBridgePollError::InvalidJson)?;
    Ok(value
        .get(constants::browser::DEVTOOLS_FIELD_BROWSER)
        .and_then(Value::as_str)
        .map(ToString::to_string))
}

fn parse_target_events(
    config: &BrowserBridgePollConfig,
    body: &str,
    observed_at: &str,
    fresh_until: &str,
) -> Result<Vec<ActivityEvent>, BrowserBridgePollError> {
    let value: Value =
        serde_json::from_str(body).map_err(|_| BrowserBridgePollError::InvalidJson)?;
    let Some(targets) = value.as_array() else {
        return Err(BrowserBridgePollError::InvalidJson);
    };

    Ok(targets
        .iter()
        .enumerate()
        .filter_map(|(index, target)| target_event(config, target, observed_at, fresh_until, index))
        .collect())
}

fn target_event(
    config: &BrowserBridgePollConfig,
    target: &Value,
    observed_at: &str,
    fresh_until: &str,
    index: usize,
) -> Option<ActivityEvent> {
    if string_field(target, constants::browser::DEVTOOLS_FIELD_TYPE)?
        != constants::browser::DEVTOOLS_TARGET_TYPE_PAGE
    {
        return None;
    }
    let observation = BrowserBridgeTargetObservation {
        browser_family: config.browser_family.clone(),
        browser_channel: config.browser_channel.clone(),
        managed_browser_session_id: config.managed_browser_session_id.clone(),
        profile_id: config.profile_id.clone(),
        process_id: config.process_id,
        target_id: string_field(target, constants::browser::DEVTOOLS_FIELD_ID)?.to_string(),
        tab_id: None,
        window_id: None,
        active_state: BrowserActiveTabState::Unknown,
        url: string_field(target, constants::browser::DEVTOOLS_FIELD_URL)?.to_string(),
        title: string_field(target, constants::browser::DEVTOOLS_FIELD_TITLE)
            .map(ToString::to_string),
        capability_status: BrowserCapabilityStatus::TabListOnly,
        custody_label: BrowserCustodyLabel::ChildDeviceLocal,
    };
    browser_tab_observation_event(observation, observed_at, fresh_until, index).ok()
}

fn string_field<'a>(target: &'a Value, key: &str) -> Option<&'a str> {
    target.get(key).and_then(Value::as_str)
}
