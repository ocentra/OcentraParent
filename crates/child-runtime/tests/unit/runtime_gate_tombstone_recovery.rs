use std::{
    fs,
    path::{Path, PathBuf},
};

use ocentra_child_runtime::service::{
    ChildAgentReadiness, ChildAgentService, ChildAgentServiceError, ChildAgentServicePaths,
};
use ocentra_eventing::{
    envelope::{EventEnvelope, EventMetadata, EventSource, StoredEventEnvelope},
    ids::{
        CorrelationId, EventCustody, RuntimeInstanceId, RuntimeRole, SourceComponent, SourceService,
    },
    journal::ndjson::{NdjsonJournalEntry, NdjsonJournalRecord},
};
use ocentra_storage_custody_core::storage_custody::{
    storage_custody_action_planned_event, storage_custody_decision_recorded_event,
    ParentExportState, RemoteSyncState, RetentionWindowState, StorageCustodyActionPlannedEvent,
    StorageCustodyAggregateId, StorageCustodyDecisionId, StorageCustodyEffect,
    StorageCustodyExecutionRequest, StorageCustodyInput, StorageCustodyLocation,
};
use serde_json::{json, Value};

mod runtime_gate_tombstone_recovery_assertions;

use runtime_gate_tombstone_recovery_assertions::require_manual_required_custody_rejection;

// Positive custody dispatch, terminal acknowledgement, and publication
// milestone/retry reporting remain owner-composition coverage. This external
// suite deliberately stops at the public service recovery boundary until a
// trusted owner can supply the opaque custody authority handle.

#[tokio::test]
async fn startup_recovery_replays_a_valid_tombstone_and_is_idempotent_across_restart(
) -> Result<(), Box<dyn std::error::Error>> {
    let root = unique_root("valid-restart");
    clean_root(&root);
    let (action, envelope) = action_and_envelope("valid-restart-decision")?;
    write_tombstone_fixture(&root, typed_tombstone(&action, &envelope))?;
    let paths = paths_for(&root);

    let first = ChildAgentService::initialize_with_paths(paths.clone()).await?;
    assert_eq!(
        first.readiness()?,
        ChildAgentReadiness::TrustBindingManualRequired
    );
    drop(first);

    let first_entries = read_journal_entries(paths.journal())?;
    assert_eq!(first_entries.len(), 1);
    assert_eq!(first_entries[0].envelope, envelope);
    assert_eq!(
        read_tombstone_records(paths.tombstones())?[0]["terminal_pending"],
        json!(true)
    );
    let first_journal = fs::read_to_string(paths.journal())?;

    let second = ChildAgentService::initialize_with_paths(paths.clone()).await?;
    assert_eq!(
        second.readiness()?,
        ChildAgentReadiness::TrustBindingManualRequired
    );
    drop(second);

    let second_journal = fs::read_to_string(paths.journal())?;
    assert_eq!(second_journal, first_journal);
    let second_entries = read_journal_entries(paths.journal())?;
    assert_eq!(second_entries, first_entries);

    clean_root(&root);
    Ok(())
}

#[tokio::test]
async fn startup_recovery_skips_a_completed_terminal_marker(
) -> Result<(), Box<dyn std::error::Error>> {
    let root = unique_root("terminal-marker");
    clean_root(&root);
    let (action, _) = action_and_envelope("terminal-marker-decision")?;
    write_tombstone_fixture(&root, terminal_marker(&action))?;
    let paths = paths_for(&root);

    let service = ChildAgentService::initialize_with_paths(paths.clone()).await?;
    assert_eq!(
        service.readiness()?,
        ChildAgentReadiness::TrustBindingManualRequired
    );
    drop(service);

    assert!(read_journal_entries(paths.journal())?.is_empty());
    assert_eq!(
        read_tombstone_records(paths.tombstones())?[0]["terminal_pending"],
        json!(false)
    );

    clean_root(&root);
    Ok(())
}

#[tokio::test]
async fn startup_recovery_rejects_corrupt_tombstone_json() -> Result<(), Box<dyn std::error::Error>>
{
    let root = unique_root("corrupt-json");
    clean_root(&root);
    let paths = paths_for(&root);
    fs::create_dir_all(paths.tombstones())?;
    fs::write(
        paths.tombstones().join("retention-delete-tombstones.json"),
        b"[{\"version\":2,",
    )?;

    let error = initialization_storage_error(&root).await?;
    assert_eq!(error.kind(), std::io::ErrorKind::InvalidData);

    clean_root(&root);
    Ok(())
}

#[tokio::test]
async fn startup_recovery_rejects_a_typed_record_marked_terminal(
) -> Result<(), Box<dyn std::error::Error>> {
    let root = unique_root("typed-terminal");
    clean_root(&root);
    let (action, envelope) = action_and_envelope("typed-terminal-decision")?;
    let mut record = typed_tombstone(&action, &envelope);
    record["terminal_pending"] = json!(false);
    write_tombstone_fixture(&root, record)?;

    let error = initialization_storage_error(&root).await?;
    assert_eq!(error.kind(), std::io::ErrorKind::InvalidData);
    assert_eq!(
        error.to_string(),
        "custody tombstone record has an invalid version, payload, or terminal state"
    );

    clean_root(&root);
    Ok(())
}

#[tokio::test]
async fn startup_recovery_rejects_a_pending_legacy_tombstone(
) -> Result<(), Box<dyn std::error::Error>> {
    let root = unique_root("legacy-pending");
    clean_root(&root);
    let (action, _) = action_and_envelope("legacy-pending-decision")?;
    write_tombstone_fixture(&root, legacy_tombstone(&action))?;

    let error = initialization_storage_error(&root).await?;
    assert_eq!(error.kind(), std::io::ErrorKind::InvalidData);
    assert_eq!(
        error.to_string(),
        "child-runtime tombstone recovery requires manual migration for a pending legacy tombstone"
    );

    clean_root(&root);
    Ok(())
}

#[tokio::test]
async fn startup_recovery_rejects_a_tampered_tombstone_identity(
) -> Result<(), Box<dyn std::error::Error>> {
    let root = unique_root("tampered-identity");
    clean_root(&root);
    let (action, envelope) = action_and_envelope("tampered-identity-decision")?;
    let mut record = typed_tombstone(&action, &envelope);
    record["deletion_ref"] = json!("storage-custody-delete:forged-decision");
    write_tombstone_fixture(&root, record)?;

    let error = initialization_storage_error(&root).await?;
    assert_eq!(error.kind(), std::io::ErrorKind::InvalidData);
    assert_eq!(
        error.to_string(),
        "child-runtime tombstone recovery identity does not match its typed custody action"
    );

    clean_root(&root);
    Ok(())
}

#[tokio::test]
async fn startup_recovery_rejects_an_action_envelope_mismatch(
) -> Result<(), Box<dyn std::error::Error>> {
    let root = unique_root("action-envelope-mismatch");
    clean_root(&root);
    let (action, envelope) = action_and_envelope("action-envelope-mismatch-decision")?;
    let mut record = typed_tombstone(&action, &envelope);
    record["action"]["aggregate_id"] = json!("child-runtime-custody-forged");
    write_tombstone_fixture(&root, record)?;

    let error = initialization_storage_error(&root).await?;
    assert_eq!(error.kind(), std::io::ErrorKind::InvalidData);
    assert_eq!(
        error.to_string(),
        "child-runtime tombstone recovery identity does not match its typed custody action"
    );

    clean_root(&root);
    Ok(())
}

#[tokio::test]
async fn startup_recovery_rejects_an_envelope_idempotency_mismatch(
) -> Result<(), Box<dyn std::error::Error>> {
    let root = unique_root("envelope-idempotency-mismatch");
    clean_root(&root);
    let (action, envelope) = action_and_envelope("envelope-idempotency-mismatch-decision")?;
    let mut record = typed_tombstone(&action, &envelope);
    record["envelope"]["idempotencyKey"] =
        json!("storage-custody.action.planned:forged-idempotency");
    write_tombstone_fixture(&root, record)?;

    let error = initialization_storage_error(&root).await?;
    assert_eq!(error.kind(), std::io::ErrorKind::InvalidData);
    assert!(error.to_string().contains("stored_event.idempotency_key"));

    clean_root(&root);
    Ok(())
}

#[tokio::test]
async fn public_storage_custody_ingress_fails_closed_without_owner_binding(
) -> Result<(), Box<dyn std::error::Error>> {
    let root = unique_root("public-ingress-manual");
    clean_root(&root);
    let paths = paths_for(&root);
    let service = ChildAgentService::initialize_with_paths(paths.clone()).await?;
    assert_eq!(
        service.readiness()?,
        ChildAgentReadiness::TrustBindingManualRequired
    );
    let ingress = service.ingress();
    let service_task = tokio::spawn(service.run_until_shutdown());

    let result = ingress
        .submit_storage_custody_action(
            StorageCustodyExecutionRequest {
                effect: StorageCustodyEffect::DeleteLocal {
                    relative_path: PathBuf::from("payload.bin"),
                },
            },
            retention_delete_metadata()?,
        )
        .await;
    require_manual_required_custody_rejection(result)?;

    service_task.abort();
    let _ = service_task.await;
    assert!(read_journal_entries(paths.journal())?.is_empty());
    clean_root(&root);
    Ok(())
}

fn unique_root(label: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "ocentra-child-runtime-tombstone-public-{label}-{}",
        std::process::id()
    ))
}

fn clean_root(root: &Path) {
    let _ = fs::remove_dir_all(root);
}

fn paths_for(root: &Path) -> ChildAgentServicePaths {
    ChildAgentServicePaths::from_root(root.to_path_buf())
}

fn write_tombstone_fixture(root: &Path, record: Value) -> Result<(), Box<dyn std::error::Error>> {
    let paths = paths_for(root);
    fs::create_dir_all(paths.tombstones())?;
    fs::write(
        paths.tombstones().join("retention-delete-tombstones.json"),
        serde_json::to_vec(&vec![record])?,
    )?;
    Ok(())
}

fn read_tombstone_records(
    tombstone_directory: &Path,
) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
    let bytes = fs::read(tombstone_directory.join("retention-delete-tombstones.json"))?;
    Ok(serde_json::from_slice(&bytes)?)
}

fn typed_tombstone(
    action: &StorageCustodyActionPlannedEvent,
    envelope: &StoredEventEnvelope,
) -> Value {
    json!({
        "version": 2,
        "deletion_ref": format!("storage-custody-delete:{}", action.source_decision_id.as_str()),
        "proof_ref": action.action_plan_id.as_str(),
        "action": action,
        "envelope": envelope,
        "terminal_pending": true,
    })
}

fn legacy_tombstone(action: &StorageCustodyActionPlannedEvent) -> Value {
    json!({
        "version": 1,
        "deletion_ref": format!("storage-custody-delete:{}", action.source_decision_id.as_str()),
        "proof_ref": action.action_plan_id.as_str(),
        "terminal_pending": true,
    })
}

fn terminal_marker(action: &StorageCustodyActionPlannedEvent) -> Value {
    json!({
        "version": 3,
        "deletion_ref": format!("storage-custody-delete:{}", action.source_decision_id.as_str()),
        "proof_ref": action.action_plan_id.as_str(),
        "terminal_pending": false,
    })
}

fn action_and_envelope(
    decision_id: &str,
) -> Result<(StorageCustodyActionPlannedEvent, StoredEventEnvelope), Box<dyn std::error::Error>> {
    let action = expired_retention_delete_action(decision_id)?;
    let envelope =
        EventEnvelope::from_event(action.clone(), retention_delete_metadata()?)?.store()?;
    Ok((action, envelope))
}

async fn initialization_storage_error(
    root: &Path,
) -> Result<std::io::Error, Box<dyn std::error::Error>> {
    match ChildAgentService::initialize_with_paths(paths_for(root)).await {
        Err(ChildAgentServiceError::Storage(error)) => Ok(error),
        Err(error) => Err(format!("expected durable storage failure, received {error:?}").into()),
        Ok(_) => Err("valid service unexpectedly initialized from invalid fixture".into()),
    }
}

fn read_journal_entries(
    path: &Path,
) -> Result<Vec<NdjsonJournalEntry>, Box<dyn std::error::Error>> {
    let contents = match fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(error) => return Err(error.into()),
    };
    let records = contents
        .lines()
        .enumerate()
        .filter(|(_, line)| !line.trim().is_empty())
        .map(|(index, line)| {
            NdjsonJournalRecord::parse(line, index + 1).map(NdjsonJournalRecord::entry)
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(records.into_iter().flatten().collect())
}

fn expired_retention_delete_action(
    decision_id: &str,
) -> Result<StorageCustodyActionPlannedEvent, Box<dyn std::error::Error>> {
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
