use ocentra_parent_agent_protocol::{
    constants, BrowserEvidenceRecentSummary, LogFieldValue, LogFields,
};

use crate::fields::fields_from_pairs;

pub fn browser_evidence_recent_payload(summary: &BrowserEvidenceRecentSummary) -> LogFields {
    fields_from_pairs(vec![
        (
            constants::field::RETURNED,
            LogFieldValue::Number(summary.returned as f64),
        ),
        (
            constants::field::LATEST_EVENT_ID,
            optional_string(&summary.latest_event_id),
        ),
        (
            constants::field::LATEST_OBSERVED_AT,
            optional_string(&summary.latest_observed_at),
        ),
        (
            constants::field::BROWSER_EVIDENCE_ID,
            optional_string(&summary.browser_evidence_id),
        ),
        (
            constants::field::SOURCE_ID,
            optional_string(&summary.source_id),
        ),
        (
            constants::field::ADAPTER_ID,
            optional_string(&summary.adapter_id),
        ),
        (
            constants::field::MANAGED_BROWSER_SESSION_ID,
            optional_string(&summary.managed_browser_session_id),
        ),
        (
            constants::field::BROWSER_FAMILY,
            optional_string(&summary.browser_family),
        ),
        (
            constants::field::ACTIVE_STATE,
            optional_string(&summary.active_state),
        ),
        (constants::field::URL, optional_string(&summary.url)),
        (constants::field::ORIGIN, optional_string(&summary.origin)),
        (constants::field::DOMAIN, optional_string(&summary.domain)),
        (constants::field::TITLE, optional_string(&summary.title)),
        (
            constants::field::CAPABILITY_STATUS,
            optional_string(&summary.capability_status),
        ),
        (
            constants::field::CUSTODY_LABEL,
            optional_string(&summary.custody_label),
        ),
    ])
}

fn optional_string(value: &Option<String>) -> LogFieldValue {
    match value {
        Some(text) => LogFieldValue::String(text.clone()),
        None => LogFieldValue::Null(()),
    }
}
