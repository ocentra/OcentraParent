use ocentra_parent_agent_protocol::browser::BROWSER_EVIDENCE_SCHEMA_VERSION;
use ocentra_parent_agent_protocol::browser::{
    BrowserActiveProofSource, BrowserActiveTabState, BrowserCapabilityStatus, BrowserChannel,
    BrowserCustodyLabel, BrowserFamily,
};
use ocentra_parent_agent_protocol::browser_managed::BrowserQueryVisibilityLabel;
use ocentra_parent_agent_protocol::browser_read_model::BrowserEvidenceReadModel;
use ocentra_parent_agent_protocol::browser_read_model::BrowserTabEvidence;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};
use rusqlite::{params, Connection, Row};

use crate::ActivityStoreError;

pub(crate) fn browser_evidence_read_model(
    connection: &Connection,
    limit: u64,
    generated_at: &str,
) -> Result<BrowserEvidenceReadModel, ActivityStoreError> {
    let rows = browser_store_rows(connection, limit)?;
    let read_rows = rows
        .into_iter()
        .filter_map(browser_read_row_from_store)
        .collect::<Vec<_>>();
    let latest = read_rows.first();
    let capability_status = latest.map(|row| row.evidence.capability_status);
    let custody_label = latest
        .map(|row| row.evidence.custody_label)
        .unwrap_or(BrowserCustodyLabel::Unavailable);
    let query_visibility = latest
        .map(|row| row.evidence.query_visibility)
        .unwrap_or(BrowserQueryVisibilityLabel::Unavailable);
    let latest_event_id = latest.map(|row| row.event_id.clone());
    let latest_observed_at = latest.map(|row| row.observed_at.clone());
    let evidence_rows = read_rows
        .into_iter()
        .map(|row| row.evidence)
        .collect::<Vec<_>>();

    Ok(BrowserEvidenceReadModel {
        schema_version: BROWSER_EVIDENCE_SCHEMA_VERSION,
        generated_at: generated_at.to_string(),
        limit,
        returned: evidence_rows.len() as u64,
        latest_event_id,
        latest_observed_at,
        capability_status,
        custody_label,
        query_visibility,
        rows: evidence_rows,
    })
}

struct BrowserStoreRow {
    event_id: String,
    observed_at: String,
    device_id: String,
    fields: LogFields,
}

struct BrowserReadRow {
    event_id: String,
    observed_at: String,
    evidence: BrowserTabEvidence,
}

fn browser_store_rows(
    connection: &Connection,
    limit: u64,
) -> Result<Vec<BrowserStoreRow>, ActivityStoreError> {
    let mut statement = connection.prepare(constants::sqlite::SELECT_RECENT_BROWSER_ACTIVITY)?;
    let rows = statement.query_map(
        params![
            constants::activity_event_kind::URL_OBSERVED,
            constants::activity_observer::MANAGED_BROWSER_BRIDGE,
            limit as i64
        ],
        browser_store_row_from_sqlite,
    )?;
    let mut results = Vec::new();
    for row in rows {
        results.push(row?);
    }
    Ok(results)
}

fn browser_store_row_from_sqlite(row: &Row<'_>) -> rusqlite::Result<BrowserStoreRow> {
    let fields_json: String = row.get(3)?;
    let fields = serde_json::from_str::<LogFields>(&fields_json)
        .map_err(|error| rusqlite::Error::ToSqlConversionFailure(Box::new(error)))?;

    Ok(BrowserStoreRow {
        event_id: row.get(0)?,
        observed_at: row.get(1)?,
        device_id: row.get(2)?,
        fields,
    })
}

fn browser_read_row_from_store(row: BrowserStoreRow) -> Option<BrowserReadRow> {
    let fields = &row.fields;
    let fresh_until = string_field(fields, constants::field::FRESH_UNTIL)
        .or_else(|| string_field(fields, constants::field::LAST_OBSERVED_AT))
        .unwrap_or_else(|| row.observed_at.clone());
    let stale_at =
        string_field(fields, constants::field::STALE_AT).unwrap_or_else(|| fresh_until.clone());
    let evidence = BrowserTabEvidence {
        schema_version: BROWSER_EVIDENCE_SCHEMA_VERSION,
        browser_evidence_id: string_field(fields, constants::field::BROWSER_EVIDENCE_ID)?,
        observed_at: row.observed_at.clone(),
        fresh_until,
        source_id: string_field(fields, constants::field::SOURCE_ID)?,
        adapter_id: string_field(fields, constants::field::ADAPTER_ID)?,
        device_id: row.device_id,
        browser_family: browser_family_field(fields)?,
        browser_channel: browser_channel_field(fields).unwrap_or(BrowserChannel::Unknown),
        managed_browser_session_id: string_field(
            fields,
            constants::field::MANAGED_BROWSER_SESSION_ID,
        )?,
        profile_id: string_field(fields, constants::field::PROFILE_ID)?,
        process_id: u32_field(fields, constants::field::PROCESS_ID)
            .unwrap_or(constants::browser::PROCESS_ID_UNKNOWN),
        window_id: string_field(fields, constants::field::WINDOW_ID),
        tab_id: string_field(fields, constants::field::TAB_ID),
        target_id: string_field(fields, constants::field::TARGET_ID),
        active_state: active_state_field(fields)?,
        active_proof_source: active_proof_source_field(fields)
            .unwrap_or(BrowserActiveProofSource::TargetListOnly),
        url: string_field(fields, constants::field::URL)?,
        origin: string_field(fields, constants::field::ORIGIN)?,
        domain: string_field(fields, constants::field::DOMAIN)?,
        title: string_field(fields, constants::field::TITLE),
        capability_status: capability_status_field(fields)?,
        degraded_reason: string_field(fields, constants::field::DEGRADED_REASON)
            .or_else(|| string_field(fields, constants::field::REASON)),
        stale_at,
        custody_label: custody_label_field(fields).unwrap_or(BrowserCustodyLabel::Unavailable),
        query_visibility: query_visibility_field(fields)
            .unwrap_or(BrowserQueryVisibilityLabel::Unavailable),
    };

    Some(BrowserReadRow {
        event_id: row.event_id,
        observed_at: row.observed_at,
        evidence,
    })
}

fn string_field(fields: &LogFields, key: &str) -> Option<String> {
    match fields.get(key) {
        Some(LogFieldValue::String(value)) => Some(value.clone()),
        _ => None,
    }
}

fn u32_field(fields: &LogFields, key: &str) -> Option<u32> {
    match fields.get(key) {
        Some(LogFieldValue::Number(value)) if value.is_finite() && *value >= 0.0 => {
            u32::try_from(*value as u64).ok()
        }
        _ => None,
    }
}

fn browser_family_field(fields: &LogFields) -> Option<BrowserFamily> {
    string_field(fields, constants::field::BROWSER_FAMILY)
        .and_then(|value| BrowserFamily::from_protocol_str(&value))
}

fn browser_channel_field(fields: &LogFields) -> Option<BrowserChannel> {
    string_field(fields, constants::field::BROWSER_CHANNEL)
        .and_then(|value| BrowserChannel::from_protocol_str(&value))
}

fn active_state_field(fields: &LogFields) -> Option<BrowserActiveTabState> {
    string_field(fields, constants::field::ACTIVE_STATE)
        .and_then(|value| BrowserActiveTabState::from_protocol_str(&value))
}

fn active_proof_source_field(fields: &LogFields) -> Option<BrowserActiveProofSource> {
    string_field(fields, constants::field::ACTIVE_PROOF_SOURCE)
        .and_then(|value| BrowserActiveProofSource::from_protocol_str(&value))
}

fn capability_status_field(fields: &LogFields) -> Option<BrowserCapabilityStatus> {
    string_field(fields, constants::field::CAPABILITY_STATUS)
        .and_then(|value| BrowserCapabilityStatus::from_protocol_str(&value))
}

fn custody_label_field(fields: &LogFields) -> Option<BrowserCustodyLabel> {
    string_field(fields, constants::field::CUSTODY_LABEL)
        .and_then(|value| BrowserCustodyLabel::from_protocol_str(&value))
}

fn query_visibility_field(fields: &LogFields) -> Option<BrowserQueryVisibilityLabel> {
    string_field(fields, constants::field::QUERY_VISIBILITY)
        .and_then(|value| BrowserQueryVisibilityLabel::from_protocol_str(&value))
}
