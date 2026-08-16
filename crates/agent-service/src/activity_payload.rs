use ocentra_parent_agent_protocol::activity_query::{ActivityIngestStatus, ActivityRecentSummary};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};

use crate::fields::fields_from_pairs;

struct OptionalActivityText<'a>(Option<&'a str>);

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
            optional_text(&OptionalActivityText(status.last_event_id.as_deref())),
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
            optional_text(&OptionalActivityText(summary.first_observed_at.as_deref())),
        ),
        (
            constants::field::LAST_OBSERVED_AT,
            optional_text(&OptionalActivityText(summary.last_observed_at.as_deref())),
        ),
        (
            constants::field::LAST_EVENT_ID,
            optional_text(&OptionalActivityText(summary.last_event_id.as_deref())),
        ),
        (
            constants::field::MOST_RECENT_KIND,
            optional_text(&OptionalActivityText(
                summary
                    .most_recent_kind
                    .as_ref()
                    .map(|kind| kind.as_protocol_str()),
            )),
        ),
        (
            constants::field::MOST_RECENT_OBSERVER,
            optional_text(&OptionalActivityText(
                summary
                    .most_recent_observer
                    .as_ref()
                    .map(|observer| observer.as_protocol_str()),
            )),
        ),
        (
            constants::field::MOST_RECENT_SUBJECT_KIND,
            optional_text(&OptionalActivityText(
                summary
                    .most_recent_subject_kind
                    .as_ref()
                    .map(|kind| kind.as_protocol_str()),
            )),
        ),
        (
            constants::field::MOST_RECENT_SUBJECT_ID,
            optional_text(&OptionalActivityText(
                summary.most_recent_subject_id.as_deref(),
            )),
        ),
        (
            constants::field::MOST_RECENT_SUBJECT_NAME,
            optional_text(&OptionalActivityText(
                summary.most_recent_subject_name.as_deref(),
            )),
        ),
    ])
}

pub fn activity_store_error_payload() -> LogFields {
    fields_from_pairs(vec![(
        constants::field::REASON,
        LogFieldValue::String(constants::value::ACTIVITY_STORE_UNAVAILABLE.to_string()),
    )])
}

fn optional_text(value: &OptionalActivityText<'_>) -> LogFieldValue {
    match value.0 {
        Some(text) => LogFieldValue::String(text.to_string()),
        None => LogFieldValue::Null(()),
    }
}
