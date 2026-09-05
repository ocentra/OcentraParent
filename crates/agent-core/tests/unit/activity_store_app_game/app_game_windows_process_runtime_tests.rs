use std::fmt::Display;

use ocentra_parent_agent_protocol::app_game::*;
use ocentra_parent_agent_protocol::constants;

use super::app_game_windows_process_runtime::{
    runtime_session_summaries_from_rows, windows_process_runtime_rows_from_records,
    WindowsProcessRuntimeRecord,
};

#[test]
fn process_appears_and_creates_runtime_evidence() {
    let rows = windows_process_runtime_rows_from_records(&[running_app_record(
        APP_GAME_TEST_RUNTIME_EVIDENCE_ID,
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
        APP_GAME_OBSERVATION_MODE_PROCESS_START,
        0,
    )]);

    assert_eq!(rows.len(), 1);
    assert_eq!(
        rows[0].runtime_evidence_id,
        APP_GAME_TEST_RUNTIME_EVIDENCE_ID
    );
    assert_eq!(rows[0].process_identity, APP_GAME_TEST_PROCESS_IDENTITY);
    assert_eq!(rows[0].process_id, APP_GAME_TEST_PROCESS_ID);
    assert_eq!(
        rows[0].parent_process_id,
        Some(APP_GAME_TEST_PARENT_PROCESS_ID)
    );
    assert_eq!(
        rows[0].publisher_signature_ref,
        Some(APP_GAME_TEST_PUBLISHER_SIGNATURE_REF.to_string())
    );
    assert_eq!(
        rows[0].file_hash_ref,
        Some(APP_GAME_TEST_FILE_HASH_REF.to_string())
    );
    assert_eq!(rows[0].runtime_state, APP_GAME_RUNTIME_RUNNING);
    assert_eq!(rows[0].foreground_state, APP_GAME_FOREGROUND_NOT_CLAIMED);
}

#[test]
fn same_process_persists_and_session_can_continue() {
    let rows = windows_process_runtime_rows_from_records(&[
        running_app_record(
            APP_GAME_TEST_RUNTIME_EVIDENCE_ID,
            constants::activity_store::TEST_FIRST_OBSERVED_AT,
            APP_GAME_OBSERVATION_MODE_PROCESS_START,
            0,
        ),
        running_app_record(
            APP_GAME_TEST_RUNTIME_EXIT_EVIDENCE_ID,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
            APP_GAME_OBSERVATION_MODE_PROCESS_SNAPSHOT,
            300000,
        ),
    ]);
    let summaries = runtime_session_summaries_from_rows(&rows);

    assert_eq!(summaries.len(), 1);
    assert_eq!(
        summaries[0].primary_process_identity,
        APP_GAME_TEST_PROCESS_IDENTITY
    );
    assert_eq!(summaries[0].observation_count, 2);
    assert_eq!(summaries[0].running_duration_ms, 300000);
    assert_eq!(summaries[0].foreground_duration_ms, 0);
    assert_eq!(summaries[0].background_duration_ms, 300000);
    assert_eq!(
        summaries[0].last_observed_at,
        constants::activity_store::TEST_SECOND_OBSERVED_AT
    );
}

#[test]
fn pid_reuse_with_new_generation_does_not_merge_runtime_sessions() {
    let mut previous_generation = running_app_record(
        APP_GAME_TEST_RUNTIME_EVIDENCE_ID,
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
        APP_GAME_OBSERVATION_MODE_PROCESS_SNAPSHOT,
        60000,
    );
    previous_generation.process_identity = Some(format!(
        "{}-100",
        constants::activity_capture::PROCESS_SUBJECT_ID_PREFIX
    ));

    let mut current_generation = running_app_record(
        APP_GAME_TEST_RUNTIME_EXIT_EVIDENCE_ID,
        constants::activity_store::TEST_SECOND_OBSERVED_AT,
        APP_GAME_OBSERVATION_MODE_PROCESS_SNAPSHOT,
        30000,
    );
    current_generation.process_identity = Some(format!(
        "{}-200",
        constants::activity_capture::PROCESS_SUBJECT_ID_PREFIX
    ));

    let rows =
        windows_process_runtime_rows_from_records(&[previous_generation, current_generation]);
    let summaries = runtime_session_summaries_from_rows(&rows);

    assert_eq!(summaries.len(), 2);
    assert_ne!(summaries[0].session_id, summaries[1].session_id);
    assert!(summaries.iter().any(|summary| {
        summary.primary_process_identity
            == format!(
                "{}-100",
                constants::activity_capture::PROCESS_SUBJECT_ID_PREFIX
            )
    }));
    assert!(summaries.iter().any(|summary| {
        summary.primary_process_identity
            == format!(
                "{}-200",
                constants::activity_capture::PROCESS_SUBJECT_ID_PREFIX
            )
    }));
}

#[test]
fn process_exit_closes_runtime_session() {
    let rows = windows_process_runtime_rows_from_records(&[
        running_app_record(
            APP_GAME_TEST_RUNTIME_EVIDENCE_ID,
            constants::activity_store::TEST_FIRST_OBSERVED_AT,
            APP_GAME_OBSERVATION_MODE_PROCESS_START,
            0,
        ),
        running_app_record(
            APP_GAME_TEST_RUNTIME_EXIT_EVIDENCE_ID,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
            APP_GAME_OBSERVATION_MODE_PROCESS_EXIT,
            600000,
        ),
    ]);
    let summaries = runtime_session_summaries_from_rows(&rows);

    assert_eq!(rows[1].runtime_state, APP_GAME_RUNTIME_NOT_RUNNING);
    assert_eq!(
        rows[1].exited_at,
        Some(constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string())
    );
    assert_eq!(
        summaries[0].ended_at,
        Some(constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string())
    );
    assert_eq!(summaries[0].running_duration_ms, 600000);
    assert_eq!(summaries[0].foreground_duration_ms, 0);
}

#[test]
fn unknown_process_remains_unknown_without_deterministic_refs() {
    let mut record = running_app_record(
        APP_GAME_TEST_RUNTIME_EVIDENCE_ID,
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
        APP_GAME_OBSERVATION_MODE_PROCESS_SNAPSHOT,
        0,
    );
    record.classification_state = APP_GAME_CLASSIFICATION_KNOWN_GAME.to_string();
    record.executable_path_ref = None;
    record.publisher_signature_ref = None;
    record.file_hash_ref = None;
    record.inventory_entry_id = None;
    record.catalog_ref = None;

    let rows = windows_process_runtime_rows_from_records(&[record]);

    assert_eq!(
        rows[0].classification_state,
        APP_GAME_CLASSIFICATION_UNKNOWN_PROCESS
    );
    assert_eq!(rows[0].confidence, 0.0);
    assert_eq!(rows[0].runtime_state, APP_GAME_RUNTIME_RUNNING);
    assert_eq!(rows[0].foreground_state, APP_GAME_FOREGROUND_NOT_CLAIMED);
}

#[test]
fn launcher_process_stays_launcher_runtime_only() {
    let rows = windows_process_runtime_rows_from_records(&[launcher_record()]);

    assert_eq!(rows.len(), 1);
    assert_eq!(
        rows[0].process_identity,
        APP_GAME_TEST_LAUNCHER_PROCESS_IDENTITY
    );
    assert_eq!(
        rows[0].classification_state,
        APP_GAME_CLASSIFICATION_KNOWN_LAUNCHER
    );
    assert_eq!(
        rows[0].launcher_ref,
        Some(APP_GAME_TEST_LAUNCHER_REF.to_string())
    );
    assert_eq!(rows[0].runtime_state, APP_GAME_RUNTIME_RUNNING);
    assert_eq!(rows[0].foreground_state, APP_GAME_FOREGROUND_NOT_CLAIMED);
}

#[test]
fn permission_limited_metadata_state_remains_explicit() {
    let rows = windows_process_runtime_rows_from_records(&[permission_limited_record()]);

    assert_eq!(rows.len(), 1);
    assert_eq!(
        rows[0].process_identity,
        APP_GAME_TEST_PERMISSION_PROCESS_IDENTITY
    );
    assert_eq!(
        rows[0].classification_state,
        APP_GAME_CLASSIFICATION_PERMISSION_LIMITED
    );
    assert_eq!(rows[0].runtime_state, APP_GAME_RUNTIME_PERMISSION_LIMITED);
    assert_eq!(
        rows[0].catalog_ready_state,
        APP_GAME_CATALOG_PERMISSION_LIMITED
    );
    assert_eq!(
        rows[0].capability_status,
        APP_GAME_CAPABILITY_STATUS_PERMISSION_LIMITED
    );
    assert_eq!(rows[0].executable_path_ref, None);
    assert_eq!(rows[0].publisher_signature_ref, None);
    assert_eq!(rows[0].file_hash_ref, None);
    assert_eq!(rows[0].foreground_state, APP_GAME_FOREGROUND_NOT_CLAIMED);
}

fn running_app_record(
    runtime_evidence_id: impl Display,
    observed_at: impl Display,
    observation_mode: impl Display,
    running_duration_ms: u64,
) -> WindowsProcessRuntimeRecord {
    WindowsProcessRuntimeRecord {
        runtime_evidence_id: runtime_evidence_id.to_string(),
        observed_at: observed_at.to_string(),
        process_identity: Some(APP_GAME_TEST_PROCESS_IDENTITY.to_string()),
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
        running_duration_ms,
        observation_mode: observation_mode.to_string(),
        classification_state: APP_GAME_CLASSIFICATION_KNOWN_APP.to_string(),
        capability_status: APP_GAME_CAPABILITY_STATUS_AVAILABLE.to_string(),
        confidence: 0.82,
        evidence: Vec::new(),
    }
}

fn launcher_record() -> WindowsProcessRuntimeRecord {
    WindowsProcessRuntimeRecord {
        runtime_evidence_id: APP_GAME_TEST_RUNTIME_LAUNCHER_EVIDENCE_ID.to_string(),
        observed_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        process_identity: Some(APP_GAME_TEST_LAUNCHER_PROCESS_IDENTITY.to_string()),
        process_id: APP_GAME_TEST_LAUNCHER_PROCESS_ID,
        parent_process_id: None,
        process_name: APP_GAME_TEST_LAUNCHER_PROCESS_NAME.to_string(),
        executable_path_ref: None,
        publisher_signature_ref: None,
        file_hash_ref: None,
        inventory_entry_id: None,
        launcher_ref: Some(APP_GAME_TEST_LAUNCHER_REF.to_string()),
        catalog_ref: None,
        started_at: Some(constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string()),
        exited_at: None,
        running_duration_ms: 0,
        observation_mode: APP_GAME_OBSERVATION_MODE_PROCESS_SNAPSHOT.to_string(),
        classification_state: APP_GAME_CLASSIFICATION_KNOWN_LAUNCHER.to_string(),
        capability_status: APP_GAME_CAPABILITY_STATUS_AVAILABLE.to_string(),
        confidence: 0.73,
        evidence: Vec::new(),
    }
}

fn permission_limited_record() -> WindowsProcessRuntimeRecord {
    WindowsProcessRuntimeRecord {
        runtime_evidence_id: APP_GAME_TEST_RUNTIME_PERMISSION_EVIDENCE_ID.to_string(),
        observed_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        process_identity: Some(APP_GAME_TEST_PERMISSION_PROCESS_IDENTITY.to_string()),
        process_id: APP_GAME_TEST_PERMISSION_PROCESS_ID,
        parent_process_id: None,
        process_name: APP_GAME_TEST_PERMISSION_PROCESS_NAME.to_string(),
        executable_path_ref: None,
        publisher_signature_ref: None,
        file_hash_ref: None,
        inventory_entry_id: None,
        launcher_ref: None,
        catalog_ref: None,
        started_at: None,
        exited_at: None,
        running_duration_ms: 0,
        observation_mode: APP_GAME_OBSERVATION_MODE_PROCESS_SNAPSHOT.to_string(),
        classification_state: APP_GAME_CLASSIFICATION_UNKNOWN_PROCESS.to_string(),
        capability_status: APP_GAME_CAPABILITY_STATUS_PERMISSION_LIMITED.to_string(),
        confidence: 0.0,
        evidence: Vec::new(),
    }
}
