use ocentra_parent_agent_protocol::{constants, ActivityEvidenceRef, LogFields};
use rusqlite::{params, Connection};

use crate::ActivityStoreError;

#[derive(Clone, Debug, PartialEq)]
pub struct NetworkStoreRow {
    pub event_id: String,
    pub observed_at: String,
    pub fields: LogFields,
    pub evidence: Vec<ActivityEvidenceRef>,
}

pub(crate) fn recent_network_rows(
    connection: &Connection,
    limit: u64,
) -> Result<Vec<NetworkStoreRow>, ActivityStoreError> {
    let mut statement = connection.prepare(constants::sqlite::SELECT_RECENT_NETWORK_ACTIVITY)?;
    let rows = statement.query_map(
        params![
            constants::activity_event_kind::DOMAIN_OBSERVED,
            constants::activity_observer::WINDOWS_NETWORK,
            limit as i64,
        ],
        row_from_sqlite,
    )?;
    let mut results = Vec::new();
    for row in rows {
        results.push(row?);
    }
    Ok(results)
}

fn row_from_sqlite(row: &rusqlite::Row<'_>) -> rusqlite::Result<NetworkStoreRow> {
    let fields_json: String = row.get(2)?;
    let evidence_json: String = row.get(3)?;
    Ok(NetworkStoreRow {
        event_id: row.get(0)?,
        observed_at: row.get(1)?,
        fields: serde_json::from_str(&fields_json).map_err(to_sqlite_error)?,
        evidence: serde_json::from_str(&evidence_json).map_err(to_sqlite_error)?,
    })
}

fn to_sqlite_error(error: serde_json::Error) -> rusqlite::Error {
    rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(error))
}
