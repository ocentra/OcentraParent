use ocentra_eventing::envelope::{EventEnvelope, EventMetadata, EventSource};
use ocentra_eventing::ids::{
    CorrelationId, EventCustody, RuntimeInstanceId, RuntimeRole, SourceComponent, SourceService,
};
use ocentra_storage_custody_core::retention_delete_tombstone_store::RetentionDeleteTombstoneStore;
use ocentra_storage_custody_core::storage_custody::{
    storage_custody_action_planned_event, storage_custody_decision_recorded_event,
    LocalPayloadRetentionAction, ParentExportState, RemoteSyncState, RetentionWindowState,
    StorageCustodyAggregateId, StorageCustodyDecisionId, StorageCustodyInput,
    StorageCustodyLocation,
};

#[test]
fn tombstone_outbox_recovers_intent_until_terminal_publish(
) -> Result<(), Box<dyn std::error::Error>> {
    let directory = std::env::temp_dir().join(format!("ocentra-tombstone-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&directory);
    let store = RetentionDeleteTombstoneStore::open(&directory)?;
    let action = action_for(RetentionWindowState::Expired)?;
    store.persist_action_plan_intent(envelope_for(&action)?, action)?;
    let reopened = RetentionDeleteTombstoneStore::open(&directory)?;
    assert_eq!(reopened.records()?.len(), 1);
    reopened.mark_terminal_published("storage-custody-delete:retention-delete-decision")?;
    let records = reopened.records()?;
    assert_eq!(records.len(), 1);
    assert!(!records[0].terminal_pending);
    let _ = std::fs::remove_dir_all(&directory);
    Ok(())
}

#[test]
fn tombstone_outbox_rejects_corrupt_durable_metadata() {
    let directory =
        std::env::temp_dir().join(format!("ocentra-tombstone-corrupt-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&directory);
    let store = RetentionDeleteTombstoneStore::open(&directory).expect("open");
    std::fs::write(
        directory.join("retention-delete-tombstones.json"),
        b"not-json",
    )
    .expect("corrupt");
    assert_eq!(
        store.records().expect_err("corrupt metadata").kind(),
        std::io::ErrorKind::InvalidData
    );
    let _ = std::fs::remove_dir_all(&directory);
}

#[test]
fn tombstone_outbox_serializes_concurrent_intents() -> Result<(), Box<dyn std::error::Error>> {
    let directory =
        std::env::temp_dir().join(format!("ocentra-tombstone-race-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&directory);
    let mut workers = Vec::new();
    for index in 0..8 {
        let directory = directory.clone();
        let action =
            action_for_index(index).map_err(|error| std::io::Error::other(error.to_string()))?;
        let envelope =
            envelope_for(&action).map_err(|error| std::io::Error::other(error.to_string()))?;
        workers.push(std::thread::spawn(move || {
            RetentionDeleteTombstoneStore::open(directory)
                .and_then(|store| store.persist_action_plan_intent(envelope, action))
        }));
    }
    for worker in workers {
        worker
            .join()
            .map_err(|_| std::io::Error::other("tombstone writer thread panicked"))??;
    }
    let count = RetentionDeleteTombstoneStore::open(&directory)?
        .records()?
        .len();
    assert_eq!(count, 8);
    let _ = std::fs::remove_dir_all(&directory);
    Ok(())
}

#[test]
fn tombstone_outbox_atomic_replacements_survive_reopen() -> Result<(), Box<dyn std::error::Error>> {
    let directory =
        std::env::temp_dir().join(format!("ocentra-tombstone-replace-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&directory);
    let store = RetentionDeleteTombstoneStore::open(&directory)?;
    let first = action_for_index(1)?;
    let second = action_for_index(2)?;
    store.persist_action_plan_intent(envelope_for(&first)?, first)?;
    store.persist_action_plan_intent(envelope_for(&second)?, second)?;
    store.mark_terminal_published("storage-custody-delete:retention-delete-decision-1")?;

    let records = RetentionDeleteTombstoneStore::open(&directory)?.records()?;
    assert_eq!(records.len(), 2);
    assert_eq!(
        records[1].deletion_ref,
        "storage-custody-delete:retention-delete-decision-2"
    );
    assert!(!records[0].terminal_pending);
    assert!(records[1].terminal_pending);
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

    let envelope = envelope_for(&delete_action)?;
    store.persist_action_plan_intent(envelope.clone(), delete_action.clone())?;

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
    assert_eq!(
        records[0].typed_action_and_envelope(),
        Some((&delete_action, &envelope))
    );
    let _ = std::fs::remove_dir_all(&directory);
    Ok(())
}

#[test]
fn tombstone_outbox_migrates_legacy_version_one_before_decoding_typed_records(
) -> Result<(), Box<dyn std::error::Error>> {
    let directory = std::env::temp_dir().join(format!(
        "ocentra-tombstone-legacy-migration-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&directory);
    std::fs::create_dir_all(&directory)?;
    std::fs::write(
        directory.join("retention-delete-tombstones.json"),
        r#"[{"version":1,"deletion_ref":"storage-custody-delete:legacy-decision","proof_ref":"storage-custody-action:legacy-decision","terminal_pending":true}]"#,
    )?;

    let store = RetentionDeleteTombstoneStore::open(&directory)?;
    let legacy = store.records()?;
    assert_eq!(legacy.len(), 1);
    assert_eq!(legacy[0].version, 1);
    assert_eq!(
        legacy[0].deletion_ref,
        "storage-custody-delete:legacy-decision"
    );
    assert!(legacy[0].terminal_pending);
    assert_eq!(legacy[0].typed_action_and_envelope(), None);

    let action = action_for_index(2)?;
    let envelope = envelope_for(&action)?;
    store.persist_action_plan_intent(envelope.clone(), action.clone())?;

    let restarted = RetentionDeleteTombstoneStore::open(&directory)?;
    let records = restarted.records()?;
    assert_eq!(records.len(), 2);
    assert_eq!(records[0].version, 1);
    assert!(records[0].terminal_pending);
    assert_eq!(records[0].typed_action_and_envelope(), None);
    assert_eq!(records[1].version, 2);
    assert_eq!(
        records[1].typed_action_and_envelope(),
        Some((&action, &envelope))
    );

    restarted.mark_terminal_published("storage-custody-delete:legacy-decision")?;
    let after_acknowledgement = RetentionDeleteTombstoneStore::open(&directory)?.records()?;
    assert!(!after_acknowledgement[0].terminal_pending);
    assert_eq!(after_acknowledgement[1].version, 2);
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
        .persist_action_plan_intent(envelope_for(&retain_action)?, retain_action)
        .expect_err("retain action must not enqueue a tombstone");
    assert_eq!(error.kind(), std::io::ErrorKind::InvalidInput);
    assert!(store.records()?.is_empty());
    let _ = std::fs::remove_dir_all(&directory);
    Ok(())
}

#[test]
fn tombstone_outbox_rejects_an_incoherent_delete_plan() -> Result<(), Box<dyn std::error::Error>> {
    let directory = std::env::temp_dir().join(format!(
        "ocentra-tombstone-incoherent-action-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&directory);
    let store = RetentionDeleteTombstoneStore::open(&directory)?;
    let mut incoherent_action = action_for(RetentionWindowState::Expired)?;
    incoherent_action.action_plan.local_payload_retention_action =
        LocalPayloadRetentionAction::Retain;

    let error = store
        .persist_action_plan_intent(envelope_for(&incoherent_action)?, incoherent_action)
        .expect_err(
            "a retain action must not enqueue a tombstone even when the tombstone field is write",
        );
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

fn action_for_index(
    index: usize,
) -> Result<
    ocentra_storage_custody_core::storage_custody::StorageCustodyActionPlannedEvent,
    Box<dyn std::error::Error>,
> {
    Ok(storage_custody_action_planned_event(
        storage_custody_decision_recorded_event(
            StorageCustodyAggregateId::parse(format!("retention-delete-family-{index}"))?,
            StorageCustodyDecisionId::parse(format!("retention-delete-decision-{index}"))?,
            StorageCustodyInput {
                location: StorageCustodyLocation::ParentDeviceLocal,
                retention_window_state: RetentionWindowState::Expired,
                parent_export_state: ParentExportState::NotRequested,
                remote_sync_state: RemoteSyncState::Disabled,
            },
        ),
    ))
}

fn envelope_for(
    action: &ocentra_storage_custody_core::storage_custody::StorageCustodyActionPlannedEvent,
) -> Result<ocentra_eventing::envelope::StoredEventEnvelope, Box<dyn std::error::Error>> {
    EventEnvelope::from_event(
        action.clone(),
        EventMetadata::new(
            CorrelationId::parse("retention-delete-store-test")?,
            EventSource::new(
                EventCustody::parse("local-journal")?,
                RuntimeRole::parse("controller")?,
                SourceService::parse("storage-custody-core")?,
                SourceComponent::parse("retention-delete-store")?,
                RuntimeInstanceId::parse("retention-delete-store-test-instance")?,
            ),
        ),
    )?
    .store()
    .map_err(Into::into)
}
