use std::fs::{read_to_string, remove_file};
use std::path::{Path, PathBuf};

use ocentra_parent_agent_protocol::activity::ACTIVITY_SCHEMA_VERSION;
use ocentra_parent_agent_protocol::activity::{
    ActivityEvent, ActivityEventKind, ActivityObserver, ActivitySource, ActivitySubject,
    ActivitySubjectKind,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::journal::{ActivityJournalLine, ActivityJournalRotationPolicy};
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};

use crate::{
    journal::ActivityJournal,
    journal_crypto::{JournalKey, JOURNAL_KEY_BYTES},
    journal_error::JournalError,
};

use crate::test_text::{test_ok as ok, TestResult, TestText};

#[derive(Clone)]
struct TestPath(PathBuf);

impl AsRef<Path> for TestPath {
    fn as_ref(&self) -> &Path {
        self.0.as_ref()
    }
}

#[test]
fn journal_appends_encrypted_activity_without_plaintext_payload() -> TestResult {
    let path = temp_journal_path(constants::journal::TEST_APPEND_SUFFIX);
    cleanup_journal_files(&path);
    let mut journal = ok(
        ActivityJournal::open(path.0.clone(), test_key()),
        constants::error::JOURNAL_OPENS,
    )?;
    let event = activity_event(constants::event_id::HEALTH_REPORTED);

    let line = ok(journal.append(&event), constants::error::JOURNAL_APPENDS)?;
    let raw = ok(read_to_string(&path), constants::error::JOURNAL_READS)?;
    cleanup_journal_files(&path);
    let persisted_line = ok(
        serde_json::from_str::<ActivityJournalLine>(raw.trim()),
        constants::error::JOURNAL_READS,
    )?;
    let expected_raw = ok(
        serde_json::to_string(&line),
        "serialize activity journal line",
    )?;
    let plaintext_event = ok(serde_json::to_string(&event), "serialize activity event")?;

    assert_eq!(line.event_id, constants::event_id::HEALTH_REPORTED);
    assert_eq!(persisted_line, line);
    assert_eq!(raw.trim(), expected_raw);
    assert_ne!(persisted_line.ciphertext, plaintext_event);
    assert_eq!(journal.status().entries_written, 1);

    Ok(())
}

#[test]
fn journal_replays_decrypted_activity_event() -> TestResult {
    let path = temp_journal_path(constants::journal::TEST_REPLAY_SUFFIX);
    cleanup_journal_files(&path);
    let key = test_key();
    let event = activity_event(constants::event_id::HEALTH_REPORTED);
    let mut journal = ok(
        ActivityJournal::open(path.0.clone(), key.clone()),
        constants::error::JOURNAL_OPENS,
    )?;

    ok(journal.append(&event), constants::error::JOURNAL_APPENDS)?;
    let reader = ok(
        ActivityJournal::open(path.0.clone(), key),
        constants::error::JOURNAL_OPENS,
    )?;
    let lines = ok(reader.lines(), constants::error::JOURNAL_READS)?;
    let replayed = ok(
        reader.decrypt_line(&lines[0]),
        constants::error::JOURNAL_DECRYPTS,
    )?;
    cleanup_journal_files(&path);

    assert_eq!(lines.len(), 1);
    assert_eq!(replayed, event);
    assert_eq!(reader.status().entries_written, 1);

    Ok(())
}

#[test]
fn journal_rotates_and_replays_segments_in_write_order() -> TestResult {
    let path = temp_journal_path(constants::journal::TEST_ROTATION_SUFFIX);
    cleanup_journal_files(&path);
    let key = test_key();
    let policy = ActivityJournalRotationPolicy {
        max_segment_bytes: constants::journal::TEST_ROTATION_BYTES,
    };
    let first = activity_event(constants::event_id::HEALTH_REPORTED);
    let second = activity_event(constants::event_id::LOG_SNAPSHOT_REPORTED);
    let mut journal = ok(
        ActivityJournal::open_with_policy(path.0.clone(), key.clone(), policy),
        constants::error::JOURNAL_OPENS,
    )?;

    ok(journal.append(&first), constants::error::JOURNAL_APPENDS)?;
    ok(journal.append(&second), constants::error::JOURNAL_APPENDS)?;
    let reader = ok(
        ActivityJournal::open_with_policy(
            path.0.clone(),
            key,
            ActivityJournalRotationPolicy {
                max_segment_bytes: constants::journal::TEST_ROTATION_BYTES,
            },
        ),
        constants::error::JOURNAL_OPENS,
    )?;
    let lines = ok(reader.lines(), constants::error::JOURNAL_READS)?;
    let replayed_first = ok(
        reader.decrypt_line(&lines[0]),
        constants::error::JOURNAL_DECRYPTS,
    )?;
    let replayed_second = ok(
        reader.decrypt_line(&lines[1]),
        constants::error::JOURNAL_DECRYPTS,
    )?;
    cleanup_journal_files(&path);

    assert_eq!(lines.len(), 2);
    assert_ne!(lines[0].segment_id, lines[1].segment_id);
    assert_eq!(replayed_first.event_id, first.event_id);
    assert_eq!(replayed_second.event_id, second.event_id);
    assert_eq!(reader.status().segment_count, 2);

    Ok(())
}

#[test]
fn journal_rejects_tampered_ciphertext() -> TestResult {
    let path = temp_journal_path(constants::journal::TEST_TAMPER_SUFFIX);
    cleanup_journal_files(&path);
    let mut journal = ok(
        ActivityJournal::open(path.0.clone(), test_key()),
        constants::error::JOURNAL_OPENS,
    )?;

    let mut line = ok(
        journal.append(&activity_event(constants::event_id::HEALTH_REPORTED)),
        constants::error::JOURNAL_APPENDS,
    )?;
    line.ciphertext = line.nonce.clone();
    let result = journal.decrypt_line(&line);
    cleanup_journal_files(&path);

    assert!(matches!(result, Err(JournalError::Crypto)));

    Ok(())
}

fn activity_event(event_id: impl std::fmt::Display) -> ActivityEvent {
    let event_id = TestText::from_display(event_id);
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

fn temp_journal_path(suffix: impl std::fmt::Display) -> TestPath {
    let mut name = String::from(constants::journal::TEST_FILE_PREFIX);
    name.push_str(&std::process::id().to_string());
    name.push(constants::delimiter::HYPHEN);
    name.push_str(&suffix.to_string());

    let mut path = std::env::temp_dir();
    path.push(name);
    path.set_extension(constants::journal::FILE_EXTENSION);
    TestPath(path)
}

fn cleanup_journal_files(path: impl AsRef<Path>) {
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
    JournalKey::from_bytes([7; JOURNAL_KEY_BYTES])
}
