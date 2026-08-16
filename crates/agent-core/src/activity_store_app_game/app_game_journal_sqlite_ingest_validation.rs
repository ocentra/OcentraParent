use ocentra_parent_agent_protocol::app_game::{
    AppGameForegroundEvidenceRow, AppGameInventoryEvidenceRow, AppGameRuntimeEvidenceRow,
    APP_GAME_CONTENT_KNOWLEDGE_NOT_CLAIMED, APP_GAME_FOREGROUND_BACKGROUND,
    APP_GAME_FOREGROUND_FOREGROUND, APP_GAME_FOREGROUND_NOT_CLAIMED,
    APP_GAME_OBSERVATION_MODE_FOREGROUND_WINDOW, APP_GAME_OBSERVATION_MODE_PROCESS_EXIT,
    APP_GAME_OBSERVATION_MODE_PROCESS_START, APP_GAME_RUNTIME_NOT_CLAIMED,
    APP_GAME_RUNTIME_NOT_RUNNING, APP_GAME_RUNTIME_RUNNING,
};

use super::AppGameJournalSqliteIngestError;

pub(super) fn validate_inventory_row(
    row: &AppGameInventoryEvidenceRow,
) -> Result<(), AppGameJournalSqliteIngestError> {
    if row.runtime_state != APP_GAME_RUNTIME_NOT_CLAIMED
        || row.foreground_state != APP_GAME_FOREGROUND_NOT_CLAIMED
        || row.running_duration_ms != 0
        || row.foreground_duration_ms != 0
    {
        return Err(AppGameJournalSqliteIngestError::InventoryClaimsUse);
    }
    Ok(())
}

pub(super) fn validate_runtime_row(
    row: &AppGameRuntimeEvidenceRow,
) -> Result<(), AppGameJournalSqliteIngestError> {
    if row.foreground_state != APP_GAME_FOREGROUND_NOT_CLAIMED {
        return Err(AppGameJournalSqliteIngestError::RuntimeClaimsForeground);
    }
    if row.observation_mode == APP_GAME_OBSERVATION_MODE_PROCESS_EXIT
        && (row.runtime_state != APP_GAME_RUNTIME_NOT_RUNNING || row.exited_at.is_none())
    {
        return Err(AppGameJournalSqliteIngestError::RuntimeExitInvalid);
    }
    if row.observation_mode == APP_GAME_OBSERVATION_MODE_PROCESS_START
        && (row.runtime_state != APP_GAME_RUNTIME_RUNNING
            || row.started_at.is_none()
            || row.exited_at.is_some())
    {
        return Err(AppGameJournalSqliteIngestError::RuntimeStartInvalid);
    }
    Ok(())
}

pub(super) fn validate_foreground_row(
    row: &AppGameForegroundEvidenceRow,
) -> Result<(), AppGameJournalSqliteIngestError> {
    if row.observation_mode != APP_GAME_OBSERVATION_MODE_FOREGROUND_WINDOW {
        return Err(AppGameJournalSqliteIngestError::ForegroundWrongMode);
    }
    if row.content_knowledge_state != APP_GAME_CONTENT_KNOWLEDGE_NOT_CLAIMED {
        return Err(AppGameJournalSqliteIngestError::ForegroundClaimsContent);
    }
    if row.foreground_state == APP_GAME_FOREGROUND_FOREGROUND
        && (row.foreground_started_at.is_none() || row.foreground_ended_at.is_some())
    {
        return Err(AppGameJournalSqliteIngestError::ForegroundOpenInvalid);
    }
    if row.foreground_state == APP_GAME_FOREGROUND_BACKGROUND && row.foreground_ended_at.is_none() {
        return Err(AppGameJournalSqliteIngestError::ForegroundBackgroundInvalid);
    }
    Ok(())
}
