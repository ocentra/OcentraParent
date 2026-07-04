use ocentra_parent_agent_protocol::app_game::AppGameServiceReadModel;
use rusqlite::Connection;

#[path = "read_model_counts.rs"]
mod read_model_counts;
#[path = "read_model_project.rs"]
mod read_model_project;
#[path = "read_model_sqlite.rs"]
mod read_model_sqlite;

use crate::ActivityStoreError;

use super::super::app_game_session_daily_rollups;

pub fn app_game_journal_sqlite_read_model(
    connection: &Connection,
    limit: u64,
    generated_at: &str,
) -> Result<AppGameServiceReadModel, ActivityStoreError> {
    let mut model = read_model_sqlite::app_game_journal_sqlite_read_model(
        connection,
        limit,
        generated_at,
        app_game_session_daily_rollups(connection, limit)?,
    )?;
    read_model_counts::refresh_returned_counts(&mut model);
    Ok(model)
}
