use ocentra_parent_runtime_core::data_custody_backup_runtime::ParentBackupRuntime;
use ocentra_parent_runtime_core::data_custody_runtime_eventing::DataCustodyRuntimeEventJournal;

#[tokio::test]
async fn backup_runtime_recovers_a_real_empty_journal_without_synthetic_jobs_or_schedules() {
    let path = journal_path("backup-empty");
    let journal = DataCustodyRuntimeEventJournal::new(
        path.clone(),
        format!("wp05-backup-{}", std::process::id()),
    )
    .expect("data-custody journal identity is valid");
    assert!(journal.is_production_durable());

    let mut runtime = ParentBackupRuntime::new(journal);
    assert_eq!(runtime.jobs().count(), 0);
    assert_eq!(runtime.schedules().count(), 0);

    runtime
        .recover()
        .await
        .expect("an empty production journal recovers successfully");
    assert_eq!(runtime.jobs().count(), 0);
    assert_eq!(runtime.schedules().count(), 0);

    let _ = std::fs::remove_file(path);
}

#[tokio::test]
async fn backup_runtime_recovery_is_repeatable_without_claiming_provider_execution() {
    let path = journal_path("backup-repeat");
    let journal = DataCustodyRuntimeEventJournal::new(
        path.clone(),
        format!("wp05-backup-repeat-{}", std::process::id()),
    )
    .expect("data-custody journal identity is valid");
    let mut runtime = ParentBackupRuntime::new(journal);

    runtime
        .recover()
        .await
        .expect("first recovery validates the durable journal");
    runtime
        .recover()
        .await
        .expect("second recovery rebuilds the same empty durable state");

    assert_eq!(runtime.jobs().count(), 0);
    assert_eq!(runtime.schedules().count(), 0);
    let _ = std::fs::remove_file(path);
}

fn journal_path(label: &str) -> std::path::PathBuf {
    std::env::temp_dir().join(format!(
        "ocentra-data-custody-wp05-{label}-{}.ndjson",
        std::process::id()
    ))
}
