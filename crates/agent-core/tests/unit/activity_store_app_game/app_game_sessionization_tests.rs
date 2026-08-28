use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::activity::ActivityEvent;
use ocentra_parent_agent_protocol::app_game::APP_GAME_OBSERVATION_MODE_PROCESS_EXIT;
use ocentra_parent_agent_protocol::app_game::*;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use std::fmt::Display;

use super::app_game_session_rollups::daily_rollups_from_summaries;
use super::app_game_sessionization::session_summaries_from_rows;
use crate::{
    activity_store_app_game_rows::app_game_rows, foreground_window_observation_event,
    process_observation_event, ActivityStore, ForegroundWindowObservation, ProcessObservation,
};

#[test]
fn process_observations_start_and_continue_session_duration() {
    let summaries = summaries_from_events(&[
        process_event(constants::activity_store::TEST_FIRST_OBSERVED_AT, 0),
        process_event(constants::activity_store::TEST_SECOND_OBSERVED_AT, 1),
    ]);

    assert_eq!(summaries.len(), 1);
    assert_eq!(
        summaries[0].primary_process_identity,
        constants::activity_store::TEST_PROCESS_SUBJECT_ID
    );
    assert_eq!(summaries[0].running_duration_ms, 60000);
    assert_eq!(summaries[0].foreground_duration_ms, 0);
    assert_eq!(summaries[0].background_duration_ms, 60000);
    assert_eq!(summaries[0].observation_gap_ms, 60000);
    assert_eq!(
        summaries[0].last_observed_at,
        constants::activity_store::TEST_SECOND_OBSERVED_AT
    );
}

#[test]
fn stale_gap_closes_and_reopens_same_process_identity() {
    let summaries = summaries_from_events(&[
        process_event(constants::activity_store::TEST_FIRST_OBSERVED_AT, 0),
        process_event(constants::activity_store::TEST_THIRD_OBSERVED_AT, 1),
    ]);

    assert_eq!(summaries.len(), 2);
    assert_eq!(summaries[0].ended_at, None);
    assert_eq!(
        summaries[1].end_reason,
        Some(APP_GAME_SESSION_END_REASON_TIMEOUT_INFERRED.to_string())
    );
    assert_eq!(
        summaries[1].ended_at,
        Some(constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string())
    );
    assert_ne!(summaries[0].session_id, summaries[1].session_id);
}

#[test]
fn process_exit_closes_running_session() {
    let summaries = summaries_from_events(&[
        process_event(constants::activity_store::TEST_FIRST_OBSERVED_AT, 0),
        process_exit_event(constants::activity_store::TEST_SECOND_OBSERVED_AT, 1),
    ]);

    assert_eq!(summaries.len(), 1);
    assert_eq!(
        summaries[0].ended_at,
        Some(constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string())
    );
    assert_eq!(
        summaries[0].end_reason,
        Some(APP_GAME_SESSION_END_REASON_PROCESS_EXIT.to_string())
    );
    assert_eq!(summaries[0].running_duration_ms, 60000);
    assert_eq!(summaries[0].background_duration_ms, 60000);
}

#[test]
fn foreground_samples_accumulate_without_restarting_the_interval() {
    let process_identity = constants::activity_store::TEST_PROCESS_SUBJECT_ID;
    let summaries = summaries_from_events(&[
        process_event(constants::activity_store::TEST_FIRST_OBSERVED_AT, 0),
        active_window_event_for_process(
            constants::activity_store::TEST_FIRST_OBSERVED_AT,
            process_identity,
        ),
        process_event(constants::activity_store::TEST_SECOND_OBSERVED_AT, 1),
        active_window_event_for_process(
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
            process_identity,
        ),
        process_event(constants::activity_store::TEST_THIRD_OBSERVED_AT, 2),
    ]);
    let summary = summaries
        .iter()
        .find(|summary| summary.primary_process_identity == process_identity)
        .expect_value(constants::error::ACTIVITY_STORE_QUERIES);

    assert_eq!(summary.running_duration_ms, 120000);
    assert_eq!(summary.foreground_duration_ms, 120000);
    assert_eq!(summary.background_duration_ms, 0);
    assert_eq!(
        summary.last_foreground_at,
        Some(constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string())
    );
}

#[test]
fn foreground_only_observation_does_not_start_running_duration() {
    let process_identity = constants::activity_store::TEST_PROCESS_SUBJECT_ID;
    let summaries = summaries_from_events(&[
        active_window_event_for_process(
            constants::activity_store::TEST_FIRST_OBSERVED_AT,
            process_identity,
        ),
        process_event(constants::activity_store::TEST_SECOND_OBSERVED_AT, 0),
    ]);
    let summary = summaries
        .iter()
        .find(|summary| summary.primary_process_identity == process_identity)
        .expect_value(constants::error::ACTIVITY_STORE_QUERIES);

    assert_eq!(summary.running_duration_ms, 0);
    assert_eq!(summary.foreground_duration_ms, 0);
    assert_eq!(summary.background_duration_ms, 0);
}

#[test]
fn foreground_rows_do_not_mask_a_stale_process_gap() {
    let process_identity = constants::activity_store::TEST_PROCESS_SUBJECT_ID;
    let process_later = "2026-05-20T00:03:00Z";
    let summaries = summaries_from_events(&[
        process_event(constants::activity_store::TEST_FIRST_OBSERVED_AT, 0),
        active_window_event_for_process(
            constants::activity_store::TEST_THIRD_OBSERVED_AT,
            process_identity,
        ),
        process_event(process_later, 1),
    ]);

    assert_eq!(summaries.len(), 2);
    let timeout_summary = summaries
        .iter()
        .find(|summary| summary.end_reason.is_some())
        .expect_value(constants::error::ACTIVITY_STORE_QUERIES);
    assert_eq!(
        timeout_summary.end_reason,
        Some(APP_GAME_SESSION_END_REASON_TIMEOUT_INFERRED.to_string())
    );
    assert_eq!(timeout_summary.running_duration_ms, 60000);
    let current_summary = summaries
        .iter()
        .find(|summary| summary.end_reason.is_none())
        .expect_value(constants::error::ACTIVITY_STORE_QUERIES);
    assert_eq!(current_summary.running_duration_ms, 0);
}

#[test]
fn equivalent_offset_timestamps_are_replayed_in_utc_order() {
    let later_local = "2026-01-01T00:00:00-05:00";
    let earlier_utc = "2026-01-01T04:00:00Z";
    let summaries = summaries_from_events(&[
        process_event_at(later_local, 0),
        process_event_at(earlier_utc, 1),
    ]);

    assert_eq!(summaries.len(), 1);
    assert_eq!(summaries[0].running_duration_ms, 60000);
    assert_eq!(summaries[0].observation_gap_ms, 60000);
    assert_eq!(summaries[0].last_observed_at, later_local);
}

#[test]
fn process_generation_identity_prevents_pid_reuse_from_merging_sessions() {
    let first = process_event(constants::activity_store::TEST_FIRST_OBSERVED_AT, 0);
    let first_identity = format!(
        "{}-100",
        constants::activity_capture::PROCESS_SUBJECT_ID_PREFIX
    );
    let second = process_event(constants::activity_store::TEST_SECOND_OBSERVED_AT, 1);
    let second_identity = format!(
        "{}-200",
        constants::activity_capture::PROCESS_SUBJECT_ID_PREFIX
    );
    let summaries = summaries_from_events(&[
        process_event_with_identity(first, &first_identity),
        process_event_with_identity(second, &second_identity),
    ]);

    assert_eq!(summaries.len(), 2);
    assert!(summaries
        .iter()
        .any(|summary| summary.primary_process_identity == first_identity));
    assert!(summaries
        .iter()
        .any(|summary| summary.primary_process_identity == second_identity));
}

#[test]
fn read_model_refresh_does_not_create_duration() {
    let store =
        ActivityStore::open_in_memory().expect_value(constants::error::ACTIVITY_STORE_OPENS);
    store
        .ingest_events(&[
            process_event(constants::activity_store::TEST_FIRST_OBSERVED_AT, 0),
            process_event(constants::activity_store::TEST_SECOND_OBSERVED_AT, 1),
        ])
        .expect_value(constants::error::ACTIVITY_STORE_INGESTS);

    let first = store
        .app_game_session_summaries(constants::activity_store::DEFAULT_RECENT_LIMIT)
        .expect_value(constants::error::ACTIVITY_STORE_QUERIES);
    let second = store
        .app_game_session_summaries(constants::activity_store::DEFAULT_RECENT_LIMIT)
        .expect_value(constants::error::ACTIVITY_STORE_QUERIES);

    assert_eq!(first, second);
}

#[test]
fn generic_window_observation_uses_window_subject_fallback() {
    let summaries = summaries_from_events(&[
        process_event(constants::activity_store::TEST_FIRST_OBSERVED_AT, 0),
        active_window_event(constants::activity_store::TEST_FIRST_OBSERVED_AT),
        process_event(constants::activity_store::TEST_SECOND_OBSERVED_AT, 1),
        other_window_event(constants::activity_store::TEST_SECOND_OBSERVED_AT),
        process_event(constants::activity_store::TEST_THIRD_OBSERVED_AT, 2),
    ]);
    let game_summary = summaries
        .iter()
        .find(|summary| {
            summary.primary_process_identity == constants::activity_store::TEST_PROCESS_SUBJECT_ID
        })
        .expect_value(constants::error::ACTIVITY_STORE_QUERIES);

    assert_eq!(game_summary.running_duration_ms, 120000);
    assert_eq!(game_summary.foreground_duration_ms, 0);
    assert_eq!(game_summary.background_duration_ms, 120000);
    assert_eq!(game_summary.last_foreground_at, None);
    assert_eq!(
        game_summary.last_background_at,
        Some(constants::activity_store::TEST_THIRD_OBSERVED_AT.to_string())
    );
    assert!(summaries.iter().any(|summary| {
        summary.primary_process_identity == constants::activity_store::TEST_WINDOW_SUBJECT_ID
    }));
}

#[test]
fn replay_order_reconstructs_same_summary() {
    let chronological = summaries_from_events(&[
        process_event(constants::activity_store::TEST_FIRST_OBSERVED_AT, 0),
        process_event(constants::activity_store::TEST_SECOND_OBSERVED_AT, 1),
    ]);
    let reverse_ingest = summaries_from_events(&[
        process_event(constants::activity_store::TEST_SECOND_OBSERVED_AT, 1),
        process_event(constants::activity_store::TEST_FIRST_OBSERVED_AT, 0),
    ]);

    assert_eq!(chronological, reverse_ingest);
}

#[test]
fn daily_rollup_sums_session_durations_by_day_and_classification() {
    let summaries = summaries_from_events(&[
        process_event(constants::activity_store::TEST_FIRST_OBSERVED_AT, 0),
        process_event(constants::activity_store::TEST_SECOND_OBSERVED_AT, 1),
    ]);
    let rollups = daily_rollups_from_summaries(&summaries);

    assert_eq!(rollups.len(), 1);
    assert_eq!(
        rollups[0].rollup_date,
        constants::activity_store::TEST_ROLLUP_DATE
    );
    assert_eq!(rollups[0].session_count, 1);
    assert_eq!(rollups[0].running_duration_ms, 60000);
    assert_eq!(rollups[0].foreground_duration_ms, 0);
    assert_eq!(rollups[0].background_duration_ms, 60000);
    assert_eq!(
        rollups[0].session_ids,
        vec![summaries[0].session_id.clone()]
    );
}

fn summaries_from_events(events: &[ActivityEvent]) -> Vec<AppGameSessionSummary> {
    let store =
        ActivityStore::open_in_memory().expect_value(constants::error::ACTIVITY_STORE_OPENS);
    store
        .ingest_events(events)
        .expect_value(constants::error::ACTIVITY_STORE_INGESTS);
    let rows = app_game_rows(
        store.connection_for_test(),
        constants::activity_store::DEFAULT_RECENT_LIMIT,
    )
    .expect_value(constants::error::ACTIVITY_STORE_QUERIES);
    session_summaries_from_rows(rows, constants::activity_store::DEFAULT_RECENT_LIMIT)
}

fn process_event(observed_at: impl Display, sequence_index: usize) -> ActivityEvent {
    let observed_at = observed_at.to_string();
    process_event_at(observed_at.as_str(), sequence_index)
}

fn process_event_at(observed_at: &str, sequence_index: usize) -> ActivityEvent {
    process_observation_event(process_observation(), observed_at, sequence_index)
}

fn process_exit_event(observed_at: impl Display, sequence_index: usize) -> ActivityEvent {
    let mut event = process_event(observed_at, sequence_index);
    event.fields.insert(
        constants::field::OBSERVATION_MODE.to_string(),
        LogFieldValue::String(APP_GAME_OBSERVATION_MODE_PROCESS_EXIT.to_string()),
    );
    event
}

fn process_observation() -> ProcessObservation {
    ProcessObservation {
        pid: 4242,
        name: constants::activity_store::TEST_APP_GAME_PROCESS_NAME.to_string(),
        executable_path: Some(std::path::PathBuf::from(
            constants::activity_store::TEST_APP_GAME_PROCESS_PATH,
        )),
    }
}

fn active_window_event(observed_at: impl Display) -> ActivityEvent {
    let observed_at = observed_at.to_string();
    foreground_window_observation_event(
        ForegroundWindowObservation::active(
            4242,
            constants::activity_store::TEST_APP_GAME_PROCESS_NAME.to_string(),
            constants::activity_store::TEST_APP_GAME_PROCESS_PATH.to_string(),
            constants::activity_store::TEST_APP_GAME_WINDOW_TITLE.to_string(),
            constants::activity_store::TEST_WINDOW_ID.to_string(),
        ),
        observed_at.as_str(),
    )
}

fn active_window_event_for_process(
    observed_at: impl Display,
    process_identity: &str,
) -> ActivityEvent {
    let mut event = active_window_event(observed_at);
    event.fields.insert(
        constants::field::PROCESS_IDENTITY.to_string(),
        LogFieldValue::String(process_identity.to_string()),
    );
    event
}

fn process_event_with_identity(mut event: ActivityEvent, process_identity: &str) -> ActivityEvent {
    event.subject.subject_id = process_identity.to_string();
    event
}

fn other_window_event(observed_at: impl Display) -> ActivityEvent {
    let observed_at = observed_at.to_string();
    foreground_window_observation_event(
        ForegroundWindowObservation::active(
            5150,
            constants::activity_store::TEST_PROCESS_SUBJECT_NAME.to_string(),
            constants::activity_store::TEST_PROCESS_SUBJECT_NAME.to_string(),
            constants::activity_store::TEST_WINDOW_TITLE.to_string(),
            constants::activity_store::TEST_WINDOW_TITLE.to_string(),
        ),
        observed_at.as_str(),
    )
}
