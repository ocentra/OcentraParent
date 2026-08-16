use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};

use crate::browser_bridge_event::BrowserBridgeTargetObservation;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct NormalizedBrowserUrl {
    pub(crate) url: String,
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
    let scheme = scheme.to_ascii_lowercase();
    if scheme.is_empty() || remainder.is_empty() {
        return None;
    }
    let (authority, suffix) = split_authority_and_suffix(remainder);
    let authority = authority
        .rsplit(constants::delimiter::AT)
        .next()
        .filter(|value| !value.is_empty())?;
    let (authority, domain) = normalized_authority(authority)?;
    let mut origin = scheme;
    origin.push_str(constants::browser::URL_SCHEME_SEPARATOR);
    origin.push_str(&authority);
    let mut normalized_url = origin.clone();
    normalized_url.push_str(suffix);
    Some(NormalizedBrowserUrl {
        url: normalized_url,
        origin,
        domain,
    })
}

fn split_authority_and_suffix(remainder: &str) -> (&str, &str) {
    let authority = remainder
        .split(constants::delimiter::SLASH)
        .next()
        .unwrap_or(remainder);
    let suffix = remainder
        .get(authority.len()..)
        .unwrap_or(constants::value::EMPTY);
    (authority, suffix)
}

fn normalized_authority(authority: &str) -> Option<(String, String)> {
    let (host, port) = split_host_and_port(authority)?;
    let domain = normalized_host(host)?;
    let mut normalized = domain.clone();
    if let Some(port) = port {
        normalized.push(constants::delimiter::COLON);
        normalized.push_str(port);
    }
    Some((normalized, domain))
}

fn split_host_and_port(authority: &str) -> Option<(&str, Option<&str>)> {
    if authority.is_empty() {
        return None;
    }
    if authority.matches(constants::delimiter::COLON).count() == 1 {
        let (host, port) = authority.rsplit_once(constants::delimiter::COLON)?;
        if !host.is_empty() && !port.is_empty() && port.chars().all(|value| value.is_ascii_digit())
        {
            return Some((host, Some(port)));
        }
    }
    Some((authority, None))
}

fn normalized_host(host: &str) -> Option<String> {
    let value = host
        .trim_end_matches(constants::delimiter::DOT)
        .to_ascii_lowercase();
    if value.is_empty() || value.contains(constants::delimiter::SLASH) {
        return None;
    }
    Some(value)
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
        constants::field::ACTIVE_PROOF_SOURCE.to_string(),
        LogFieldValue::String(
            observation
                .active_proof_source
                .as_protocol_str()
                .to_string(),
        ),
    );
    fields.insert(
        constants::field::URL.to_string(),
        LogFieldValue::String(normalized.url.clone()),
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
        LogFieldValue::String(observation.query_visibility.as_protocol_str().to_string()),
    );
    insert_optional_text(
        fields,
        constants::field::DEGRADED_REASON,
        &observation.degraded_reason,
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
