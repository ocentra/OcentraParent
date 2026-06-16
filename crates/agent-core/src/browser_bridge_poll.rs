use std::net::SocketAddr;

use ocentra_parent_agent_protocol::{
    constants, ActivityEvent, BrowserActiveProofSource, BrowserActiveTabState,
    BrowserCapabilityStatus, BrowserChannel, BrowserCustodyLabel, BrowserFamily,
    BrowserQueryVisibilityLabel,
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
        match self {
            Self::NonLoopbackEndpoint => constants::value::BROWSER_BRIDGE_NON_LOOPBACK_ENDPOINT,
            Self::StaleSession => constants::value::BROWSER_BRIDGE_STALE_SESSION,
            Self::Timeout => constants::value::BROWSER_BRIDGE_TIMEOUT,
            Self::UntrustedBridgePort => constants::value::BROWSER_BRIDGE_UNTRUSTED_PORT,
            Self::UntrustedBrowserIdentity => {
                constants::value::BROWSER_BRIDGE_UNTRUSTED_BROWSER_IDENTITY
            }
            Self::UntrustedProcess => constants::value::BROWSER_BRIDGE_UNTRUSTED_PROCESS,
            Self::UntrustedProfile => constants::value::BROWSER_BRIDGE_UNTRUSTED_PROFILE,
            Self::UntrustedSession => constants::value::BROWSER_BRIDGE_UNTRUSTED_SESSION,
            Self::Io => constants::value::BROWSER_BRIDGE_IO_ERROR,
            Self::InvalidHttpResponse => constants::value::BROWSER_BRIDGE_INVALID_RESPONSE,
            Self::InvalidJson => constants::value::BROWSER_BRIDGE_INVALID_JSON,
            Self::InvalidTargetPayload => constants::value::BROWSER_BRIDGE_INVALID_TARGET_PAYLOAD,
            Self::ResponseTooLarge => constants::value::BROWSER_BRIDGE_RESPONSE_TOO_LARGE,
        }
    }

    pub fn capability_status(&self) -> BrowserCapabilityStatus {
        match self {
            Self::InvalidHttpResponse
            | Self::InvalidJson
            | Self::InvalidTargetPayload
            | Self::ResponseTooLarge => BrowserCapabilityStatus::AdapterError,
            _ => BrowserCapabilityStatus::BridgeMissing,
        }
    }
}

pub fn poll_chromium_bridge(
    config: &BrowserBridgePollConfig,
    observed_at: &str,
    fresh_until: &str,
) -> Result<BrowserBridgePollSnapshot, BrowserBridgePollError> {
    if !config.endpoint.ip().is_loopback() {
        return Err(BrowserBridgePollError::NonLoopbackEndpoint);
    }
    validate_bridge_custody(config, observed_at)?;

    let version_body =
        read_devtools_body(&config.endpoint, constants::browser::HTTP_GET_JSON_VERSION)?;
    let list_body = read_devtools_body(&config.endpoint, constants::browser::HTTP_GET_JSON_LIST)?;
    let browser_version = parse_browser_version(&version_body)?;
    let events = parse_target_events(config, &list_body, observed_at, fresh_until)?;

    Ok(BrowserBridgePollSnapshot {
        browser_version,
        page_target_count: events.len(),
        events,
    })
}

fn parse_browser_version(body: &str) -> Result<Option<String>, BrowserBridgePollError> {
    let value: Value =
        serde_json::from_str(body).map_err(|_error| BrowserBridgePollError::InvalidJson)?;
    if !value.is_object() {
        return Err(BrowserBridgePollError::InvalidTargetPayload);
    }
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
        serde_json::from_str(body).map_err(|_error| BrowserBridgePollError::InvalidJson)?;
    let Some(targets) = value.as_array() else {
        return Err(BrowserBridgePollError::InvalidTargetPayload);
    };

    let mut events = Vec::new();
    for (index, target) in targets.iter().enumerate() {
        if let Some(event) = target_event(config, target, observed_at, fresh_until, index)? {
            events.push(event);
        }
    }
    Ok(events)
}

fn target_event(
    config: &BrowserBridgePollConfig,
    target: &Value,
    observed_at: &str,
    fresh_until: &str,
    index: usize,
) -> Result<Option<ActivityEvent>, BrowserBridgePollError> {
    let target_type = string_field(target, constants::browser::DEVTOOLS_FIELD_TYPE)
        .ok_or(BrowserBridgePollError::InvalidTargetPayload)?;
    if target_type != constants::browser::DEVTOOLS_TARGET_TYPE_PAGE {
        return Ok(None);
    }
    let url = string_field(target, constants::browser::DEVTOOLS_FIELD_URL)
        .ok_or(BrowserBridgePollError::InvalidTargetPayload)?;
    if !target_url_is_observable(url) {
        return Ok(None);
    }
    let observation = BrowserBridgeTargetObservation {
        browser_family: config.browser_family.clone(),
        browser_channel: config.browser_channel.clone(),
        managed_browser_session_id: config.managed_browser_session_id.clone(),
        profile_id: config.profile_id.clone(),
        process_id: config.process_id,
        target_id: string_field(target, constants::browser::DEVTOOLS_FIELD_ID)
            .ok_or(BrowserBridgePollError::InvalidTargetPayload)?
            .to_string(),
        tab_id: string_field(target, constants::browser::DEVTOOLS_FIELD_TAB_ID)
            .map(ToString::to_string),
        window_id: string_field(target, constants::browser::DEVTOOLS_FIELD_WINDOW_ID)
            .map(ToString::to_string),
        active_state: BrowserActiveTabState::Unknown,
        active_proof_source: BrowserActiveProofSource::TargetListOnly,
        url: url.to_string(),
        title: string_field(target, constants::browser::DEVTOOLS_FIELD_TITLE)
            .map(ToString::to_string),
        capability_status: BrowserCapabilityStatus::TabListOnly,
        degraded_reason: None,
        custody_label: BrowserCustodyLabel::ChildDeviceLocal,
        query_visibility: BrowserQueryVisibilityLabel::LiveLocal,
    };
    browser_tab_observation_event(observation, observed_at, fresh_until, index)
        .map(Some)
        .map_err(|_error| BrowserBridgePollError::InvalidTargetPayload)
}

fn validate_bridge_custody(
    config: &BrowserBridgePollConfig,
    observed_at: &str,
) -> Result<(), BrowserBridgePollError> {
    if config.expected_custody.bridge_port == constants::browser::DEVTOOLS_PORT_UNRESERVED
        || config.endpoint.port() != config.expected_custody.bridge_port
    {
        return Err(BrowserBridgePollError::UntrustedBridgePort);
    }
    if observed_at > config.expected_custody.session_fresh_until.as_str() {
        return Err(BrowserBridgePollError::StaleSession);
    }
    if config.managed_browser_session_id != config.expected_custody.managed_browser_session_id
        || !config
            .managed_browser_session_id
            .starts_with(constants::browser::SESSION_ID_PREFIX_MANAGED)
    {
        return Err(BrowserBridgePollError::UntrustedSession);
    }
    if config.profile_id != config.expected_custody.profile_id
        || !managed_profile_id_is_trusted(&config.profile_id)
    {
        return Err(BrowserBridgePollError::UntrustedProfile);
    }
    if config.process_id == constants::browser::PROCESS_ID_UNKNOWN
        || config.process_id != config.expected_custody.process_id
    {
        return Err(BrowserBridgePollError::UntrustedProcess);
    }
    if browser_identity_is_unknown(&config.browser_family, &config.browser_channel)
        || config.browser_family != config.expected_custody.browser_family
        || config.browser_channel != config.expected_custody.browser_channel
    {
        return Err(BrowserBridgePollError::UntrustedBrowserIdentity);
    }
    Ok(())
}

fn managed_profile_id_is_trusted(profile_id: &str) -> bool {
    profile_id.starts_with(constants::browser::PROFILE_ID_PREFIX_MANAGED)
        && !profile_id.contains(constants::browser::PATH_SEPARATOR_FORWARD)
        && !profile_id.contains(constants::browser::PATH_SEPARATOR_BACKSLASH)
        && !profile_id.contains(constants::browser::PATH_SEPARATOR_COLON)
        && profile_id != constants::browser::PATH_SEGMENT_DEFAULT
        && profile_id != constants::browser::PATH_SEGMENT_USER_DATA
}

fn browser_identity_is_unknown(family: &BrowserFamily, channel: &BrowserChannel) -> bool {
    matches!(
        family,
        BrowserFamily::Unknown | BrowserFamily::UnknownChromium
    ) || matches!(channel, BrowserChannel::Unknown)
}

fn target_url_is_observable(url: &str) -> bool {
    url != constants::browser::CHROMIUM_DEFAULT_URL
        && !url.starts_with(constants::browser::CHROMIUM_INTERNAL_CHROME_PREFIX)
        && !url.starts_with(constants::browser::CHROMIUM_INTERNAL_DEVTOOLS_PREFIX)
        && !url.starts_with(constants::browser::CHROMIUM_INTERNAL_EDGE_PREFIX)
}

fn string_field<'a>(target: &'a Value, key: &str) -> Option<&'a str> {
    target
        .get(key)
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
}
