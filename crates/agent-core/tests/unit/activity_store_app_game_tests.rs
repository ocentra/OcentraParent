use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::activity::{ActivityEvidenceKind, ActivityEvidenceRef};
use ocentra_parent_agent_protocol::app_game::{
    AppGameServiceReadModel, APP_GAME_CAPABILITY_STATUS_ADAPTER_ERROR,
    APP_GAME_CAPABILITY_STATUS_AVAILABLE, APP_GAME_CAPABILITY_STATUS_DEGRADED,
    APP_GAME_CAPABILITY_STATUS_MANUAL_REQUIRED, APP_GAME_CAPABILITY_STATUS_NOT_CLAIMED,
    APP_GAME_CAPABILITY_STATUS_PERMISSION_LIMITED, APP_GAME_CAPABILITY_STATUS_STALE,
    APP_GAME_CAPABILITY_STATUS_UNAVAILABLE, APP_GAME_CAPABILITY_STATUS_UNSUPPORTED_PLATFORM,
    APP_GAME_CATALOG_NOT_LOADED, APP_GAME_CLASSIFICATION_POSSIBLY_GAME,
    APP_GAME_CLASSIFICATION_UNKNOWN_PROCESS, APP_GAME_JOURNAL_REPLAY_STATE_REPLAYED,
    APP_GAME_SESSION_ID_PREFIX,
};
use ocentra_parent_agent_protocol::app_game_boundary_read_model::AppGameHealthStatus;
use ocentra_parent_agent_protocol::constants;

use ocentra_parent_agent_core::activity_store_app_game::app_game_performance_health::app_game_performance_health;

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

#[test]
fn app_game_health_matrix_is_fail_closed_and_preserves_persisted_bounds() {
    for (status, expected) in [
        (
            APP_GAME_CAPABILITY_STATUS_AVAILABLE,
            AppGameHealthStatus::Healthy,
        ),
        (
            APP_GAME_CAPABILITY_STATUS_ADAPTER_ERROR,
            AppGameHealthStatus::Degraded,
        ),
        (
            APP_GAME_CAPABILITY_STATUS_PERMISSION_LIMITED,
            AppGameHealthStatus::Degraded,
        ),
        (
            APP_GAME_CAPABILITY_STATUS_STALE,
            AppGameHealthStatus::Degraded,
        ),
        (
            APP_GAME_CAPABILITY_STATUS_DEGRADED,
            AppGameHealthStatus::Degraded,
        ),
        (
            APP_GAME_CAPABILITY_STATUS_UNAVAILABLE,
            AppGameHealthStatus::Unavailable,
        ),
        (
            APP_GAME_CAPABILITY_STATUS_UNSUPPORTED_PLATFORM,
            AppGameHealthStatus::Unavailable,
        ),
        (
            APP_GAME_CAPABILITY_STATUS_MANUAL_REQUIRED,
            AppGameHealthStatus::ManualRequired,
        ),
        (
            APP_GAME_CAPABILITY_STATUS_NOT_CLAIMED,
            AppGameHealthStatus::NotClaimed,
        ),
        ("unknown", AppGameHealthStatus::Unavailable),
    ] {
        assert_eq!(
            app_game_performance_health(&health_model(status)).status,
            expected
        );
    }
}

#[test]
fn app_game_health_rejects_missing_custody_zero_limit_and_count_mismatch() {
    let mut missing = health_model(APP_GAME_CAPABILITY_STATUS_AVAILABLE);
    missing.custody_label.clear();
    assert_eq!(
        app_game_performance_health(&missing).status,
        AppGameHealthStatus::Unavailable
    );
    let mut invalid = health_model(APP_GAME_CAPABILITY_STATUS_AVAILABLE);
    invalid.limit = 0;
    assert_eq!(
        app_game_performance_health(&invalid).status,
        AppGameHealthStatus::Degraded
    );
    invalid.limit = 10;
    invalid.inventory_returned = 1;
    assert_eq!(
        app_game_performance_health(&invalid).status,
        AppGameHealthStatus::Degraded
    );
    for field in 0..5 {
        let mut over = health_model(APP_GAME_CAPABILITY_STATUS_AVAILABLE);
        match field {
            0 => over.inventory_returned = 11,
            1 => over.running_now_returned = 11,
            2 => over.foreground_now_returned = 11,
            3 => over.launcher_returned = 11,
            _ => over.daily_rollup_returned = 11,
        }
        assert_eq!(
            app_game_performance_health(&over).status,
            AppGameHealthStatus::Degraded
        );
    }
}

fn health_model(status: &str) -> AppGameServiceReadModel {
    AppGameServiceReadModel {
        schema_version: 1,
        generated_at: "2026-08-28T00:00:00Z".into(),
        limit: 10,
        custody_label: "local-sqlite".into(),
        replay_state: APP_GAME_JOURNAL_REPLAY_STATE_REPLAYED.into(),
        capability_status: status.into(),
        inventory_returned: 0,
        running_now_returned: 0,
        foreground_now_returned: 0,
        launcher_returned: 0,
        daily_rollup_returned: 0,
        evidence_claim_returned: 0,
        identity_returned: 0,
        approval_authority_returned: 0,
        approval_action_result_returned: 0,
        platform_authority_matrix_returned: 0,
        ai_classifier_result_returned: 0,
        inventory_rows: Vec::new(),
        running_now_rows: Vec::new(),
        foreground_now_rows: Vec::new(),
        launcher_rows: Vec::new(),
        daily_rollups: Vec::new(),
        evidence_claim_rows: Vec::new(),
        identity_rows: Vec::new(),
        approval_authority_rows: Vec::new(),
        approval_action_result_rows: Vec::new(),
        platform_authority_matrices: Vec::new(),
        ai_classifier_result_rows: Vec::new(),
    }
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
