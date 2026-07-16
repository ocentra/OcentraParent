use super::constants;
use crate::app_game::*;
use ocentra_eventing::expect_value::ExpectValue;

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
        ended_at: Some(constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string()),
        end_reason: Some(APP_GAME_SESSION_END_REASON_PROCESS_EXIT.to_string()),
        running_duration_ms: 60000,
        foreground_duration_ms: 30000,
        background_duration_ms: 30000,
        last_foreground_at: Some(constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string()),
        last_background_at: Some(constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string()),
        observation_gap_ms: 60000,
        observation_count: 2,
        evidence_count: 1,
        evidence: Vec::new(),
        ai_digest_ref: None,
        confidence: APP_GAME_CONFIDENCE_FOREGROUND_CANDIDATE,
    };

    let serialized =
        serde_json::to_value(summary).expect_value(constants::error::AGENT_EVENT_SERIALIZES);

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
    assert_eq!(
        serialized["endReason"],
        APP_GAME_SESSION_END_REASON_PROCESS_EXIT
    );
    assert_eq!(serialized["observationGapMs"], 60000);
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

    let serialized =
        serde_json::to_value(report).expect_value(constants::error::AGENT_EVENT_SERIALIZES);

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
fn app_game_service_read_model_serializes_replayed_row_groups_for_service_events() {
    let serialized = serde_json::to_value(replayed_row_group_read_model())
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(serialized["schemaVersion"], APP_GAME_SCHEMA_VERSION);
    assert_eq!(
        serialized["custodyLabel"],
        APP_GAME_JOURNAL_CUSTODY_LOCAL_SQLITE
    );
    assert_eq!(
        serialized["replayState"],
        APP_GAME_JOURNAL_REPLAY_STATE_REPLAYED
    );
    assert_eq!(serialized["inventoryReturned"], 1);
    assert_eq!(serialized["runningNowReturned"], 1);
    assert_eq!(serialized["foregroundNowReturned"], 1);
    assert_eq!(serialized["launcherReturned"], 1);
    assert_eq!(serialized["dailyRollupReturned"], 1);
    assert_eq!(serialized["evidenceClaimReturned"], 0);
    assert_eq!(serialized["identityReturned"], 0);
    assert_eq!(serialized["approvalAuthorityReturned"], 0);
    assert_eq!(serialized["approvalActionResultReturned"], 0);
    assert_eq!(serialized["platformAuthorityMatrixReturned"], 0);
    assert_eq!(serialized["aiClassifierResultReturned"], 0);
    assert_eq!(
        serialized["inventoryRows"][0]["runtimeState"],
        APP_GAME_RUNTIME_NOT_CLAIMED
    );
    assert_eq!(
        serialized["runningNowRows"][0]["foregroundState"],
        APP_GAME_FOREGROUND_NOT_CLAIMED
    );
    assert_eq!(
        serialized["foregroundNowRows"][0]["contentKnowledgeState"],
        APP_GAME_CONTENT_KNOWLEDGE_NOT_CLAIMED
    );
    assert_eq!(
        serialized["launcherRows"][0]["gameProofState"],
        APP_GAME_LAUNCHER_PROOF_LAUNCHER_ONLY
    );
}

fn replayed_row_group_read_model() -> AppGameServiceReadModel {
    AppGameServiceReadModel {
        schema_version: APP_GAME_SCHEMA_VERSION,
        generated_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        limit: constants::activity_store::DEFAULT_RECENT_LIMIT,
        custody_label: APP_GAME_JOURNAL_CUSTODY_LOCAL_SQLITE.to_string(),
        replay_state: APP_GAME_JOURNAL_REPLAY_STATE_REPLAYED.to_string(),
        capability_status: APP_GAME_CAPABILITY_STATUS_AVAILABLE.to_string(),
        inventory_returned: 1,
        running_now_returned: 1,
        foreground_now_returned: 1,
        launcher_returned: 1,
        daily_rollup_returned: 1,
        evidence_claim_returned: 0,
        identity_returned: 0,
        approval_authority_returned: 0,
        approval_action_result_returned: 0,
        platform_authority_matrix_returned: 0,
        ai_classifier_result_returned: 0,
        inventory_rows: vec![launcher_inventory_row()],
        running_now_rows: vec![runtime_evidence_row()],
        foreground_now_rows: vec![foreground_evidence_row()],
        launcher_rows: vec![launcher_evidence_row()],
        daily_rollups: vec![known_app_daily_rollup()],
        evidence_claim_rows: Vec::new(),
        identity_rows: Vec::new(),
        approval_authority_rows: Vec::new(),
        approval_action_result_rows: Vec::new(),
        platform_authority_matrices: Vec::new(),
        ai_classifier_result_rows: Vec::new(),
    }
}

fn known_app_daily_rollup() -> AppGameSessionDailyRollup {
    AppGameSessionDailyRollup {
        schema_version: APP_GAME_SCHEMA_VERSION,
        rollup_date: "2026-05-20".to_string(),
        classification_state: APP_GAME_CLASSIFICATION_KNOWN_APP.to_string(),
        session_count: 1,
        running_duration_ms: 60000,
        foreground_duration_ms: 60000,
        background_duration_ms: 0,
        evidence_count: 1,
        session_ids: vec![constants::activity_store::TEST_APP_GAME_SESSION_ID.to_string()],
        evidence: Vec::new(),
    }
}

#[test]
fn app_game_daily_rollup_serializes_duration_totals() {
    let rollup = AppGameSessionDailyRollup {
        schema_version: APP_GAME_SCHEMA_VERSION,
        rollup_date: "2026-05-20".to_string(),
        classification_state: APP_GAME_CLASSIFICATION_POSSIBLY_GAME.to_string(),
        session_count: 1,
        running_duration_ms: 60000,
        foreground_duration_ms: 30000,
        background_duration_ms: 30000,
        evidence_count: 1,
        session_ids: vec![constants::activity_store::TEST_APP_GAME_SESSION_ID.to_string()],
        evidence: Vec::new(),
    };

    let serialized =
        serde_json::to_value(rollup).expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(serialized["schemaVersion"], APP_GAME_SCHEMA_VERSION);
    assert_eq!(serialized["rollupDate"], "2026-05-20");
    assert_eq!(
        serialized["classificationState"],
        APP_GAME_CLASSIFICATION_POSSIBLY_GAME
    );
    assert_eq!(serialized["runningDurationMs"], 60000);
    assert_eq!(serialized["foregroundDurationMs"], 30000);
    assert_eq!(serialized["backgroundDurationMs"], 30000);
}

#[test]
fn app_game_runtime_evidence_row_serializes_process_runtime_without_foreground_claim() {
    let row = runtime_evidence_row();

    let serialized =
        serde_json::to_value(row).expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(serialized["schemaVersion"], APP_GAME_SCHEMA_VERSION);
    assert_eq!(
        serialized["runtimeEvidenceId"],
        APP_GAME_TEST_RUNTIME_EVIDENCE_ID
    );
    assert_eq!(
        serialized["processIdentity"],
        APP_GAME_TEST_PROCESS_IDENTITY
    );
    assert_eq!(serialized["processId"], APP_GAME_TEST_PROCESS_ID);
    assert_eq!(
        serialized["parentProcessId"],
        APP_GAME_TEST_PARENT_PROCESS_ID
    );
    assert_eq!(
        serialized["publisherSignatureRef"],
        APP_GAME_TEST_PUBLISHER_SIGNATURE_REF
    );
    assert_eq!(serialized["fileHashRef"], APP_GAME_TEST_FILE_HASH_REF);
    assert_eq!(serialized["runtimeState"], APP_GAME_RUNTIME_RUNNING);
    assert_eq!(
        serialized["foregroundState"],
        APP_GAME_FOREGROUND_NOT_CLAIMED
    );
    assert_eq!(
        serialized["observationMode"],
        APP_GAME_OBSERVATION_MODE_PROCESS_SNAPSHOT
    );
}

#[test]
fn app_game_process_observation_serializes_schema_domain_mirror_shape() {
    let observation = AppGameProcessObservation {
        schema_version: APP_GAME_SCHEMA_VERSION,
        observed_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        process_identity: constants::activity_store::TEST_PROCESS_SUBJECT_ID.to_string(),
        process_id: APP_GAME_TEST_PROCESS_ID,
        process_name: constants::activity_store::TEST_APP_GAME_PROCESS_NAME.to_string(),
        executable_path: Some(constants::activity_store::TEST_APP_GAME_PROCESS_PATH.to_string()),
        foreground_state: APP_GAME_FOREGROUND_FOREGROUND.to_string(),
        observation_mode: APP_GAME_OBSERVATION_MODE_PROCESS_SNAPSHOT.to_string(),
        classification_state: APP_GAME_CLASSIFICATION_KNOWN_GAME.to_string(),
        inventory_entry_id: Some(APP_GAME_TEST_REGISTRY_SOURCE_REF.to_string()),
        launcher_ref: Some(APP_GAME_TEST_LAUNCHER_REF.to_string()),
        catalog_ref: Some(APP_GAME_TEST_CATALOG_REF.to_string()),
        confidence: APP_GAME_CONFIDENCE_FOREGROUND_CANDIDATE,
        evidence: Vec::new(),
    };

    let serialized =
        serde_json::to_value(observation).expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(serialized["schemaVersion"], APP_GAME_SCHEMA_VERSION);
    assert_eq!(
        serialized["observedAt"],
        constants::activity_store::TEST_SECOND_OBSERVED_AT
    );
    assert_eq!(
        serialized["processIdentity"],
        constants::activity_store::TEST_PROCESS_SUBJECT_ID
    );
    assert_eq!(serialized["processId"], APP_GAME_TEST_PROCESS_ID);
    assert_eq!(
        serialized["processName"],
        constants::activity_store::TEST_APP_GAME_PROCESS_NAME
    );
    assert_eq!(
        serialized["executablePath"],
        constants::activity_store::TEST_APP_GAME_PROCESS_PATH
    );
    assert_eq!(
        serialized["foregroundState"],
        APP_GAME_FOREGROUND_FOREGROUND
    );
    assert_eq!(
        serialized["observationMode"],
        APP_GAME_OBSERVATION_MODE_PROCESS_SNAPSHOT
    );
    assert_eq!(
        serialized["classificationState"],
        APP_GAME_CLASSIFICATION_KNOWN_GAME
    );
    assert_eq!(
        serialized["inventoryEntryId"],
        APP_GAME_TEST_REGISTRY_SOURCE_REF
    );
    assert_eq!(serialized["launcherRef"], APP_GAME_TEST_LAUNCHER_REF);
    assert_eq!(serialized["catalogRef"], APP_GAME_TEST_CATALOG_REF);
    assert_eq!(
        serialized["confidence"],
        APP_GAME_CONFIDENCE_FOREGROUND_CANDIDATE
    );
}

#[test]
fn app_game_foreground_evidence_row_serializes_focus_without_content_claim() {
    let row = foreground_evidence_row();

    let serialized =
        serde_json::to_value(row).expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(serialized["schemaVersion"], APP_GAME_SCHEMA_VERSION);
    assert_eq!(
        serialized["foregroundEvidenceId"],
        APP_GAME_TEST_FOREGROUND_EVIDENCE_ID
    );
    assert_eq!(
        serialized["processIdentity"],
        APP_GAME_TEST_PROCESS_IDENTITY
    );
    assert_eq!(
        serialized["observationMode"],
        APP_GAME_OBSERVATION_MODE_FOREGROUND_WINDOW
    );
    assert_eq!(serialized["windowRef"], APP_GAME_TEST_WINDOW_REF);
    assert_eq!(serialized["windowTitleRef"], APP_GAME_TEST_WINDOW_TITLE_REF);
    assert_eq!(
        serialized["titleCaptureState"],
        APP_GAME_TITLE_CAPTURE_TITLE_REF
    );
    assert_eq!(
        serialized["foregroundState"],
        APP_GAME_FOREGROUND_FOREGROUND
    );
    assert_eq!(
        serialized["contentKnowledgeState"],
        APP_GAME_CONTENT_KNOWLEDGE_NOT_CLAIMED
    );
}

#[test]
fn app_game_launcher_evidence_row_serializes_launcher_without_game_claim() {
    let row = launcher_evidence_row();

    let serialized =
        serde_json::to_value(row).expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(serialized["schemaVersion"], APP_GAME_SCHEMA_VERSION);
    assert_eq!(
        serialized["launcherEvidenceId"],
        APP_GAME_TEST_LAUNCHER_EVIDENCE_ID
    );
    assert_eq!(serialized["launcherKind"], APP_GAME_LAUNCHER_KIND_STEAM);
    assert_eq!(serialized["launcherRef"], APP_GAME_TEST_LAUNCHER_REF);
    assert_eq!(
        serialized["launcherProcessIdentity"],
        APP_GAME_TEST_LAUNCHER_PROCESS_IDENTITY
    );
    assert_eq!(
        serialized["launcherProcessName"],
        APP_GAME_TEST_LAUNCHER_PROCESS_NAME
    );
    assert_eq!(
        serialized["classificationState"],
        APP_GAME_CLASSIFICATION_KNOWN_LAUNCHER
    );
    assert_eq!(
        serialized["gameProofState"],
        APP_GAME_LAUNCHER_PROOF_LAUNCHER_ONLY
    );
    assert!(serialized["childGameEvidenceClaimId"].is_null());
}

#[test]
fn app_game_inventory_row_serializes_to_typescript_contract_shape() {
    let row = launcher_inventory_row();

    let serialized =
        serde_json::to_value(row).expect_value(constants::error::AGENT_EVENT_SERIALIZES);

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

#[test]
fn app_game_store_inventory_row_serializes_first_class_package_identity() {
    let row = store_game_inventory_row();

    let serialized =
        serde_json::to_value(row).expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        serialized["sourceKind"],
        APP_GAME_INVENTORY_SOURCE_STORE_PACKAGE
    );
    assert_eq!(
        serialized["custodyState"],
        APP_GAME_INVENTORY_CUSTODY_STORE_PACKAGE
    );
    assert_eq!(serialized["packageId"], APP_GAME_TEST_STORE_GAME_PACKAGE_ID);
    assert_eq!(serialized["bundleId"], APP_GAME_TEST_STORE_GAME_BUNDLE_ID);
    assert_eq!(
        serialized["appUserModelId"],
        APP_GAME_TEST_STORE_GAME_USER_MODEL_ID
    );
    assert_eq!(serialized["storeId"], APP_GAME_TEST_STORE_GAME_STORE_ID);
    assert_eq!(serialized["runtimeState"], APP_GAME_RUNTIME_NOT_CLAIMED);
    assert_eq!(
        serialized["foregroundState"],
        APP_GAME_FOREGROUND_NOT_CLAIMED
    );
}

#[test]
fn app_game_journal_projection_constants_name_replay_custody() {
    assert_eq!(APP_GAME_JOURNAL_FIELD_ROW_KIND, "appGameRowKind");
    assert_eq!(APP_GAME_JOURNAL_FIELD_ROW_JSON, "appGameRowJson");
    assert_eq!(
        APP_GAME_JOURNAL_FIELD_CLASSIFICATION_STATE,
        "appGameClassificationState"
    );
    assert_eq!(APP_GAME_JOURNAL_FIELD_CUSTODY_LABEL, "appGameCustodyLabel");
    assert_eq!(APP_GAME_JOURNAL_FIELD_REPLAY_STATE, "appGameReplayState");
    assert_eq!(APP_GAME_JOURNAL_ROW_KIND_INVENTORY, "inventory");
    assert_eq!(APP_GAME_JOURNAL_ROW_KIND_RUNTIME, "runtime");
    assert_eq!(APP_GAME_JOURNAL_ROW_KIND_FOREGROUND, "foreground");
    assert_eq!(APP_GAME_JOURNAL_ROW_KIND_LAUNCHER, "launcher");
    assert_eq!(APP_GAME_JOURNAL_CUSTODY_LOCAL_JOURNAL, "localJournal");
    assert_eq!(APP_GAME_JOURNAL_CUSTODY_LOCAL_SQLITE, "localSqlite");
    assert_eq!(APP_GAME_JOURNAL_REPLAY_STATE_STORED, "stored");
    assert_eq!(APP_GAME_JOURNAL_REPLAY_STATE_REPLAYED, "replayed");
}

fn runtime_evidence_row() -> AppGameRuntimeEvidenceRow {
    AppGameRuntimeEvidenceRow {
        schema_version: APP_GAME_SCHEMA_VERSION,
        runtime_evidence_id: APP_GAME_TEST_RUNTIME_EVIDENCE_ID.to_string(),
        observed_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        process_identity: APP_GAME_TEST_PROCESS_IDENTITY.to_string(),
        process_id: APP_GAME_TEST_PROCESS_ID,
        parent_process_id: Some(APP_GAME_TEST_PARENT_PROCESS_ID),
        process_name: APP_GAME_TEST_PROCESS_NAME.to_string(),
        executable_path_ref: Some(APP_GAME_TEST_EXECUTABLE_PATH_REF.to_string()),
        publisher_signature_ref: Some(APP_GAME_TEST_PUBLISHER_SIGNATURE_REF.to_string()),
        file_hash_ref: Some(APP_GAME_TEST_FILE_HASH_REF.to_string()),
        inventory_entry_id: Some(APP_GAME_TEST_REGISTRY_SOURCE_REF.to_string()),
        launcher_ref: None,
        catalog_ref: Some(APP_GAME_TEST_CATALOG_REF.to_string()),
        started_at: Some(constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string()),
        exited_at: None,
        running_duration_ms: 300000,
        runtime_state: APP_GAME_RUNTIME_RUNNING.to_string(),
        foreground_state: APP_GAME_FOREGROUND_NOT_CLAIMED.to_string(),
        observation_mode: APP_GAME_OBSERVATION_MODE_PROCESS_SNAPSHOT.to_string(),
        classification_state: APP_GAME_CLASSIFICATION_KNOWN_APP.to_string(),
        catalog_ready_state: APP_GAME_CATALOG_READY.to_string(),
        capability_status: APP_GAME_CAPABILITY_STATUS_AVAILABLE.to_string(),
        confidence: 0.82,
        evidence: Vec::new(),
    }
}

fn foreground_evidence_row() -> AppGameForegroundEvidenceRow {
    AppGameForegroundEvidenceRow {
        schema_version: APP_GAME_SCHEMA_VERSION,
        foreground_evidence_id: APP_GAME_TEST_FOREGROUND_EVIDENCE_ID.to_string(),
        observed_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        process_identity: APP_GAME_TEST_PROCESS_IDENTITY.to_string(),
        process_id: APP_GAME_TEST_PROCESS_ID,
        process_name: APP_GAME_TEST_PROCESS_NAME.to_string(),
        inventory_entry_id: Some(APP_GAME_TEST_REGISTRY_SOURCE_REF.to_string()),
        launcher_ref: None,
        catalog_ref: Some(APP_GAME_TEST_CATALOG_REF.to_string()),
        window_ref: Some(APP_GAME_TEST_WINDOW_REF.to_string()),
        window_title_ref: Some(APP_GAME_TEST_WINDOW_TITLE_REF.to_string()),
        title_capture_state: APP_GAME_TITLE_CAPTURE_TITLE_REF.to_string(),
        foreground_started_at: Some(constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string()),
        foreground_ended_at: None,
        foreground_duration_ms: 0,
        runtime_state: APP_GAME_RUNTIME_RUNNING.to_string(),
        foreground_state: APP_GAME_FOREGROUND_FOREGROUND.to_string(),
        observation_mode: APP_GAME_OBSERVATION_MODE_FOREGROUND_WINDOW.to_string(),
        classification_state: APP_GAME_CLASSIFICATION_KNOWN_APP.to_string(),
        catalog_ready_state: APP_GAME_CATALOG_READY.to_string(),
        capability_status: APP_GAME_CAPABILITY_STATUS_AVAILABLE.to_string(),
        content_knowledge_state: APP_GAME_CONTENT_KNOWLEDGE_NOT_CLAIMED.to_string(),
        confidence: 0.84,
        evidence: Vec::new(),
    }
}

fn launcher_evidence_row() -> AppGameLauncherEvidenceRow {
    AppGameLauncherEvidenceRow {
        schema_version: APP_GAME_SCHEMA_VERSION,
        launcher_evidence_id: APP_GAME_TEST_LAUNCHER_EVIDENCE_ID.to_string(),
        observed_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        launcher_kind: APP_GAME_LAUNCHER_KIND_STEAM.to_string(),
        launcher_ref: APP_GAME_TEST_LAUNCHER_REF.to_string(),
        launcher_inventory_entry_id: Some(APP_GAME_TEST_LAUNCHER_SOURCE_REF.to_string()),
        launcher_manifest_id: Some(APP_GAME_TEST_LAUNCHER_MANIFEST_ID.to_string()),
        launcher_app_id: None,
        launcher_process_identity: Some(APP_GAME_TEST_LAUNCHER_PROCESS_IDENTITY.to_string()),
        launcher_process_id: Some(APP_GAME_TEST_LAUNCHER_PROCESS_ID),
        launcher_process_name: Some(APP_GAME_TEST_LAUNCHER_PROCESS_NAME.to_string()),
        child_process_identity: None,
        child_inventory_entry_id: None,
        child_game_evidence_claim_id: None,
        catalog_ref: None,
        runtime_state: APP_GAME_RUNTIME_RUNNING.to_string(),
        foreground_state: APP_GAME_FOREGROUND_NOT_CLAIMED.to_string(),
        observation_mode: APP_GAME_OBSERVATION_MODE_PROCESS_SNAPSHOT.to_string(),
        classification_state: APP_GAME_CLASSIFICATION_KNOWN_LAUNCHER.to_string(),
        catalog_ready_state: APP_GAME_CATALOG_NOT_LOADED.to_string(),
        capability_status: APP_GAME_CAPABILITY_STATUS_AVAILABLE.to_string(),
        game_proof_state: APP_GAME_LAUNCHER_PROOF_LAUNCHER_ONLY.to_string(),
        confidence: 0.74,
        evidence: Vec::new(),
    }
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

fn store_game_inventory_row() -> AppGameInventoryEvidenceRow {
    AppGameInventoryEvidenceRow {
        schema_version: APP_GAME_SCHEMA_VERSION,
        inventory_entry_id: APP_GAME_TEST_STORE_GAME_SOURCE_REF.to_string(),
        observed_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        source_kind: APP_GAME_INVENTORY_SOURCE_STORE_PACKAGE.to_string(),
        source_ref: APP_GAME_TEST_STORE_GAME_SOURCE_REF.to_string(),
        custody_state: APP_GAME_INVENTORY_CUSTODY_STORE_PACKAGE.to_string(),
        product_kind: APP_GAME_PRODUCT_NATIVE_GAME.to_string(),
        display_label: APP_GAME_TEST_STORE_GAME_DISPLAY_LABEL.to_string(),
        identity_id: None,
        package_id: Some(APP_GAME_TEST_STORE_GAME_PACKAGE_ID.to_string()),
        bundle_id: Some(APP_GAME_TEST_STORE_GAME_BUNDLE_ID.to_string()),
        app_user_model_id: Some(APP_GAME_TEST_STORE_GAME_USER_MODEL_ID.to_string()),
        desktop_entry_id: None,
        executable_path_ref: None,
        launcher_ref: None,
        launcher_app_id: None,
        launcher_manifest_id: None,
        store_id: Some(APP_GAME_TEST_STORE_GAME_STORE_ID.to_string()),
        catalog_ref: Some(APP_GAME_TEST_STORE_GAME_CATALOG_REF.to_string()),
        inventory_state: APP_GAME_INVENTORY_STATE_INSTALLED.to_string(),
        classification_state: APP_GAME_CLASSIFICATION_KNOWN_GAME.to_string(),
        catalog_ready_state: APP_GAME_CATALOG_READY.to_string(),
        capability_status: APP_GAME_CAPABILITY_STATUS_AVAILABLE.to_string(),
        confidence: 0.9,
        category_candidates: vec![AppGameInventoryCategoryCandidate {
            category_kind: APP_GAME_INVENTORY_CATEGORY_GAME.to_string(),
            confidence: 0.9,
            catalog_ref: Some(APP_GAME_TEST_STORE_GAME_CATALOG_REF.to_string()),
            evidence: Vec::new(),
        }],
        runtime_state: APP_GAME_RUNTIME_NOT_CLAIMED.to_string(),
        foreground_state: APP_GAME_FOREGROUND_NOT_CLAIMED.to_string(),
        running_duration_ms: 0,
        foreground_duration_ms: 0,
        evidence: Vec::new(),
    }
}
