use ocentra_parent_agent_protocol::browser_read_model::{
    BrowserEvidenceReadModel, BrowserTabEvidence,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};

use crate::fields::fields_from_pairs;

type FieldPair = (&'static str, LogFieldValue);

pub fn browser_evidence_read_model_payload(read_model: &BrowserEvidenceReadModel) -> LogFields {
    let latest = read_model.rows.first();
    let mut pairs = read_model_pairs(read_model);
    pairs.extend(latest_identity_pairs(latest));
    pairs.extend(latest_target_pairs(latest));
    pairs.extend(latest_state_pairs(latest));
    fields_from_pairs(pairs)
}

fn read_model_pairs(read_model: &BrowserEvidenceReadModel) -> Vec<FieldPair> {
    vec![
        (
            constants::field::GENERATED_AT,
            LogFieldValue::String(read_model.generated_at.clone()),
        ),
        (
            constants::field::LIMIT,
            LogFieldValue::Number(read_model.limit as f64),
        ),
        (
            constants::field::RETURNED,
            LogFieldValue::Number(read_model.returned as f64),
        ),
        (
            constants::field::LATEST_EVENT_ID,
            optional_string(read_model.latest_event_id.as_ref()),
        ),
        (
            constants::field::LATEST_OBSERVED_AT,
            optional_string(read_model.latest_observed_at.as_ref()),
        ),
        (
            constants::field::CAPABILITY_STATUS,
            optional_enum(
                read_model
                    .capability_status
                    .as_ref()
                    .map(|status| status.as_protocol_str()),
            ),
        ),
        (
            constants::field::CUSTODY_LABEL,
            LogFieldValue::String(read_model.custody_label.as_protocol_str().to_string()),
        ),
        (
            constants::field::QUERY_VISIBILITY,
            LogFieldValue::String(read_model.query_visibility.as_protocol_str().to_string()),
        ),
    ]
}

fn latest_identity_pairs(row: Option<&BrowserTabEvidence>) -> Vec<FieldPair> {
    vec![
        (
            constants::field::BROWSER_EVIDENCE_ID,
            optional_string(row.map(|value| &value.browser_evidence_id)),
        ),
        (
            constants::field::SOURCE_ID,
            optional_string(row.map(|value| &value.source_id)),
        ),
        (
            constants::field::ADAPTER_ID,
            optional_string(row.map(|value| &value.adapter_id)),
        ),
        (
            constants::field::MANAGED_BROWSER_SESSION_ID,
            optional_string(row.map(|value| &value.managed_browser_session_id)),
        ),
        (
            constants::field::BROWSER_FAMILY,
            optional_enum(row.map(|value| value.browser_family.as_protocol_str())),
        ),
        (
            constants::field::BROWSER_CHANNEL,
            optional_enum(row.map(|value| value.browser_channel.as_protocol_str())),
        ),
        (
            constants::field::PROFILE_ID,
            optional_string(row.map(|value| &value.profile_id)),
        ),
        (
            constants::field::PROCESS_ID,
            optional_u32(row.map(|value| value.process_id)),
        ),
    ]
}

fn latest_target_pairs(row: Option<&BrowserTabEvidence>) -> Vec<FieldPair> {
    vec![
        (
            constants::field::WINDOW_ID,
            optional_string(row.and_then(|value| value.window_id.as_ref())),
        ),
        (
            constants::field::TAB_ID,
            optional_string(row.and_then(|value| value.tab_id.as_ref())),
        ),
        (
            constants::field::TARGET_ID,
            optional_string(row.and_then(|value| value.target_id.as_ref())),
        ),
        (
            constants::field::URL,
            optional_string(row.map(|value| &value.url)),
        ),
        (
            constants::field::ORIGIN,
            optional_string(row.map(|value| &value.origin)),
        ),
        (
            constants::field::DOMAIN,
            optional_string(row.map(|value| &value.domain)),
        ),
        (
            constants::field::TITLE,
            optional_string(row.and_then(|value| value.title.as_ref())),
        ),
    ]
}

fn latest_state_pairs(row: Option<&BrowserTabEvidence>) -> Vec<FieldPair> {
    vec![
        (
            constants::field::ACTIVE_STATE,
            optional_enum(row.map(|value| value.active_state.as_protocol_str())),
        ),
        (
            constants::field::ACTIVE_PROOF_SOURCE,
            optional_enum(row.map(|value| value.active_proof_source.as_protocol_str())),
        ),
        (
            constants::field::FRESH_UNTIL,
            optional_string(row.map(|value| &value.fresh_until)),
        ),
        (
            constants::field::STALE_AT,
            optional_string(row.map(|value| &value.stale_at)),
        ),
        (
            constants::field::DEGRADED_REASON,
            optional_string(row.and_then(|value| value.degraded_reason.as_ref())),
        ),
    ]
}

fn optional_string(value: Option<&String>) -> LogFieldValue {
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
