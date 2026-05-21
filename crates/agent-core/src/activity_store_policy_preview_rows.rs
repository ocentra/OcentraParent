use ocentra_parent_agent_protocol::{constants, ActivityEvidenceRef, LogFields};
use rusqlite::{params, Connection};

use crate::ActivityStoreError;

pub(crate) struct PolicyPreviewStoreRow {
    pub event_id: String,
    pub observed_at: String,
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
    let mut statement = connection.prepare(constants::sqlite::SELECT_POLICY_PREVIEW_ACTIVITY)?;
    let rows = statement.query_map(params![limit as i64], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, String>(3)?,
            row.get::<_, Option<String>>(4)?,
            row.get::<_, String>(5)?,
            row.get::<_, String>(6)?,
        ))
    })?;

    let mut results = Vec::new();
    for row in rows {
        results.push(store_row_from_sqlite(row?)?);
    }

    Ok(results)
}

fn store_row_from_sqlite(
    row: (
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
        subject_kind,
        subject_id,
        subject_display_name,
        fields_json,
        evidence_json,
    ) = row;
    Ok(PolicyPreviewStoreRow {
        event_id,
        observed_at,
        subject_kind,
        subject_id,
        subject_display_name,
        fields: serde_json::from_str::<LogFields>(&fields_json)?,
        evidence: serde_json::from_str::<Vec<ActivityEvidenceRef>>(&evidence_json)?,
    })
}
