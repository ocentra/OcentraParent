use super::{
    constants, AppGameSessionReport, AppGameSessionSummary, APP_GAME_CATALOG_NOT_LOADED,
    APP_GAME_CLASSIFICATION_POSSIBLY_GAME, APP_GAME_CONFIDENCE_FOREGROUND_CANDIDATE,
    APP_GAME_SCHEMA_VERSION,
};

#[test]
fn app_game_session_summary_serializes_to_contract_shape() {
    let summary = AppGameSessionSummary {
        schema_version: APP_GAME_SCHEMA_VERSION,
        session_id: constants::activity_store::TEST_APP_GAME_SESSION_ID.to_string(),
        primary_process_identity: constants::activity_store::TEST_PROCESS_SUBJECT_ID.to_string(),
        display_name: constants::activity_store::TEST_APP_GAME_PROCESS_NAME.to_string(),
        classification_state: APP_GAME_CLASSIFICATION_POSSIBLY_GAME.to_string(),
        catalog_ready_state: APP_GAME_CATALOG_NOT_LOADED.to_string(),
        inventory_entry_id: None,
        launcher_ref: None,
        catalog_ref: None,
        started_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        last_observed_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        ended_at: None,
        running_duration_ms: 0,
        foreground_duration_ms: 0,
        background_duration_ms: 0,
        observation_count: 2,
        evidence_count: 1,
        evidence: Vec::new(),
        ai_digest_ref: None,
        confidence: APP_GAME_CONFIDENCE_FOREGROUND_CANDIDATE,
    };

    let serialized = serde_json::to_value(summary).expect(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(serialized["schemaVersion"], APP_GAME_SCHEMA_VERSION);
    assert_eq!(
        serialized["sessionId"],
        constants::activity_store::TEST_APP_GAME_SESSION_ID
    );
    assert_eq!(
        serialized["classificationState"],
        APP_GAME_CLASSIFICATION_POSSIBLY_GAME
    );
    assert_eq!(serialized["catalogReadyState"], APP_GAME_CATALOG_NOT_LOADED);
    assert!(serialized["inventoryEntryId"].is_null());
}

#[test]
fn app_game_session_report_serializes_flat_portal_visibility_shape() {
    let report = AppGameSessionReport {
        schema_version: APP_GAME_SCHEMA_VERSION,
        limit: constants::activity_store::DEFAULT_RECENT_LIMIT,
        returned: 1,
        catalog_ready_state: APP_GAME_CATALOG_NOT_LOADED.to_string(),
        first_observed_at: Some(constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string()),
        last_observed_at: Some(constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string()),
        most_recent_session_id: Some(
            constants::activity_store::TEST_APP_GAME_SESSION_ID.to_string(),
        ),
        most_recent_classification_state: Some(APP_GAME_CLASSIFICATION_POSSIBLY_GAME.to_string()),
        most_recent_process_identity: Some(
            constants::activity_store::TEST_PROCESS_SUBJECT_ID.to_string(),
        ),
        most_recent_display_name: Some(
            constants::activity_store::TEST_APP_GAME_PROCESS_NAME.to_string(),
        ),
        most_recent_running_duration_ms: Some(0),
        most_recent_foreground_duration_ms: Some(0),
        most_recent_evidence_count: Some(1),
    };

    let serialized = serde_json::to_value(report).expect(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(serialized["returned"], 1);
    assert_eq!(
        serialized["mostRecentSessionId"],
        constants::activity_store::TEST_APP_GAME_SESSION_ID
    );
    assert_eq!(
        serialized["mostRecentClassificationState"],
        APP_GAME_CLASSIFICATION_POSSIBLY_GAME
    );
    assert_eq!(serialized["mostRecentEvidenceCount"], 1);
}
