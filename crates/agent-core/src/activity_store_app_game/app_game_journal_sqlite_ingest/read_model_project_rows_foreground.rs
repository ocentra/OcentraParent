use ocentra_parent_agent_protocol::app_game::{
    AppGameForegroundEvidenceRow, APP_GAME_FOREGROUND_FOREGROUND,
};

use crate::ActivityStoreError;

pub(crate) fn project_foreground_row(
    model: &mut ocentra_parent_agent_protocol::app_game::AppGameServiceReadModel,
    row_json: &str,
    seen_foreground_processes: &mut Vec<String>,
) -> Result<(), ActivityStoreError> {
    let foreground =
        serde_json::from_str::<AppGameForegroundEvidenceRow>(row_json).map_err(|_error| {
            ActivityStoreError::InvalidAppGameJournalRow {
                reason: "invalid-foreground-row",
            }
        })?;
    super::super::super::super::app_game_journal_sqlite_ingest_validation::validate_foreground_row(
        &foreground,
    )
    .map_err(|_error| ActivityStoreError::InvalidAppGameJournalRow {
        reason: "invalid-foreground-row",
    })?;
    if !seen_foreground_processes
        .iter()
        .any(|candidate| candidate == &foreground.process_identity)
    {
        seen_foreground_processes.push(foreground.process_identity.clone());
    } else {
        return Ok(());
    }
    if foreground.foreground_state == APP_GAME_FOREGROUND_FOREGROUND {
        model.foreground_now_rows.push(foreground);
    }
    Ok(())
}
