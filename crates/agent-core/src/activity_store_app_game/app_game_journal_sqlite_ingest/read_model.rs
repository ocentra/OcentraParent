use ocentra_parent_agent_protocol::{
    constants, AppGameAiClassifierResult, AppGameControlActionResult,
    AppGameControlApprovalAuthority, AppGameEvidenceClaim, AppGameForegroundEvidenceRow,
    AppGameIdentity, AppGameInventoryEvidenceRow, AppGameLauncherEvidenceRow,
    AppGamePlatformAuthorityMatrix, AppGameRuntimeEvidenceRow, AppGameServiceReadModel,
    LogFieldValue, LogFields, APP_GAME_CAPABILITY_STATUS_ADAPTER_ERROR,
    APP_GAME_CAPABILITY_STATUS_AVAILABLE, APP_GAME_CAPABILITY_STATUS_DEGRADED,
    APP_GAME_CAPABILITY_STATUS_MANUAL_REQUIRED, APP_GAME_CAPABILITY_STATUS_NOT_CLAIMED,
    APP_GAME_CAPABILITY_STATUS_PERMISSION_LIMITED, APP_GAME_CAPABILITY_STATUS_STALE,
    APP_GAME_CAPABILITY_STATUS_UNAVAILABLE, APP_GAME_CAPABILITY_STATUS_UNSUPPORTED_PLATFORM,
    APP_GAME_FOREGROUND_FOREGROUND, APP_GAME_JOURNAL_CUSTODY_LOCAL_SQLITE,
    APP_GAME_JOURNAL_FIELD_ROW_JSON, APP_GAME_JOURNAL_FIELD_ROW_KIND,
    APP_GAME_JOURNAL_REPLAY_STATE_REPLAYED, APP_GAME_JOURNAL_ROW_KIND_AI_CLASSIFIER_RESULT,
    APP_GAME_JOURNAL_ROW_KIND_APPROVAL_ACTION_RESULT, APP_GAME_JOURNAL_ROW_KIND_APPROVAL_AUTHORITY,
    APP_GAME_JOURNAL_ROW_KIND_EVIDENCE_CLAIM, APP_GAME_JOURNAL_ROW_KIND_FOREGROUND,
    APP_GAME_JOURNAL_ROW_KIND_IDENTITY, APP_GAME_JOURNAL_ROW_KIND_INVENTORY,
    APP_GAME_JOURNAL_ROW_KIND_LAUNCHER, APP_GAME_JOURNAL_ROW_KIND_PLATFORM_AUTHORITY_MATRIX,
    APP_GAME_JOURNAL_ROW_KIND_RUNTIME, APP_GAME_RUNTIME_RUNNING, APP_GAME_SCHEMA_VERSION,
};
use rusqlite::{params, Connection, Row};

use crate::ActivityStoreError;

use super::super::app_game_session_daily_rollups;

struct StoredAppGameJournalRow {
    fields: LogFields,
}

pub(crate) fn app_game_journal_sqlite_read_model(
    connection: &Connection,
    limit: u64,
    generated_at: &str,
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
        daily_rollups: app_game_session_daily_rollups(connection, limit)?,
        evidence_claim_rows: Vec::new(),
        identity_rows: Vec::new(),
        approval_authority_rows: Vec::new(),
        approval_action_result_rows: Vec::new(),
        platform_authority_matrices: Vec::new(),
        ai_classifier_result_rows: Vec::new(),
    };
    let mut statement = connection.prepare(constants::sqlite::SELECT_POLICY_PREVIEW_ACTIVITY)?;
    let rows = statement.query_map(params![limit as i64], stored_row_from_sqlite)?;
    for row in rows {
        project_stored_row(
            row?,
            &mut model,
            &mut seen_runtime_processes,
            &mut seen_foreground_processes,
        )?;
    }
    refresh_returned_counts(&mut model);
    Ok(model)
}

fn stored_row_from_sqlite(row: &Row<'_>) -> rusqlite::Result<StoredAppGameJournalRow> {
    let fields_json: String = row.get(5)?;
    let fields = serde_json::from_str::<LogFields>(&fields_json).map_err(json_to_sqlite_error)?;
    Ok(StoredAppGameJournalRow { fields })
}

fn project_stored_row(
    row: StoredAppGameJournalRow,
    model: &mut AppGameServiceReadModel,
    seen_runtime_processes: &mut Vec<String>,
    seen_foreground_processes: &mut Vec<String>,
) -> Result<(), ActivityStoreError> {
    let Some(row_kind) = string_field(&row.fields, APP_GAME_JOURNAL_FIELD_ROW_KIND) else {
        return Ok(());
    };
    let Some(row_json) = string_field(&row.fields, APP_GAME_JOURNAL_FIELD_ROW_JSON) else {
        return Ok(());
    };
    if project_protocol_boundary_row(row_kind.as_str(), &row_json, model)? {
        return Ok(());
    }
    match row_kind.as_str() {
        APP_GAME_JOURNAL_ROW_KIND_INVENTORY => model.inventory_rows.push(serde_json::from_str::<
            AppGameInventoryEvidenceRow,
        >(&row_json)?),
        APP_GAME_JOURNAL_ROW_KIND_RUNTIME => {
            let runtime = serde_json::from_str::<AppGameRuntimeEvidenceRow>(&row_json)?;
            if !seen_runtime_processes
                .iter()
                .any(|candidate| candidate == &runtime.process_identity)
            {
                seen_runtime_processes.push(runtime.process_identity.clone());
            } else {
                return Ok(());
            }
            if runtime.runtime_state == APP_GAME_RUNTIME_RUNNING {
                model.running_now_rows.push(runtime);
            }
        }
        APP_GAME_JOURNAL_ROW_KIND_FOREGROUND => {
            let foreground = serde_json::from_str::<AppGameForegroundEvidenceRow>(&row_json)?;
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
        }
        APP_GAME_JOURNAL_ROW_KIND_LAUNCHER => model.launcher_rows.push(serde_json::from_str::<
            AppGameLauncherEvidenceRow,
        >(&row_json)?),
        _ => {}
    }
    Ok(())
}

fn project_protocol_boundary_row(
    row_kind: &str,
    row_json: &str,
    model: &mut AppGameServiceReadModel,
) -> Result<bool, ActivityStoreError> {
    match row_kind {
        APP_GAME_JOURNAL_ROW_KIND_EVIDENCE_CLAIM => model
            .evidence_claim_rows
            .push(serde_json::from_str::<AppGameEvidenceClaim>(row_json)?),
        APP_GAME_JOURNAL_ROW_KIND_IDENTITY => model
            .identity_rows
            .push(serde_json::from_str::<AppGameIdentity>(row_json)?),
        APP_GAME_JOURNAL_ROW_KIND_APPROVAL_AUTHORITY => {
            model.approval_authority_rows.push(serde_json::from_str::<
                AppGameControlApprovalAuthority,
            >(row_json)?)
        }
        APP_GAME_JOURNAL_ROW_KIND_APPROVAL_ACTION_RESULT => {
            model
                .approval_action_result_rows
                .push(serde_json::from_str::<AppGameControlActionResult>(
                    row_json,
                )?)
        }
        APP_GAME_JOURNAL_ROW_KIND_PLATFORM_AUTHORITY_MATRIX => model
            .platform_authority_matrices
            .push(serde_json::from_str::<AppGamePlatformAuthorityMatrix>(
                row_json,
            )?),
        APP_GAME_JOURNAL_ROW_KIND_AI_CLASSIFIER_RESULT => model
            .ai_classifier_result_rows
            .push(serde_json::from_str::<AppGameAiClassifierResult>(row_json)?),
        _ => return Ok(false),
    }
    Ok(true)
}

fn refresh_returned_counts(model: &mut AppGameServiceReadModel) {
    model.inventory_returned = model.inventory_rows.len() as u64;
    model.running_now_returned = model.running_now_rows.len() as u64;
    model.foreground_now_returned = model.foreground_now_rows.len() as u64;
    model.launcher_returned = model.launcher_rows.len() as u64;
    model.daily_rollup_returned = model.daily_rollups.len() as u64;
    model.evidence_claim_returned = model.evidence_claim_rows.len() as u64;
    model.identity_returned = model.identity_rows.len() as u64;
    model.approval_authority_returned = model.approval_authority_rows.len() as u64;
    model.approval_action_result_returned = model.approval_action_result_rows.len() as u64;
    model.platform_authority_matrix_returned = model.platform_authority_matrices.len() as u64;
    model.ai_classifier_result_returned = model.ai_classifier_result_rows.len() as u64;
    model.capability_status = model_capability_status(model);
}

fn model_capability_status(model: &AppGameServiceReadModel) -> String {
    for status in [
        APP_GAME_CAPABILITY_STATUS_ADAPTER_ERROR,
        APP_GAME_CAPABILITY_STATUS_PERMISSION_LIMITED,
        APP_GAME_CAPABILITY_STATUS_DEGRADED,
        APP_GAME_CAPABILITY_STATUS_STALE,
        APP_GAME_CAPABILITY_STATUS_UNAVAILABLE,
        APP_GAME_CAPABILITY_STATUS_UNSUPPORTED_PLATFORM,
        APP_GAME_CAPABILITY_STATUS_MANUAL_REQUIRED,
        APP_GAME_CAPABILITY_STATUS_NOT_CLAIMED,
    ] {
        if has_capability_status(model, status) {
            return status.to_string();
        }
    }
    if model.inventory_returned
        + model.running_now_returned
        + model.foreground_now_returned
        + model.launcher_returned
        + model.daily_rollup_returned
        > 0
    {
        return APP_GAME_CAPABILITY_STATUS_AVAILABLE.to_string();
    }
    APP_GAME_CAPABILITY_STATUS_NOT_CLAIMED.to_string()
}

fn has_capability_status(model: &AppGameServiceReadModel, status: &str) -> bool {
    model
        .inventory_rows
        .iter()
        .any(|row| row.capability_status == status)
        || model
            .running_now_rows
            .iter()
            .any(|row| row.capability_status == status)
        || model
            .foreground_now_rows
            .iter()
            .any(|row| row.capability_status == status)
        || model
            .launcher_rows
            .iter()
            .any(|row| row.capability_status == status)
}

fn string_field(fields: &LogFields, key: &str) -> Option<String> {
    match fields.get(key) {
        Some(LogFieldValue::String(value)) => Some(value.clone()),
        _ => None,
    }
}

fn json_to_sqlite_error(error: serde_json::Error) -> rusqlite::Error {
    rusqlite::Error::ToSqlConversionFailure(Box::new(error))
}
