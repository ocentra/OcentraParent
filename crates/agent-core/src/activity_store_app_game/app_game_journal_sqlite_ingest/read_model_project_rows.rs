#[path = "read_model_project_rows_foreground.rs"]
mod read_model_project_rows_foreground;
#[path = "read_model_project_rows_runtime.rs"]
mod read_model_project_rows_runtime;

use ocentra_parent_agent_protocol::app_game::{
    AppGameInventoryEvidenceRow, AppGameLauncherEvidenceRow, APP_GAME_JOURNAL_FIELD_ROW_JSON,
    APP_GAME_JOURNAL_FIELD_ROW_KIND, APP_GAME_JOURNAL_ROW_KIND_FOREGROUND,
    APP_GAME_JOURNAL_ROW_KIND_INVENTORY, APP_GAME_JOURNAL_ROW_KIND_LAUNCHER,
    APP_GAME_JOURNAL_ROW_KIND_RUNTIME,
};
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};

use crate::ActivityStoreError;

use super::read_model_project_boundary::project_protocol_boundary_row;

pub(crate) struct StoredAppGameJournalRow {
    pub(crate) fields: LogFields,
}

pub(crate) fn project_stored_row(
    row: &StoredAppGameJournalRow,
    model: &mut ocentra_parent_agent_protocol::app_game::AppGameServiceReadModel,
    seen_runtime_processes: &mut Vec<String>,
    seen_foreground_processes: &mut Vec<String>,
) -> Result<(), ActivityStoreError> {
    let row_kind = required_string_field(
        &row.fields,
        APP_GAME_JOURNAL_FIELD_ROW_KIND,
        "missing-row-kind",
    )?;
    let row_json = required_string_field(
        &row.fields,
        APP_GAME_JOURNAL_FIELD_ROW_JSON,
        "missing-row-json",
    )?;
    if project_protocol_boundary_row(row_kind.as_str(), &row_json, model)? {
        return Ok(());
    }
    match row_kind.as_str() {
        APP_GAME_JOURNAL_ROW_KIND_INVENTORY => project_inventory_row(model, &row_json)?,
        APP_GAME_JOURNAL_ROW_KIND_RUNTIME => read_model_project_rows_runtime::project_runtime_row(
            model,
            &row_json,
            seen_runtime_processes,
        )?,
        APP_GAME_JOURNAL_ROW_KIND_FOREGROUND => {
            read_model_project_rows_foreground::project_foreground_row(
                model,
                &row_json,
                seen_foreground_processes,
            )?
        }
        APP_GAME_JOURNAL_ROW_KIND_LAUNCHER => project_launcher_row(model, &row_json)?,
        _ => {
            return Err(ActivityStoreError::InvalidAppGameJournalRow {
                reason: "unknown-row-kind",
            })
        }
    }
    Ok(())
}

fn project_inventory_row(
    model: &mut ocentra_parent_agent_protocol::app_game::AppGameServiceReadModel,
    row_json: &str,
) -> Result<(), ActivityStoreError> {
    let inventory =
        serde_json::from_str::<AppGameInventoryEvidenceRow>(row_json).map_err(|_error| {
            ActivityStoreError::InvalidAppGameJournalRow {
                reason: "invalid-inventory-row",
            }
        })?;
    super::super::super::app_game_journal_sqlite_ingest_validation::validate_inventory_row(
        &inventory,
    )
    .map_err(|_error| ActivityStoreError::InvalidAppGameJournalRow {
        reason: "invalid-inventory-row",
    })?;
    model.inventory_rows.push(inventory);
    Ok(())
}

fn project_launcher_row(
    model: &mut ocentra_parent_agent_protocol::app_game::AppGameServiceReadModel,
    row_json: &str,
) -> Result<(), ActivityStoreError> {
    let launcher =
        serde_json::from_str::<AppGameLauncherEvidenceRow>(row_json).map_err(|_error| {
            ActivityStoreError::InvalidAppGameJournalRow {
                reason: "invalid-launcher-row",
            }
        })?;
    super::super::super::app_game_journal_sqlite_ingest_launcher_validation::validate_launcher_row(
        &launcher,
    )
    .map_err(|_error| ActivityStoreError::InvalidAppGameJournalRow {
        reason: "invalid-launcher-row",
    })?;
    model.launcher_rows.push(launcher);
    Ok(())
}

fn required_string_field(
    fields: &LogFields,
    key: &str,
    missing_reason: &'static str,
) -> Result<String, ActivityStoreError> {
    string_field(fields, key).ok_or(ActivityStoreError::InvalidAppGameJournalRow {
        reason: missing_reason,
    })
}

fn string_field(fields: &LogFields, key: &str) -> Option<String> {
    match fields.get(key) {
        Some(LogFieldValue::String(value)) => Some(value.clone()),
        _ => None,
    }
}
