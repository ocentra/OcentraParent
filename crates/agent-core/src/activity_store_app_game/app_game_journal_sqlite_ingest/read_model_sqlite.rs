use ocentra_parent_agent_protocol::app_game::{
    AppGameServiceReadModel, APP_GAME_CAPABILITY_STATUS_NOT_CLAIMED,
    APP_GAME_JOURNAL_CUSTODY_LOCAL_SQLITE, APP_GAME_JOURNAL_REPLAY_STATE_REPLAYED,
    APP_GAME_SCHEMA_VERSION,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFields;
use rusqlite::{params, Connection, Row};

use crate::ActivityStoreError;

use super::read_model_project::StoredAppGameJournalRow;

pub(super) fn app_game_journal_sqlite_read_model(
    connection: &Connection,
    limit: u64,
    generated_at: &str,
    daily_rollups: Vec<ocentra_parent_agent_protocol::app_game::AppGameSessionDailyRollup>,
) -> Result<AppGameServiceReadModel, ActivityStoreError> {
    let mut seen_runtime_processes = Vec::new();
    let mut seen_foreground_processes = Vec::new();
    let mut model = AppGameServiceReadModel {
        schema_version: APP_GAME_SCHEMA_VERSION,
        generated_at: generated_at.to_string(),
        limit,
        custody_label: APP_GAME_JOURNAL_CUSTODY_LOCAL_SQLITE.to_string(),
        replay_state: APP_GAME_JOURNAL_REPLAY_STATE_REPLAYED.to_string(),
        capability_status: APP_GAME_CAPABILITY_STATUS_NOT_CLAIMED.to_string(),
        inventory_returned: 0,
        running_now_returned: 0,
        foreground_now_returned: 0,
        launcher_returned: 0,
        daily_rollup_returned: 0,
        evidence_claim_returned: 0,
        identity_returned: 0,
        approval_authority_returned: 0,
        approval_action_result_returned: 0,
        platform_authority_matrix_returned: 0,
        ai_classifier_result_returned: 0,
        inventory_rows: Vec::new(),
        running_now_rows: Vec::new(),
        foreground_now_rows: Vec::new(),
        launcher_rows: Vec::new(),
        daily_rollups,
        evidence_claim_rows: Vec::new(),
        identity_rows: Vec::new(),
        approval_authority_rows: Vec::new(),
        approval_action_result_rows: Vec::new(),
        platform_authority_matrices: Vec::new(),
        ai_classifier_result_rows: Vec::new(),
    };
    let mut statement = connection.prepare(constants::sqlite::SELECT_APP_GAME_JOURNAL_ACTIVITY)?;
    let rows = statement.query_map(params![limit as i64], stored_row_from_sqlite)?;
    for row in rows {
        let row = row?;
        super::read_model_project::project_stored_row(
            &row,
            &mut model,
            &mut seen_runtime_processes,
            &mut seen_foreground_processes,
        )?;
    }
    Ok(model)
}

fn stored_row_from_sqlite(row: &Row<'_>) -> rusqlite::Result<StoredAppGameJournalRow> {
    let fields_json: String = row.get(0)?;
    let fields = serde_json::from_str::<LogFields>(&fields_json).map_err(json_to_sqlite_error)?;
    Ok(StoredAppGameJournalRow { fields })
}

fn json_to_sqlite_error(error: serde_json::Error) -> rusqlite::Error {
    rusqlite::Error::ToSqlConversionFailure(Box::new(error))
}
