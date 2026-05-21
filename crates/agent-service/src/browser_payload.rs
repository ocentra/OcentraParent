use ocentra_parent_agent_protocol::{
    constants, BrowserManagedSessionStatus, LogFieldValue, LogFields,
};

use crate::fields::fields_from_pairs;

type PayloadPairs = Vec<(&'static str, LogFieldValue)>;

pub fn browser_managed_status_payload(status: &BrowserManagedSessionStatus) -> LogFields {
    let mut pairs = browser_managed_identity_pairs(status);
    pairs.extend(browser_managed_state_pairs(status));
    fields_from_pairs(pairs)
}

fn browser_managed_identity_pairs(status: &BrowserManagedSessionStatus) -> PayloadPairs {
    vec![
        (
            constants::field::CHECKED_AT,
            LogFieldValue::String(status.checked_at.clone()),
        ),
        (
            constants::field::MANAGED_BROWSER_SESSION_ID,
            optional_string(&status.managed_browser_session_id),
        ),
        (
            constants::field::BROWSER_FAMILY,
            optional_enum(
                status
                    .browser_family
                    .as_ref()
                    .map(|family| family.as_protocol_str()),
            ),
        ),
        (
            constants::field::BROWSER_CHANNEL,
            optional_enum(
                status
                    .browser_channel
                    .as_ref()
                    .map(|channel| channel.as_protocol_str()),
            ),
        ),
        (
            constants::field::BROWSER_VERSION,
            optional_string(&status.browser_version),
        ),
        (
            constants::field::PROFILE_ID,
            optional_string(&status.profile_id),
        ),
        (
            constants::field::PROFILE_PATH_REF,
            optional_string(&status.profile_path_ref),
        ),
        (
            constants::field::PROCESS_ID,
            optional_u32(status.process_id),
        ),
        (
            constants::field::BRIDGE_KIND,
            optional_enum(
                status
                    .bridge_kind
                    .as_ref()
                    .map(|bridge_kind| bridge_kind.as_protocol_str()),
            ),
        ),
        (
            constants::field::BRIDGE_ENDPOINT_REF,
            optional_string(&status.bridge_endpoint_ref),
        ),
    ]
}

fn browser_managed_state_pairs(status: &BrowserManagedSessionStatus) -> PayloadPairs {
    vec![
        (
            constants::field::MANAGED_STATE,
            LogFieldValue::String(status.managed_state.as_protocol_str().to_string()),
        ),
        (
            constants::field::CAPABILITY_STATUS,
            LogFieldValue::String(status.capability_status.as_protocol_str().to_string()),
        ),
        (
            constants::field::REASON,
            optional_string(&status.degraded_reason),
        ),
        (
            constants::field::STARTED_AT,
            optional_string(&status.started_at),
        ),
        (
            constants::field::CUSTODY_LABEL,
            LogFieldValue::String(status.custody_label.as_protocol_str().to_string()),
        ),
        (
            constants::field::QUERY_VISIBILITY,
            LogFieldValue::String(status.query_visibility.as_protocol_str().to_string()),
        ),
    ]
}

fn optional_string(value: &Option<String>) -> LogFieldValue {
    match value {
        Some(text) => LogFieldValue::String(text.clone()),
        None => LogFieldValue::Null(()),
    }
}

fn optional_enum(value: Option<&str>) -> LogFieldValue {
    match value {
        Some(text) => LogFieldValue::String(text.to_string()),
        None => LogFieldValue::Null(()),
    }
}

fn optional_u32(value: Option<u32>) -> LogFieldValue {
    match value {
        Some(number) => LogFieldValue::Number(number as f64),
        None => LogFieldValue::Null(()),
    }
}
