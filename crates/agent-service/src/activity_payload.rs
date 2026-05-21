use ocentra_parent_agent_protocol::{
    constants, ActivityIngestStatus, ActivityRecentSummary, BrowserEvidenceRecentSummary,
    LogFieldValue, LogFields,
};

use crate::fields::fields_from_pairs;

pub fn ingest_status_payload(status: &ActivityIngestStatus) -> LogFields {
    fields_from_pairs(vec![
        (
            constants::field::DATABASE_READY,
            LogFieldValue::Boolean(status.database_ready),
        ),
        (
            constants::field::EVENTS_INGESTED,
            LogFieldValue::Number(status.events_ingested as f64),
        ),
        (
            constants::field::EVENTS_STORED,
            LogFieldValue::Number(status.events_stored as f64),
        ),
        (
            constants::field::DUPLICATE_EVENTS,
            LogFieldValue::Number(status.duplicate_events as f64),
        ),
        (
            constants::field::LAST_EVENT_ID,
            optional_string(&status.last_event_id),
        ),
    ])
}

pub fn recent_summary_payload(summary: &ActivityRecentSummary) -> LogFields {
    fields_from_pairs(vec![
        (
            constants::field::LIMIT,
            LogFieldValue::Number(summary.limit as f64),
        ),
        (
            constants::field::RETURNED,
            LogFieldValue::Number(summary.returned as f64),
        ),
        (
            constants::field::FIRST_OBSERVED_AT,
            optional_string(&summary.first_observed_at),
        ),
        (
            constants::field::LAST_OBSERVED_AT,
            optional_string(&summary.last_observed_at),
        ),
        (
            constants::field::LAST_EVENT_ID,
            optional_string(&summary.last_event_id),
        ),
        (
            constants::field::MOST_RECENT_KIND,
            optional_enum(
                summary
                    .most_recent_kind
                    .as_ref()
                    .map(|kind| kind.as_protocol_str()),
            ),
        ),
        (
            constants::field::MOST_RECENT_OBSERVER,
            optional_enum(
                summary
                    .most_recent_observer
                    .as_ref()
                    .map(|observer| observer.as_protocol_str()),
            ),
        ),
        (
            constants::field::MOST_RECENT_SUBJECT_KIND,
            optional_enum(
                summary
                    .most_recent_subject_kind
                    .as_ref()
                    .map(|kind| kind.as_protocol_str()),
            ),
        ),
        (
            constants::field::MOST_RECENT_SUBJECT_ID,
            optional_string(&summary.most_recent_subject_id),
        ),
        (
            constants::field::MOST_RECENT_SUBJECT_NAME,
            optional_string(&summary.most_recent_subject_name),
        ),
    ])
}

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

pub fn activity_store_error_payload() -> LogFields {
    fields_from_pairs(vec![(
        constants::field::REASON,
        LogFieldValue::String(constants::value::ACTIVITY_STORE_UNAVAILABLE.to_string()),
    )])
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
