use std::{
    error::Error,
    fs::{read, remove_file, write},
    io::Error as IoError,
};

use crate::test_text::TestText;
use ocentra_parent_agent_core::activity_store::ActivityStore;
use ocentra_parent_agent_core::journal::ActivityJournal;
use ocentra_parent_agent_core::journal_crypto::{JournalKey, JOURNAL_KEY_BYTES};
use ocentra_parent_agent_protocol::activity::{
    ActivityEvent, ActivityEventKind, ActivityObserver, ActivitySource, ActivitySubject,
    ActivitySubjectKind, ACTIVITY_SCHEMA_VERSION,
};
#[cfg(windows)]
use ocentra_parent_agent_protocol::app_game::{
    APP_GAME_CLASSIFICATION_UNKNOWN_PROCESS, APP_GAME_CONTENT_KNOWLEDGE_NOT_CLAIMED,
    APP_GAME_FOREGROUND_FOREGROUND, APP_GAME_FOREGROUND_NOT_CLAIMED, APP_GAME_RUNTIME_RUNNING,
    APP_GAME_WINDOW_REF_PREFIX, APP_GAME_WINDOW_TITLE_REF_PREFIX,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};
use ocentra_parent_agent_service::test_support::{
    record_activity_capture_to_paths_for_test, record_activity_events_to_paths_for_test,
    startup_activity_capture_enabled_for_value_for_test,
};

type TestResult = Result<(), Box<dyn Error>>;

const FAIL_ACTIVITY_INSERT_TRIGGER: &str = "
CREATE TRIGGER activity_capture_test_fail_insert
BEFORE INSERT ON activity_events
BEGIN
  SELECT RAISE(ABORT, 'forced activity ingest failure');
END;
";
const DROP_FAIL_ACTIVITY_INSERT_TRIGGER: &str = "DROP TRIGGER activity_capture_test_fail_insert;";

#[test]
fn startup_activity_capture_can_be_suppressed_for_isolated_service_proofs() {
    assert!(!startup_activity_capture_enabled_for_value_for_test(Some(
        TestText::from_display(constants::value::TRUE)
    )));
}

#[test]
fn record_process_snapshot_writes_encrypted_journal_and_sqlite_rows() -> TestResult {
    let build_path = |suffix: &str, extension: &str| {
        let mut name = String::from(constants::activity_store::TEST_FILE_PREFIX);
        name.push_str(&std::process::id().to_string());
        name.push(constants::delimiter::HYPHEN);
        name.push_str(suffix);

        let mut path = std::env::temp_dir();
        path.push(name);
        path.set_extension(extension);
        path
    };
    let journal_path = build_path(
        constants::activity_store::TEST_CAPTURE_JOURNAL_SUFFIX,
        constants::journal::FILE_EXTENSION,
    );
    let key_path = build_path(
        constants::activity_store::TEST_CAPTURE_KEY_SUFFIX,
        constants::activity_store::FILE_EXTENSION,
    );
    let store_path = build_path(
        constants::activity_store::TEST_CAPTURE_STORE_SUFFIX,
        constants::activity_store::FILE_EXTENSION,
    );
    cleanup_paths(&journal_path, &key_path, &store_path);

    let result = (|| -> TestResult {
        let status =
            record_activity_capture_to_paths_for_test(&journal_path, &key_path, &store_path, 1, 1)
                .map_err(|error| {
                    IoError::other(format!(
                        "{}: {error:?}",
                        constants::error::ACTIVITY_CAPTURE_RECORDS
                    ))
                })?;
        let journal_bytes = read(&journal_path)?;
        let store = ActivityStore::open(&store_path).map_err(|error| {
            IoError::other(format!(
                "{}: {error:?}",
                constants::error::ACTIVITY_STORE_OPENS
            ))
        })?;
        let summary = store
            .recent_summary(constants::activity_store::DEFAULT_RECENT_LIMIT)
            .map_err(|error| {
                IoError::other(format!(
                    "{}: {error:?}",
                    constants::error::ACTIVITY_STORE_QUERIES
                ))
            })?;
        let app_game = store
            .app_game_service_read_model(
                constants::activity_store::DEFAULT_RECENT_LIMIT,
                constants::activity_store::TEST_THIRD_OBSERVED_AT,
            )
            .map_err(|error| {
                IoError::other(format!(
                    "{}: {error:?}",
                    constants::error::ACTIVITY_STORE_QUERIES
                ))
            })?;

        assert_capture_event_count(status.events_ingested);
        assert_capture_event_count(status.events_stored);
        assert!(!String::from_utf8_lossy(&journal_bytes)
            .contains(constants::activity_store::TEST_PROCESS_SUBJECT_NAME));
        assert!(matches!(
            summary.most_recent_kind,
            Some(ActivityEventKind::WindowFocused) | Some(ActivityEventKind::ProcessObserved)
        ));
        assert!(matches!(
            summary.most_recent_observer,
            Some(ActivityObserver::WindowsWindow) | Some(ActivityObserver::WindowsProcess)
        ));
        assert_app_game_capture_read_model(&app_game);

        Ok(())
    })();

    cleanup_paths(&journal_path, &key_path, &store_path);

    result
}

#[test]
fn record_process_snapshot_reuses_journal_key_for_replay() -> TestResult {
    let build_path = |suffix: &str, extension: &str| {
        let mut name = String::from(constants::activity_store::TEST_FILE_PREFIX);
        name.push_str(&std::process::id().to_string());
        name.push(constants::delimiter::HYPHEN);
        name.push_str(suffix);

        let mut path = std::env::temp_dir();
        path.push(name);
        path.set_extension(extension);
        path
    };
    let journal_path = build_path(
        constants::activity_store::TEST_CAPTURE_REPLAY_JOURNAL_SUFFIX,
        constants::journal::FILE_EXTENSION,
    );
    let key_path = build_path(
        constants::activity_store::TEST_CAPTURE_REPLAY_KEY_SUFFIX,
        constants::activity_store::FILE_EXTENSION,
    );
    let store_path = build_path(
        constants::activity_store::TEST_CAPTURE_REPLAY_STORE_SUFFIX,
        constants::activity_store::FILE_EXTENSION,
    );
    cleanup_paths(&journal_path, &key_path, &store_path);

    let result = (|| -> TestResult {
        record_activity_capture_to_paths_for_test(&journal_path, &key_path, &store_path, 1, 1)
            .map_err(|error| {
                IoError::other(format!(
                    "{}: {error:?}",
                    constants::error::ACTIVITY_CAPTURE_RECORDS
                ))
            })?;
        let key_bytes = read(&key_path)?;
        let mut key = [0; JOURNAL_KEY_BYTES];
        key.copy_from_slice(&key_bytes);
        let journal = ActivityJournal::open(journal_path.clone(), JournalKey::from_bytes(key))
            .map_err(|error| {
                IoError::other(format!("{}: {error:?}", constants::error::JOURNAL_OPENS))
            })?;
        let lines = journal.lines().map_err(|error| {
            IoError::other(format!("{}: {error:?}", constants::error::JOURNAL_READS))
        })?;
        assert_optional_foreground_event_count(lines.len() as u64);
        let process_event = journal.decrypt_line(&lines[0]).map_err(|error| {
            IoError::other(format!("{}: {error:?}", constants::error::JOURNAL_DECRYPTS))
        })?;
        let window_event = journal.decrypt_line(&lines[1]).map_err(|error| {
            IoError::other(format!("{}: {error:?}", constants::error::JOURNAL_DECRYPTS))
        })?;
        let network_event = journal.decrypt_line(&lines[2]).map_err(|error| {
            IoError::other(format!("{}: {error:?}", constants::error::JOURNAL_DECRYPTS))
        })?;

        assert_eq!(process_event.kind, ActivityEventKind::ProcessObserved);
        assert_eq!(
            process_event.source.observer,
            ActivityObserver::WindowsProcess
        );
        assert_eq!(window_event.kind, ActivityEventKind::WindowFocused);
        assert_eq!(
            window_event.source.observer,
            ActivityObserver::WindowsWindow
        );
        assert_eq!(network_event.kind, ActivityEventKind::DomainObserved);
        assert_eq!(
            network_event.source.observer,
            ActivityObserver::WindowsNetwork
        );

        Ok(())
    })();

    cleanup_paths(&journal_path, &key_path, &store_path);

    result
}

#[test]
fn retry_after_sqlite_ingest_failure_reuses_durable_journal_event() -> TestResult {
    let unique_suffix = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_nanos()
        .to_string();
    let mut journal_path = std::env::temp_dir();
    journal_path.push(format!(
        "{}{}-{}",
        constants::activity_store::TEST_FILE_PREFIX,
        std::process::id(),
        unique_suffix
    ));
    journal_path.set_extension(constants::journal::FILE_EXTENSION);
    let mut key_path = journal_path.clone();
    key_path.set_extension(constants::activity_store::TEST_CAPTURE_REPLAY_KEY_SUFFIX);
    let mut store_path = journal_path.clone();
    store_path.set_extension(constants::activity_store::TEST_CAPTURE_REPLAY_STORE_SUFFIX);
    cleanup_paths(&journal_path, &key_path, &store_path);

    let result = (|| -> TestResult {
        let store = ActivityStore::open(&store_path).map_err(debug_io_error)?;
        store
            .connection_for_test()
            .execute_batch(FAIL_ACTIVITY_INSERT_TRIGGER)?;
        drop(store);

        let event = deterministic_process_event();
        let failed = record_activity_events_to_paths_for_test(
            &journal_path,
            &key_path,
            &store_path,
            std::slice::from_ref(&event),
        );
        assert!(
            failed.is_err(),
            "the SQLite insert trigger must fail ingest"
        );

        let journal = open_test_journal(&journal_path, &key_path)?;
        let failed_lines = journal.lines().map_err(debug_io_error)?;
        assert_eq!(
            failed_lines.len(),
            1,
            "the failed ingest must leave one durable journal event"
        );
        let failed_event = journal
            .decrypt_line(&failed_lines[0])
            .map_err(debug_io_error)?;
        assert_eq!(failed_event.event_id, event.event_id);
        drop(journal);

        let store = ActivityStore::open(&store_path).map_err(debug_io_error)?;
        assert!(!store
            .contains_event_id(&event.event_id)
            .map_err(debug_io_error)?);
        store
            .connection_for_test()
            .execute_batch(DROP_FAIL_ACTIVITY_INSERT_TRIGGER)?;
        drop(store);

        let recovered = record_activity_events_to_paths_for_test(
            &journal_path,
            &key_path,
            &store_path,
            std::slice::from_ref(&event),
        )
        .map_err(|error| {
            IoError::other(format!(
                "{}: {error:?}",
                constants::error::ACTIVITY_CAPTURE_RECORDS
            ))
        })?;
        let restarted_journal = open_test_journal(&journal_path, &key_path)?;
        let recovered_lines = restarted_journal.lines().map_err(debug_io_error)?;
        let restarted_store = ActivityStore::open(&store_path).map_err(debug_io_error)?;

        assert_eq!(
            recovered_lines.len(),
            1,
            "retry must replay the durable event without a second journal append"
        );
        assert_eq!(recovered.events_ingested, 1);
        assert_eq!(recovered.events_stored, 1);
        assert_eq!(
            recovered.last_event_id.as_deref(),
            Some(event.event_id.as_str())
        );
        assert!(restarted_store
            .contains_event_id(&event.event_id)
            .map_err(debug_io_error)?);

        Ok(())
    })();

    cleanup_paths(&journal_path, &key_path, &store_path);

    result
}

#[test]
fn record_process_snapshot_rejects_invalid_journal_key() -> TestResult {
    let build_path = |suffix: &str, extension: &str| {
        let mut name = String::from(constants::activity_store::TEST_FILE_PREFIX);
        name.push_str(&std::process::id().to_string());
        name.push(constants::delimiter::HYPHEN);
        name.push_str(suffix);

        let mut path = std::env::temp_dir();
        path.push(name);
        path.set_extension(extension);
        path
    };
    let journal_path = build_path(
        constants::activity_store::TEST_CAPTURE_INVALID_KEY_JOURNAL_SUFFIX,
        constants::journal::FILE_EXTENSION,
    );
    let key_path = build_path(
        constants::activity_store::TEST_CAPTURE_INVALID_KEY_SUFFIX,
        constants::activity_store::FILE_EXTENSION,
    );
    let store_path = build_path(
        constants::activity_store::TEST_CAPTURE_INVALID_KEY_STORE_SUFFIX,
        constants::activity_store::FILE_EXTENSION,
    );
    cleanup_paths(&journal_path, &key_path, &store_path);
    write(&key_path, [])?;

    let result = match record_activity_capture_to_paths_for_test(
        &journal_path,
        &key_path,
        &store_path,
        1,
        1,
    ) {
        Ok(_) => Err(IoError::other(constants::error::ACTIVITY_CAPTURE_REJECTS_INVALID_KEY).into()),
        Err(error) => {
            assert_eq!(
                error.reason().0,
                constants::value::ACTIVITY_CAPTURE_INVALID_KEY_LENGTH
            );
            Ok(())
        }
    };

    cleanup_paths(&journal_path, &key_path, &store_path);

    result
}

fn open_test_journal(
    journal_path: impl AsRef<std::path::Path>,
    key_path: impl AsRef<std::path::Path>,
) -> Result<ActivityJournal, Box<dyn Error>> {
    let key_bytes = read(key_path)?;
    let mut key = [0; JOURNAL_KEY_BYTES];
    key.copy_from_slice(&key_bytes);
    ActivityJournal::open(
        journal_path.as_ref().to_path_buf(),
        JournalKey::from_bytes(key),
    )
    .map_err(|error| {
        IoError::other(format!("{}: {error:?}", constants::error::JOURNAL_OPENS)).into()
    })
}

fn debug_io_error(error: impl std::fmt::Debug) -> IoError {
    IoError::other(format!("{error:?}"))
}

fn deterministic_process_event() -> ActivityEvent {
    let mut fields = LogFields::new();
    fields.insert(
        constants::field::PID.to_string(),
        LogFieldValue::Number(4242.0),
    );

    ActivityEvent {
        schema_version: ACTIVITY_SCHEMA_VERSION,
        event_id: constants::event_id::HEALTH_REPORTED.to_string(),
        observed_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        source: ActivitySource {
            device_id: constants::activity_surface::DEFAULT_DEVICE_ID.to_string(),
            platform:
                ocentra_parent_agent_protocol::policy_constants::TEST_PARENT_DEVICE_PLATFORM_WINDOWS
                    .to_string(),
            observer: ActivityObserver::WindowsProcess,
            source_id: constants::activity_surface::DEFAULT_DEVICE_ID.to_string(),
        },
        kind: ActivityEventKind::ProcessObserved,
        subject: ActivitySubject {
            kind: ActivitySubjectKind::Process,
            subject_id: constants::activity_store::TEST_PROCESS_SUBJECT_ID.to_string(),
            display_name: Some(constants::activity_store::TEST_PROCESS_SUBJECT_NAME.to_string()),
        },
        fields,
        evidence: Vec::new(),
    }
}

fn cleanup_paths(
    journal_path: impl AsRef<std::path::Path>,
    key_path: impl AsRef<std::path::Path>,
    store_path: impl AsRef<std::path::Path>,
) {
    let journal_path = journal_path.as_ref().to_path_buf();
    let key_path = key_path.as_ref().to_path_buf();
    let store_path = store_path.as_ref().to_path_buf();
    let _ = remove_file(&journal_path);
    let _ = remove_file(&key_path);
    let _ = remove_file(&store_path);
    let mut store_wal_path = store_path.clone();
    store_wal_path.set_extension(constants::activity_store::WAL_FILE_EXTENSION);
    let _ = remove_file(store_wal_path);
    let mut store_shm_path = store_path;
    store_shm_path.set_extension(constants::activity_store::SHM_FILE_EXTENSION);
    let _ = remove_file(store_shm_path);
    for index in 1..=3 {
        let mut rotated_path = journal_path.clone();
        let mut extension = index.to_string();
        extension.push(constants::delimiter::DOT);
        extension.push_str(constants::journal::FILE_EXTENSION);
        rotated_path.set_extension(extension);
        let _ = remove_file(rotated_path);
    }
}

fn assert_capture_event_count(event_count: u64) {
    let min_count = expected_capture_event_base_count();
    let max_count = expected_capture_event_base_count()
        + constants::activity_capture::APP_GAME_INVENTORY_SNAPSHOT_LIMIT as u64
        + constants::activity_capture::APP_GAME_INVENTORY_SNAPSHOT_LIMIT as u64
        + constants::activity_capture::APP_GAME_INVENTORY_SNAPSHOT_LIMIT as u64
        + 1;
    assert!(event_count >= min_count && event_count <= max_count);
}

fn assert_optional_foreground_event_count(event_count: u64) {
    let base_count = expected_capture_event_base_count();
    let max_count = base_count
        + constants::activity_capture::APP_GAME_INVENTORY_SNAPSHOT_LIMIT as u64
        + constants::activity_capture::APP_GAME_INVENTORY_SNAPSHOT_LIMIT as u64
        + constants::activity_capture::APP_GAME_INVENTORY_SNAPSHOT_LIMIT as u64
        + 1;
    assert!(event_count >= base_count && event_count <= max_count);
}

#[cfg(windows)]
fn expected_capture_event_base_count() -> u64 {
    4
}

#[cfg(not(windows))]
fn expected_capture_event_base_count() -> u64 {
    3
}

#[cfg(windows)]
fn assert_app_game_capture_read_model(
    model: &ocentra_parent_agent_protocol::app_game::AppGameServiceReadModel,
) {
    assert_eq!(model.running_now_returned, 1);
    assert_eq!(
        model.running_now_rows[0].runtime_state,
        APP_GAME_RUNTIME_RUNNING
    );
    assert_eq!(
        model.running_now_rows[0].foreground_state,
        APP_GAME_FOREGROUND_NOT_CLAIMED
    );
    assert_eq!(
        model.running_now_rows[0].classification_state,
        APP_GAME_CLASSIFICATION_UNKNOWN_PROCESS
    );
    assert!(model.foreground_now_returned <= 1);
    if let Some(row) = model.foreground_now_rows.first() {
        assert_eq!(row.runtime_state, APP_GAME_RUNTIME_RUNNING);
        assert_eq!(row.foreground_state, APP_GAME_FOREGROUND_FOREGROUND);
        assert_eq!(
            row.content_knowledge_state,
            APP_GAME_CONTENT_KNOWLEDGE_NOT_CLAIMED
        );
        if let Some(window_ref) = &row.window_ref {
            assert!(window_ref.starts_with(APP_GAME_WINDOW_REF_PREFIX));
        }
        if let Some(title_ref) = &row.window_title_ref {
            assert!(title_ref.starts_with(APP_GAME_WINDOW_TITLE_REF_PREFIX));
        }
    }
}

#[cfg(not(windows))]
fn assert_app_game_capture_read_model(
    model: &ocentra_parent_agent_protocol::app_game::AppGameServiceReadModel,
) {
    assert_eq!(model.running_now_returned, 0);
}
