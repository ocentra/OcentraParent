use ocentra_parent_runtime_core::data_custody_restore_runtime::ParentRestoreRuntime;
use ocentra_parent_runtime_core::data_custody_runtime_eventing::DataCustodyRuntimeEventJournal;
use ocentra_schema::export_import_backup_recovery as contracts;

#[tokio::test]
async fn restore_runtime_recovers_empty_durable_state_without_a_restore_receipt() {
    let path = journal_path("restore-empty");
    let journal = DataCustodyRuntimeEventJournal::new(
        path.clone(),
        format!("wp05-restore-{}", std::process::id()),
    )
    .expect("data-custody journal identity is valid");
    let operation_ref = contracts::ExportImportOperationRef::parse("restore-operation-wp05")
        .expect("operation reference is non-empty");
    let mut runtime = ParentRestoreRuntime::new(journal);

    assert_eq!(runtime.pending_operation_count(), 0);
    assert!(runtime.restore_receipt(&operation_ref).is_none());

    runtime
        .recover()
        .await
        .expect("an empty production journal recovers successfully");
    assert_eq!(runtime.pending_operation_count(), 0);
    assert!(runtime.restore_receipt(&operation_ref).is_none());

    let _ = std::fs::remove_file(path);
}

#[tokio::test]
async fn restore_runtime_recovery_does_not_invent_pending_work_after_restart() {
    let path = journal_path("restore-restart");
    let journal = DataCustodyRuntimeEventJournal::new(
        path.clone(),
        format!("wp05-restore-restart-{}", std::process::id()),
    )
    .expect("data-custody journal identity is valid");
    let mut runtime = ParentRestoreRuntime::new(journal);

    runtime
        .recover()
        .await
        .expect("first recovery validates the durable journal");
    runtime
        .recover()
        .await
        .expect("restart recovery replays only durable records");

    assert_eq!(runtime.pending_operation_count(), 0);
    let operation_ref = contracts::ExportImportOperationRef::parse("restore-restart-wp05")
        .expect("operation reference is non-empty");
    assert!(runtime.restore_receipt(&operation_ref).is_none());
    let _ = std::fs::remove_file(path);
}

fn journal_path(label: &str) -> std::path::PathBuf {
    std::env::temp_dir().join(format!(
        "ocentra-data-custody-wp05-{label}-{}.ndjson",
        std::process::id()
    ))
}
