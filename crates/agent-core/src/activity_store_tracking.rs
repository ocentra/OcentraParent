use ocentra_parent_agent_protocol::{
    constants, ActivityEvidenceRef, LogFieldValue, LogFields, TrackingReadModel,
    TrackingReadModelRow, ACTIVITY_QUERY_SCHEMA_VERSION,
    TRACKING_READ_MODEL_CUSTODY_CHILD_DEVICE_QUERY_STORE,
    TRACKING_READ_MODEL_STATUS_NO_TRACKING_EVENTS,
};
use rusqlite::Connection;

use crate::{
    activity_store_tracking_rows::{tracking_rows, TrackingStoreRow},
    ActivityStore, ActivityStoreError,
};

pub fn tracking_read_model_for_store(
    store: &ActivityStore,
    limit: u64,
    generated_at: &str,
) -> Result<TrackingReadModel, ActivityStoreError> {
    tracking_read_model(&store.connection, limit, generated_at)
}

fn tracking_read_model(
    connection: &Connection,
    limit: u64,
    generated_at: &str,
) -> Result<TrackingReadModel, ActivityStoreError> {
    let rows = tracking_rows(connection, limit)?;
    let read_rows = rows.into_iter().map(row_from_store).collect::<Vec<_>>();
    let latest = read_rows.first();
    let capability_status = latest
        .and_then(|row| row.capability_status.clone())
        .unwrap_or_else(|| TRACKING_READ_MODEL_STATUS_NO_TRACKING_EVENTS.to_string());
    let evidence_reference_ids = collect_evidence_reference_ids(&read_rows);
    let retention_tombstone_evidence_reference_ids =
        collect_retention_tombstone_evidence_reference_ids(&read_rows);
    let retention_tombstone_count = read_rows
        .iter()
        .filter(|row| row.kind == constants::activity_event_kind::TRACKING_RETENTION_DELETED)
        .count() as u64;

    Ok(TrackingReadModel {
        schema_version: ACTIVITY_QUERY_SCHEMA_VERSION,
        generated_at: generated_at.to_string(),
        custody_label: TRACKING_READ_MODEL_CUSTODY_CHILD_DEVICE_QUERY_STORE.to_string(),
        limit,
        returned: read_rows.len() as u64,
        capability_status,
        latest_event_id: latest.map(|row| row.event_id.clone()),
        latest_observed_at: latest.map(|row| row.observed_at.clone()),
        evidence_reference_ids,
        retention_tombstone_count,
        retention_tombstone_evidence_reference_ids,
        rows: read_rows,
    })
}

fn row_from_store(row: TrackingStoreRow) -> TrackingReadModelRow {
    let evidence_reference_ids = evidence_reference_ids(&row.fields, &row.evidence);
    let capability_status = string_field(&row.fields, constants::field::CAPABILITY_STATUS);

    TrackingReadModelRow {
        schema_version: ACTIVITY_QUERY_SCHEMA_VERSION,
        event_id: row.event_id,
        observed_at: row.observed_at,
        device_id: row.device_id,
        platform: row.platform,
        observer: row.observer,
        kind: row.kind,
        subject_kind: row.subject_kind,
        subject_id: row.subject_id,
        subject_display_name: row.subject_display_name,
        capability_status,
        evidence_reference_ids,
        evidence: row.evidence,
    }
}

fn evidence_reference_ids(fields: &LogFields, evidence: &[ActivityEvidenceRef]) -> Vec<String> {
    let mut ids = string_field(fields, constants::field::EVIDENCE_REFERENCE_IDS)
        .map(|value| split_evidence_reference_ids(&value))
        .unwrap_or_default();

    for reference in evidence {
        if !ids.iter().any(|id| id == &reference.evidence_id) {
            ids.push(reference.evidence_id.clone());
        }
    }
    ids
}

fn split_evidence_reference_ids(value: &str) -> Vec<String> {
    value
        .split(constants::delimiter::LIST)
        .map(str::trim)
        .filter(|id| !id.is_empty())
        .map(ToOwned::to_owned)
        .collect()
}

fn collect_evidence_reference_ids(rows: &[TrackingReadModelRow]) -> Vec<String> {
    let mut ids = Vec::new();
    for row in rows {
        push_unique_ids(&mut ids, &row.evidence_reference_ids);
    }
    ids
}

fn collect_retention_tombstone_evidence_reference_ids(
    rows: &[TrackingReadModelRow],
) -> Vec<String> {
    let mut ids = Vec::new();
    for row in rows
        .iter()
        .filter(|row| row.kind == constants::activity_event_kind::TRACKING_RETENTION_DELETED)
    {
        push_unique_ids(&mut ids, &row.evidence_reference_ids);
    }
    ids
}

fn push_unique_ids(target: &mut Vec<String>, source: &[String]) {
    for id in source {
        if !target.iter().any(|existing| existing == id) {
            target.push(id.clone());
        }
    }
}

fn string_field(fields: &LogFields, key: &str) -> Option<String> {
    match fields.get(key) {
        Some(LogFieldValue::String(value)) => Some(value.clone()),
        _ => None,
    }
}
