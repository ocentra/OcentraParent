use super::*;

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
