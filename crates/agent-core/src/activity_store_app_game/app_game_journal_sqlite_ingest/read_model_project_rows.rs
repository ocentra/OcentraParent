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
        APP_GAME_JOURNAL_ROW_KIND_LAUNCHER => model.launcher_rows.push(serde_json::from_str::<
            AppGameLauncherEvidenceRow,
        >(&row_json)?),
        _ => {}
    }
    Ok(())
}

fn string_field(fields: &LogFields, key: &str) -> Option<String> {
    match fields.get(key) {
        Some(LogFieldValue::String(value)) => Some(value.clone()),
        _ => None,
    }
}
