use ocentra_parent_agent_protocol::app_game::*;
use ocentra_parent_agent_protocol::constants;
use std::fmt::Display;

use super::{
    app_game_windows_foreground::{
        apply_foreground_rows_to_runtime_summaries, windows_foreground_rows_from_records,
        WindowsForegroundWindowRecord,
    },
    app_game_windows_process_runtime::{
        runtime_session_summaries_from_rows, windows_process_runtime_rows_from_records,
        WindowsProcessRuntimeRecord,
    },
};

#[test]
fn active_window_creates_foreground_evidence_without_content_claim() {
    let rows = windows_foreground_rows_from_records(&[active_window_record(
        APP_GAME_TEST_FOREGROUND_EVIDENCE_ID,
        constants::activity_store::TEST_SECOND_OBSERVED_AT,
        APP_GAME_FOREGROUND_FOREGROUND,
        0,
    )]);

    assert_eq!(rows.len(), 1);
    assert_eq!(
        rows[0].foreground_evidence_id,
        APP_GAME_TEST_FOREGROUND_EVIDENCE_ID
    );
    assert_eq!(rows[0].process_identity, APP_GAME_TEST_PROCESS_IDENTITY);
    assert_eq!(rows[0].process_id, APP_GAME_TEST_PROCESS_ID);
    assert_eq!(rows[0].runtime_state, APP_GAME_RUNTIME_RUNNING);
    assert_eq!(rows[0].foreground_state, APP_GAME_FOREGROUND_FOREGROUND);
    assert_eq!(
        rows[0].observation_mode,
        APP_GAME_OBSERVATION_MODE_FOREGROUND_WINDOW
    );
    assert_eq!(
        rows[0].window_ref,
        Some(APP_GAME_TEST_WINDOW_REF.to_string())
    );
    assert_eq!(
        rows[0].window_title_ref,
        Some(APP_GAME_TEST_WINDOW_TITLE_REF.to_string())
    );
    assert_eq!(
        rows[0].title_capture_state,
        APP_GAME_TITLE_CAPTURE_TITLE_REF
    );
    assert_eq!(
        rows[0].content_knowledge_state,
        APP_GAME_CONTENT_KNOWLEDGE_NOT_CLAIMED
    );
}

#[test]
fn foreground_switch_closes_previous_interval_and_updates_runtime_summary() {
    let mut summaries = runtime_summaries(600000);
    let rows = windows_foreground_rows_from_records(&[
        active_window_record(
            APP_GAME_TEST_FOREGROUND_EVIDENCE_ID,
            constants::activity_store::TEST_FIRST_OBSERVED_AT,
            APP_GAME_FOREGROUND_FOREGROUND,
            0,
        ),
        active_window_record(
            APP_GAME_TEST_FOREGROUND_CLOSED_EVIDENCE_ID,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
            APP_GAME_FOREGROUND_BACKGROUND,
            300000,
        ),
    ]);

    apply_foreground_rows_to_runtime_summaries(&mut summaries, &rows);

    assert_eq!(rows[1].foreground_state, APP_GAME_FOREGROUND_BACKGROUND);
    assert_eq!(
        rows[1].foreground_ended_at,
        Some(constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string())
    );
    assert_eq!(summaries[0].running_duration_ms, 600000);
    assert_eq!(summaries[0].foreground_duration_ms, 300000);
    assert_eq!(summaries[0].background_duration_ms, 300000);
    assert_eq!(
        summaries[0].last_observed_at,
        constants::activity_store::TEST_SECOND_OBSERVED_AT
    );
}

#[test]
fn background_process_does_not_gain_foreground_time() {
    let mut summaries = runtime_summaries(300000);
    let rows = windows_foreground_rows_from_records(&[active_window_record(
        APP_GAME_TEST_FOREGROUND_CLOSED_EVIDENCE_ID,
        constants::activity_store::TEST_SECOND_OBSERVED_AT,
        APP_GAME_FOREGROUND_BACKGROUND,
        0,
    )]);

    apply_foreground_rows_to_runtime_summaries(&mut summaries, &rows);

    assert_eq!(rows[0].foreground_duration_ms, 0);
    assert_eq!(summaries[0].foreground_duration_ms, 0);
    assert_eq!(summaries[0].background_duration_ms, 300000);
}

#[test]
fn foreground_duration_cannot_exceed_running_duration() {
    let mut summaries = runtime_summaries(300000);
    let rows = windows_foreground_rows_from_records(&[active_window_record(
        APP_GAME_TEST_FOREGROUND_CLOSED_EVIDENCE_ID,
        constants::activity_store::TEST_SECOND_OBSERVED_AT,
        APP_GAME_FOREGROUND_BACKGROUND,
        900000,
    )]);

    apply_foreground_rows_to_runtime_summaries(&mut summaries, &rows);

    assert_eq!(summaries[0].running_duration_ms, 300000);
    assert_eq!(summaries[0].foreground_duration_ms, 300000);
    assert_eq!(summaries[0].background_duration_ms, 0);
}

#[test]
fn title_can_be_omitted_without_content_capture() {
    let mut record = active_window_record(
        APP_GAME_TEST_FOREGROUND_EVIDENCE_ID,
        constants::activity_store::TEST_SECOND_OBSERVED_AT,
        APP_GAME_FOREGROUND_FOREGROUND,
        0,
    );
    record.window_title_ref = None;
    record.title_capture_state = APP_GAME_TITLE_CAPTURE_TITLE_OMITTED.to_string();

    let rows = windows_foreground_rows_from_records(&[record]);

    assert_eq!(rows[0].window_title_ref, None);
    assert_eq!(
        rows[0].title_capture_state,
        APP_GAME_TITLE_CAPTURE_TITLE_OMITTED
    );
    assert_eq!(
        rows[0].content_knowledge_state,
        APP_GAME_CONTENT_KNOWLEDGE_NOT_CLAIMED
    );
}

#[test]
fn permission_limited_foreground_metadata_stays_explicit() {
    let rows = windows_foreground_rows_from_records(&[permission_limited_record()]);

    assert_eq!(rows.len(), 1);
    assert_eq!(
        rows[0].foreground_evidence_id,
        APP_GAME_TEST_FOREGROUND_PERMISSION_EVIDENCE_ID
    );
    assert_eq!(
        rows[0].process_identity,
        APP_GAME_TEST_PERMISSION_PROCESS_IDENTITY
    );
    assert_eq!(
        rows[0].classification_state,
        APP_GAME_CLASSIFICATION_PERMISSION_LIMITED
    );
    assert_eq!(rows[0].runtime_state, APP_GAME_RUNTIME_UNKNOWN);
    assert_eq!(
        rows[0].foreground_state,
        APP_GAME_FOREGROUND_PERMISSION_LIMITED
    );
    assert_eq!(
        rows[0].catalog_ready_state,
        APP_GAME_CATALOG_PERMISSION_LIMITED
    );
    assert_eq!(rows[0].window_ref, None);
    assert_eq!(rows[0].window_title_ref, None);
    assert_eq!(
        rows[0].title_capture_state,
        APP_GAME_TITLE_CAPTURE_PERMISSION_LIMITED
    );
}

#[test]
fn unknown_foreground_process_remains_unknown_without_deterministic_refs() {
    let mut record = active_window_record(
        APP_GAME_TEST_FOREGROUND_EVIDENCE_ID,
        constants::activity_store::TEST_SECOND_OBSERVED_AT,
        APP_GAME_FOREGROUND_FOREGROUND,
        0,
    );
    record.classification_state = APP_GAME_CLASSIFICATION_KNOWN_APP.to_string();
    record.inventory_entry_id = None;
    record.catalog_ref = None;
    record.window_ref = None;
    record.window_title_ref = None;

    let rows = windows_foreground_rows_from_records(&[record]);

    assert_eq!(
        rows[0].classification_state,
        APP_GAME_CLASSIFICATION_UNKNOWN_PROCESS
    );
    assert_eq!(rows[0].confidence, 0.0);
    assert_eq!(rows[0].foreground_state, APP_GAME_FOREGROUND_FOREGROUND);
}

#[test]
fn launcher_focus_stays_launcher_not_active_game() {
    let rows = windows_foreground_rows_from_records(&[launcher_record()]);

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
    assert_eq!(rows[0].foreground_state, APP_GAME_FOREGROUND_FOREGROUND);
    assert_ne!(rows[0].foreground_state, APP_GAME_FOREGROUND_NOT_CLAIMED);
}

fn runtime_summaries(running_duration_ms: u64) -> Vec<AppGameSessionSummary> {
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
            running_duration_ms,
        ),
    ]);
    runtime_session_summaries_from_rows(&rows)
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
        executable_path_ref: None,
        publisher_signature_ref: None,
        file_hash_ref: None,
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

fn active_window_record(
    foreground_evidence_id: impl Display,
    observed_at: impl Display,
    foreground_state: impl Display,
    foreground_duration_ms: u64,
) -> WindowsForegroundWindowRecord {
    WindowsForegroundWindowRecord {
        foreground_evidence_id: foreground_evidence_id.to_string(),
        observed_at: observed_at.to_string(),
        process_identity: Some(APP_GAME_TEST_PROCESS_IDENTITY.to_string()),
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
        foreground_duration_ms,
        foreground_state: foreground_state.to_string(),
        classification_state: APP_GAME_CLASSIFICATION_KNOWN_APP.to_string(),
        capability_status: APP_GAME_CAPABILITY_STATUS_AVAILABLE.to_string(),
        confidence: 0.84,
        evidence: Vec::new(),
    }
}

fn permission_limited_record() -> WindowsForegroundWindowRecord {
    WindowsForegroundWindowRecord {
        foreground_evidence_id: APP_GAME_TEST_FOREGROUND_PERMISSION_EVIDENCE_ID.to_string(),
        observed_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        process_identity: Some(APP_GAME_TEST_PERMISSION_PROCESS_IDENTITY.to_string()),
        process_id: APP_GAME_TEST_PERMISSION_PROCESS_ID,
        process_name: APP_GAME_TEST_PERMISSION_PROCESS_NAME.to_string(),
        inventory_entry_id: None,
        launcher_ref: None,
        catalog_ref: None,
        window_ref: None,
        window_title_ref: None,
        title_capture_state: APP_GAME_TITLE_CAPTURE_PERMISSION_LIMITED.to_string(),
        foreground_started_at: None,
        foreground_ended_at: None,
        foreground_duration_ms: 0,
        foreground_state: APP_GAME_FOREGROUND_PERMISSION_LIMITED.to_string(),
        classification_state: APP_GAME_CLASSIFICATION_UNKNOWN_PROCESS.to_string(),
        capability_status: APP_GAME_CAPABILITY_STATUS_PERMISSION_LIMITED.to_string(),
        confidence: 0.0,
        evidence: Vec::new(),
    }
}

fn launcher_record() -> WindowsForegroundWindowRecord {
    WindowsForegroundWindowRecord {
        foreground_evidence_id: APP_GAME_TEST_RUNTIME_LAUNCHER_EVIDENCE_ID.to_string(),
        observed_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        process_identity: Some(APP_GAME_TEST_LAUNCHER_PROCESS_IDENTITY.to_string()),
        process_id: APP_GAME_TEST_LAUNCHER_PROCESS_ID,
        process_name: APP_GAME_TEST_LAUNCHER_PROCESS_NAME.to_string(),
        inventory_entry_id: None,
        launcher_ref: Some(APP_GAME_TEST_LAUNCHER_REF.to_string()),
        catalog_ref: None,
        window_ref: Some(APP_GAME_TEST_WINDOW_REF.to_string()),
        window_title_ref: None,
        title_capture_state: APP_GAME_TITLE_CAPTURE_TITLE_OMITTED.to_string(),
        foreground_started_at: Some(constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string()),
        foreground_ended_at: None,
        foreground_duration_ms: 0,
        foreground_state: APP_GAME_FOREGROUND_FOREGROUND.to_string(),
        classification_state: APP_GAME_CLASSIFICATION_KNOWN_LAUNCHER.to_string(),
        capability_status: APP_GAME_CAPABILITY_STATUS_AVAILABLE.to_string(),
        confidence: 0.73,
        evidence: Vec::new(),
    }
}
