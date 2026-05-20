use std::fs::{read_to_string, remove_file};

use ocentra_parent_agent_protocol::{
    constants, ActivityEvent, ActivityEventKind, ActivityJournalRotationPolicy, ActivityObserver,
    ActivitySource, ActivitySubject, ActivitySubjectKind, LogFieldValue, LogFields,
    ACTIVITY_SCHEMA_VERSION,
};

use super::{ActivityJournal, JournalError, JournalKey, JOURNAL_KEY_BYTES};

#[test]
fn journal_appends_encrypted_activity_without_plaintext_payload() {
    let path = temp_journal_path(constants::journal::TEST_APPEND_SUFFIX);
    cleanup_journal_files(&path);
    let mut journal =
        ActivityJournal::open(path.clone(), test_key()).expect(constants::error::JOURNAL_OPENS);
    let event = activity_event(constants::event_id::HEALTH_REPORTED);

    let line = journal
        .append(&event)
        .expect(constants::error::JOURNAL_APPENDS);
    let raw = read_to_string(&path).expect(constants::error::JOURNAL_READS);
    cleanup_journal_files(&path);

    assert_eq!(line.event_id, constants::event_id::HEALTH_REPORTED);
    assert!(!raw.contains(constants::field::ONLINE));
    assert!(!raw.contains(constants::peer::LOCAL_DEV_AGENT));
    assert_eq!(journal.status().entries_written, 1);
}

#[test]
fn journal_replays_decrypted_activity_event() {
    let path = temp_journal_path(constants::journal::TEST_REPLAY_SUFFIX);
    cleanup_journal_files(&path);
    let key = test_key();
    let event = activity_event(constants::event_id::HEALTH_REPORTED);
    let mut journal =
        ActivityJournal::open(path.clone(), key.clone()).expect(constants::error::JOURNAL_OPENS);

    journal
        .append(&event)
        .expect(constants::error::JOURNAL_APPENDS);
    let reader = ActivityJournal::open(path.clone(), key).expect(constants::error::JOURNAL_OPENS);
    let lines = reader.lines().expect(constants::error::JOURNAL_READS);
    let replayed = reader
        .decrypt_line(&lines[0])
        .expect(constants::error::JOURNAL_DECRYPTS);
    cleanup_journal_files(&path);

    assert_eq!(lines.len(), 1);
    assert_eq!(replayed, event);
    assert_eq!(reader.status().entries_written, 1);
}

#[test]
fn journal_rotates_and_replays_segments_in_write_order() {
    let path = temp_journal_path(constants::journal::TEST_ROTATION_SUFFIX);
    cleanup_journal_files(&path);
    let key = test_key();
    let policy = ActivityJournalRotationPolicy {
        max_segment_bytes: constants::journal::TEST_ROTATION_BYTES,
    };
    let first = activity_event(constants::event_id::HEALTH_REPORTED);
    let second = activity_event(constants::event_id::LOG_SNAPSHOT_REPORTED);
    let mut journal = ActivityJournal::open_with_policy(path.clone(), key.clone(), policy)
        .expect(constants::error::JOURNAL_OPENS);

    journal
        .append(&first)
        .expect(constants::error::JOURNAL_APPENDS);
    journal
        .append(&second)
        .expect(constants::error::JOURNAL_APPENDS);
    let reader = ActivityJournal::open_with_policy(
        path.clone(),
        key,
        ActivityJournalRotationPolicy {
            max_segment_bytes: constants::journal::TEST_ROTATION_BYTES,
        },
    )
    .expect(constants::error::JOURNAL_OPENS);
    let lines = reader.lines().expect(constants::error::JOURNAL_READS);
    let replayed_first = reader
        .decrypt_line(&lines[0])
        .expect(constants::error::JOURNAL_DECRYPTS);
    let replayed_second = reader
        .decrypt_line(&lines[1])
        .expect(constants::error::JOURNAL_DECRYPTS);
    cleanup_journal_files(&path);

    assert_eq!(lines.len(), 2);
    assert_ne!(lines[0].segment_id, lines[1].segment_id);
    assert_eq!(replayed_first.event_id, first.event_id);
    assert_eq!(replayed_second.event_id, second.event_id);
    assert_eq!(reader.status().segment_count, 2);
}

#[test]
fn journal_rejects_tampered_ciphertext() {
    let path = temp_journal_path(constants::journal::TEST_TAMPER_SUFFIX);
    cleanup_journal_files(&path);
    let mut journal =
        ActivityJournal::open(path.clone(), test_key()).expect(constants::error::JOURNAL_OPENS);

    let mut line = journal
        .append(&activity_event(constants::event_id::HEALTH_REPORTED))
        .expect(constants::error::JOURNAL_APPENDS);
    line.ciphertext = line.nonce.clone();
    let result = journal.decrypt_line(&line);
    cleanup_journal_files(&path);

    assert!(matches!(result, Err(JournalError::Crypto)));
}

fn activity_event(event_id: &str) -> ActivityEvent {
    let mut fields = LogFields::new();
    fields.insert(
        constants::field::ONLINE.to_string(),
        LogFieldValue::Boolean(true),
    );

    ActivityEvent {
        schema_version: ACTIVITY_SCHEMA_VERSION,
        event_id: event_id.to_string(),
        observed_at: std::process::id().to_string(),
        source: ActivitySource {
            device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            platform: std::env::consts::OS.to_string(),
            observer: ActivityObserver::AgentService,
            source_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
        },
        kind: ActivityEventKind::DeviceIdleStateObserved,
        subject: ActivitySubject {
            kind: ActivitySubjectKind::Device,
            subject_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            display_name: None,
        },
        fields,
        evidence: Vec::new(),
    }
}

fn temp_journal_path(suffix: &str) -> std::path::PathBuf {
    let mut name = String::from(constants::journal::TEST_FILE_PREFIX);
    name.push_str(&std::process::id().to_string());
    name.push(constants::delimiter::HYPHEN);
    name.push_str(suffix);

    let mut path = std::env::temp_dir();
    path.push(name);
    path.set_extension(constants::journal::FILE_EXTENSION);
    path
}

fn cleanup_journal_files(path: &std::path::PathBuf) {
    let _ = remove_file(path);
    for index in 1..=3 {
        let mut rotated_path = path.clone();
        let mut extension = index.to_string();
        extension.push(constants::delimiter::DOT);
        extension.push_str(constants::journal::FILE_EXTENSION);
        rotated_path.set_extension(extension);
        let _ = remove_file(rotated_path);
    }
}

fn test_key() -> JournalKey {
    JournalKey::from_bytes([7; JOURNAL_KEY_BYTES])
}
