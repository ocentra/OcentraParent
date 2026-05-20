use std::fs::remove_file;

use ocentra_parent_agent_protocol::{
    constants, ActivityEvent, ActivityEventKind, ActivityJournalRotationPolicy, ActivityObserver,
    ActivitySource, ActivitySubject, ActivitySubjectKind, LogFieldValue, LogFields,
    ACTIVITY_SCHEMA_VERSION,
};

use super::{ActivityJournal, ActivityStore, JournalKey, JOURNAL_KEY_BYTES};

#[test]
fn activity_store_ingests_journal_replay_into_duckdb() {
    let journal_path = temp_path(
        constants::activity_store::TEST_JOURNAL_SUFFIX,
        constants::journal::FILE_EXTENSION,
    );
    let store_path = temp_path(
        constants::activity_store::TEST_STORE_SUFFIX,
        constants::activity_store::FILE_EXTENSION,
    );
    cleanup_paths(&journal_path, &store_path);
    let key = test_key();
    let mut journal = ActivityJournal::open_with_policy(
        journal_path.clone(),
        key.clone(),
        ActivityJournalRotationPolicy {
            max_segment_bytes: constants::journal::TEST_ROTATION_BYTES,
        },
    )
    .expect(constants::error::JOURNAL_OPENS);
    journal
        .append(&activity_event(
            constants::event_id::HEALTH_REPORTED,
            constants::activity_store::TEST_FIRST_OBSERVED_AT,
        ))
        .expect(constants::error::JOURNAL_APPENDS);
    journal
        .append(&activity_event(
            constants::event_id::LOG_SNAPSHOT_REPORTED,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        ))
        .expect(constants::error::JOURNAL_APPENDS);
    let reader =
        ActivityJournal::open(journal_path.clone(), key).expect(constants::error::JOURNAL_OPENS);
    let store = ActivityStore::open(&store_path).expect(constants::error::ACTIVITY_STORE_OPENS);

    let status = store
        .ingest_journal(&reader)
        .expect(constants::error::ACTIVITY_STORE_INGESTS);
    let summary = store
        .recent_summary(constants::activity_store::DEFAULT_RECENT_LIMIT)
        .expect(constants::error::ACTIVITY_STORE_QUERIES);
    cleanup_paths(&journal_path, &store_path);

    assert_eq!(status.events_ingested, 2);
    assert_eq!(status.events_stored, 2);
    assert_eq!(status.duplicate_events, 0);
    assert_eq!(
        summary.last_event_id,
        Some(constants::event_id::LOG_SNAPSHOT_REPORTED.to_string())
    );
    assert_eq!(
        summary.most_recent_kind,
        Some(ActivityEventKind::ProcessObserved)
    );
    assert_eq!(summary.returned, 2);
}

#[test]
fn activity_store_reports_duplicate_ingest_without_double_counting() {
    let store = ActivityStore::open_in_memory().expect(constants::error::ACTIVITY_STORE_OPENS);
    let event = activity_event(
        constants::event_id::HEALTH_REPORTED,
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
    );

    let first = store
        .ingest_events(std::slice::from_ref(&event))
        .expect(constants::error::ACTIVITY_STORE_INGESTS);
    let second = store
        .ingest_events(std::slice::from_ref(&event))
        .expect(constants::error::ACTIVITY_STORE_INGESTS);

    assert_eq!(first.events_ingested, 1);
    assert_eq!(first.events_stored, 1);
    assert_eq!(second.events_ingested, 0);
    assert_eq!(second.duplicate_events, 1);
    assert_eq!(second.events_stored, 1);
}

fn activity_event(event_id: &str, observed_at: &str) -> ActivityEvent {
    let mut fields = LogFields::new();
    fields.insert(
        constants::field::PID.to_string(),
        LogFieldValue::Number(std::process::id() as f64),
    );

    ActivityEvent {
        schema_version: ACTIVITY_SCHEMA_VERSION,
        event_id: event_id.to_string(),
        observed_at: observed_at.to_string(),
        source: ActivitySource {
            device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            platform: std::env::consts::OS.to_string(),
            observer: ActivityObserver::WindowsProcess,
            source_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
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

fn temp_path(suffix: &str, extension: &str) -> std::path::PathBuf {
    let mut name = String::from(constants::activity_store::TEST_FILE_PREFIX);
    name.push_str(&std::process::id().to_string());
    name.push(constants::delimiter::HYPHEN);
    name.push_str(suffix);

    let mut path = std::env::temp_dir();
    path.push(name);
    path.set_extension(extension);
    path
}

fn cleanup_paths(journal_path: &std::path::PathBuf, store_path: &std::path::PathBuf) {
    let _ = remove_file(journal_path);
    let _ = remove_file(store_path);
    let mut store_wal_path = store_path.clone();
    store_wal_path.set_extension(constants::activity_store::WAL_EXTENSION);
    let _ = remove_file(store_wal_path);
    for index in 1..=3 {
        let mut rotated_path = journal_path.clone();
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
