use serde::{Deserialize, Serialize};

use crate::ActivityEvidenceRef;

pub const APP_GAME_SCHEMA_VERSION: u16 = 1;
pub const APP_GAME_CLASSIFICATION_UNKNOWN_PROCESS: &str = "unknownProcess";
pub const APP_GAME_CLASSIFICATION_POSSIBLY_GAME: &str = "possiblyGame";
pub const APP_GAME_CLASSIFICATION_PERMISSION_LIMITED: &str = "permissionLimited";
pub const APP_GAME_CLASSIFICATION_ADAPTER_ERROR: &str = "adapterError";
pub const APP_GAME_CATALOG_NOT_LOADED: &str = "catalogNotLoaded";
pub const APP_GAME_SESSION_ID_PREFIX: &str = "app-game-session-";
pub const APP_GAME_CONFIDENCE_UNKNOWN: f64 = 0.0;
pub const APP_GAME_CONFIDENCE_FOREGROUND_CANDIDATE: f64 = 0.25;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameSessionSummary {
    pub schema_version: u16,
    pub session_id: String,
    pub primary_process_identity: String,
    pub display_name: String,
    pub classification_state: String,
    pub catalog_ready_state: String,
    pub inventory_entry_id: Option<String>,
    pub launcher_ref: Option<String>,
    pub catalog_ref: Option<String>,
    pub started_at: String,
    pub last_observed_at: String,
    pub ended_at: Option<String>,
    pub running_duration_ms: u64,
    pub foreground_duration_ms: u64,
    pub background_duration_ms: u64,
    pub observation_count: u64,
    pub evidence_count: u64,
    pub evidence: Vec<ActivityEvidenceRef>,
    pub ai_digest_ref: Option<String>,
    pub confidence: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameSessionReport {
    pub schema_version: u16,
    pub limit: u64,
    pub returned: u64,
    pub catalog_ready_state: String,
    pub first_observed_at: Option<String>,
    pub last_observed_at: Option<String>,
    pub most_recent_session_id: Option<String>,
    pub most_recent_classification_state: Option<String>,
    pub most_recent_process_identity: Option<String>,
    pub most_recent_display_name: Option<String>,
    pub most_recent_running_duration_ms: Option<u64>,
    pub most_recent_foreground_duration_ms: Option<u64>,
    pub most_recent_evidence_count: Option<u64>,
}
