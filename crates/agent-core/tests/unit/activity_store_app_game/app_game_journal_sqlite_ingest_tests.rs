use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::app_game::*;
use std::fs::remove_file;

use ocentra_parent_agent_protocol::activity::{
    ActivityEvent, ActivityEvidenceKind, ActivityEvidenceRef,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::journal::ActivityJournalLine;
use std::fmt::Display;
use std::path::{Path, PathBuf};

use crate::{
    activity_store::ActivityStore,
    journal::ActivityJournal,
    journal_crypto::{JournalKey, JOURNAL_KEY_BYTES},
};

use super::app_game_journal_sqlite_ingest::{
    app_game_foreground_journal_event, app_game_inventory_journal_event,
    app_game_launcher_journal_event, app_game_runtime_journal_event,
    read_model::app_game_journal_sqlite_read_model, AppGameJournalSqliteIngestError,
};

#[derive(Clone)]
struct TestPath(PathBuf);

impl AsRef<Path> for TestPath {
    fn as_ref(&self) -> &Path {
        self.0.as_ref()
    }
}

#[test]
fn journal_replay_produces_app_game_sqlite_read_model_rows() {
    let (store, lines) = append_and_replay(
        &journal_replay_events(),
        constants::journal::TEST_REPLAY_SUFFIX,
    );
    let model = app_game_journal_sqlite_read_model(
        store.connection_for_test(),
        constants::activity_store::DEFAULT_RECENT_LIMIT,
        constants::activity_store::TEST_SECOND_OBSERVED_AT,
    )
    .expect_value(constants::error::ACTIVITY_STORE_QUERIES);

    assert_eq!(lines.len(), 5);

    assert_eq!(
        model.generated_at,
        constants::activity_store::TEST_SECOND_OBSERVED_AT
    );
    assert_eq!(model.inventory_returned, 1);
    assert_eq!(model.running_now_returned, 1);
    assert_eq!(model.foreground_now_returned, 1);
    assert_eq!(model.launcher_returned, 1);
    assert_eq!(model.daily_rollup_returned, 1);
    assert_eq!(model.custody_label, APP_GAME_JOURNAL_CUSTODY_LOCAL_SQLITE);
    assert_eq!(model.replay_state, APP_GAME_JOURNAL_REPLAY_STATE_REPLAYED);

    assert_eq!(model.inventory_rows.len(), 1);
    assert_eq!(model.running_now_rows.len(), 1);
    assert_eq!(model.foreground_now_rows.len(), 1);
    assert_eq!(model.launcher_rows.len(), 1);
    assert_eq!(model.daily_rollups.len(), 1);

    let inventory = &model.inventory_rows[0];
    let runtime = &model.running_now_rows[0];
    let foreground = &model.foreground_now_rows[0];
    let launcher = &model.launcher_rows[0];
    let rollup = &model.daily_rollups[0];

    assert_eq!(
        inventory.inventory_entry_id,
        APP_GAME_TEST_REGISTRY_SOURCE_REF
    );
    assert_eq!(
        runtime.observed_at,
        constants::activity_store::TEST_SECOND_OBSERVED_AT
    );
    assert_eq!(foreground.foreground_state, APP_GAME_FOREGROUND_FOREGROUND);
    assert_eq!(
        launcher.classification_state,
        APP_GAME_CLASSIFICATION_KNOWN_LAUNCHER
    );
    assert_eq!(
        rollup.rollup_date,
        constants::activity_store::TEST_ROLLUP_DATE
    );
    assert_eq!(rollup.running_duration_ms, 60000);
    assert_eq!(rollup.foreground_duration_ms, 60000);
}

fn journal_replay_events() -> [ActivityEvent; 5] {
    [
        app_game_inventory_journal_event(
            constants::peer::LOCAL_DEV_AGENT,
            std::env::consts::OS,
            &inventory_row(),
        )
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES),
        app_game_runtime_journal_event(
            constants::peer::LOCAL_DEV_AGENT,
            std::env::consts::OS,
            &runtime_row(
                APP_GAME_TEST_RUNTIME_EVIDENCE_ID,
                constants::activity_store::TEST_FIRST_OBSERVED_AT,
            ),
        )
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES),
        app_game_runtime_journal_event(
            constants::peer::LOCAL_DEV_AGENT,
            std::env::consts::OS,
            &runtime_row(
                constants::event_id::LOG_SNAPSHOT_REPORTED,
                constants::activity_store::TEST_SECOND_OBSERVED_AT,
            ),
        )
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES),
        app_game_foreground_journal_event(
            constants::peer::LOCAL_DEV_AGENT,
            std::env::consts::OS,
            &foreground_row(),
        )
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES),
        app_game_launcher_journal_event(
            constants::peer::LOCAL_DEV_AGENT,
            std::env::consts::OS,
            &launcher_row(),
        )
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES),
    ]
}

#[test]
fn duplicate_runtime_observations_do_not_double_count_duration_after_replay() {
    let runtime_start = runtime_row(
        APP_GAME_TEST_RUNTIME_EVIDENCE_ID,
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
    );
    let mut duplicate = runtime_start.clone();
    duplicate
        .runtime_evidence_id
        .push(constants::delimiter::HYPHEN);
    duplicate.runtime_evidence_id.push('2');
    let runtime_latest = runtime_row(
        constants::event_id::LOG_SNAPSHOT_REPORTED,
        constants::activity_store::TEST_SECOND_OBSERVED_AT,
    );
    let events = [
        app_game_runtime_journal_event(
            constants::peer::LOCAL_DEV_AGENT,
            std::env::consts::OS,
            &runtime_start,
        )
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES),
        app_game_runtime_journal_event(
            constants::peer::LOCAL_DEV_AGENT,
            std::env::consts::OS,
            &duplicate,
        )
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES),
        app_game_runtime_journal_event(
            constants::peer::LOCAL_DEV_AGENT,
            std::env::consts::OS,
            &runtime_latest,
        )
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES),
    ];

    let (store, _) = append_and_replay(&events, constants::journal::TEST_ROTATION_SUFFIX);
    let model = app_game_journal_sqlite_read_model(
        store.connection_for_test(),
        constants::activity_store::DEFAULT_RECENT_LIMIT,
        constants::activity_store::TEST_SECOND_OBSERVED_AT,
    )
    .expect_value(constants::error::ACTIVITY_STORE_QUERIES);

    assert_eq!(model.daily_rollups.len(), 1);
    assert_eq!(model.daily_rollups[0].running_duration_ms, 60000);
    assert_eq!(model.daily_rollups[0].foreground_duration_ms, 0);
    assert_eq!(model.daily_rollups[0].session_count, 1);
}

#[test]
fn invalid_inventory_evidence_is_rejected_before_sqlite_ingest() {
    let mut invalid = inventory_row();
    invalid.runtime_state = APP_GAME_RUNTIME_RUNNING.to_string();

    let result = app_game_inventory_journal_event(
        constants::peer::LOCAL_DEV_AGENT,
        std::env::consts::OS,
        &invalid,
    );
    let store =
        ActivityStore::open_in_memory().expect_value(constants::error::ACTIVITY_STORE_OPENS);
    let status = store
        .status()
        .expect_value(constants::error::ACTIVITY_STORE_QUERIES);

    assert_eq!(
        result,
        Err(AppGameJournalSqliteIngestError::InventoryClaimsUse)
    );
    assert_eq!(status.events_stored, 0);
}

#[test]
fn launcher_known_game_claim_without_child_proof_is_rejected_before_ingest() {
    let mut invalid = launcher_row();
    invalid.classification_state = APP_GAME_CLASSIFICATION_KNOWN_GAME.to_string();
    invalid.game_proof_state = APP_GAME_LAUNCHER_PROOF_CHILD_PROCESS_CANDIDATE.to_string();
    invalid.child_process_identity =
        Some(APP_GAME_TEST_LAUNCHER_CHILD_PROCESS_IDENTITY.to_string());
    invalid.child_game_evidence_claim_id = None;

    let result = app_game_launcher_journal_event(
        constants::peer::LOCAL_DEV_AGENT,
        std::env::consts::OS,
        &invalid,
    );

    assert_eq!(
        result,
        Err(AppGameJournalSqliteIngestError::LauncherKnownGameMissingProof)
    );
}

fn append_and_replay(
    events: &[ActivityEvent],
    suffix: impl Display,
) -> (ActivityStore, Vec<ActivityJournalLine>) {
    let path = temp_journal_path(suffix);
    cleanup_journal_files(&path);
    let key = test_key();
    let mut journal = ActivityJournal::open(path.0.clone(), key.clone())
        .expect_value(constants::error::JOURNAL_OPENS);
    let mut lines = Vec::new();
    for event in events {
        lines.push(
            journal
                .append(event)
                .expect_value(constants::error::JOURNAL_APPENDS),
        );
    }
    let reader =
        ActivityJournal::open(path.0.clone(), key).expect_value(constants::error::JOURNAL_OPENS);
    let store =
        ActivityStore::open_in_memory().expect_value(constants::error::ACTIVITY_STORE_OPENS);
    let status = store
        .ingest_journal(&reader)
        .expect_value(constants::error::ACTIVITY_STORE_INGESTS);
    cleanup_journal_files(&path);

    assert_eq!(status.events_ingested, events.len() as u64);
    assert_eq!(status.events_stored, events.len() as u64);
    (store, lines)
}

fn inventory_row() -> AppGameInventoryEvidenceRow {
    AppGameInventoryEvidenceRow {
        schema_version: APP_GAME_SCHEMA_VERSION,
        inventory_entry_id: APP_GAME_TEST_REGISTRY_SOURCE_REF.to_string(),
        observed_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        source_kind: APP_GAME_INVENTORY_SOURCE_OS_INSTALLED_RECORD.to_string(),
        source_ref: APP_GAME_TEST_REGISTRY_SOURCE_REF.to_string(),
        custody_state: APP_GAME_INVENTORY_CUSTODY_LOCAL_AGENT.to_string(),
        product_kind: APP_GAME_PRODUCT_NATIVE_APP.to_string(),
        display_label: APP_GAME_TEST_DISPLAY_LABEL.to_string(),
        identity_id: None,
        package_id: None,
        bundle_id: None,
        app_user_model_id: None,
        desktop_entry_id: None,
        executable_path_ref: Some(APP_GAME_TEST_EXECUTABLE_PATH_REF.to_string()),
        launcher_ref: None,
        launcher_app_id: None,
        launcher_manifest_id: None,
        store_id: None,
        catalog_ref: Some(APP_GAME_TEST_CATALOG_REF.to_string()),
        inventory_state: APP_GAME_INVENTORY_STATE_INSTALLED.to_string(),
        classification_state: APP_GAME_CLASSIFICATION_KNOWN_APP.to_string(),
        catalog_ready_state: APP_GAME_CATALOG_READY.to_string(),
        capability_status: APP_GAME_CAPABILITY_STATUS_AVAILABLE.to_string(),
        confidence: 0.86,
        category_candidates: vec![AppGameInventoryCategoryCandidate {
            category_kind: APP_GAME_INVENTORY_CATEGORY_GAME.to_string(),
            confidence: 0.74,
            catalog_ref: Some(APP_GAME_TEST_CATALOG_REF.to_string()),
            evidence: vec![source_evidence(APP_GAME_TEST_REGISTRY_SOURCE_REF)],
        }],
        runtime_state: APP_GAME_RUNTIME_NOT_CLAIMED.to_string(),
        foreground_state: APP_GAME_FOREGROUND_NOT_CLAIMED.to_string(),
        running_duration_ms: 0,
        foreground_duration_ms: 0,
        evidence: vec![source_evidence(APP_GAME_TEST_REGISTRY_SOURCE_REF)],
    }
}

fn runtime_row(
    runtime_evidence_id: impl Display,
    observed_at: impl Display,
) -> AppGameRuntimeEvidenceRow {
    let runtime_evidence_id = runtime_evidence_id.to_string();
    let observed_at = observed_at.to_string();
    AppGameRuntimeEvidenceRow {
        schema_version: APP_GAME_SCHEMA_VERSION,
        runtime_evidence_id: runtime_evidence_id.to_string(),
        observed_at,
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
        running_duration_ms: 0,
        runtime_state: APP_GAME_RUNTIME_RUNNING.to_string(),
        foreground_state: APP_GAME_FOREGROUND_NOT_CLAIMED.to_string(),
        observation_mode: APP_GAME_OBSERVATION_MODE_PROCESS_START.to_string(),
        classification_state: APP_GAME_CLASSIFICATION_KNOWN_APP.to_string(),
        catalog_ready_state: APP_GAME_CATALOG_READY.to_string(),
        capability_status: APP_GAME_CAPABILITY_STATUS_AVAILABLE.to_string(),
        confidence: 0.82,
        evidence: vec![source_evidence(runtime_evidence_id)],
    }
}

fn foreground_row() -> AppGameForegroundEvidenceRow {
    AppGameForegroundEvidenceRow {
        schema_version: APP_GAME_SCHEMA_VERSION,
        foreground_evidence_id: APP_GAME_TEST_FOREGROUND_EVIDENCE_ID.to_string(),
        observed_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        process_identity: APP_GAME_TEST_PROCESS_IDENTITY.to_string(),
        process_id: APP_GAME_TEST_PROCESS_ID,
        process_name: APP_GAME_TEST_PROCESS_NAME.to_string(),
        inventory_entry_id: Some(APP_GAME_TEST_REGISTRY_SOURCE_REF.to_string()),
        launcher_ref: None,
        catalog_ref: Some(APP_GAME_TEST_CATALOG_REF.to_string()),
        window_ref: Some(APP_GAME_TEST_WINDOW_REF.to_string()),
        window_title_ref: Some(APP_GAME_TEST_WINDOW_TITLE_REF.to_string()),
        title_capture_state: APP_GAME_TITLE_CAPTURE_TITLE_REF.to_string(),
        foreground_started_at: Some(constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string()),
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
        evidence: vec![source_evidence(APP_GAME_TEST_FOREGROUND_EVIDENCE_ID)],
    }
}

fn launcher_row() -> AppGameLauncherEvidenceRow {
    AppGameLauncherEvidenceRow {
        schema_version: APP_GAME_SCHEMA_VERSION,
        launcher_evidence_id: APP_GAME_TEST_LAUNCHER_EVIDENCE_ID.to_string(),
        observed_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        launcher_kind: APP_GAME_LAUNCHER_KIND_STEAM.to_string(),
        launcher_ref: APP_GAME_TEST_LAUNCHER_REF.to_string(),
        launcher_inventory_entry_id: Some(APP_GAME_TEST_REGISTRY_SOURCE_REF.to_string()),
        launcher_manifest_id: None,
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
        evidence: vec![source_evidence(APP_GAME_TEST_LAUNCHER_EVIDENCE_ID)],
    }
}

fn source_evidence(evidence_id: impl Display) -> ActivityEvidenceRef {
    ActivityEvidenceRef {
        evidence_id: evidence_id.to_string(),
        kind: ActivityEvidenceKind::StorageObject,
        digest: None,
        uri: None,
    }
}

fn temp_journal_path(suffix: impl Display) -> TestPath {
    let suffix = suffix.to_string();
    let mut name = String::from(constants::journal::TEST_FILE_PREFIX);
    name.push_str(&std::process::id().to_string());
    name.push(constants::delimiter::HYPHEN);
    name.push_str(suffix.as_str());
    name.push(constants::delimiter::HYPHEN);
    name.push_str(APP_GAME_TEST_RUNTIME_EVIDENCE_ID);

    let mut path = std::env::temp_dir();
    path.push(name);
    path.set_extension(constants::journal::FILE_EXTENSION);
    TestPath(path)
}

fn cleanup_journal_files(path: impl AsRef<std::path::Path>) {
    let path = path.as_ref();
    let _ = remove_file(path);
    for index in 1..=3 {
        let mut rotated_path = path.to_path_buf();
        let mut extension = index.to_string();
        extension.push(constants::delimiter::DOT);
        extension.push_str(constants::journal::FILE_EXTENSION);
        rotated_path.set_extension(extension);
        let _ = remove_file(rotated_path);
    }
}

fn test_key() -> JournalKey {
    JournalKey::from_bytes([9; JOURNAL_KEY_BYTES])
}
