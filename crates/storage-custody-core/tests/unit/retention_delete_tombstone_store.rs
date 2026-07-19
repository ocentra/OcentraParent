use ocentra_storage_custody_core::retention_delete_tombstone_store::RetentionDeleteTombstoneStore;

#[test]
fn tombstone_outbox_recovers_intent_until_terminal_publish() {
    let directory = std::env::temp_dir().join(format!("ocentra-tombstone-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&directory);
    let store = RetentionDeleteTombstoneStore::open(&directory).expect("open");
    store
        .persist_intent("delete:one".to_string(), "proof:one".to_string())
        .expect("intent");
    let reopened = RetentionDeleteTombstoneStore::open(&directory).expect("reopen");
    assert_eq!(reopened.records().expect("records").len(), 1);
    reopened
        .mark_terminal_published("delete:one")
        .expect("terminal");
    assert_eq!(reopened.records().expect("empty").len(), 0);
    let _ = std::fs::remove_dir_all(&directory);
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
fn tombstone_outbox_serializes_concurrent_intents() {
    let directory = std::env::temp_dir().join(format!("ocentra-tombstone-race-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&directory);
    let mut workers = Vec::new();
    for index in 0..8 {
        let directory = directory.clone();
        workers.push(std::thread::spawn(move || {
            RetentionDeleteTombstoneStore::open(directory)
                .and_then(|store| store.persist_intent(format!("delete:{index}"), format!("proof:{index}")))
        }));
    }
    for worker in workers { worker.join().expect("join").expect("intent"); }
    let count = RetentionDeleteTombstoneStore::open(&directory).expect("open").records().expect("records").len();
    assert_eq!(count, 8);
    let _ = std::fs::remove_dir_all(&directory);
}
