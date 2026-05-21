use ocentra_parent_agent_protocol::{
    constants, BrowserEvidenceRecentSummary, LogFieldValue, LogFields,
    BROWSER_EVIDENCE_SCHEMA_VERSION,
};
use rusqlite::{params, Connection, OptionalExtension};

use crate::ActivityStoreError;

pub(crate) fn browser_recent_summary(
    connection: &Connection,
) -> Result<BrowserEvidenceRecentSummary, ActivityStoreError> {
    let mut statement = connection.prepare(constants::sqlite::SELECT_LATEST_BROWSER_ACTIVITY)?;
    let row = statement
        .query_row(
            params![
                constants::activity_event_kind::URL_OBSERVED,
                constants::activity_observer::MANAGED_BROWSER_BRIDGE
            ],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                ))
            },
        )
        .optional()?;

    match row {
        Some((event_id, observed_at, fields_json)) => {
            let fields = serde_json::from_str::<LogFields>(&fields_json)?;
            Ok(summary_from_fields(event_id, observed_at, &fields))
        }
        None => Ok(empty_browser_summary()),
    }
}

fn summary_from_fields(
    event_id: String,
    observed_at: String,
    fields: &LogFields,
) -> BrowserEvidenceRecentSummary {
    BrowserEvidenceRecentSummary {
        schema_version: BROWSER_EVIDENCE_SCHEMA_VERSION,
        returned: 1,
        latest_event_id: Some(event_id),
        latest_observed_at: Some(observed_at),
        browser_evidence_id: string_field(fields, constants::field::BROWSER_EVIDENCE_ID),
        source_id: string_field(fields, constants::field::SOURCE_ID),
        adapter_id: string_field(fields, constants::field::ADAPTER_ID),
        managed_browser_session_id: string_field(
            fields,
            constants::field::MANAGED_BROWSER_SESSION_ID,
        ),
        browser_family: string_field(fields, constants::field::BROWSER_FAMILY),
        active_state: string_field(fields, constants::field::ACTIVE_STATE),
        url: string_field(fields, constants::field::URL),
        origin: string_field(fields, constants::field::ORIGIN),
        domain: string_field(fields, constants::field::DOMAIN),
        title: string_field(fields, constants::field::TITLE),
        capability_status: string_field(fields, constants::field::CAPABILITY_STATUS),
        custody_label: string_field(fields, constants::field::CUSTODY_LABEL),
    }
}

fn empty_browser_summary() -> BrowserEvidenceRecentSummary {
    BrowserEvidenceRecentSummary {
        schema_version: BROWSER_EVIDENCE_SCHEMA_VERSION,
        returned: 0,
        latest_event_id: None,
        latest_observed_at: None,
        browser_evidence_id: None,
        source_id: None,
        adapter_id: None,
        managed_browser_session_id: None,
        browser_family: None,
        active_state: None,
        url: None,
        origin: None,
        domain: None,
        title: None,
        capability_status: None,
        custody_label: None,
    }
}

fn string_field(fields: &LogFields, key: &str) -> Option<String> {
    match fields.get(key) {
        Some(LogFieldValue::String(value)) => Some(value.clone()),
        _ => None,
    }
}
