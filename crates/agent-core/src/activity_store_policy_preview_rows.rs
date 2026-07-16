use ocentra_parent_agent_protocol::activity::ActivityEvidenceRef;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};
use rusqlite::{params, Connection};
use std::collections::HashSet;

use crate::ActivityStoreError;

pub(crate) struct PolicyPreviewStoreRow {
    pub event_id: String,
    pub observed_at: String,
    pub device_id: String,
    pub platform: String,
    pub kind: String,
    pub subject_kind: String,
    pub subject_id: String,
    pub subject_display_name: Option<String>,
    pub fields: LogFields,
    pub evidence: Vec<ActivityEvidenceRef>,
}

pub(crate) fn policy_preview_rows(
    connection: &Connection,
    limit: u64,
) -> Result<Vec<PolicyPreviewStoreRow>, ActivityStoreError> {
    if limit == 0 {
        return Ok(Vec::new());
    }

    let deleted_ids = deleted_evidence_reference_ids(connection)?;
    let mut statement = connection.prepare(constants::sqlite::SELECT_POLICY_PREVIEW_ACTIVITY)?;
    let rows = statement.query_map(
        params![constants::activity_event_kind::NETWORK_RETENTION_DELETED],
        |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, String>(5)?,
                row.get::<_, String>(6)?,
                row.get::<_, Option<String>>(7)?,
                row.get::<_, String>(8)?,
                row.get::<_, String>(9)?,
            ))
        },
    )?;

    let mut results = Vec::new();
    for row in rows {
        let store_row = store_row_from_sqlite(row?)?;
        if row_deleted(&store_row, &deleted_ids) {
            continue;
        }
        results.push(store_row);
        if results.len() >= limit as usize {
            break;
        }
    }

    Ok(results)
}

fn store_row_from_sqlite(
    row: (
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        Option<String>,
        String,
        String,
    ),
) -> Result<PolicyPreviewStoreRow, ActivityStoreError> {
    let (
        event_id,
        observed_at,
        device_id,
        platform,
        kind,
        subject_kind,
        subject_id,
        subject_display_name,
        fields_json,
        evidence_json,
    ) = row;
    Ok(PolicyPreviewStoreRow {
        event_id,
        observed_at,
        device_id,
        platform,
        kind,
        subject_kind,
        subject_id,
        subject_display_name,
        fields: serde_json::from_str::<LogFields>(&fields_json)?,
        evidence: serde_json::from_str::<Vec<ActivityEvidenceRef>>(&evidence_json)?,
    })
}

fn row_deleted(row: &PolicyPreviewStoreRow, deleted_ids: &HashSet<String>) -> bool {
    deleted_ids.contains(&row.event_id)
        || row
            .evidence
            .iter()
            .any(|reference| deleted_ids.contains(&reference.evidence_id))
}

fn deleted_evidence_reference_ids(
    connection: &Connection,
) -> Result<HashSet<String>, ActivityStoreError> {
    let mut statement =
        connection.prepare(constants::sqlite::SELECT_NETWORK_RETENTION_DELETED_ACTIVITY)?;
    let rows = statement.query_map(
        params![constants::activity_event_kind::NETWORK_RETENTION_DELETED],
        |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)),
    )?;

    let mut ids = HashSet::new();
    for row in rows {
        let (fields_json, evidence_json) = row?;
        let fields = serde_json::from_str::<LogFields>(&fields_json)?;
        let evidence = serde_json::from_str::<Vec<ActivityEvidenceRef>>(&evidence_json)?;
        for id in evidence_reference_ids(&fields, &evidence) {
            ids.insert(id);
        }
    }
    Ok(ids)
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

fn string_field(fields: &LogFields, key: &str) -> Option<String> {
    match fields.get(key) {
        Some(LogFieldValue::String(value)) => Some(value.clone()),
        _ => None,
    }
}
