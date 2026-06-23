use ocentra_parent_agent_protocol::activity::ActivityEvidenceRef;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFields;
use rusqlite::{params, Connection, Row};

use crate::ActivityStoreError;

pub(crate) struct NetworkFlowStoreRow {
    pub event_id: String,
    pub observed_at: String,
    pub observer: String,
    pub kind: String,
    pub fields: LogFields,
    pub evidence: Vec<ActivityEvidenceRef>,
}

pub(crate) fn network_flow_rows(
    connection: &Connection,
    limit: u64,
) -> Result<Vec<NetworkFlowStoreRow>, ActivityStoreError> {
    let mut statement =
        connection.prepare(constants::sqlite::SELECT_RECENT_NETWORK_FLOW_ACTIVITY)?;
    let rows = statement.query_map(
        params![
            constants::activity_event_kind::DOMAIN_OBSERVED,
            constants::activity_observer::WINDOWS_NETWORK,
            constants::activity_event_kind::NETWORK_RETENTION_DELETED,
            limit as i64
        ],
        row_from_sqlite,
    )?;
    let mut results = Vec::new();
    for row in rows {
        results.push(row?);
    }
    Ok(results)
}

fn row_from_sqlite(row: &Row<'_>) -> rusqlite::Result<NetworkFlowStoreRow> {
    let fields_json: String = row.get(4)?;
    let evidence_json: String = row.get(5)?;
    let fields = serde_json::from_str::<LogFields>(&fields_json).map_err(json_to_sqlite_error)?;
    let evidence = serde_json::from_str::<Vec<ActivityEvidenceRef>>(&evidence_json)
        .map_err(json_to_sqlite_error)?;

    Ok(NetworkFlowStoreRow {
        event_id: row.get(0)?,
        observed_at: row.get(1)?,
        observer: row.get(2)?,
        kind: row.get(3)?,
        fields,
        evidence,
    })
}

fn json_to_sqlite_error(error: serde_json::Error) -> rusqlite::Error {
    rusqlite::Error::ToSqlConversionFailure(Box::new(error))
}
