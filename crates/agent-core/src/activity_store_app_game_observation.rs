use ocentra_parent_agent_protocol::activity::ActivityEvidenceRef;
use ocentra_parent_agent_protocol::app_game::{
    AppGameSessionSummary, APP_GAME_OBSERVATION_MODE_PROCESS_EXIT,
};

use crate::activity_store_app_game_rows::AppGameStoreRow;

#[path = "activity_store_app_game_observation_fields.rs"]
mod activity_store_app_game_observation_fields;
#[path = "activity_store_app_game_observation_summary.rs"]
mod activity_store_app_game_observation_summary;

use activity_store_app_game_observation_fields::{
    classification_state, confidence_for_classification, display_name, process_identity,
    string_field,
};
use activity_store_app_game_observation_summary::summary_from_observation;

#[derive(Clone)]
pub(crate) struct AppGameObservation {
    pub observed_at: String,
    pub process_identity: String,
    pub display_name: String,
    pub classification_state: String,
    pub evidence: Vec<ActivityEvidenceRef>,
    pub confidence: f64,
    pub kind: String,
    pub observation_mode: Option<String>,
    pub foreground_active: bool,
}

impl AppGameObservation {
    pub(crate) fn from_row(row: AppGameStoreRow) -> Self {
        let process_identity = process_identity(&row);
        let display_name = display_name(&row);
        let classification_state = classification_state(&row);
        let confidence = confidence_for_classification(&classification_state);
        let foreground_active = activity_store_app_game_observation_fields::foreground_active(&row);
        let observation_mode = string_field(
            &row.fields,
            ocentra_parent_agent_protocol::constants::field::OBSERVATION_MODE,
        );
        Self {
            observed_at: row.observed_at,
            process_identity,
            display_name,
            classification_state,
            evidence: row.evidence,
            confidence,
            kind: row.kind,
            observation_mode,
            foreground_active,
        }
    }

    pub(crate) fn into_summary(self) -> AppGameSessionSummary {
        summary_from_observation(self)
    }

    pub(crate) fn is_process_observation(&self) -> bool {
        self.kind == ocentra_parent_agent_protocol::constants::activity_event_kind::PROCESS_OBSERVED
    }

    pub(crate) fn is_foreground_observation(&self) -> bool {
        self.kind == ocentra_parent_agent_protocol::constants::activity_event_kind::WINDOW_FOCUSED
    }

    pub(crate) fn is_process_exit(&self) -> bool {
        self.observation_mode.as_deref() == Some(APP_GAME_OBSERVATION_MODE_PROCESS_EXIT)
    }
}
