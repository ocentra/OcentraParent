use std::{fs, path::Path};

use ocentra_parent_agent_core::{
    activity_store::ActivityStore,
    journal::ActivityJournal,
    journal_crypto::{JournalKey, JOURNAL_KEY_BYTES},
};
use ocentra_parent_agent_protocol::{
    activity::ActivityEvent, activity_query::ActivityIngestStatus,
};

use super::error::ActivityCapturePersistenceError;

pub(super) struct ActivityJournalAppend {
    pub(super) replay_status: ActivityIngestStatus,
    pub(super) appended_events: Vec<ActivityEvent>,
}

pub(super) fn replay_and_append(
    journal_path: &Path,
    key_path: &Path,
    store: &ActivityStore,
    events: Vec<&ActivityEvent>,
) -> Result<ActivityJournalAppend, ActivityCapturePersistenceError> {
    let key = load_or_create_journal_key(key_path)?;
    let mut journal = ActivityJournal::open(journal_path.to_path_buf(), key)?;
    let replay_status = store.ingest_journal(&journal)?;
    let events_missing_after_replay = events_missing_after_replay(store, events)?;
    if events_missing_after_replay.is_empty() {
        return Ok(ActivityJournalAppend {
            replay_status,
            appended_events: Vec::new(),
        });
    }

    let existing_line_count = journal.lines()?.len();
    for event in events_missing_after_replay {
        journal.append(event)?;
    }
    let appended_events = decrypt_appended_events(&journal, existing_line_count)?;
    Ok(ActivityJournalAppend {
        replay_status,
        appended_events,
    })
}

fn events_missing_after_replay<'a>(
    store: &ActivityStore,
    events: Vec<&'a ActivityEvent>,
) -> Result<Vec<&'a ActivityEvent>, ActivityCapturePersistenceError> {
    let mut missing = Vec::new();
    for event in events {
        if !store.contains_event_id(&event.event_id)? {
            missing.push(event);
        }
    }
    Ok(missing)
}

fn decrypt_appended_events(
    journal: &ActivityJournal,
    existing_line_count: usize,
) -> Result<Vec<ActivityEvent>, ActivityCapturePersistenceError> {
    let mut events = Vec::new();
    for line in journal.lines()?.into_iter().skip(existing_line_count) {
        events.push(journal.decrypt_line(&line)?);
    }
    Ok(events)
}

fn load_or_create_journal_key(path: &Path) -> Result<JournalKey, ActivityCapturePersistenceError> {
    match fs::read(path) {
        Ok(bytes) => journal_key_from_bytes(&bytes),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => create_journal_key(path),
        Err(error) => Err(error.into()),
    }
}

fn create_journal_key(path: &Path) -> Result<JournalKey, ActivityCapturePersistenceError> {
    let key = JournalKey::generate();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, key.as_bytes())?;
    Ok(key)
}

fn journal_key_from_bytes(bytes: &[u8]) -> Result<JournalKey, ActivityCapturePersistenceError> {
    if bytes.len() != JOURNAL_KEY_BYTES {
        return Err(ActivityCapturePersistenceError::InvalidKeyLength);
    }
    let mut key = [0; JOURNAL_KEY_BYTES];
    key.copy_from_slice(bytes);
    Ok(JournalKey::from_bytes(key))
}
