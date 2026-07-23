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
    let directory =
        std::env::temp_dir().join(format!("ocentra-tombstone-race-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&directory);
    let mut workers = Vec::new();
    for index in 0..8 {
        let directory = directory.clone();
        workers.push(std::thread::spawn(move || {
            RetentionDeleteTombstoneStore::open(directory).and_then(|store| {
                store.persist_intent(format!("delete:{index}"), format!("proof:{index}"))
            })
        }));
    }
    for worker in workers {
        worker.join().expect("join").expect("intent");
    }
    let count = RetentionDeleteTombstoneStore::open(&directory)
        .expect("open")
        .records()
        .expect("records")
        .len();
    assert_eq!(count, 8);
    let _ = std::fs::remove_dir_all(&directory);
}

#[test]
fn tombstone_outbox_atomic_replacements_survive_reopen() {
    let directory =
        std::env::temp_dir().join(format!("ocentra-tombstone-replace-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&directory);
    let store = RetentionDeleteTombstoneStore::open(&directory).expect("open");
    store
        .persist_intent("delete:first".to_string(), "proof:first".to_string())
        .expect("first intent");
    store
        .persist_intent("delete:second".to_string(), "proof:second".to_string())
        .expect("second intent");
    store
        .mark_terminal_published("delete:first")
        .expect("first terminal");

    let records = RetentionDeleteTombstoneStore::open(&directory)
        .expect("reopen")
        .records()
        .expect("records");
    assert_eq!(records.len(), 1);
    assert_eq!(records[0].deletion_ref, "delete:second");
    assert!(records[0].terminal_pending);
    let _ = std::fs::remove_dir_all(&directory);
}
