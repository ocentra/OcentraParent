use std::fs;

use ocentra_eventing::{
    envelope::{EventEnvelope, EventMetadata, EventSource},
    ids::{
        CorrelationId, EventCustody, RuntimeInstanceId, RuntimeRole, SourceComponent, SourceService,
    },
};
use ocentra_parent_agent_service::custody_tombstone_runtime::{
    recover_pending_tombstone_actions, TombstoneJournalPath, TombstoneStoreDirectory,
};
use ocentra_storage_custody_core::{
    retention_delete_tombstone_store::RetentionDeleteTombstoneStore,
    storage_custody::{
        storage_custody_action_planned_event, storage_custody_decision_recorded_event,
        ParentExportState, RemoteSyncState, RetentionWindowState, StorageCustodyAggregateId,
        StorageCustodyDecisionId, StorageCustodyInput, StorageCustodyLocation,
    },
};

#[tokio::test]
async fn startup_recovery_replays_pending_typed_action_once_and_keeps_terminal_record_closed(
) -> Result<(), Box<dyn std::error::Error>> {
    let directory = temp_directory("recovery")?;
    let store = RetentionDeleteTombstoneStore::open(&directory)?;
    let action = expired_delete_action("service-recovery")?;
    let envelope =
        EventEnvelope::from_event(action.clone(), metadata("service-recovery")?)?.store()?;
    store.persist_action_plan_intent(envelope, action)?;
    let journal_path = directory.join("service-recovery.ndjson");

    let report = recover_pending_tombstone_actions(
        TombstoneStoreDirectory::from(directory.clone()),
        TombstoneJournalPath::from(journal_path.clone()),
    )
    .await;
    assert_eq!(report.recovered_count, 1);
    assert_eq!(report.failed_count, 0);
    assert!(!RetentionDeleteTombstoneStore::open(&directory)?.records()?[0].terminal_pending);
    assert!(fs::metadata(&journal_path)?.len() > 0);

    let second = recover_pending_tombstone_actions(
        TombstoneStoreDirectory::from(directory.clone()),
        TombstoneJournalPath::from(journal_path),
    )
    .await;
    assert_eq!(second.recovered_count, 0);
    assert_eq!(second.failed_count, 0);
    assert!(!RetentionDeleteTombstoneStore::open(&directory)?.records()?[0].terminal_pending);
    let _ = fs::remove_dir_all(&directory);
    Ok(())
}

fn expired_delete_action(
    id: &str,
) -> Result<
    ocentra_storage_custody_core::storage_custody::StorageCustodyActionPlannedEvent,
    Box<dyn std::error::Error>,
> {
    Ok(storage_custody_action_planned_event(
        storage_custody_decision_recorded_event(
            StorageCustodyAggregateId::parse(format!("service-family-{id}"))?,
            StorageCustodyDecisionId::parse(format!("service-decision-{id}"))?,
            StorageCustodyInput {
                location: StorageCustodyLocation::ParentDeviceLocal,
                retention_window_state: RetentionWindowState::Expired,
                parent_export_state: ParentExportState::NotRequested,
                remote_sync_state: RemoteSyncState::Disabled,
            },
        ),
    ))
}

fn metadata(id: &str) -> Result<EventMetadata, Box<dyn std::error::Error>> {
    Ok(EventMetadata::new(
        CorrelationId::parse(format!("service-custody-{id}"))?,
        EventSource::new(
            EventCustody::parse("local-journal")?,
            RuntimeRole::parse("controller")?,
            SourceService::parse("agent-service")?,
            SourceComponent::parse("custody-tombstone-runtime")?,
            RuntimeInstanceId::parse("service-test")?,
        ),
    ))
}

fn temp_directory(name: &str) -> Result<std::path::PathBuf, std::io::Error> {
    let directory = std::env::temp_dir().join(format!(
        "ocentra-agent-service-custody-{name}-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&directory);
    fs::create_dir_all(&directory)?;
    Ok(directory)
}
