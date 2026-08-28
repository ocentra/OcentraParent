use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::activity::{ActivityEvidenceKind, ActivityEvidenceRef};
use ocentra_parent_agent_protocol::app_game::{
    APP_GAME_CATALOG_NOT_LOADED, APP_GAME_CLASSIFICATION_POSSIBLY_GAME,
    APP_GAME_CLASSIFICATION_UNKNOWN_PROCESS, APP_GAME_SESSION_ID_PREFIX,
};
use ocentra_parent_agent_protocol::constants;

use crate::{
    foreground_window_observation_event, process_observation_event, ActivityStore,
    ForegroundWindowObservation, ProcessObservation,
};

#[test]
fn activity_store_reports_generic_window_subject_without_process_identity() {
    let store =
        ActivityStore::open_in_memory().expect_value(constants::error::ACTIVITY_STORE_OPENS);
    let process = process_observation_event(
        ProcessObservation {
            pid: 4242,
            name: constants::activity_store::TEST_APP_GAME_PROCESS_NAME.to_string(),
            executable_path: None,
        },
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
        0,
    );
    let mut window = foreground_window_observation_event(
        active_window(),
        constants::activity_store::TEST_SECOND_OBSERVED_AT,
    );
    window.evidence.push(journal_evidence());

    store
        .ingest_events(&[window, process])
        .expect_value(constants::error::ACTIVITY_STORE_INGESTS);
    let report = store
        .app_game_session_report(constants::activity_store::DEFAULT_RECENT_LIMIT)
        .expect_value(constants::error::ACTIVITY_STORE_QUERIES);

    assert_eq!(
        report.most_recent_session_id,
        Some(format!(
            "{}{}",
            APP_GAME_SESSION_ID_PREFIX,
            constants::activity_store::TEST_WINDOW_SUBJECT_ID
        ))
    );
    assert_eq!(report.returned, 2);
    assert_eq!(
        report.most_recent_process_identity,
        Some(constants::activity_store::TEST_WINDOW_SUBJECT_ID.to_string())
    );
    assert_eq!(
        report.most_recent_classification_state,
        Some(APP_GAME_CLASSIFICATION_POSSIBLY_GAME.to_string())
    );
    assert_eq!(report.most_recent_evidence_count, Some(2));
}

#[test]
fn activity_store_reports_unknown_process_without_catalog_claims() {
    let store =
        ActivityStore::open_in_memory().expect_value(constants::error::ACTIVITY_STORE_OPENS);
    let process = process_observation_event(
        ProcessObservation {
            pid: 4242,
            name: constants::activity_store::TEST_PROCESS_SUBJECT_NAME.to_string(),
            executable_path: None,
        },
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
        0,
    );

    store
        .ingest_events(&[process])
        .expect_value(constants::error::ACTIVITY_STORE_INGESTS);
    let report = store
        .app_game_session_report(constants::activity_store::DEFAULT_RECENT_LIMIT)
        .expect_value(constants::error::ACTIVITY_STORE_QUERIES);

    assert_eq!(report.returned, 1);
    assert_eq!(report.catalog_ready_state, APP_GAME_CATALOG_NOT_LOADED);
    assert_eq!(
        report.most_recent_classification_state,
        Some(APP_GAME_CLASSIFICATION_UNKNOWN_PROCESS.to_string())
    );
}

#[test]
fn activity_store_reports_empty_app_game_sessions_without_inventing_rows() {
    let store =
        ActivityStore::open_in_memory().expect_value(constants::error::ACTIVITY_STORE_OPENS);

    let report = store
        .app_game_session_report(constants::activity_store::DEFAULT_RECENT_LIMIT)
        .expect_value(constants::error::ACTIVITY_STORE_QUERIES);

    assert_eq!(report.returned, 0);
    assert_eq!(report.first_observed_at, None);
    assert_eq!(report.most_recent_session_id, None);
}

fn active_window() -> ForegroundWindowObservation {
    ForegroundWindowObservation::active(
        4242,
        constants::activity_store::TEST_APP_GAME_PROCESS_NAME.to_string(),
        constants::activity_store::TEST_APP_GAME_PROCESS_PATH.to_string(),
        constants::activity_store::TEST_APP_GAME_WINDOW_TITLE.to_string(),
        constants::activity_store::TEST_WINDOW_ID.to_string(),
    )
}

fn journal_evidence() -> ActivityEvidenceRef {
    ActivityEvidenceRef {
        evidence_id: constants::activity_store::TEST_JOURNAL_SUFFIX.to_string(),
        kind: ActivityEvidenceKind::JournalEntry,
        digest: None,
        uri: None,
    }
}
