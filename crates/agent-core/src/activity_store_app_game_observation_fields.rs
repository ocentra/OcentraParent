use ocentra_parent_agent_protocol::app_game::{
    APP_GAME_CLASSIFICATION_ADAPTER_ERROR, APP_GAME_CLASSIFICATION_PERMISSION_LIMITED,
    APP_GAME_CLASSIFICATION_POSSIBLY_GAME, APP_GAME_CLASSIFICATION_UNKNOWN_PROCESS,
    APP_GAME_CONFIDENCE_FOREGROUND_CANDIDATE, APP_GAME_CONFIDENCE_UNKNOWN,
    APP_GAME_JOURNAL_FIELD_CLASSIFICATION_STATE,
};
use ocentra_parent_agent_protocol::constants;

use crate::activity_store_app_game_rows::AppGameStoreRow;

#[path = "activity_store_app_game_observation_fields/helpers.rs"]
mod helpers;

use self::helpers::{boolean_field, number_field, process_identity_from_pid};

pub(crate) fn process_identity(row: &AppGameStoreRow) -> String {
    number_field(&row.fields, constants::field::PID)
        .map(process_identity_from_pid)
        .unwrap_or_else(|| row.subject_id.clone())
}

pub(crate) fn display_name(row: &AppGameStoreRow) -> String {
    helpers::string_field(&row.fields, constants::field::PROCESS_NAME)
        .or_else(|| helpers::string_field(&row.fields, constants::field::APP_NAME))
        .or_else(|| helpers::string_field(&row.fields, constants::field::WINDOW_TITLE))
        .or(row.subject_display_name.clone())
        .unwrap_or_else(|| row.event_id.clone())
}

pub(crate) fn classification_state(row: &AppGameStoreRow) -> String {
    if let Some(classification_state) =
        helpers::string_field(&row.fields, APP_GAME_JOURNAL_FIELD_CLASSIFICATION_STATE)
    {
        return classification_state;
    }

    match helpers::string_field(&row.fields, constants::field::CAPABILITY_STATUS).as_deref() {
        Some(constants::activity_capture::CAPABILITY_STATUS_ACCESS_DENIED) => {
            APP_GAME_CLASSIFICATION_PERMISSION_LIMITED.to_string()
        }
        Some(constants::activity_capture::CAPABILITY_STATUS_ADAPTER_ERROR) => {
            APP_GAME_CLASSIFICATION_ADAPTER_ERROR.to_string()
        }
        _ if row.kind == constants::activity_event_kind::WINDOW_FOCUSED
            && boolean_field(&row.fields, constants::field::FOREGROUND) == Some(true) =>
        {
            APP_GAME_CLASSIFICATION_POSSIBLY_GAME.to_string()
        }
        _ => APP_GAME_CLASSIFICATION_UNKNOWN_PROCESS.to_string(),
    }
}

pub(crate) fn confidence_for_classification(classification_state: &str) -> f64 {
    match classification_state {
        APP_GAME_CLASSIFICATION_POSSIBLY_GAME => APP_GAME_CONFIDENCE_FOREGROUND_CANDIDATE,
        _ => APP_GAME_CONFIDENCE_UNKNOWN,
    }
}

pub(crate) fn foreground_active(row: &AppGameStoreRow) -> bool {
    row.kind == constants::activity_event_kind::WINDOW_FOCUSED
        && boolean_field(&row.fields, constants::field::FOREGROUND) == Some(true)
}

pub(crate) fn string_field(
    fields: &ocentra_parent_agent_protocol::logging::LogFields,
    key: &str,
) -> Option<String> {
    helpers::string_field(fields, key)
}
