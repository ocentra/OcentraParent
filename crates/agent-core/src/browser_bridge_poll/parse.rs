use ocentra_parent_agent_protocol::activity::ActivityEvent;
use ocentra_parent_agent_protocol::browser::{
    BrowserActiveProofSource, BrowserActiveTabState, BrowserCapabilityStatus, BrowserCustodyLabel,
};
use ocentra_parent_agent_protocol::browser_managed::BrowserQueryVisibilityLabel;
use ocentra_parent_agent_protocol::constants;
use serde_json::Value;

use crate::{browser_tab_observation_event, BrowserBridgeTargetObservation};

use super::{BrowserBridgePollConfig, BrowserBridgePollError};

pub(crate) fn parse_browser_version(body: &str) -> Result<Option<String>, BrowserBridgePollError> {
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

pub(crate) fn parse_target_events(
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
        browser_family: config.browser_family,
        browser_channel: config.browser_channel,
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
