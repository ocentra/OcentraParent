use ocentra_parent_agent_protocol::{constants, LogFieldValue, LogFields};

use crate::browser_bridge_event::BrowserBridgeTargetObservation;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct NormalizedBrowserUrl {
    origin: String,
    pub(crate) domain: String,
}

pub(crate) fn base_browser_fields(
    observation: &BrowserBridgeTargetObservation,
    evidence_id: &str,
    fresh_until: &str,
    normalized: &NormalizedBrowserUrl,
) -> LogFields {
    let mut fields = LogFields::new();
    insert_identity_fields(&mut fields, observation, evidence_id);
    insert_target_fields(&mut fields, observation, normalized);
    insert_state_fields(&mut fields, observation, fresh_until);
    fields
}

pub(crate) fn insert_optional_text(fields: &mut LogFields, key: &str, value: &Option<String>) {
    if let Some(value) = value {
        fields.insert(key.to_string(), LogFieldValue::String(value.clone()));
    }
}

pub(crate) fn normalized_browser_url(url: &str) -> Option<NormalizedBrowserUrl> {
    let (scheme, remainder) = url.split_once(constants::browser::URL_SCHEME_SEPARATOR)?;
    if scheme.is_empty() || remainder.is_empty() {
        return None;
    }
    let authority = remainder
        .split(constants::delimiter::SLASH)
        .next()
        .filter(|value| !value.is_empty())?;
    let domain = authority
        .rsplit(constants::delimiter::AT)
        .next()
        .filter(|value| !value.is_empty())?;
    let mut origin = String::from(scheme);
    origin.push_str(constants::browser::URL_SCHEME_SEPARATOR);
    origin.push_str(authority);
    Some(NormalizedBrowserUrl {
        origin,
        domain: domain.to_string(),
    })
}

fn insert_identity_fields(
    fields: &mut LogFields,
    observation: &BrowserBridgeTargetObservation,
    evidence_id: &str,
) {
    fields.insert(
        constants::field::BROWSER_EVIDENCE_ID.to_string(),
        LogFieldValue::String(evidence_id.to_string()),
    );
    fields.insert(
        constants::field::SOURCE_ID.to_string(),
        LogFieldValue::String(constants::browser::SOURCE_ID_MANAGED_CHROMIUM_DEVTOOLS.to_string()),
    );
    fields.insert(
        constants::field::ADAPTER_ID.to_string(),
        LogFieldValue::String(constants::browser::ADAPTER_ID_MANAGED_CHROMIUM_DEVTOOLS.to_string()),
    );
    fields.insert(
        constants::field::MANAGED_BROWSER_SESSION_ID.to_string(),
        LogFieldValue::String(observation.managed_browser_session_id.clone()),
    );
    fields.insert(
        constants::field::PROFILE_ID.to_string(),
        LogFieldValue::String(observation.profile_id.clone()),
    );
    fields.insert(
        constants::field::BROWSER_FAMILY.to_string(),
        LogFieldValue::String(observation.browser_family.as_protocol_str().to_string()),
    );
    fields.insert(
        constants::field::BROWSER_CHANNEL.to_string(),
        LogFieldValue::String(observation.browser_channel.as_protocol_str().to_string()),
    );
}

fn insert_target_fields(
    fields: &mut LogFields,
    observation: &BrowserBridgeTargetObservation,
    normalized: &NormalizedBrowserUrl,
) {
    fields.insert(
        constants::field::PROCESS_ID.to_string(),
        LogFieldValue::Number(f64::from(observation.process_id)),
    );
    fields.insert(
        constants::field::TARGET_ID.to_string(),
        LogFieldValue::String(observation.target_id.clone()),
    );
    fields.insert(
        constants::field::ACTIVE_STATE.to_string(),
        LogFieldValue::String(observation.active_state.as_protocol_str().to_string()),
    );
    fields.insert(
        constants::field::URL.to_string(),
        LogFieldValue::String(observation.url.clone()),
    );
    fields.insert(
        constants::field::ORIGIN.to_string(),
        LogFieldValue::String(normalized.origin.clone()),
    );
    fields.insert(
        constants::field::DOMAIN.to_string(),
        LogFieldValue::String(normalized.domain.clone()),
    );
}

fn insert_state_fields(
    fields: &mut LogFields,
    observation: &BrowserBridgeTargetObservation,
    fresh_until: &str,
) {
    fields.insert(
        constants::field::CAPABILITY_STATUS.to_string(),
        LogFieldValue::String(observation.capability_status.as_protocol_str().to_string()),
    );
    fields.insert(
        constants::field::CUSTODY_LABEL.to_string(),
        LogFieldValue::String(observation.custody_label.as_protocol_str().to_string()),
    );
    fields.insert(
        constants::field::QUERY_VISIBILITY.to_string(),
        LogFieldValue::String(constants::browser::QUERY_VISIBILITY_LIVE_LOCAL.to_string()),
    );
    fields.insert(
        constants::field::FRESH_UNTIL.to_string(),
        LogFieldValue::String(fresh_until.to_string()),
    );
    fields.insert(
        constants::field::STALE_AT.to_string(),
        LogFieldValue::String(fresh_until.to_string()),
    );
}
