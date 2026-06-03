use super::{
    constants, AppGameInventoryCategoryCandidate, AppGameInventoryEvidenceRow,
    AppGameSessionReport, AppGameSessionSummary, APP_GAME_CAPABILITY_STATUS_AVAILABLE,
    APP_GAME_CATALOG_NOT_LOADED, APP_GAME_CATALOG_READY, APP_GAME_CLASSIFICATION_KNOWN_GAME,
    APP_GAME_CLASSIFICATION_POSSIBLY_GAME, APP_GAME_CONFIDENCE_FOREGROUND_CANDIDATE,
    APP_GAME_FOREGROUND_NOT_CLAIMED, APP_GAME_INVENTORY_CATEGORY_GAME,
    APP_GAME_INVENTORY_CUSTODY_LAUNCHER_MANIFEST, APP_GAME_INVENTORY_SOURCE_LAUNCHER_MANIFEST,
    APP_GAME_INVENTORY_STATE_INSTALLED, APP_GAME_PRODUCT_NATIVE_GAME, APP_GAME_RUNTIME_NOT_CLAIMED,
    APP_GAME_SCHEMA_VERSION, APP_GAME_TEST_CATALOG_REF, APP_GAME_TEST_DISPLAY_LABEL,
    APP_GAME_TEST_EXECUTABLE_PATH_REF, APP_GAME_TEST_LAUNCHER_APP_ID,
    APP_GAME_TEST_LAUNCHER_MANIFEST_ID, APP_GAME_TEST_LAUNCHER_REF,
    APP_GAME_TEST_LAUNCHER_SOURCE_REF, APP_GAME_TEST_STORE_ID,
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

#[test]
fn app_game_inventory_row_serializes_to_typescript_contract_shape() {
    let row = launcher_inventory_row();

    let serialized = serde_json::to_value(row).expect(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(serialized["schemaVersion"], APP_GAME_SCHEMA_VERSION);
    assert_eq!(
        serialized["sourceKind"],
        APP_GAME_INVENTORY_SOURCE_LAUNCHER_MANIFEST
    );
    assert_eq!(
        serialized["custodyState"],
        APP_GAME_INVENTORY_CUSTODY_LAUNCHER_MANIFEST
    );
    assert_eq!(serialized["productKind"], APP_GAME_PRODUCT_NATIVE_GAME);
    assert_eq!(serialized["displayLabel"], APP_GAME_TEST_DISPLAY_LABEL);
    assert_eq!(
        serialized["classificationState"],
        APP_GAME_CLASSIFICATION_KNOWN_GAME
    );
    assert_eq!(serialized["catalogReadyState"], APP_GAME_CATALOG_READY);
    assert_eq!(
        serialized["capabilityStatus"],
        APP_GAME_CAPABILITY_STATUS_AVAILABLE
    );
    assert_eq!(
        serialized["categoryCandidates"][0]["categoryKind"],
        APP_GAME_INVENTORY_CATEGORY_GAME
    );
    assert_eq!(
        serialized["categoryCandidates"][0]["catalogRef"],
        APP_GAME_TEST_CATALOG_REF
    );
}

#[test]
fn app_game_inventory_row_preserves_no_use_claims() {
    let row = launcher_inventory_row();

    assert_eq!(row.runtime_state, APP_GAME_RUNTIME_NOT_CLAIMED);
    assert_eq!(row.foreground_state, APP_GAME_FOREGROUND_NOT_CLAIMED);
    assert_eq!(row.running_duration_ms, 0);
    assert_eq!(row.foreground_duration_ms, 0);
}

fn launcher_inventory_row() -> AppGameInventoryEvidenceRow {
    AppGameInventoryEvidenceRow {
        schema_version: APP_GAME_SCHEMA_VERSION,
        inventory_entry_id: APP_GAME_TEST_LAUNCHER_SOURCE_REF.to_string(),
        observed_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        source_kind: APP_GAME_INVENTORY_SOURCE_LAUNCHER_MANIFEST.to_string(),
        source_ref: APP_GAME_TEST_LAUNCHER_SOURCE_REF.to_string(),
        custody_state: APP_GAME_INVENTORY_CUSTODY_LAUNCHER_MANIFEST.to_string(),
        product_kind: APP_GAME_PRODUCT_NATIVE_GAME.to_string(),
        display_label: APP_GAME_TEST_DISPLAY_LABEL.to_string(),
        identity_id: None,
        package_id: None,
        bundle_id: None,
        app_user_model_id: None,
        desktop_entry_id: None,
        executable_path_ref: Some(APP_GAME_TEST_EXECUTABLE_PATH_REF.to_string()),
        launcher_ref: Some(APP_GAME_TEST_LAUNCHER_REF.to_string()),
        launcher_app_id: Some(APP_GAME_TEST_LAUNCHER_APP_ID.to_string()),
        launcher_manifest_id: Some(APP_GAME_TEST_LAUNCHER_MANIFEST_ID.to_string()),
        store_id: Some(APP_GAME_TEST_STORE_ID.to_string()),
        catalog_ref: Some(APP_GAME_TEST_CATALOG_REF.to_string()),
        inventory_state: APP_GAME_INVENTORY_STATE_INSTALLED.to_string(),
        classification_state: APP_GAME_CLASSIFICATION_KNOWN_GAME.to_string(),
        catalog_ready_state: APP_GAME_CATALOG_READY.to_string(),
        capability_status: APP_GAME_CAPABILITY_STATUS_AVAILABLE.to_string(),
        confidence: 0.96,
        category_candidates: vec![AppGameInventoryCategoryCandidate {
            category_kind: APP_GAME_INVENTORY_CATEGORY_GAME.to_string(),
            confidence: 0.98,
            catalog_ref: Some(APP_GAME_TEST_CATALOG_REF.to_string()),
            evidence: Vec::new(),
        }],
        runtime_state: APP_GAME_RUNTIME_NOT_CLAIMED.to_string(),
        foreground_state: APP_GAME_FOREGROUND_NOT_CLAIMED.to_string(),
        running_duration_ms: 0,
        foreground_duration_ms: 0,
        evidence: Vec::new(),
    }
}
