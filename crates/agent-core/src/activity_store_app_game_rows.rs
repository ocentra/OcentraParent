use ocentra_parent_agent_protocol::activity::ActivityEvidenceRef;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFields;
use rusqlite::{params, Connection, Row};

use crate::ActivityStoreError;

pub struct AppGameStoreRow {
    pub event_id: String,
    pub observed_at: String,
    pub kind: String,
    pub subject_id: String,
    pub subject_display_name: Option<String>,
    pub fields: LogFields,
    pub evidence: Vec<ActivityEvidenceRef>,
}

pub fn app_game_rows(
    connection: &Connection,
    limit: u64,
) -> Result<Vec<AppGameStoreRow>, ActivityStoreError> {
    let mut statement = connection.prepare(constants::sqlite::SELECT_RECENT_APP_GAME_ACTIVITY)?;
    let rows = statement.query_map(
        params![
            constants::activity_event_kind::PROCESS_OBSERVED,
            constants::activity_event_kind::WINDOW_FOCUSED,
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

fn row_from_sqlite(row: &Row<'_>) -> rusqlite::Result<AppGameStoreRow> {
    let fields_json: String = row.get(5)?;
    let evidence_json: String = row.get(6)?;
    let fields = serde_json::from_str::<LogFields>(&fields_json).map_err(json_to_sqlite_error)?;
    let evidence = serde_json::from_str::<Vec<ActivityEvidenceRef>>(&evidence_json)
        .map_err(json_to_sqlite_error)?;

    Ok(AppGameStoreRow {
        event_id: row.get(0)?,
        observed_at: row.get(1)?,
        kind: row.get(2)?,
        subject_id: row.get(3)?,
        subject_display_name: row.get(4)?,
        fields,
        evidence,
    })
}

fn json_to_sqlite_error(error: serde_json::Error) -> rusqlite::Error {
    rusqlite::Error::ToSqlConversionFailure(Box::new(error))
}
