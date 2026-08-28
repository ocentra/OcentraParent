use super::*;
use ::ocentra_child_runtime::{
    child_runtime_tombstone_event_flow::ChildRuntimeTombstoneEventFlow,
    runtime_gate_tombstone::{
        self, ChildRuntimeTombstoneMilestone, ChildRuntimeTombstonePublicationOutcome,
    },
};
use ocentra_eventing::envelope::{EventEnvelope, EventMetadata, EventSource};
use ocentra_eventing::expect_value::ExpectErrValue;
use ocentra_eventing::ids::{
    CorrelationId, EventCustody, IdempotencyKey, RuntimeInstanceId, RuntimeRole, SourceComponent,
    SourceService,
};
use ocentra_eventing::journal::ndjson::{NdjsonEventJournal, NdjsonJournalOptions};
use ocentra_storage_custody_core::retention_delete_tombstone_store::RetentionDeleteTombstoneStore;
use ocentra_storage_custody_core::storage_custody::{
    storage_custody_action_planned_event, storage_custody_decision_recorded_event,
    StorageCustodyAggregateId, StorageCustodyDecisionId,
};

#[tokio::test]
async fn child_runtime_startup_recovery_rejects_a_typed_record_falsely_marked_terminal(
) -> Result<(), Box<dyn std::error::Error>> {
    let directory = std::env::temp_dir().join(format!(
        "ocentra-child-runtime-false-terminal-recovery-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&directory);
    std::fs::create_dir_all(&directory)?;
    let action = storage_custody_action_planned_event(storage_custody_decision_recorded_event(
        StorageCustodyAggregateId::parse("child-runtime-false-terminal-family")?,
        StorageCustodyDecisionId::parse("child-runtime-false-terminal-decision")?,
        StorageCustodyInput {
            location: StorageCustodyLocation::ParentDeviceLocal,
            retention_window_state: RetentionWindowState::Expired,
            parent_export_state: ParentExportState::NotRequested,
            remote_sync_state: RemoteSyncState::Disabled,
        },
    ));
    let journal = NdjsonEventJournal::with_options(
        directory.join("retention-delete.ndjson"),
        NdjsonJournalOptions::hash_chain(),
    );
    let store = RetentionDeleteTombstoneStore::open(&directory)?;
    let envelope =
        EventEnvelope::from_event(action.clone(), retention_delete_metadata()?)?.store()?;
    store.persist_action_plan_intent(envelope, action)?;
    let tombstone_path = directory.join("retention-delete-tombstones.json");
    let encoded = String::from_utf8(std::fs::read(&tombstone_path)?)?
        .replace("\"terminal_pending\":true", "\"terminal_pending\":false");
    std::fs::write(&tombstone_path, encoded.as_bytes())?;

    let event_flow = ChildRuntimeTombstoneEventFlow::new(journal, store);
    let error = event_flow
        .recover_pending()
        .await
        .expect_err_value("typed tombstone cannot self-mark terminal");
    assert_eq!(error.kind(), std::io::ErrorKind::InvalidData);
    assert_eq!(
        error.to_string(),
        "child-runtime tombstone recovery requires a version 3 terminal marker for a completed tombstone"
    );
    let _ = std::fs::remove_dir_all(&directory);
    Ok(())
}

#[tokio::test]
async fn child_runtime_persists_and_recovers_typed_tombstone_action_before_acknowledgement(
) -> Result<(), Box<dyn std::error::Error>> {
    let directory = std::env::temp_dir().join(format!(
        "ocentra-child-runtime-tombstone-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&directory);
    let action = storage_custody_action_planned_event(storage_custody_decision_recorded_event(
        StorageCustodyAggregateId::parse("child-runtime-retention-family")?,
        StorageCustodyDecisionId::parse("child-runtime-retention-decision")?,
        StorageCustodyInput {
            location: StorageCustodyLocation::ParentDeviceLocal,
            retention_window_state: RetentionWindowState::Expired,
            parent_export_state: ParentExportState::NotRequested,
            remote_sync_state: RemoteSyncState::Disabled,
        },
    ));
    std::fs::create_dir_all(&directory)?;
    let journal = NdjsonEventJournal::with_options(
        directory.join("retention-delete.ndjson"),
        NdjsonJournalOptions::hash_chain(),
    );
    let store = RetentionDeleteTombstoneStore::open(&directory)?;
    let envelope =
        EventEnvelope::from_event(action.clone(), retention_delete_metadata()?)?.store()?;

    let append = runtime_gate_tombstone::persist_child_runtime_tombstone_action(
        &journal, &store, &envelope, &action,
    )
    .await?;
    assert_eq!(append.sequence, 1);

    let recovered = RetentionDeleteTombstoneStore::open(&directory)?.records()?;
    assert_eq!(recovered.len(), 1);
    assert_eq!(
        recovered[0].deletion_ref,
        "storage-custody-delete:child-runtime-retention-decision"
    );
    runtime_gate_tombstone::acknowledge_child_runtime_tombstone_publication(
        &store,
        &recovered[0].deletion_ref,
    )
    .await?;
    let acknowledged = RetentionDeleteTombstoneStore::open(&directory)?.records()?;
    assert_eq!(acknowledged.len(), 1);
    assert!(!acknowledged[0].terminal_pending);
    runtime_gate_tombstone::persist_child_runtime_tombstone_action(
        &journal, &store, &envelope, &action,
    )
    .await?;
    let replayed = RetentionDeleteTombstoneStore::open(&directory)?.records()?;
    assert_eq!(replayed.len(), 1);
    assert!(!replayed[0].terminal_pending);
    let _ = std::fs::remove_dir_all(&directory);
    Ok(())
}

#[tokio::test]
async fn child_runtime_rejects_acknowledgement_for_an_unknown_tombstone(
) -> Result<(), Box<dyn std::error::Error>> {
    let directory = std::env::temp_dir().join(format!(
        "ocentra-child-runtime-unknown-ack-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&directory);
    let store = RetentionDeleteTombstoneStore::open(&directory)?;

    let error = runtime_gate_tombstone::acknowledge_child_runtime_tombstone_publication(
        &store,
        "storage-custody-delete:does-not-exist",
    )
    .await
    .expect_err_value("unknown tombstones must not be acknowledged");
    assert_eq!(error.kind(), std::io::ErrorKind::NotFound);
    assert!(store.records()?.is_empty());
    let _ = std::fs::remove_dir_all(&directory);
    Ok(())
}

#[tokio::test]
async fn child_runtime_replays_a_durable_tombstone_obligation_after_journal_failure(
) -> Result<(), Box<dyn std::error::Error>> {
    let directory = std::env::temp_dir().join(format!(
        "ocentra-child-runtime-journal-tombstone-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&directory);
    std::fs::create_dir_all(&directory)?;
    let action = storage_custody_action_planned_event(storage_custody_decision_recorded_event(
        StorageCustodyAggregateId::parse("child-runtime-journal-family")?,
        StorageCustodyDecisionId::parse("child-runtime-journal-decision")?,
        StorageCustodyInput {
            location: StorageCustodyLocation::ParentDeviceLocal,
            retention_window_state: RetentionWindowState::Expired,
            parent_export_state: ParentExportState::NotRequested,
            remote_sync_state: RemoteSyncState::Disabled,
        },
    ));
    let journal = NdjsonEventJournal::with_options(
        directory.join("retention-delete.ndjson"),
        NdjsonJournalOptions::hash_chain(),
    );
    let store = RetentionDeleteTombstoneStore::open(&directory)?;
    let envelope =
        EventEnvelope::from_event(action.clone(), retention_delete_metadata()?)?.store()?;
    journal.inject_next_sync_failure_for_debug();
    assert!(
        runtime_gate_tombstone::persist_child_runtime_tombstone_action(
            &journal, &store, &envelope, &action,
        )
        .await
        .is_err()
    );
    let recovered = RetentionDeleteTombstoneStore::open(&directory)?.records()?;
    assert_eq!(recovered.len(), 1);
    assert_eq!(recovered[0].proof_ref, action.action_plan_id.as_str());

    let restarted_journal = NdjsonEventJournal::with_options(
        directory.join("retention-delete.ndjson"),
        NdjsonJournalOptions::hash_chain(),
    );
    let append = runtime_gate_tombstone::persist_child_runtime_tombstone_action(
        &restarted_journal,
        &RetentionDeleteTombstoneStore::open(&directory)?,
        &envelope,
        &action,
    )
    .await?;
    assert_eq!(append.sequence, 1);
    let _ = std::fs::remove_dir_all(&directory);
    Ok(())
}

#[tokio::test]
async fn child_runtime_startup_recovery_replays_pending_outbox_through_event_flow(
) -> Result<(), Box<dyn std::error::Error>> {
    let directory = std::env::temp_dir().join(format!(
        "ocentra-child-runtime-startup-recovery-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&directory);
    std::fs::create_dir_all(&directory)?;
    let action = storage_custody_action_planned_event(storage_custody_decision_recorded_event(
        StorageCustodyAggregateId::parse("child-runtime-startup-family")?,
        StorageCustodyDecisionId::parse("child-runtime-startup-decision")?,
        StorageCustodyInput {
            location: StorageCustodyLocation::ParentDeviceLocal,
            retention_window_state: RetentionWindowState::Expired,
            parent_export_state: ParentExportState::NotRequested,
            remote_sync_state: RemoteSyncState::Disabled,
        },
    ));
    let journal_path = directory.join("retention-delete.ndjson");
    let journal =
        NdjsonEventJournal::with_options(&journal_path, NdjsonJournalOptions::hash_chain());
    let store = RetentionDeleteTombstoneStore::open(&directory)?;
    let envelope =
        EventEnvelope::from_event(action.clone(), retention_delete_metadata()?)?.store()?;
    journal.inject_next_sync_failure_for_debug();
    assert!(
        runtime_gate_tombstone::persist_child_runtime_tombstone_action(
            &journal, &store, &envelope, &action,
        )
        .await
        .is_err()
    );

    let restarted = ChildRuntimeTombstoneEventFlow::new(
        NdjsonEventJournal::with_options(&journal_path, NdjsonJournalOptions::hash_chain()),
        RetentionDeleteTombstoneStore::open(&directory)?,
    );
    let recovered = restarted.recover_pending().await?;
    assert_eq!(recovered.journaled.len(), 1);
    assert!(recovered.pending_journal_retry.is_empty());
    assert_eq!(recovered.journaled[0].sequence, 1);
    assert!(RetentionDeleteTombstoneStore::open(&directory)?.records()?[0].terminal_pending);

    let replayed = restarted.recover_pending().await?;
    assert_eq!(replayed.journaled.len(), 1);
    assert_eq!(replayed.journaled[0].sequence, 1);
    assert!(replayed.pending_journal_retry.is_empty());

    let records = RetentionDeleteTombstoneStore::open(&directory)?.records()?;
    runtime_gate_tombstone::acknowledge_child_runtime_tombstone_publication(
        &store,
        &records[0].deletion_ref,
    )
    .await?;
    assert!(!RetentionDeleteTombstoneStore::open(&directory)?.records()?[0].terminal_pending);
    let _ = std::fs::remove_dir_all(&directory);
    Ok(())
}

#[tokio::test]
async fn child_runtime_startup_recovery_refuses_pending_legacy_tombstone(
) -> Result<(), Box<dyn std::error::Error>> {
    let directory = std::env::temp_dir().join(format!(
        "ocentra-child-runtime-legacy-recovery-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&directory);
    std::fs::create_dir_all(&directory)?;
    std::fs::write(
        directory.join("retention-delete-tombstones.json"),
        r#"[{"version":1,"deletion_ref":"storage-custody-delete:legacy-decision","proof_ref":"storage-custody-action:legacy-decision","terminal_pending":true}]"#,
    )?;

    let event_flow = ChildRuntimeTombstoneEventFlow::new(
        NdjsonEventJournal::with_options(
            directory.join("retention-delete.ndjson"),
            NdjsonJournalOptions::hash_chain(),
        ),
        RetentionDeleteTombstoneStore::open(&directory)?,
    );
    let error = event_flow
        .recover_pending()
        .await
        .expect_err_value("pending legacy tombstone must require migration");
    assert_eq!(error.kind(), std::io::ErrorKind::InvalidData);
    assert_eq!(
        error.to_string(),
        "child-runtime tombstone recovery requires manual migration for a pending legacy tombstone"
    );
    let _ = std::fs::remove_dir_all(&directory);
    Ok(())
}

#[tokio::test]
async fn child_runtime_startup_recovery_rejects_tampered_tombstone_identity(
) -> Result<(), Box<dyn std::error::Error>> {
    let directory = std::env::temp_dir().join(format!(
        "ocentra-child-runtime-tampered-recovery-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&directory);
    std::fs::create_dir_all(&directory)?;
    let action = storage_custody_action_planned_event(storage_custody_decision_recorded_event(
        StorageCustodyAggregateId::parse("child-runtime-tampered-family")?,
        StorageCustodyDecisionId::parse("child-runtime-tampered-decision")?,
        StorageCustodyInput {
            location: StorageCustodyLocation::ParentDeviceLocal,
            retention_window_state: RetentionWindowState::Expired,
            parent_export_state: ParentExportState::NotRequested,
            remote_sync_state: RemoteSyncState::Disabled,
        },
    ));
    let journal_path = directory.join("journal.ndjson");
    let journal =
        NdjsonEventJournal::with_options(&journal_path, NdjsonJournalOptions::hash_chain());
    let store = RetentionDeleteTombstoneStore::open(&directory)?;
    let envelope =
        EventEnvelope::from_event(action.clone(), retention_delete_metadata()?)?.store()?;
    journal.inject_next_sync_failure_for_debug();
    assert!(
        runtime_gate_tombstone::persist_child_runtime_tombstone_action(
            &journal, &store, &envelope, &action,
        )
        .await
        .is_err()
    );

    let tombstone_path = directory.join("retention-delete-tombstones.json");
    let encoded = String::from_utf8(std::fs::read(&tombstone_path)?)?.replace(
        "\"aggregate_id\":\"child-runtime-tampered-family\"",
        "\"aggregate_id\":\"tampered-family\"",
    );
    std::fs::write(&tombstone_path, encoded.as_bytes())?;

    let restarted = ChildRuntimeTombstoneEventFlow::new(
        NdjsonEventJournal::with_options(&journal_path, NdjsonJournalOptions::hash_chain()),
        RetentionDeleteTombstoneStore::open(&directory)?,
    );
    let error = restarted
        .recover_pending()
        .await
        .expect_err_value("tampered durable identity must fail closed");
    assert_eq!(error.kind(), std::io::ErrorKind::InvalidData);
    let _ = std::fs::remove_dir_all(&directory);
    Ok(())
}

#[tokio::test]
async fn child_runtime_rejects_a_journal_envelope_for_a_different_custody_action(
) -> Result<(), Box<dyn std::error::Error>> {
    let directory = std::env::temp_dir().join(format!(
        "ocentra-child-runtime-tombstone-mismatch-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&directory);
    std::fs::create_dir_all(&directory)?;
    let delete_action =
        storage_custody_action_planned_event(storage_custody_decision_recorded_event(
            StorageCustodyAggregateId::parse("child-runtime-mismatch-family")?,
            StorageCustodyDecisionId::parse("child-runtime-mismatch-delete")?,
            StorageCustodyInput {
                location: StorageCustodyLocation::ParentDeviceLocal,
                retention_window_state: RetentionWindowState::Expired,
                parent_export_state: ParentExportState::NotRequested,
                remote_sync_state: RemoteSyncState::Disabled,
            },
        ));
    let different_action =
        storage_custody_action_planned_event(storage_custody_decision_recorded_event(
            StorageCustodyAggregateId::parse("child-runtime-mismatch-family")?,
            StorageCustodyDecisionId::parse("child-runtime-mismatch-retain")?,
            StorageCustodyInput {
                location: StorageCustodyLocation::ParentDeviceLocal,
                retention_window_state: RetentionWindowState::Active,
                parent_export_state: ParentExportState::NotRequested,
                remote_sync_state: RemoteSyncState::Disabled,
            },
        ));
    let journal = NdjsonEventJournal::with_options(
        directory.join("retention-delete.ndjson"),
        NdjsonJournalOptions::hash_chain(),
    );
    let store = RetentionDeleteTombstoneStore::open(&directory)?;
    let envelope =
        EventEnvelope::from_event(different_action, retention_delete_metadata()?)?.store()?;

    let error = match runtime_gate_tombstone::persist_child_runtime_tombstone_action(
        &journal,
        &store,
        &envelope,
        &delete_action,
    )
    .await
    {
        Err(error) => error,
        Ok(_) => return Err("mismatched custody action unexpectedly persisted".into()),
    };
    assert_eq!(error.kind(), std::io::ErrorKind::InvalidInput);
    assert!(store.records()?.is_empty());
    let _ = std::fs::remove_dir_all(&directory);
    Ok(())
}

#[tokio::test]
async fn child_runtime_rejects_a_journal_envelope_with_a_different_idempotency_identity(
) -> Result<(), Box<dyn std::error::Error>> {
    let directory = std::env::temp_dir().join(format!(
        "ocentra-child-runtime-tombstone-envelope-identity-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&directory);
    std::fs::create_dir_all(&directory)?;
    let action = storage_custody_action_planned_event(storage_custody_decision_recorded_event(
        StorageCustodyAggregateId::parse("child-runtime-envelope-identity-family")?,
        StorageCustodyDecisionId::parse("child-runtime-envelope-identity-decision")?,
        StorageCustodyInput {
            location: StorageCustodyLocation::ParentDeviceLocal,
            retention_window_state: RetentionWindowState::Expired,
            parent_export_state: ParentExportState::NotRequested,
            remote_sync_state: RemoteSyncState::Disabled,
        },
    ));
    let journal = NdjsonEventJournal::with_options(
        directory.join("retention-delete.ndjson"),
        NdjsonJournalOptions::hash_chain(),
    );
    let store = RetentionDeleteTombstoneStore::open(&directory)?;
    let mut envelope =
        EventEnvelope::from_event(action.clone(), retention_delete_metadata()?)?.store()?;
    envelope.idempotency_key = IdempotencyKey::parse("storage-custody.action.planned:forged")?;

    let error = match runtime_gate_tombstone::persist_child_runtime_tombstone_action(
        &journal, &store, &envelope, &action,
    )
    .await
    {
        Err(error) => error,
        Ok(_) => return Err("forged custody envelope unexpectedly persisted".into()),
    };
    assert_eq!(error.kind(), std::io::ErrorKind::InvalidInput);
    assert!(store.records()?.is_empty());
    let _ = std::fs::remove_dir_all(&directory);
    Ok(())
}

#[tokio::test]
async fn child_runtime_custody_event_flow_proves_correlated_outbox_and_journal_milestones(
) -> Result<(), Box<dyn std::error::Error>> {
    let directory = std::env::temp_dir().join(format!(
        "ocentra-child-runtime-custody-flow-success-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&directory);
    std::fs::create_dir_all(&directory)?;
    let journal = NdjsonEventJournal::with_options(
        directory.join("retention-delete.ndjson"),
        NdjsonJournalOptions::hash_chain(),
    );
    let flow = ChildRuntimeTombstoneEventFlow::new(
        journal,
        RetentionDeleteTombstoneStore::open(&directory)?,
    );
    let action = expired_retention_delete_action("child-runtime-flow-success")?;
    let metadata = retention_delete_metadata()?;
    let correlation_id = metadata.correlation_id.clone();

    let outcome = flow.publish_action(action, metadata).await?;

    let ChildRuntimeTombstonePublicationOutcome::Journaled(report) = outcome else {
        return Err("expected journaled custody event flow outcome".into());
    };
    assert_eq!(report.correlation_id, correlation_id);
    assert_eq!(
        report.milestones,
        vec![
            ChildRuntimeTombstoneMilestone::DurableOutboxWritten,
            ChildRuntimeTombstoneMilestone::JournalAppendConfirmed,
        ]
    );
    assert_eq!(report.append.map(|append| append.sequence), Some(1));
    let strict_append = flow
        .publish_action_and_require_journal(
            expired_retention_delete_action("child-runtime-flow-strict")?,
            retention_delete_metadata()?,
        )
        .await?;
    assert_eq!(strict_append.sequence, 2);
    let _ = std::fs::remove_dir_all(&directory);
    Ok(())
}

#[tokio::test]
async fn child_runtime_custody_event_flow_keeps_correlated_pending_retry_evidence_after_journal_failure(
) -> Result<(), Box<dyn std::error::Error>> {
    let directory = std::env::temp_dir().join(format!(
        "ocentra-child-runtime-custody-flow-retry-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&directory);
    std::fs::create_dir_all(&directory)?;
    let journal = NdjsonEventJournal::with_options(
        directory.join("retention-delete.ndjson"),
        NdjsonJournalOptions::hash_chain(),
    );
    journal.inject_next_sync_failure_for_debug();
    let flow = ChildRuntimeTombstoneEventFlow::new(
        journal,
        RetentionDeleteTombstoneStore::open(&directory)?,
    );
    let action = expired_retention_delete_action("child-runtime-flow-retry")?;
    let metadata = retention_delete_metadata()?;
    let correlation_id = metadata.correlation_id.clone();

    let outcome = flow.publish_action(action, metadata).await?;

    let ChildRuntimeTombstonePublicationOutcome::PendingJournalRetry(report) = outcome else {
        return Err("expected pending retry custody event flow outcome".into());
    };
    assert_eq!(report.correlation_id, correlation_id);
    assert_eq!(
        report.milestones,
        vec![
            ChildRuntimeTombstoneMilestone::DurableOutboxWritten,
            ChildRuntimeTombstoneMilestone::JournalAppendPendingRetry,
        ]
    );
    assert_eq!(report.append, None);
    let records = RetentionDeleteTombstoneStore::open(&directory)?.records()?;
    assert_eq!(records.len(), 1);
    assert!(records[0].terminal_pending);
    let _ = std::fs::remove_dir_all(&directory);
    Ok(())
}

fn expired_retention_delete_action(
    decision_id: &str,
) -> Result<
    ocentra_storage_custody_core::storage_custody::StorageCustodyActionPlannedEvent,
    Box<dyn std::error::Error>,
> {
    Ok(storage_custody_action_planned_event(
        storage_custody_decision_recorded_event(
            StorageCustodyAggregateId::parse("child-runtime-custody-flow-family")?,
            StorageCustodyDecisionId::parse(decision_id)?,
            StorageCustodyInput {
                location: StorageCustodyLocation::ParentDeviceLocal,
                retention_window_state: RetentionWindowState::Expired,
                parent_export_state: ParentExportState::NotRequested,
                remote_sync_state: RemoteSyncState::Disabled,
            },
        ),
    ))
}

fn retention_delete_metadata() -> Result<EventMetadata, Box<dyn std::error::Error>> {
    Ok(EventMetadata::new(
        CorrelationId::parse("child-runtime-retention-delete-correlation")?,
        EventSource::new(
            EventCustody::parse("local-journal")?,
            RuntimeRole::parse("controller")?,
            SourceService::parse("child-runtime")?,
            SourceComponent::parse("retention-delete-runtime")?,
            RuntimeInstanceId::parse("child-runtime-test-instance")?,
        ),
    ))
}
