use ocentra_parent_agent_protocol::app_game::{
    AppGameSessionSummary, APP_GAME_CATALOG_NOT_LOADED, APP_GAME_OBSERVATION_MODE_PROCESS_EXIT,
    APP_GAME_SCHEMA_VERSION, APP_GAME_SESSION_END_REASON_PROCESS_EXIT, APP_GAME_SESSION_ID_PREFIX,
};

use super::AppGameObservation;

pub(crate) fn summary_from_observation(observation: AppGameObservation) -> AppGameSessionSummary {
    let end_reason = if observation.observation_mode.as_deref()
        == Some(APP_GAME_OBSERVATION_MODE_PROCESS_EXIT)
    {
        Some(APP_GAME_SESSION_END_REASON_PROCESS_EXIT.to_string())
    } else {
        None
    };
    AppGameSessionSummary {
        schema_version: APP_GAME_SCHEMA_VERSION,
        session_id: session_id(&observation.process_identity),
        primary_process_identity: observation.process_identity,
        display_name: observation.display_name,
        classification_state: observation.classification_state,
        catalog_ready_state: APP_GAME_CATALOG_NOT_LOADED.to_string(),
        inventory_entry_id: None,
        launcher_ref: None,
        catalog_ref: None,
        started_at: observation.observed_at.clone(),
        last_observed_at: observation.observed_at,
        ended_at: None,
        end_reason,
        running_duration_ms: 0,
        foreground_duration_ms: 0,
        background_duration_ms: 0,
        last_foreground_at: None,
        last_background_at: None,
        observation_gap_ms: 0,
        observation_count: 1,
        evidence_count: observation.evidence.len() as u64,
        evidence: observation.evidence,
        ai_digest_ref: None,
        confidence: observation.confidence,
    }
}

fn session_id(process_identity: &str) -> String {
    let mut session_id = APP_GAME_SESSION_ID_PREFIX.to_string();
    session_id.push_str(process_identity);
    session_id
}
