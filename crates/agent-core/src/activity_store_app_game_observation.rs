use ocentra_parent_agent_protocol::{
    constants, ActivityEvidenceRef, AppGameSessionSummary, LogFieldValue, LogFields,
    APP_GAME_CATALOG_NOT_LOADED, APP_GAME_CLASSIFICATION_ADAPTER_ERROR,
    APP_GAME_CLASSIFICATION_PERMISSION_LIMITED, APP_GAME_CLASSIFICATION_POSSIBLY_GAME,
    APP_GAME_CLASSIFICATION_UNKNOWN_PROCESS, APP_GAME_CONFIDENCE_FOREGROUND_CANDIDATE,
    APP_GAME_CONFIDENCE_UNKNOWN, APP_GAME_SCHEMA_VERSION, APP_GAME_SESSION_END_REASON_PROCESS_EXIT,
    APP_GAME_SESSION_ID_PREFIX,
};

use crate::activity_store_app_game_rows::AppGameStoreRow;

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
        let foreground_active = row.kind == constants::activity_event_kind::WINDOW_FOCUSED
            && boolean_field(&row.fields, constants::field::FOREGROUND) == Some(true);
        let observation_mode = string_field(&row.fields, constants::field::OBSERVATION_MODE);
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
        let end_reason = if self.is_process_exit() {
            Some(APP_GAME_SESSION_END_REASON_PROCESS_EXIT.to_string())
        } else {
            None
        };
        AppGameSessionSummary {
            schema_version: APP_GAME_SCHEMA_VERSION,
            session_id: session_id(&self.process_identity),
            primary_process_identity: self.process_identity,
            display_name: self.display_name,
            classification_state: self.classification_state,
            catalog_ready_state: APP_GAME_CATALOG_NOT_LOADED.to_string(),
            inventory_entry_id: None,
            launcher_ref: None,
            catalog_ref: None,
            started_at: self.observed_at.clone(),
            last_observed_at: self.observed_at,
            ended_at: None,
            end_reason,
            running_duration_ms: 0,
            foreground_duration_ms: 0,
            background_duration_ms: 0,
            last_foreground_at: None,
            last_background_at: None,
            observation_gap_ms: 0,
            observation_count: 1,
            evidence_count: self.evidence.len() as u64,
            evidence: self.evidence,
            ai_digest_ref: None,
            confidence: self.confidence,
        }
    }

    pub(crate) fn is_process_observation(&self) -> bool {
        self.kind == constants::activity_event_kind::PROCESS_OBSERVED
    }

    pub(crate) fn is_foreground_observation(&self) -> bool {
        self.kind == constants::activity_event_kind::WINDOW_FOCUSED
    }

    pub(crate) fn is_process_exit(&self) -> bool {
        self.observation_mode.as_deref()
            == Some(ocentra_parent_agent_protocol::APP_GAME_OBSERVATION_MODE_PROCESS_EXIT)
    }
}

fn process_identity(row: &AppGameStoreRow) -> String {
    number_field(&row.fields, constants::field::PID)
        .map(process_identity_from_pid)
        .unwrap_or_else(|| row.subject_id.clone())
}

fn process_identity_from_pid(pid: u64) -> String {
    let mut identity = String::from(constants::activity_capture::PROCESS_SUBJECT_ID_PREFIX);
    identity.push_str(&pid.to_string());
    identity
}

fn session_id(process_identity: &str) -> String {
    let mut session_id = String::from(APP_GAME_SESSION_ID_PREFIX);
    session_id.push_str(process_identity);
    session_id
}

fn display_name(row: &AppGameStoreRow) -> String {
    string_field(&row.fields, constants::field::PROCESS_NAME)
        .or_else(|| string_field(&row.fields, constants::field::APP_NAME))
        .or_else(|| string_field(&row.fields, constants::field::WINDOW_TITLE))
        .or(row.subject_display_name.clone())
        .unwrap_or_else(|| row.event_id.clone())
}

fn classification_state(row: &AppGameStoreRow) -> String {
    match string_field(&row.fields, constants::field::CAPABILITY_STATUS).as_deref() {
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

fn confidence_for_classification(classification_state: &str) -> f64 {
    match classification_state {
        APP_GAME_CLASSIFICATION_POSSIBLY_GAME => APP_GAME_CONFIDENCE_FOREGROUND_CANDIDATE,
        _ => APP_GAME_CONFIDENCE_UNKNOWN,
    }
}

fn string_field(fields: &LogFields, key: &str) -> Option<String> {
    match fields.get(key) {
        Some(LogFieldValue::String(value)) => Some(value.clone()),
        _ => None,
    }
}

fn number_field(fields: &LogFields, key: &str) -> Option<u64> {
    match fields.get(key) {
        Some(LogFieldValue::Number(value)) if value.is_finite() && *value >= 0.0 => {
            Some(*value as u64)
        }
        _ => None,
    }
}

fn boolean_field(fields: &LogFields, key: &str) -> Option<bool> {
    match fields.get(key) {
        Some(LogFieldValue::Boolean(value)) => Some(*value),
        _ => None,
    }
}
