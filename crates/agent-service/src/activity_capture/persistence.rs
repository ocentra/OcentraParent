use std::path::Path;

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
    for event in events {
        if !store.contains_event_id(&event.event_id)? {
            events_to_append.push(event);
        }
    }
    if events_to_append.is_empty() {
        return Ok(store.status()?);
    }
    let key = load_or_create_journal_key(key_path)?;
    let mut journal = ActivityJournal::open(journal_path.to_path_buf(), key)?;
    let existing_line_count = journal.lines()?.len();
    for event in events_to_append {
        journal.append(event)?;
    }
    let mut appended_events = Vec::new();
    for line in journal.lines()?.into_iter().skip(existing_line_count) {
        appended_events.push(journal.decrypt_line(&line)?);
    }
    Ok(store.ingest_events(&appended_events)?)
}
