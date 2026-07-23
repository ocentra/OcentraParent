use std::{collections::HashSet, path::Path};

use ocentra_parent_agent_core::{activity_store::ActivityStore, journal::ActivityJournal};
use ocentra_parent_agent_protocol::{
    activity::ActivityEvent, activity_query::ActivityIngestStatus,
};

use super::{load_or_create_journal_key, ActivityCaptureError};

pub(super) fn record_unseen_activity_events(
    journal_path: &Path,
    key_path: &Path,
    store_path: &Path,
    events: &[ActivityEvent],
) -> Result<ActivityIngestStatus, ActivityCaptureError> {
    let store = ActivityStore::open(store_path)?;
    let mut events_to_append = Vec::new();
    let mut accepted_event_ids = HashSet::new();
    let mut duplicate_events_in_batch = 0;
    for event in events {
        if !accepted_event_ids.insert(event.event_id.as_str()) {
            duplicate_events_in_batch += 1;
        } else if !store.contains_event_id(&event.event_id)? {
            events_to_append.push(event);
        }
    }
    if events_to_append.is_empty() {
        return Ok(store.status()?);
    }
    let key = load_or_create_journal_key(key_path)?;
    let mut journal = ActivityJournal::open(journal_path.to_path_buf(), key)?;
    let replay_status = store.ingest_journal(&journal)?;
    let mut events_missing_after_replay = Vec::new();
    for event in events_to_append {
        if !store.contains_event_id(&event.event_id)? {
            events_missing_after_replay.push(event);
        }
    }
    if events_missing_after_replay.is_empty() {
        let mut status = replay_status;
        status.duplicate_events += duplicate_events_in_batch;
        return Ok(status);
    }
    let existing_line_count = journal.lines()?.len();
    for event in events_missing_after_replay {
        journal.append(event)?;
    }
    let mut appended_events = Vec::new();
    for line in journal.lines()?.into_iter().skip(existing_line_count) {
        appended_events.push(journal.decrypt_line(&line)?);
    }
    let mut status = store.ingest_events(&appended_events)?;
    status.events_ingested += replay_status.events_ingested;
    status.duplicate_events += replay_status.duplicate_events + duplicate_events_in_batch;
    Ok(status)
}
