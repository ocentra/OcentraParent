use ocentra_parent_runtime_core::data_custody_runtime_eventing::DataCustodyRuntimeEventJournal;

#[tokio::test]
async fn production_data_custody_journal_recovery_is_durable_and_empty_without_owner_events() {
    let path = journal_path();
    let first = DataCustodyRuntimeEventJournal::new(
        path.clone(),
        format!("wp05-integration-first-{}", std::process::id()),
    )
    .expect("data-custody journal identity is valid");
    assert!(first.is_production_durable());
    first
        .recover()
        .await
        .expect("first production journal recovery succeeds");
    let first_report = first
        .replay()
        .await
        .expect("first production journal replay succeeds");
    assert_eq!(first_report.skipped_count, 0);
    assert!(first_report.records.is_empty());

    let second = DataCustodyRuntimeEventJournal::new(
        path.clone(),
        format!("wp05-integration-second-{}", std::process::id()),
    )
    .expect("second data-custody journal identity is valid");
    second
        .recover()
        .await
        .expect("restart recovery validates the same durable file");
    let second_report = second.replay().await.expect("restart replay succeeds");
    assert_eq!(second_report.skipped_count, 0);
    assert!(second_report.records.is_empty());

    let _ = std::fs::remove_file(path);
}

fn journal_path() -> std::path::PathBuf {
    std::env::temp_dir().join(format!(
        "ocentra-data-custody-wp05-integration-{}.ndjson",
        std::process::id()
    ))
}
