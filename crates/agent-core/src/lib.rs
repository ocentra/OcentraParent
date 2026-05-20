#![forbid(unsafe_code)]

mod journal;
mod journal_crypto;
mod journal_error;

pub use journal::ActivityJournal;
pub use journal_crypto::{JournalKey, JOURNAL_KEY_BYTES};
pub use journal_error::JournalError;

pub fn crate_name() -> &'static str {
    env!("CARGO_PKG_NAME")
}

#[cfg(test)]
mod tests {
    use std::fs::{read_to_string, remove_file};

    use ocentra_parent_agent_protocol::{
        constants, ActivityEvent, ActivityEventKind, ActivityObserver, ActivitySource,
        ActivitySubject, ActivitySubjectKind, LogFieldValue, LogFields, ACTIVITY_SCHEMA_VERSION,
    };

    use super::{crate_name, ActivityJournal, JournalError, JournalKey, JOURNAL_KEY_BYTES};

    #[test]
    fn crate_name_identifies_agent_core_boundary() {
        assert_eq!(crate_name(), env!("CARGO_PKG_NAME"));
    }

    #[test]
    fn journal_appends_encrypted_activity_without_plaintext_payload() {
        let path = temp_journal_path(constants::journal::TEST_APPEND_SUFFIX);
        let _ = remove_file(&path);
        let mut journal =
            ActivityJournal::open(path.clone(), test_key()).expect(constants::error::JOURNAL_OPENS);
        let event = activity_event();

        let line = journal
            .append(&event)
            .expect(constants::error::JOURNAL_APPENDS);
        let raw = read_to_string(&path).expect(constants::error::JOURNAL_READS);
        let _ = remove_file(&path);

        assert_eq!(line.event_id, constants::event_id::HEALTH_REPORTED);
        assert!(!raw.contains(constants::field::ONLINE));
        assert!(!raw.contains(constants::peer::LOCAL_DEV_AGENT));
        assert_eq!(journal.status().entries_written, 1);
    }

    #[test]
    fn journal_replays_decrypted_activity_event() {
        let path = temp_journal_path(constants::journal::TEST_REPLAY_SUFFIX);
        let _ = remove_file(&path);
        let key = test_key();
        let event = activity_event();
        let mut journal = ActivityJournal::open(path.clone(), key.clone())
            .expect(constants::error::JOURNAL_OPENS);

        journal
            .append(&event)
            .expect(constants::error::JOURNAL_APPENDS);
        let reader =
            ActivityJournal::open(path.clone(), key).expect(constants::error::JOURNAL_OPENS);
        let lines = reader.lines().expect(constants::error::JOURNAL_READS);
        let replayed = reader
            .decrypt_line(&lines[0])
            .expect(constants::error::JOURNAL_DECRYPTS);
        let _ = remove_file(&path);

        assert_eq!(lines.len(), 1);
        assert_eq!(replayed, event);
        assert_eq!(reader.status().entries_written, 1);
    }

    #[test]
    fn journal_rejects_tampered_ciphertext() {
        let path = temp_journal_path(constants::journal::TEST_TAMPER_SUFFIX);
        let _ = remove_file(&path);
        let mut journal =
            ActivityJournal::open(path.clone(), test_key()).expect(constants::error::JOURNAL_OPENS);

        let mut line = journal
            .append(&activity_event())
            .expect(constants::error::JOURNAL_APPENDS);
        line.ciphertext = line.nonce.clone();
        let result = journal.decrypt_line(&line);
        let _ = remove_file(&path);

        assert!(matches!(result, Err(JournalError::Crypto)));
    }

    fn activity_event() -> ActivityEvent {
        let mut fields = LogFields::new();
        fields.insert(
            constants::field::ONLINE.to_string(),
            LogFieldValue::Boolean(true),
        );

        ActivityEvent {
            schema_version: ACTIVITY_SCHEMA_VERSION,
            event_id: constants::event_id::HEALTH_REPORTED.to_string(),
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

    fn test_key() -> JournalKey {
        JournalKey::from_bytes([7; JOURNAL_KEY_BYTES])
    }
}
