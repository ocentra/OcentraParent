use ocentra_eventing::error::EventingError;
use ocentra_parent_runtime_core::data_custody_runtime_eventing::DataCustodyRuntimeEventJournal;

#[tokio::test]
async fn malformed_data_custody_journal_row_is_rejected_during_recovery() {
    let path = journal_path();
    std::fs::write(&path, b"{malformed durable row}\n").expect("malformed journal fixture writes");
    let journal = DataCustodyRuntimeEventJournal::new(
        path.clone(),
        format!("wp05-eventing-malformed-{}", std::process::id()),
    )
    .expect("data-custody journal identity is valid");

    let error = journal
        .recover()
        .await
        .expect_err("malformed durable journal row must reject recovery");
    assert!(matches!(
        error,
        EventingError::JournalCorruptLine { line: 1, .. }
    ));
    let _ = std::fs::remove_file(path);
}

fn journal_path() -> std::path::PathBuf {
    std::env::temp_dir().join(format!(
        "ocentra-data-custody-wp05-eventing-malformed-{}.ndjson",
        std::process::id()
    ))
}
