use ocentra_storage_custody_core::retention_delete_tombstone_store::RetentionDeleteTombstoneStore;
use ocentra_storage_custody_core::storage_custody::{
    storage_custody_action_planned_event, storage_custody_decision_recorded_event,
    ParentExportState, RemoteSyncState, RetentionWindowState, StorageCustodyAggregateId,
    StorageCustodyDecisionId, StorageCustodyInput, StorageCustodyLocation,
};

#[test]
fn tombstone_outbox_recovers_intent_until_terminal_publish() -> Result<(), String> {
    let directory = std::env::temp_dir().join(format!("ocentra-tombstone-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&directory);
    let store = RetentionDeleteTombstoneStore::open(&directory)
        .map_err(|error| format!("open tombstone outbox: {error}"))?;
    store
        .persist_intent("delete:one".to_string(), "proof:one".to_string())
        .map_err(|error| format!("persist tombstone intent: {error}"))?;
    let reopened = RetentionDeleteTombstoneStore::open(&directory)
        .map_err(|error| format!("reopen tombstone outbox: {error}"))?;
    assert_eq!(
        reopened
            .records()
            .map_err(|error| format!("read persisted tombstone records: {error}"))?
            .len(),
        1
    );
    reopened
        .mark_terminal_published("delete:one")
        .map_err(|error| format!("mark terminal tombstone: {error}"))?;
    assert_eq!(
        reopened
            .records()
            .map_err(|error| format!("read empty tombstone records: {error}"))?
            .len(),
        0
    );
    let _ = std::fs::remove_dir_all(&directory);
    Ok(())
}

#[test]
fn tombstone_outbox_rejects_corrupt_durable_metadata() -> Result<(), String> {
    let directory =
        std::env::temp_dir().join(format!("ocentra-tombstone-corrupt-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&directory);
    let store = RetentionDeleteTombstoneStore::open(&directory)
        .map_err(|error| format!("open tombstone outbox: {error}"))?;
    std::fs::write(
        directory.join("retention-delete-tombstones.json"),
        b"not-json",
    )
    .map_err(|error| format!("write corrupt tombstone metadata: {error}"))?;
    match store.records() {
        Err(error) => assert_eq!(error.kind(), std::io::ErrorKind::InvalidData),
        Ok(records) => {
            return Err(format!(
                "expected corrupt metadata rejection, got {records:?}"
            ))
        }
    }
    let _ = std::fs::remove_dir_all(&directory);
    Ok(())
}

#[test]
fn tombstone_outbox_serializes_concurrent_intents() -> Result<(), String> {
    let directory =
        std::env::temp_dir().join(format!("ocentra-tombstone-race-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&directory);
    let mut workers = Vec::new();
    for index in 0..8 {
        let directory = directory.clone();
        workers.push(std::thread::spawn(move || {
            RetentionDeleteTombstoneStore::open(directory).and_then(|store| {
                store.persist_intent(format!("delete:{index}"), format!("proof:{index}"))
            })
        }));
    }
    for worker in workers {
        worker
            .join()
            .map_err(|_error| "tombstone worker panicked".to_string())?
            .map_err(|error| format!("persist concurrent tombstone intent: {error}"))?;
    }
    let count = RetentionDeleteTombstoneStore::open(&directory)
        .map_err(|error| format!("reopen tombstone outbox: {error}"))?
        .records()
        .map_err(|error| format!("read concurrent tombstone records: {error}"))?
        .len();
    assert_eq!(count, 8);
    let _ = std::fs::remove_dir_all(&directory);
    Ok(())
}

#[test]
fn tombstone_outbox_atomic_replacements_survive_reopen() -> Result<(), String> {
    let directory =
        std::env::temp_dir().join(format!("ocentra-tombstone-replace-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&directory);
    let store = RetentionDeleteTombstoneStore::open(&directory)
        .map_err(|error| format!("open tombstone outbox: {error}"))?;
    store
        .persist_intent("delete:first".to_string(), "proof:first".to_string())
        .map_err(|error| format!("persist first tombstone intent: {error}"))?;
    store
        .persist_intent("delete:second".to_string(), "proof:second".to_string())
        .map_err(|error| format!("persist second tombstone intent: {error}"))?;
    store
        .mark_terminal_published("delete:first")
        .map_err(|error| format!("mark first tombstone terminal: {error}"))?;

    let records = RetentionDeleteTombstoneStore::open(&directory)
        .map_err(|error| format!("reopen tombstone outbox: {error}"))?
        .records()
        .map_err(|error| format!("read replacement tombstone records: {error}"))?;
    assert_eq!(records.len(), 1);
    assert_eq!(records[0].deletion_ref, "delete:second");
    assert!(records[0].terminal_pending);
    let _ = std::fs::remove_dir_all(&directory);
    Ok(())
}

#[test]
fn tombstone_outbox_persists_only_a_typed_delete_action() -> Result<(), Box<dyn std::error::Error>>
{
    let directory =
        std::env::temp_dir().join(format!("ocentra-tombstone-action-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&directory);
    let store = RetentionDeleteTombstoneStore::open(&directory)?;
    let delete_action = action_for(RetentionWindowState::Expired)?;

    store.persist_action_plan_intent(&delete_action)?;

    let records = RetentionDeleteTombstoneStore::open(&directory)?.records()?;
    assert_eq!(records.len(), 1);
    assert_eq!(
        records[0].deletion_ref,
        "storage-custody-delete:retention-delete-decision"
    );
    assert_eq!(
        records[0].proof_ref,
        "storage-custody-action:retention-delete-decision"
    );
    let _ = std::fs::remove_dir_all(&directory);
    Ok(())
}

#[test]
fn tombstone_outbox_rejects_a_typed_non_delete_action() -> Result<(), Box<dyn std::error::Error>> {
    let directory = std::env::temp_dir().join(format!(
        "ocentra-tombstone-no-action-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&directory);
    let store = RetentionDeleteTombstoneStore::open(&directory)?;
    let retain_action = action_for(RetentionWindowState::Active)?;

    let error = store
        .persist_action_plan_intent(&retain_action)
        .expect_err("retain action must not enqueue a tombstone");
    assert_eq!(error.kind(), std::io::ErrorKind::InvalidInput);
    assert!(store.records()?.is_empty());
    let _ = std::fs::remove_dir_all(&directory);
    Ok(())
}

fn action_for(
    retention_window_state: RetentionWindowState,
) -> Result<
    ocentra_storage_custody_core::storage_custody::StorageCustodyActionPlannedEvent,
    Box<dyn std::error::Error>,
> {
    Ok(storage_custody_action_planned_event(
        storage_custody_decision_recorded_event(
            StorageCustodyAggregateId::parse("retention-delete-family")?,
            StorageCustodyDecisionId::parse("retention-delete-decision")?,
            StorageCustodyInput {
                location: StorageCustodyLocation::ParentDeviceLocal,
                retention_window_state,
                parent_export_state: ParentExportState::NotRequested,
                remote_sync_state: RemoteSyncState::Disabled,
            },
        ),
    ))
}
