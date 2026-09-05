use std::{collections::HashSet, path::Path};

use ocentra_parent_agent_core::activity_store::ActivityStore;
use ocentra_parent_agent_protocol::{
    activity::ActivityEvent, activity_query::ActivityIngestStatus,
};

#[path = "persistence/error.rs"]
pub(crate) mod error;
#[path = "persistence/journal.rs"]
mod journal;

use error::ActivityCapturePersistenceError;

pub(crate) fn record_activity_events_to_paths(
    journal_path: &Path,
    key_path: &Path,
    store_path: &Path,
    events: &[ActivityEvent],
) -> Result<ActivityIngestStatus, ActivityCapturePersistenceError> {
    let store = ActivityStore::open(store_path)?;
    let (events_to_append, duplicate_events_in_batch) = unseen_events(&store, events)?;
    if events_to_append.is_empty() {
        return Ok(store.status()?);
    }

    let journal_append =
        journal::replay_and_append(journal_path, key_path, &store, events_to_append)?;
    if journal_append.appended_events.is_empty() {
        let mut status = journal_append.replay_status;
        status.duplicate_events += duplicate_events_in_batch;
        return Ok(status);
    }

    let mut status = store.ingest_events(&journal_append.appended_events)?;
    status.events_ingested += journal_append.replay_status.events_ingested;
    status.duplicate_events +=
        journal_append.replay_status.duplicate_events + duplicate_events_in_batch;
    Ok(status)
}

fn unseen_events<'a>(
    store: &ActivityStore,
    events: &'a [ActivityEvent],
) -> Result<(Vec<&'a ActivityEvent>, u64), ActivityCapturePersistenceError> {
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

    Ok((events_to_append, duplicate_events_in_batch))
}
