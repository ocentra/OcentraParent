use ocentra_parent_agent_protocol::activity::ActivityEvidenceRef;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFields;
use rusqlite::{params, Connection, Row};

use crate::ActivityStoreError;

pub(crate) struct MemoryGraphStoreRow {
    pub event_id: String,
    pub observed_at: String,
    pub device_id: String,
    pub platform: String,
    pub kind: String,
    pub subject_id: String,
    pub subject_display_name: Option<String>,
    pub fields: LogFields,
    pub evidence: Vec<ActivityEvidenceRef>,
}

pub(crate) fn memory_graph_index_rows(
    connection: &Connection,
) -> Result<Vec<MemoryGraphStoreRow>, ActivityStoreError> {
    let mut statement =
        connection.prepare(constants::sqlite::SELECT_MEMORY_GRAPH_ACTIVITY_FOR_INDEX)?;
    let rows = statement.query_map(
        params![
            constants::activity_event_kind::URL_OBSERVED,
            constants::activity_event_kind::VIDEO_OBSERVED,
            constants::activity_event_kind::WINDOW_FOCUSED,
            constants::activity_store::MEMORY_GRAPH_INDEX_REFRESH_LIMIT as i64
        ],
        row_from_sqlite,
    )?;
    let mut results = Vec::new();
    for row in rows {
        results.push(row?);
    }
    Ok(results)
}

fn row_from_sqlite(row: &Row<'_>) -> rusqlite::Result<MemoryGraphStoreRow> {
    let fields_json: String = row.get(9)?;
    let evidence_json: String = row.get(10)?;
    let fields = serde_json::from_str::<LogFields>(&fields_json).map_err(json_to_sqlite_error)?;
    let evidence = serde_json::from_str::<Vec<ActivityEvidenceRef>>(&evidence_json)
        .map_err(json_to_sqlite_error)?;

    Ok(MemoryGraphStoreRow {
        event_id: row.get(0)?,
        observed_at: row.get(1)?,
        device_id: row.get(2)?,
        platform: row.get(3)?,
        kind: row.get(5)?,
        subject_id: row.get(7)?,
        subject_display_name: row.get(8)?,
        fields,
        evidence,
    })
}

fn json_to_sqlite_error(error: serde_json::Error) -> rusqlite::Error {
    rusqlite::Error::ToSqlConversionFailure(Box::new(error))
}
