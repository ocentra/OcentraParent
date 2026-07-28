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
async fn startup_recovery_replays_pending_action_once_and_marks_auditable_terminal_result(
) -> Result<(), Box<dyn std::error::Error>> {
    let directory = std::env::temp_dir().join(format!(
        "ocentra-agent-service-custody-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&directory);
    fs::create_dir_all(&directory)?;
    let store = RetentionDeleteTombstoneStore::open(&directory)?;
    let action = storage_custody_action_planned_event(storage_custody_decision_recorded_event(
        StorageCustodyAggregateId::parse("service-recovery-family")?,
        StorageCustodyDecisionId::parse("service-recovery-decision")?,
        StorageCustodyInput {
            location: StorageCustodyLocation::ParentDeviceLocal,
            retention_window_state: RetentionWindowState::Expired,
            parent_export_state: ParentExportState::NotRequested,
            remote_sync_state: RemoteSyncState::Disabled,
        },
    ));
    let envelope = EventEnvelope::from_event(action.clone(), metadata()?)?.store()?;
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
    let _ = fs::remove_dir_all(&directory);
    Ok(())
}

fn metadata() -> Result<EventMetadata, Box<dyn std::error::Error>> {
    Ok(EventMetadata::new(
        CorrelationId::parse("service-custody-recovery")?,
        EventSource::new(
            EventCustody::parse("local-journal")?,
            RuntimeRole::parse("controller")?,
            SourceService::parse("agent-service")?,
            SourceComponent::parse("custody-tombstone-runtime")?,
            RuntimeInstanceId::parse("service-test")?,
        ),
    ))
}
