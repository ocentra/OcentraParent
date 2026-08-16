use ocentra_parent_agent_core::activity_store::ActivityStore;
use ocentra_parent_agent_protocol::activity::ActivityEvent;

use crate::activity_store_path::ActivityDbPath;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ActivityPersistenceError {
    Unavailable,
}

pub(crate) async fn persist_activity_event(
    path: ActivityDbPath,
    event: ActivityEvent,
) -> Result<(), ActivityPersistenceError> {
    let persisted = tokio::task::spawn_blocking(move || {
        ActivityStore::open(&path)
            .and_then(|store| store.ingest_events(&[event]))
            .is_ok()
    })
    .await
    .unwrap_or(false);
    if persisted {
        Ok(())
    } else {
        Err(ActivityPersistenceError::Unavailable)
    }
}
